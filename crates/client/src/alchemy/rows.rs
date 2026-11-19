//! Drawing the panel's two rows into their canvases: along the top, every
//! apparatus whose sub-skill the character knows (`ApparatusNode`); under
//! it, the process: the source's space, each step's box (`StepNode`, its
//! border red past the steps allowed) with its drop spaces under it
//! (`SpaceNode`), and a last box (`EndNode`) for a new step, where what the
//! last run made is shown. Drawn again whenever the process, what is carried,
//! the skills or a drag's aim changes.

use bevy::prelude::*;
use shared::alchemy::{AlchemySkills, Apparatus, steps_allowed};
use shared::item::Inventory;
use shared::skill::{SkillId, Skills};

use super::bench::Bench;
use super::boxes::{
    BOX, id, space_shown, spawn_box, spawn_insert_mark, spawn_picture, spawn_running,
};
use super::drag::{BenchAim, Dragging};
use super::spaces::{Place, SpaceIcons, in_place, places, tip};
use crate::character::CharacterSheet;
use crate::dom::Solid;
use crate::inventory::Carried;
use crate::widgets::HoverTip;

/// The top row's canvas.
#[derive(Component)]
pub(super) struct ApparatusRow;

/// The process row's canvas.
#[derive(Component)]
pub(super) struct ProcessRow;

/// An apparatus in the top row.
#[derive(Component, Clone, Copy)]
pub(super) struct ApparatusNode(pub(super) Apparatus);

/// A step's box in the process row.
#[derive(Component, Clone, Copy)]
pub(super) struct StepNode(pub(super) usize);

/// The box after the last step.
#[derive(Component)]
pub(super) struct EndNode;

/// A drop space.
#[derive(Component, Clone, Copy)]
pub(super) struct SpaceNode(pub(super) Place);

/// The apparatus whose sub-skills are known, in `Apparatus::ALL`'s order.
pub(super) fn known_apparatus(skills: &Skills) -> Vec<Apparatus> {
    let knows = |a: &Apparatus| SkillId::find(a.sub_skill()).is_some_and(|id| skills.knows(id));
    Apparatus::ALL.into_iter().filter(knows).collect()
}

/// A canvas just built.
type NewRow = Or<(Added<ApparatusRow>, Added<ProcessRow>)>;

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_rows(
    mut bench: ResMut<Bench>,
    (carried, sheet): (Res<Carried>, Res<CharacterSheet>),
    (aim, icons): (Res<BenchAim>, Res<SpaceIcons>),
    top: Query<Entity, With<ApparatusRow>>,
    process: Query<Entity, With<ProcessRow>>,
    added: Query<(), NewRow>,
    mut commands: Commands,
) {
    if carried.is_changed() {
        let carried = &carried.0;
        let gone = bench.stacks().any(|id| carried.find(id).is_none());
        if gone {
            bench.keep_carried(|id| carried.find(id).is_some());
        }
    }
    let changed = bench.is_changed() || carried.is_changed() || sheet.is_changed();
    if !(changed || aim.is_changed() || !added.is_empty()) {
        return;
    }
    let known = known_apparatus(&sheet.0.skills);
    for row in &top {
        commands.entity(row).despawn_related::<Children>();
        commands.entity(row).with_children(|row| {
            row.spawn(row_node()).with_children(|row| {
                for apparatus in &known {
                    let kind = apparatus.item();
                    let carried = kind.is_some_and(|kind| carried.0.count_of(kind) > 0);
                    let tip = match carried {
                        true => apparatus.name().to_owned(),
                        false => format!("{} (not carried)", apparatus.name()),
                    };
                    let marks = (ApparatusNode(*apparatus), id("kit", apparatus.index()));
                    spawn_box(row, (kind, false), tip, marks, |_| {});
                }
            });
        });
    }
    let allowed = steps_allowed(AlchemySkills::of(&sheet.0.skills).alchemy_percent()) as usize;
    for row in &process {
        commands.entity(row).despawn_related::<Children>();
        commands.entity(row).with_children(|row| {
            row.spawn(row_node()).with_children(|row| {
                draw_process(row, &bench, &carried.0, allowed, (&aim, &icons));
            });
        });
    }
}

/// The source, the steps with their spaces, and the end box.
fn draw_process(
    row: &mut ChildSpawnerCommands,
    bench: &Bench,
    carried: &Inventory,
    allowed: usize,
    (aim, icons): (&BenchAim, &SpaceIcons),
) {
    let space = |row: &mut ChildSpawnerCommands, place: Place, at: UiIdAt| {
        let stack = in_place(bench, place).and_then(|id| carried.find(id));
        let aimed = aim.space.filter(|(on, _)| *on == place).map(|(_, ok)| ok);
        let shown = space_shown(stack, icons.of(place), aimed);
        let tip = HoverTip(tip(place).to_owned());
        row.spawn((shown, SpaceNode(place), tip, Solid, id(at.0, at.1)));
    };
    row.spawn(column()).with_children(|column| {
        column.spawn(Node {
            height: px(BOX / 4.0),
            ..default()
        });
        space(column, Place::Source, ("source", 0));
    });
    let insert = aim.insert.filter(|_| aim.dragging.is_some());
    for (at, station) in bench.steps.iter().enumerate() {
        if insert == Some(at) {
            spawn_insert_mark(row);
        }
        let dragged = aim.dragging == Some(Dragging::Step(at));
        row.spawn(column()).with_children(|column| {
            let over = at >= allowed;
            let tip = match over {
                true => format!(
                    "{}: past the steps allowed, halving the chance",
                    station.apparatus.name()
                ),
                false => station.apparatus.name().to_owned(),
            };
            let kind = station.apparatus.item().filter(|_| !dragged);
            let marks = (StepNode(at), id("step", at));
            spawn_box(column, (kind, over), tip, marks, |inside| {
                spawn_running(inside, at)
            });
            column.spawn(spaces_node()).with_children(|spaces| {
                for place in places(at, station) {
                    space(spaces, place, place_id(place));
                }
            });
        });
    }
    if insert == Some(bench.steps.len()) {
        spawn_insert_mark(row);
    }
    let tip = "Drop an apparatus here".to_owned();
    spawn_box(row, (None, false), tip, (EndNode, id("end", 0)), |inside| {
        if let Some(made) = &bench.made {
            spawn_picture(inside, made);
        }
    });
}

/// A widget id's name and index.
type UiIdAt = (&'static str, usize);

/// A space's id: `alchemy/herb.<step × 100 + n>`, `oil.<step>`, `alcohol.<step>`,
/// `liquid.<step × 100 + n>`, `powder.<step>`, `source.0`.
fn place_id(place: Place) -> UiIdAt {
    match place {
        Place::Source => ("source", 0),
        Place::Herb(step, n) => ("herb", step * 100 + n),
        Place::Oil(step) => ("oil", step),
        Place::Alcohol(step) => ("alcohol", step),
        Place::Liquid(step, n) => ("liquid", step * 100 + n),
        Place::Powder(step) => ("powder", step),
    }
}

/// A row of boxes, 8 apart.
fn row_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        column_gap: px(8),
        min_height: px(BOX),
        ..default()
    }
}

/// A step: its box with its spaces under it.
fn column() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(4),
        ..default()
    }
}

/// A step's spaces, two to a row under its box.
fn spaces_node() -> Node {
    Node {
        width: px(BOX),
        flex_wrap: FlexWrap::Wrap,
        ..default()
    }
}
