//! The action bar: ten slots, five either side of the Health and Mana circle, each
//! with its key's short name over its bottom right. A slot whose ability is used
//! by itself has a sliding metal edge, as set items do; one queued to be used
//! has hazard stripes chasing round inside it. A kind of item shows its picture
//! and how many are carried, in bags too (owner, 2026-11-15).

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use shared::ability::{Ability, ActionBar, BAR_SLOTS, BarSlot, BarThing};
use shared::creature::CreatureKind;
use shared::item::Inventory;

use super::drag::Aim;
use super::item_look::{can_drink, potion_words, thing_look};
use super::manual::Manual;
use super::marks::Marks;
use super::own_bar::OwnBar;
use super::page::usable;
use super::peace::Peace;
use super::vitals_now::affords;
use crate::character::CharacterSheet;
use crate::common::BenchHere;
use crate::common::{
    Action, OpenPanels, OwnVitals, Preferences, ServerTick, Sneaking, VitalsCircle, interface_area,
};
use crate::dom::{AbilityMarks, AbilitySlotShown, AbilityThing};
use crate::inventory::Carried;
use crate::widgets::{ABILITY, UiId};

/// Between slots, and between the bar and the circle.
pub(super) const GAP: f32 = 6.0;
/// How many slots stand on each side of the circle.
const PER_SIDE: usize = BAR_SLOTS / 2;

/// One side of the bar: 0 left of the circle, 1 right.
#[derive(Component)]
pub(super) struct BarSide(usize);

/// One slot, holding its place on the bar.
#[derive(Component, Clone, Copy)]
pub(super) struct SlotNode(pub u8);

/// The two sides and their slots, empty until `show_bar` fills them.
pub(super) fn spawn_bar(mut commands: Commands) {
    for side in 0..2 {
        let row = commands
            .spawn((
                BarSide(side),
                Node {
                    position_type: PositionType::Absolute,
                    column_gap: px(GAP),
                    ..default()
                },
                Visibility::Hidden,
            ))
            .id();
        commands.entity(row).insert(crate::dom::Opaque);
        commands.entity(row).with_children(|row| {
            for index in 0..PER_SIDE {
                let slot = (side * PER_SIDE + index) as u8;
                // A slot that draws itself (`dom`): `show_bar` says what it shows.
                let id = UiId::new("bar", "slot").at(slot as usize);
                row.spawn((SlotNode(slot), id, AbilitySlotShown::default()));
            }
        });
    }
}

/// Stands each side by the circle, hidden while it is.
pub(super) fn place_bar(
    circle: Res<VitalsCircle>,
    window: Single<&Window, With<PrimaryWindow>>,
    ui_scale: Res<UiScale>,
    mut sides: Query<(&BarSide, &mut Node, &mut Visibility)>,
) {
    let room = interface_area(&window, &ui_scale);
    let width = PER_SIDE as f32 * (ABILITY + GAP) - GAP;
    for (side, mut node, mut visibility) in &mut sides {
        if circle.across <= 0.0 {
            visibility.set_if_neq(Visibility::Hidden);
            continue;
        }
        visibility.set_if_neq(Visibility::Inherited);
        let half = circle.across / 2.0 + GAP;
        let left = match side.0 {
            0 => room.x / 2.0 - half - width,
            _ => room.x / 2.0 + half,
        };
        node.left = px(left.round());
        node.bottom = px(circle.bottom);
    }
}

/// What the slots last showed.
#[derive(PartialEq)]
pub(super) struct Shown {
    bar: ActionBar,
    kind: CreatureKind,
    keys: Vec<(String, String)>,
    aim: Option<(u8, bool)>,
    /// Too little Mana for what costs it (all cost the same so far: Heal's).
    low_mana: bool,
    /// How each ability is being used: automatic, on, queued.
    marks: Marks,
    /// How many are carried of each slot's kind of item or potion (0 for the
    /// rest).
    counts: Vec<u32>,
    /// Whether each slot's ability can be used now (`page::usable`): known,
    /// with its tool; a potion's, whether it can be drunk (`can_drink`); true
    /// for the rest.
    usable: Vec<bool>,
}

