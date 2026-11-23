//! Using what a slot holds, or an ability clicked on the Abilities page:
//! queued by hand (`manual.rs`), asked for at once, eaten, drunk, or a panel
//! opened or closed.

use bevy::prelude::*;
use shared::ability::{Ability, BarThing};
use shared::protocol::ClientMessage;

use super::ferment::Fermenting;
use super::item_look::{can_drink, can_eat};
use super::manual::Manual;
use super::own_bar::OwnBar;
use super::peace::Peace;
use super::press_juice::JuiceChooser;
use super::press_oil::PressChooser;
use super::use_ability::{UseListed, UseSlot};
use crate::alchemy::OpenAlchemy;
use crate::character::CharacterSheet;
use crate::common::{BenchHere, OpenPanels, ServerTick};
use crate::connection::ToServer;
use crate::cooking::OpenCooking;
use crate::inventory::{Carried, DrinkPotion, EatKind};

/// A slot used, or an ability on the Abilities page clicked: its ability
/// queued, or taken out of the queue; a toggle (Sneak) asked for at once, as
/// it needs no target and takes no time; one of a kind of item eaten
/// (`inventory::EatKind`), a potion drunk (`DrinkPotion`); Press Oil's, Press
/// Juice's or Ferment's panel opened or closed, or the Alchemy or Cooking
/// panel (`alchemy::OpenAlchemy`, `cooking::OpenCooking`), which can be closed
/// so even when the ability can't be used now.
#[allow(clippy::too_many_arguments)]
pub(super) fn use_slots(
    (mut used, mut listed): (MessageReader<UseSlot>, MessageReader<UseListed>),
    bar: Res<OwnBar>,
    (carried, sheet, here, peace, clock, time): Standing,
    mut manual: ResMut<Manual>,
    mut to_server: MessageWriter<ToServer>,
    (mut eat, mut drink): (MessageWriter<EatKind>, MessageWriter<DrinkPotion>),
    (mut press, mut juice, mut fermenting, mut alchemy, mut cooking): Panels,
    open: Res<OpenPanels>,
) {
    let slots = used.read().filter_map(|&UseSlot(slot)| {
        let held = bar.0.slots.get(usize::from(slot)).copied().flatten()?;
        Some(held.thing)
    });
    let things: Vec<BarThing> = slots
        .chain(listed.read().map(|&UseListed(ability)| ability.into()))
        .collect();
    for thing in things {
        let ability = match thing {
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
            // Queued by hand like an ability (`manual.rs`), which checks it
            // can be cast when its turn comes.
            BarThing::Spell(_) => {
                manual.press(thing);
                continue;
            }
        };
        // An open panel closes, whatever has changed since.
        let can = ability.can_use_at(&carried.0, &sheet.0, here.tools()) && !peace.refuses(ability);
        if !open.is_open(ability) && !can {
            continue;
        }
        if ability == Ability::PressOil {
            // What to press is chosen first (`press_oil.rs`).
            press.open = !press.open;
        } else if ability == Ability::PressJuice {
            // And the fruit for Press Juice (`press_juice.rs`).
            juice.open = !juice.open;
        } else if ability == Ability::Alchemy {
            // Asked for by its panel (`alchemy`), never queued.
            alchemy.write(OpenAlchemy);
        } else if ability == Ability::Cook {
            // And Cook by the Cooking panel (`cooking`).
            cooking.write(OpenCooking);
        } else if ability == Ability::Magic {
            // Opens the spell design panel, which comes in magic plan PR 9.
            continue;
        } else if ability == Ability::Ferment {
            // And Ferment's in its own (`ferment/`).
            fermenting.open = !fermenting.open;
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

/// What the character carries and knows, the bench they are at, and the time.
type Standing<'w> = (
    Res<'w, Carried>,
    Res<'w, CharacterSheet>,
    Res<'w, BenchHere>,
    Res<'w, Peace>,
    Res<'w, ServerTick>,
    Res<'w, Time>,
);

/// The requesters and panels abilities open.
type Panels<'w> = (
    ResMut<'w, PressChooser>,
    ResMut<'w, JuiceChooser>,
    ResMut<'w, Fermenting>,
    MessageWriter<'w, OpenAlchemy>,
    MessageWriter<'w, OpenCooking>,
);
