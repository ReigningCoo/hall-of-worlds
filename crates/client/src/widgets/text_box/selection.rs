//! The bands behind what is selected: one piece from the selection's start to
//! its end on a line (rather than a background per character, which leaves
//! seams), paler in a box without the keyboard, which keeps its selection. A
//! selection over several lines takes up to three: the rest of its first line,
//! the whole lines between, and the start of its last.

use bevy::prelude::*;

use super::keys::current;
use super::layout::Labels;
use super::spots::Spots;
use super::{BORDER, Cursor, SIDE_PADDING, TextBox, TextFocus};
use crate::widgets::theme::{SELECTION, SELECTION_WITHOUT_KEYBOARD};

/// A band, in the box it holds, and which of its three it is (a single-line
/// box has only the first). Spawned before the writing, so behind it.
#[derive(Component)]
pub(in crate::widgets) struct TextBoxSelection(pub Entity, pub usize);

/// The band's colour.
pub(super) fn band_colour(keyboard: bool) -> Color {
    if keyboard {
        SELECTION
    } else {
        SELECTION_WITHOUT_KEYBOARD
    }
}

/// The bands covering the places `from` to `to`: left, top, width and height
/// in the writing.
pub(super) fn bands(spots: &Spots, from: usize, to: usize) -> Vec<Rect> {
    let line = spots.line_height;
    let band = |left: f32, top: f32, right: f32, bottom: f32| {
        Rect::new(left, top, right.max(left), bottom)
    };
    let (start, end) = (spots.x(from), spots.x(to));
    let (top, last) = (spots.top(from), spots.top(to));
    if spots.line(from) == spots.line(to) {
        return vec![band(start, top, end, top + line)];
    }
    let mut all = vec![band(start, top, spots.width, top + line)];
    if last > top + line {
        all.push(band(0.0, top + line, spots.width, last));
    }
    all.push(band(0.0, last, end, last + line));
    all
}

/// Places each box's bands. After `show_carets`, which slides the writing.
#[allow(clippy::type_complexity)]
pub(in crate::widgets) fn show_selections(
    focus: Res<TextFocus>,
    boxes: Query<(&TextBox, &Cursor, &ComputedNode)>,
    labels: Labels,
    mut bands_shown: Query<(
        &TextBoxSelection,
        &mut Node,
        &mut BackgroundColor,
        &mut Visibility,
        &ComputedNode,
    )>,
) {
    for (band, mut node, mut colour, mut visibility, band_node) in &mut bands_shown {
        let Ok((text_box, cursor, box_node)) = boxes.get(band.0) else {
            continue;
        };
        let edited = current(text_box, cursor);
        let spots = labels.spots(band.0, &edited);
        let (Some((from, to)), Some(spots)) = (edited.selection(), spots) else {
            visibility.set_if_neq(Visibility::Hidden);
            continue;
        };
        let Some(rect) = bands(&spots, from, to).get(band.1).copied() else {
            visibility.set_if_neq(Visibility::Hidden);
            continue;
        };
        let place = if text_box.multiline {
            (
                px(rect.min.x),
                px(rect.min.y),
                px(rect.width()),
                px(rect.height()),
            )
        } else {
            // Slid with the writing, and level with the middle of the box.
            let inside = box_node.size().y * box_node.inverse_scale_factor - BORDER * 2.0;
            let height = band_node.size().y * band_node.inverse_scale_factor;
            (
                px(SIDE_PADDING + rect.min.x - cursor.scroll),
                px((inside - height) / 2.0),
                px(rect.width()),
                node.height,
            )
        };
        if (node.left, node.top, node.width, node.height) != place {
            (node.left, node.top, node.width, node.height) = place;
        }
        colour.set_if_neq(BackgroundColor(band_colour(focus.0 == Some(band.0))));
        visibility.set_if_neq(Visibility::Inherited);
    }
}

#[cfg(test)]
mod tests;
