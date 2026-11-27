//! Abilities and spells used by hand (a slot clicked, tapped or its key):
//! queued in the order they were pressed, never lost, and ahead of automatic ones, which wait
//! until the queue is done and the last has landed (owner, 2026-11-07).

use std::collections::VecDeque;

use bevy::prelude::*;
use shared::ability::{Ability, BarThing};
use shared::campfire::can_make_fire;
use shared::id::Id;
use shared::protocol::ClientMessage;
use shared::spot::Spot;

use super::approach::Reaching;
use super::peace::Peace;
use super::rain_here::WaterHere;
use super::spells::{ask_cast, can_cast, hand_aim, spell_of};
use super::use_ability::{ASK_WAIT, ask};
use super::woodcutting::{log_aim, tree_to_fell};
use crate::character::CharacterSheet;
use crate::common::{
    CurrentDungeon, FaceTowards, Facing, OwnAbility, OwnVitals, SpotPosition, Target,
};
use crate::connection::ToServer;
use crate::inventory::{Carried, GroundShown};
use crate::player::{Approach, Player};

/// The abilities the player chose by hand, from pressing them until the last
/// lands.
#[derive(Resource, Default, Debug, PartialEq)]
pub(super) struct Manual {
    /// Pressed, in order, each at most once, waiting their turn to be asked
    /// for. The bar marks them with a moving stripe.
    /// Only abilities and spells (`BarThing::Spell`) are.
    pub(super) queued: VecDeque<BarThing>,
    /// Asked for and not yet begun: the server's answer is awaited until
    /// `sent_until`.
    pub(super) sent: Option<BarThing>,
    pub(super) sent_until: f64,
    /// Begun: when it lands.
    pub(super) lands: f64,
    /// The target being walked into reach of (`approach.rs`) for the first
    /// queued: nothing is asked for meanwhile.
    pub(super) approaching: Option<Id>,
    /// Just come into reach: asked for next frame, once the last stretch
    /// walked has gone to the server.
    pub(super) settle: bool,
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
    pub(super) fn press(&mut self, thing: impl Into<BarThing>) {
        let thing = thing.into();
        match self.queued.iter().position(|&one| one == thing) {
            Some(index) => {
                self.queued.remove(index);
            }
            None => self.queued.push_back(thing),
        }
    }

    /// The server began an ability: if it was this one, it holds until it
    /// lands. A spell begins as the Magic ability.
    pub(super) fn begun(&mut self, ability: Ability, lands: f64) {
        let this = match self.sent {
            Some(BarThing::Spell(_)) => ability == Ability::Magic,
            sent => sent == Some(BarThing::Ability(ability)),
        };
        if this {
            self.sent = None;
            self.lands = lands;
        }
    }

    /// Moving takes the place of everything chosen.
    pub(super) fn forget(&mut self) {
        self.queued.clear();
        self.sent = None;
        self.approaching = None;
        self.settle = false;
    }
}

/// Where the player stands and which way they face.
type PlayerFacing<'w, 's> = Query<'w, 's, (&'static SpotPosition, &'static Facing), With<Player>>;

