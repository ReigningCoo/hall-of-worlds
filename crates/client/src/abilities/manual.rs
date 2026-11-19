//! Abilities used by hand (a slot clicked, tapped or its key): queued in the
//! order they were pressed, never lost, and ahead of automatic ones, which wait
//! until the queue is done and the last has landed (owner, 2026-11-07).

use std::collections::VecDeque;

use bevy::prelude::*;
use shared::ability::{Ability, BarThing};
use shared::id::Id;
use shared::potion::water_faced;
use shared::protocol::ClientMessage;
use shared::sheet::Feeling;
use shared::spot::Spot;

use super::brew::Brewing;
use super::grind::Mixing;
use super::item_look::{can_drink, can_eat};
use super::own_bar::OwnBar;
use super::press_juice::JuiceChooser;
use super::press_oil::PressChooser;
use super::use_ability::{ASK_WAIT, UseSlot, ask};
use super::vitals_now::affords;
use super::woodcutting::{log_aim, tree_to_fell};
use crate::character::CharacterSheet;
use crate::common::{
    CurrentDungeon, FaceTowards, Facing, Other, OwnAbility, OwnVitals, ServerTick, SpotPosition,
    Target,
};
use crate::connection::ToServer;
use crate::inventory::{Carried, DrinkPotion, EatKind, GroundShown};
use crate::player::Player;

/// The abilities the player chose by hand, from pressing them until the last
/// lands.
#[derive(Resource, Default, Debug, PartialEq)]
pub(super) struct Manual {
    /// Pressed, in order, each at most once, waiting their turn to be asked
    /// for. The bar marks them with a moving stripe.
    pub(super) queued: VecDeque<Ability>,
    /// Asked for and not yet begun: the server's answer is awaited until
    /// `sent_until`.
    pub(super) sent: Option<Ability>,
    pub(super) sent_until: f64,
    /// Begun: when it lands.
    pub(super) lands: f64,
}

impl Manual {
    /// Whether automatic abilities must wait for these.
    pub(super) fn holds(&self, now: f64) -> bool {
        !self.queued.is_empty() || self.is_sent(now) || now < self.lands
    }

    /// Whether one was asked for and the server's answer is still awaited.
    fn is_sent(&self, now: f64) -> bool {
        self.sent.is_some() && now < self.sent_until
    }

    /// A press: queued at the back, or taken out if it already waits.
    pub(super) fn press(&mut self, ability: Ability) {
        match self.queued.iter().position(|&one| one == ability) {
            Some(index) => {
                self.queued.remove(index);
            }
            None => self.queued.push_back(ability),
        }
    }

    /// The server began an ability: if it was this one, it holds until it lands.
    pub(super) fn begun(&mut self, ability: Ability, lands: f64) {
        if self.sent == Some(ability) {
            self.sent = None;
            self.lands = lands;
        }
    }

    /// Moving takes the place of everything chosen.
    pub(super) fn forget(&mut self) {
        self.queued.clear();
        self.sent = None;
    }
}

/// A slot used: its ability queued, or taken out of the queue; a toggle
/// (Sneak) asked for at once, as it needs no target and takes no time; one of
/// a kind of item eaten (`inventory::EatKind`), a potion drunk
/// (`DrinkPotion`); Press Oil's requester, Grind's mixing panel or Brew's
/// panel opened.
pub(super) fn use_slots(
    mut used: MessageReader<UseSlot>,
    bar: Res<OwnBar>,
    (carried, sheet, clock, time): (
        Res<Carried>,
        Res<CharacterSheet>,
        Res<ServerTick>,
        Res<Time>,
    ),
    mut manual: ResMut<Manual>,
    mut to_server: MessageWriter<ToServer>,
    (mut eat, mut drink): (MessageWriter<EatKind>, MessageWriter<DrinkPotion>),
    (mut press, mut juice, mut mixing, mut brewing): (
        ResMut<PressChooser>,
        ResMut<JuiceChooser>,
        ResMut<Mixing>,
        ResMut<Brewing>,
    ),
) {
    for &UseSlot(slot) in used.read() {
        let Some(held) = bar.0.slots.get(usize::from(slot)).copied().flatten() else {
            continue;
        };
        let ability = match held.thing {
            BarThing::Ability(ability) => ability,
            // Eaten at once, as on the clothes panel: never queued.
            BarThing::Item(kind) => {
                let count = carried.0.count_of(kind);
                if can_eat(kind, count, carried.0.creature) {
                    eat.write(EatKind(kind));
                }
                continue;
            }
            // Drunk at once too, unless cooling down.
            BarThing::Potion(potion) => {
                let count = carried.0.count_of_potion(potion);
                let now = clock.now(time.elapsed_secs_f64());
                if can_drink(potion, count, &sheet.0, now) {
                    drink.write(DrinkPotion(potion));
                }
                continue;
            }
        };
        if !ability.can_use(&carried.0, &sheet.0) {
            continue;
        }
        if ability == Ability::PressOil {
            // What to press is chosen first (`press_oil.rs`).
            press.open = true;
        } else if ability == Ability::PressJuice {
            // And the fruit for Press Juice (`press_juice.rs`).
            juice.open = true;
        } else if ability == Ability::Grind {
            // What to grind is chosen in the mixing panel (`grind/`).
            mixing.open = true;
        } else if ability == Ability::Brew {
            // What to brew is chosen in its panel (`brew.rs`).
            brewing.open = true;
        } else if ability.is_toggle() {
            to_server.write(ToServer(ClientMessage::UseAbility {
                ability,
                target: None,
            }));
        } else {
            manual.press(ability);
        }
    }
}

