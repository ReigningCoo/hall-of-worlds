//! Drawing the steps: each in a box (`StepNode`), its wheres or effects in
//! order one under another, each as often as it was added (`EntryNode`),
//! and in the first step any filters used, above its where (`FilterNode`);
//! ">" between; a last box for a new step
//! (`EndNode`). A step outlined red is wrong where it is; one a dragged
//! effect would join is outlined green; the gold mark is where a dragged
//! piece or step would go.

use bevy::prelude::*;
use shared::spell::{Filters, Step};

use super::drag::{DesignAim, Dragging};
use super::nodes::{WRONG, id, spawn_filter, spawn_insert_mark, spawn_piece, spawn_words};
use super::piece::FilterKind;
use super::units::units;
use crate::dom::Solid;
use crate::widgets::{ABILITY, ALLOWED, HoverTip, INK};

/// A step's box.
#[derive(Component, Clone, Copy)]
pub(super) struct StepNode(pub(super) usize);

/// A where or an effect in a step: the step and its place there.
#[derive(Component, Clone, Copy)]
pub(super) struct EntryNode(pub(super) usize, pub(super) usize);

/// The box after the last step.
#[derive(Component)]
pub(super) struct EndNode;

/// Behind a step.
const STEP_BACKGROUND: Color = Color::srgba(1.0, 1.0, 1.0, 0.18);

/// Whether the step at `at` is wrong where it is: a where not unlocked or
/// with nothing after it, or (first) filters with no where to filter.
pub(super) fn step_wrong(steps: &[Step], at: usize, (unlocked, filters): (usize, Filters)) -> bool {
    match &steps[at] {
        Step::Where(step) => {
            step.parts().any(|part| part.place as usize >= unlocked) || at + 1 == steps.len()
        }
        Step::Effects(step) => step.effects.is_empty() || (at == 0 && filters.any()),
    }
}

pub(super) fn draw_steps(
    row: &mut ChildSpawnerCommands,
    (steps, filters, unlocked): (&[Step], Filters, usize),
    aim: &DesignAim,
    assets: &AssetServer,
) {
    row.spawn(row_node()).with_children(|row| {
        let insert = aim.insert.filter(|_| aim.dragging.is_some());
        for (at, step) in steps.iter().enumerate() {
            if insert == Some(at) {
                spawn_insert_mark(row);
            } else if at > 0 {
                spawn_words(row, ">", None);
            }
            let edge = match () {
                _ if aim.into == Some(at) => ALLOWED.with_alpha(1.0),
                _ if step_wrong(steps, at, (unlocked, filters)) => WRONG,
                _ => INK,
            };
            let dragged = aim.dragging == Some(Dragging::Step(at));
            let node = Node {
                flex_direction: FlexDirection::Row,
                column_gap: px(4),
                padding: UiRect::all(px(4)),
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::all(px(6)),
                ..default()
            };
            let background = if dragged {
                Color::NONE
            } else {
                STEP_BACKGROUND
            };
            let marks = (
                StepNode(at),
                Solid,
                id("step", at),
                BackgroundColor(background),
            );
            row.spawn((node, BorderColor::from(edge), marks))
                .with_children(|inside| {
                    // One under another, first at the top, the filters (only
                    // ever in the first step) above its where (owner,
                    // 2026-11-23).
                    inside.spawn(stack()).with_children(|stack| {
                        if at == 0 {
                            let stranded = !matches!(step, Step::Where(_));
                            let on = FilterKind::ALL.into_iter().filter(|f| f.of(filters));
                            for filter in on {
                                spawn_filter(stack, filter, stranded, assets);
                            }
                        }
                        for (n, unit) in units(step).into_iter().enumerate() {
                            let marks = (EntryNode(at, n), id("entry", at * 100 + n));
                            spawn_piece(stack, (unit.piece(), true), marks, assets);
                        }
                    });
                });
        }
        if insert == Some(steps.len()) {
            spawn_insert_mark(row);
        }
        let end = Node {
            width: px(ABILITY),
            height: px(ABILITY),
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(px(6)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            ..default()
        };
        let tip = HoverTip("Drop here for a new step".to_owned());
        row.spawn((end, BorderColor::from(INK.with_alpha(0.4)), tip))
            .insert((EndNode, Solid, id("end", 0)))
            .with_children(|end| spawn_words(end, "+", None));
    });
}

/// A step's wheres or effects, one under another.
fn stack() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(4),
        ..default()
    }
}

/// The row of steps, centred on each other.
fn row_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        column_gap: px(6),
        align_items: AlignItems::Center,
        min_height: px(ABILITY + 12.0),
        ..default()
    }
}
