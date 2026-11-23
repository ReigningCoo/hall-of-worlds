//! Drawing the panel's two canvases: along the top the pieces (the wheres
//! unlocked, the filters, every effect: `PieceNode`), and under them the
//! steps (`steps.rs`). Drawn again whenever the design, the skills or a
//! drag's aim changes.

use bevy::prelude::*;
use shared::spell::{Effect, Step, Where, unlocked_wheres};

use super::draft::Draft;
use super::drag::DesignAim;
use super::nodes::{id, spawn_piece, spawn_words};
use super::piece::{FilterKind, Piece};
use super::steps::draw_steps;
use crate::character::CharacterSheet;

/// The top canvas.
#[derive(Component)]
pub(super) struct PiecesRow;

/// The steps' canvas.
#[derive(Component)]
pub(super) struct StepsRow;

/// A piece along the top.
#[derive(Component, Clone, Copy)]
pub(super) struct PieceNode(pub(super) Piece);

/// Effects in a line along the top before the next.
const EFFECTS_A_LINE: usize = 9;

/// A canvas just built.
type NewRow = Or<(Added<PiecesRow>, Added<StepsRow>)>;

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_rows(
    draft: Res<Draft>,
    (sheet, aim, assets): (Res<CharacterSheet>, Res<DesignAim>, Res<AssetServer>),
    top: Query<Entity, With<PiecesRow>>,
    steps: Query<Entity, With<StepsRow>>,
    added: Query<(), NewRow>,
    mut commands: Commands,
) {
    let changed = draft.is_changed() || sheet.is_changed() || aim.is_changed();
    if !(changed || !added.is_empty()) {
        return;
    }
    let unlocked = unlocked_wheres(Some(&sheet.0.skills));
    let first_where = matches!(draft.steps.first(), Some(Step::Where(_)));
    for row in &top {
        commands.entity(row).despawn_related::<Children>();
        commands.entity(row).with_children(|row| {
            row.spawn(column()).with_children(|lines| {
                let wheres = Where::ALL.into_iter().take(unlocked);
                line(
                    lines,
                    "Wheres",
                    wheres.map(|w| (Piece::Where(w), true)),
                    &assets,
                );
                // Greyed with no where first to filter, or once used.
                let filters = FilterKind::ALL.into_iter().map(|filter| {
                    (
                        Piece::Filter(filter),
                        first_where && !draft.has_filter(filter),
                    )
                });
                line(lines, "Filters", filters, &assets);
                for (at, chunk) in Effect::ALL.chunks(EFFECTS_A_LINE).enumerate() {
                    let label = if at == 0 { "Effects" } else { "" };
                    let effects = chunk.iter().map(|effect| (Piece::Effect(*effect), true));
                    line(lines, label, effects, &assets);
                }
            });
        });
    }
    for row in &steps {
        commands.entity(row).despawn_related::<Children>();
        commands.entity(row).with_children(|row| {
            draw_steps(row, (&draft.steps, draft.filters, unlocked), &aim, &assets);
        });
    }
}

/// A labelled line of pieces.
fn line(
    lines: &mut ChildSpawnerCommands,
    label: &str,
    pieces: impl Iterator<Item = (Piece, bool)>,
    assets: &AssetServer,
) {
    lines.spawn(row_node()).with_children(|line| {
        spawn_words(line, label, Some(70.0));
        for (piece, usable) in pieces {
            let (name, at) = piece.id();
            spawn_piece(
                line,
                (piece, usable),
                (PieceNode(piece), id(name, at)),
                assets,
            );
        }
    });
}

/// Lines one under another.
fn column() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(4),
        ..default()
    }
}

/// A line of pieces, 4 apart.
fn row_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        column_gap: px(4),
        ..default()
    }
}