/// Where the player stands and which way they face.
type PlayerFacing<'w, 's> = Query<'w, 's, (&'static SpotPosition, &'static Facing), With<Player>>;

/// Asks for the first queued ability that can be done, at the target, as soon
/// as the player can act; those that can't be done when their turn comes are
/// passed over. An automatic one asked for and still waiting on the server is
/// taken back first, so this goes in its place. Fell Tree turns the player to
/// a tree in reach first if none is faced.
#[allow(clippy::too_many_arguments)]
pub(super) fn ask_wanted(
    (carried, sheet, ground): (Res<Carried>, Res<CharacterSheet>, Res<GroundShown>),
    target: Res<Target>,
    others: Query<&Other>,
    vitals: Res<OwnVitals>,
    time: Res<Time>,
    mut manual: ResMut<Manual>,
    mut own: ResMut<OwnAbility>,
    (mut to_server, mut face): (MessageWriter<ToServer>, MessageWriter<FaceTowards>),
    (dungeon, player): (Res<CurrentDungeon>, PlayerFacing),
) {
    let now = time.elapsed_secs_f64();
    if manual.queued.is_empty() {
        return;
    }
    let automatic_waiting = now < own.asked_until && !manual.is_sent(now);
    if !automatic_waiting && own.is_busy(now) {
        return;
    }
    let standing = player.single().ok().map(|(at, facing)| (at.0, facing.0));
    while let Some(ability) = manual.queued.pop_front() {
        let fits = ability.can_use(&carried.0, &sheet.0) && affords(ability, &vitals, now);
        let mut turn = None;
        let aimed = match ability {
            // Collect Water is only done facing water, as the server checks.
            Ability::CollectWater => standing
                .filter(|&(at, heading)| water_faced(&dungeon.level, at, heading).is_some())
                .map(|_| None),
            // A tree faced, or one in reach turned to.
            Ability::FellTree => standing
                .and_then(|(at, heading)| tree_to_fell(&dungeon.level, at, heading))
                .map(|turning| {
                    turn = turning;
                    None
                }),
            Ability::CutKindling => log_aim(&carried.0, &ground.0),
            _ => aim(ability, target.0, &others),
        };
        let Some(at) = aimed.filter(|_| fits) else {
            debug!("{} can't be done now", ability.name());
            continue;
        };
        if automatic_waiting {
            to_server.write(ToServer(ClientMessage::WithdrawAbility));
        }
        if let Some((cell, heading)) = turn {
            to_server.write(ToServer(ClientMessage::Face(heading)));
            face.write(FaceTowards(Spot::middle_of(cell)));
        }
        ask(ability, at, now, &mut own, &mut to_server);
        manual.sent = Some(ability);
        manual.sent_until = now + ASK_WAIT;
        return;
    }
}

/// Who an ability chosen by hand is aimed at: `Some(None)` for the player
/// themselves, `None` if it can't be aimed. Heal goes to the target unless it
/// is an enemy, otherwise to the player; Kick at the target or a door; anything
/// else needs a target.
fn aim(ability: Ability, target: Option<Id>, others: &Query<&Other>) -> Option<Option<Id>> {
    match ability {
        // Never an enemy by hand: healing one is to come another way.
        Ability::Heal => Some(target.filter(|id| {
            others
                .iter()
                .any(|other| other.id == *id && other.feeling != Feeling::Unfriendly)
        })),
        // At the target, or with none (or none in reach: the server's to say)
        // the closed door beside the player.
        Ability::Kick => Some(target),
        // Asked for by their panels (`press_oil.rs`, `press_juice.rs`,
        // `grind/`, `brew.rs`), never queued.
        Ability::PressOil | Ability::PressJuice | Ability::Grind | Ability::Brew => None,
        // On no one: the water or tree faced, or the log chosen
        // (`ask_wanted`).
        Ability::CollectWater | Ability::FellTree | Ability::CutKindling => Some(None),
        _ => target.map(Some),
    }
}

#[cfg(test)]
mod tests;
