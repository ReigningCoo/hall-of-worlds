//! The pieces the rows are drawn from: a piece as an ability (wheres round,
//! effects square) or, for a filter, an octagon (owner, 2026-11-23); a word;
//! the insertion mark; widget ids.

use bevy::prelude::*;

use super::octagon::Octagon;
use super::piece::{FilterKind, Piece};
use crate::dom::Solid;
use crate::widgets::{
    ABILITY, AbilityLook, HoverTip, INK, ITEM_EDGE, TextSize, UiId, spawn_ability,
};

/// Gold: where a dragged step or effect would go.
const GOLD: Color = Color::srgb(0.85, 0.65, 0.13);
/// Red: something wrong where it is.
pub(super) const WRONG: Color = Color::srgb(0.8, 0.1, 0.1);
/// A filter greyed, as an ability that can't be used is.
const GREYED: Color = Color::srgb(0.16, 0.15, 0.14);
const GREYED_ICON: Color = Color::srgb(0.45, 0.45, 0.45);
/// Room inside a piece's edge, as an ability's.
const INSET: f32 = 4.0;

/// A filter in the first step.
#[derive(Component, Clone, Copy)]
pub(super) struct FilterNode(pub(super) FilterKind);

/// A widget id for the debug API: `spell_design/<name>.<at>`.
pub(super) fn id(name: &'static str, at: usize) -> UiId {
    UiId::new("spell_design", name).at(at)
}

/// A piece drawn as an ability, in a frame carrying `marks`.
pub(super) fn spawn_piece(
    parent: &mut ChildSpawnerCommands,
    (piece, usable): (Piece, bool),
    marks: impl Bundle,
    assets: &AssetServer,
) {
    let look = piece.look(usable, assets);
    if let Piece::Filter(_) = piece {
        spawn_octagon(parent, look, false, marks);
        return;
    }
    parent
        .spawn((Node::default(), Solid, marks))
        .with_children(|frame| {
            spawn_ability(frame, look);
        });
}

/// A filter in the first step: an octagon above the where, red when there is
/// no where there for it to filter.
pub(super) fn spawn_filter(
    parent: &mut ChildSpawnerCommands,
    filter: FilterKind,
    wrong: bool,
    assets: &AssetServer,
) {
    let piece = Piece::Filter(filter);
    let marks = (FilterNode(filter), id("filter_on", filter as usize));
    spawn_octagon(parent, piece.look(true, assets), wrong, marks);
}

/// A word or two in ink.
pub(super) fn spawn_words(parent: &mut ChildSpawnerCommands, words: &str, width: Option<f32>) {
    let node = Node {
        width: width.map_or(Val::Auto, px),
        align_self: AlignSelf::Center,
        ..default()
    };
    parent.spawn((
        node,
        Text::new(words),
        TextSize::Medium.font(),
        TextColor(INK),
    ));
}

/// A filter as an octagon: its icon on its colour, the edge red when it is
/// wrong where it is (no where to filter).
fn spawn_octagon(
    parent: &mut ChildSpawnerCommands,
    look: AbilityLook,
    wrong: bool,
    marks: impl Bundle,
) {
    let node = Node {
        width: px(ABILITY),
        height: px(ABILITY),
        padding: UiRect::all(px(INSET)),
        ..default()
    };
    let (fill, tint) = match look.usable {
        true => (look.colour, Color::WHITE),
        false => (GREYED, GREYED_ICON),
    };
    let edge = if wrong { WRONG } else { ITEM_EDGE };
    let image = ImageNode::new(look.icon).with_color(tint);
    let filling = Node {
        width: percent(100),
        height: percent(100),
        ..default()
    };
    parent
        .spawn((node, Octagon { fill, edge }, HoverTip(look.name), Solid))
        .insert(marks)
        .with_child((filling, image));
}

/// Where a dragged step or effect would go: a gold bar.
pub(super) fn spawn_insert_mark(parent: &mut ChildSpawnerCommands) {
    let node = Node {
        width: px(4.0),
        height: px(ABILITY + 12.0),
        align_self: AlignSelf::Center,
        ..default()
    };
    parent.spawn((node, BackgroundColor(GOLD)));
}