/// Asks for the first queued ability that can be done, at the target, as soon
/// as the player can act; those that can't be done when their turn comes are
/// passed over. An automatic one asked for and still waiting on the server is
/// taken back first, so this goes in its place. Fell Tree turns the player to
/// a tree in reach first if none is faced. Attack, Kick and spells at a
/// target out of reach walk into reach first (`player::Approach`), waiting at
/// the front meanwhile.
#[allow(clippy::too_many_arguments)]
pub(super) fn ask_wanted(
    (carried, sheet, ground): (Res<Carried>, Res<CharacterSheet>, Res<GroundShown>),
    target: Res<Target>,
    vitals: Res<OwnVitals>,
    time: Res<Time>,
    mut manual: ResMut<Manual>,
    mut own: ResMut<OwnAbility>,
    (mut to_server, mut face): (MessageWriter<ToServer>, MessageWriter<FaceTowards>),
    (dungeon, player, water): (Res<CurrentDungeon>, PlayerFacing, WaterHere),
    (peace, reaching, mut approach): (Res<Peace>, Reaching, MessageWriter<Approach>),
) {
    let now = time.elapsed_secs_f64();
    let settling = std::mem::take(&mut manual.settle);
    if manual.queued.is_empty() || manual.approaching.is_some() || settling {
        return;
    }
    let automatic_waiting = now < own.asked_until && !manual.is_sent(now);
    if !automatic_waiting && own.is_busy(now) {
        return;
    }
    let standing = player.single().ok().map(|(at, facing)| (at.0, facing.0));
    while let Some(thing) = manual.queued.pop_front() {
        let ability = match thing {
            BarThing::Ability(ability) => ability,
            BarThing::Spell(key) => {
                let Some(spell) = spell_of(key, &sheet.0)
                    .filter(|_| can_cast(key, &sheet.0, (&vitals, now), peace.0))
                else {
                    debug!("A spell can't be cast now");
                    continue;
                };
                if automatic_waiting {
                    to_server.write(ToServer(ClientMessage::WithdrawAbility));
                }
                let target = hand_aim(spell, target.0);
                let carrying = (&sheet.0, &carried.0);
                if let Some(going) = reaching.approach_for(thing, target, carrying) {
                    manual.queued.push_front(thing);
                    manual.approaching = Some(going.target);
                    approach.write(going);
                    return;
                }
                ask_cast(key, target, now, &mut own, &mut to_server);
                manual.sent = Some(thing);
                manual.sent_until = now + ASK_WAIT;
                return;
            }
            // Never queued (`use_slots.rs`).
            BarThing::Item(_) | BarThing::Potion(_) => continue,
        };
        let fits = ability.can_use(&carried.0, &sheet.0) && !peace.refuses(ability);
        let mut turn = None;
        let aimed = match ability {
            // Collect Water is only done facing water or in the rain, as the
            // server checks.
            Ability::CollectWater => standing
                .filter(|&(at, heading)| water.can_collect(at, heading))
                .map(|_| None),
            // A tree faced, or one in reach turned to.
            Ability::FellTree => standing
                .and_then(|(at, heading)| tree_to_fell(&dungeon.level, at, heading))
                .map(|turning| {
                    turn = turning;
                    None
                }),
            Ability::CutKindling => log_aim(&carried.0, &ground.0),
            // A fire faced to feed, or ground faced to light one on.
            Ability::MakeCampfire => standing
                .filter(|&(at, heading)| can_make_fire(&dungeon.level, at, heading))
                .map(|_| None),
            _ => aim(ability, target.0),
        };
        let Some(at) = aimed.filter(|_| fits) else {
            debug!("{} can't be done now", ability.name());
            continue;
        };
        if automatic_waiting {
            to_server.write(ToServer(ClientMessage::WithdrawAbility));
        }
        if let Some(going) = reaching.approach_for(thing, at, (&sheet.0, &carried.0)) {
            manual.queued.push_front(thing);
            manual.approaching = Some(going.target);
            approach.write(going);
            return;
        }
        if let Some((cell, heading)) = turn {
            to_server.write(ToServer(ClientMessage::Face(heading)));
            face.write(FaceTowards(Spot::middle_of(cell)));
        }
        ask(ability, at, now, &mut own, &mut to_server);
        manual.sent = Some(thing);
        manual.sent_until = now + ASK_WAIT;
        return;
    }
}

/// Who an ability chosen by hand is aimed at: `Some(None)` for the player
/// themselves, `None` if it can't be aimed. Kick at the target or a door; anything
/// else needs a target.
fn aim(ability: Ability, target: Option<Id>) -> Option<Option<Id>> {
    match ability {
        // At the target, or with none (or none in reach: the server's to say)
        // the closed door beside the player.
        Ability::Kick => Some(target),
        // Asked for by their panels (`press_oil.rs`, `press_juice.rs`,
        // `ferment/`, `alchemy`, `cooking`), never queued.
        Ability::PressOil
        | Ability::PressJuice
        | Ability::Alchemy
        | Ability::Ferment
        | Ability::Cook
        // Opens the design panel (magic plan PR 9), never sent.
        | Ability::Magic => None,
        // On no one: the water or tree faced, or the log chosen
        // (`ask_wanted`).
        Ability::CollectWater
        | Ability::FellTree
        | Ability::CutKindling
        | Ability::MakeCampfire => Some(None),
        _ => target.map(Some),
    }
}

#[cfg(test)]
mod tests;
