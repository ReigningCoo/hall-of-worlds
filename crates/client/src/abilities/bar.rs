//! The action bar: ten slots, five either side of the Health and Mana circle, each
//! with its key's short name over its bottom right. A slot whose ability is used
//! by itself has a sliding metal edge, as set items do; one queued to be used
//! has hazard stripes chasing round inside it. A kind of item shows its picture
//! and how many are carried, in bags too (owner, 2026-11-15).

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use shared::ability::{Ability, ActionBar, BAR_SLOTS, BarSlot, BarThing};
use shared::creature::CreatureKind;

use super::drag::Aim;
use super::item_look::thing_look;
use super::manual::Manual;
use super::own_bar::OwnBar;
use super::page::usable;
use super::vitals_now::affords;
use crate::character::CharacterSheet;
use crate::common::{Action, OwnVitals, Preferences, Sneaking, VitalsCircle, interface_area};
use crate::dom::{AbilitySlotShown, AbilityThing};
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
    /// Queued to be used, by hand.
    queued: Vec<Ability>,
    /// Sneak is on: its slot is edged.
    sneaking: bool,
    /// How many are carried of each slot's kind of item (0 for the rest).
    counts: Vec<u32>,
    /// Whether each slot's ability can be used now (`page::usable`; true for
    /// the rest): known, with its tool.
    usable: Vec<bool>,
}

/// Says what each slot shows again when that changes.
#[allow(clippy::too_many_arguments)]
pub(super) fn show_bar(
    bar: Res<OwnBar>,
    (carried, sheet): (Res<Carried>, Res<CharacterSheet>),
    preferences: Res<Preferences>,
    (aim, manual, sneaking): (Res<Aim>, Res<Manual>, Res<Sneaking>),
    (vitals, time): (Res<OwnVitals>, Res<Time>),
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
        queued: manual.queued.iter().copied().collect(),
        sneaking: sneaking.0,
        counts: bar
            .0
            .slots
            .iter()
            .map(|held| {
                let item = held.and_then(|held| held.thing.item());
                item.map_or(0, |kind| carried.0.count_of(kind))
            })
            .collect(),
        usable: (bar.0.slots.iter())
            .map(|held| {
                let ability = held.and_then(|held| held.ability());
                ability.is_none_or(|ability| usable(ability, &carried, &sheet))
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
            BarThing::Ability(ability) => (ability.name(), ability.description()),
            BarThing::Item(kind) => (kind.def().name, ""),
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
    let item = held.and_then(|held| held.thing.item());
    AbilitySlotShown {
        thing,
        key: short.clone(),
        auto: held.is_some_and(|held| held.auto),
        // A toggle that is on.
        on: ability.is_some_and(|ability| ability.is_toggle()) && now.sneaking,
        queued: ability.is_some_and(|ability| now.queued.contains(&ability)),
        // The wait to act again, over what is done when asked (eating too).
        shade: item.is_some() || ability.is_some_and(|ability| ability.is_active()),
        aim: now
            .aim
            .filter(|(aimed, _)| *aimed == slot)
            .map(|(_, allowed)| allowed),
        count: item.map(|_| count),
    }
}