/// Says what each slot shows again when that changes.
#[allow(clippy::too_many_arguments)]
pub(super) fn show_bar(
    bar: Res<OwnBar>,
    (carried, sheet, here, peace): (
        Res<Carried>,
        Res<CharacterSheet>,
        Res<BenchHere>,
        Res<Peace>,
    ),
    preferences: Res<Preferences>,
    (aim, manual, sneaking, panels): (Res<Aim>, Res<Manual>, Res<Sneaking>, Res<OpenPanels>),
    (vitals, time, clock): (Res<OwnVitals>, Res<Time>, Res<ServerTick>),
    assets: Res<AssetServer>,
    mut slots: Query<(&SlotNode, &mut AbilitySlotShown)>,
    mut shown: Local<Option<Shown>>,
) {
    let keys = (0..BAR_SLOTS as u8)
        .map(|slot| {
            let [first, _] = preferences.controls.keys(Action::UseSlot(slot));
            first.map_or_else(Default::default, |key| (key.short_label(), key.label()))
        })
        .collect();
    let now = Shown {
        bar: bar.0.clone(),
        kind: carried.0.creature,
        keys,
        aim: aim.0,
        low_mana: !affords(Ability::Heal, &vitals, time.elapsed_secs_f64()),
        marks: Marks::now(&bar.0, &manual, *sneaking, &panels),
        counts: bar
            .0
            .slots
            .iter()
            .map(|held| held.map_or(0, |held| count_of(held.thing, &carried.0)))
            .collect(),
        usable: (bar.0.slots.iter())
            .map(|held| match held.map(|held| held.thing) {
                Some(BarThing::Ability(ability)) => {
                    usable(ability, &carried, &sheet, (&here, *peace))
                }
                Some(BarThing::Potion(potion)) => {
                    let count = carried.0.count_of_potion(potion);
                    let now = clock.now(time.elapsed_secs_f64());
                    can_drink(potion, count, &sheet.0, now)
                }
                _ => true,
            })
            .collect(),
    };
    if shown.as_ref() == Some(&now) {
        return;
    }
    for (&SlotNode(slot), mut slot_shown) in &mut slots {
        let slot_now = slot_of(slot, &now, &assets);
        // (Only when it differs: the slot draws itself again when changed.)
        if *slot_shown != slot_now {
            *slot_shown = slot_now;
        }
    }
    *shown = Some(now);
}

/// What one slot shows: its ability or kind of item, its key, and how it is
/// used.
fn slot_of(slot: u8, now: &Shown, assets: &AssetServer) -> AbilitySlotShown {
    let held = now.bar.slots[slot as usize];
    let (short, full) = &now.keys[slot as usize];
    let count = now.counts[slot as usize];
    let usable = now.usable[slot as usize];
    let thing = held.map(|held| {
        let mut look = thing_look(held.thing, count, now.kind, usable, assets);
        let (name, description) = match held.thing {
            BarThing::Ability(ability) => (ability.name(), ability.description().to_owned()),
            BarThing::Item(kind) => (kind.def().name, String::new()),
            BarThing::Potion(potion) => ("Potion", potion_words(potion)),
        };
        if !full.is_empty() {
            look.name = format!(
                "{name} ({full})
{description}"
            )
            .trim_end()
            .to_string();
        }
        // Greyed while there isn't the Mana for it.
        let ability = held.ability();
        look.usable &= !now.low_mana || ability.is_none_or(|ability| ability.mana_cost() == 0);
        AbilityThing {
            thing: held.thing,
            look,
        }
    });
    let ability = held.and_then(BarSlot::ability);
    let item = held.filter(|held| held.ability().is_none());
    AbilitySlotShown {
        thing,
        key: short.clone(),
        // As on the Abilities page (`marks.rs`).
        marks: ability.map_or(AbilityMarks::default(), |ability| now.marks.of(ability)),
        // The wait to act again, over what is done when asked (eating too).
        shade: item.is_some() || ability.is_some_and(|ability| ability.is_active()),
        aim: now
            .aim
            .filter(|(aimed, _)| *aimed == slot)
            .map(|(_, allowed)| allowed),
        count: item.map(|_| count),
    }
}

/// How many are carried of what a slot holds: a kind of item, or a potion; 0
/// for an ability.
pub(super) fn count_of(thing: BarThing, carried: &Inventory) -> u32 {
    match thing {
        BarThing::Ability(_) => 0,
        BarThing::Item(kind) => carried.count_of(kind),
        BarThing::Potion(potion) => carried.count_of_potion(potion),
    }
}
