//! Where the places between a box's characters are on the screen, from how
//! Bevy laid its writing out: each character is a span of its own, so each has
//! a run of its own, as wide as it advances (spaces too); each item is an inline
//! box. Which line a run is on comes from how far down it is (`spots` has the
//! rules).

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::text::{ComputedTextBlock, TextLayoutInfo};
use bevy::ui::UiGlobalTransform;

use super::edit::Edited;
use super::spots::{Run, Spots, spots_of};
use super::{TextBoxChar, TextBoxLabel};
use crate::widgets::item_text::InlineHeight;

/// The line a run from `top` to `bottom` (interface pixels) is on.
fn line_of(top: f32, bottom: f32, line_height: f32) -> usize {
    ((top + bottom) / 2.0 / line_height.max(1.0))
        .floor()
        .max(0.0) as usize
}

/// A label's runs, in interface pixels from its top left corner.
fn runs<'a>(
    info: &'a TextLayoutInfo,
    block: &'a ComputedTextBlock,
    scale: f32,
    line_height: f32,
    chars: &'a Query<&TextBoxChar>,
) -> impl Iterator<Item = Run> + 'a {
    let entities = block.entities();
    let char_run = move |entity: Entity, rect: Rect| {
        let rect = Rect::from_corners(rect.min * scale, rect.max * scale);
        let line = line_of(rect.min.y, rect.max.y, line_height);
        chars
            .get(entity)
            .map_or(Run::Other, |c| Run::Char(c.0, line, rect.min.x, rect.max.x))
    };
    let writing =
        info.run_geometry
            .iter()
            .map(move |run| match entities.get(run.section_index as usize) {
                Some(_) if run.section_index == 0 => {
                    Run::Prefix(run.bounds.min.x * scale, run.bounds.max.x * scale)
                }
                Some(section) => char_run(section.entity, run.bounds),
                None => Run::Other,
            });
    // Items are boxes in the layout, not runs of writing.
    let items = (info.inline_boxes.iter()).map(move |(entity, _, rect)| char_run(*entity, *rect));
    writing.chain(items)
}

/// The boxes' labels as last laid out, to find places in them.
#[derive(SystemParam)]
pub(in crate::widgets) struct Labels<'w, 's> {
    labels: Query<
        'w,
        's,
        (
            &'static TextBoxLabel,
            &'static TextLayoutInfo,
            &'static ComputedTextBlock,
            &'static ComputedNode,
            &'static UiGlobalTransform,
            &'static InlineHeight,
        ),
    >,
    chars: Query<'w, 's, &'static TextBoxChar>,
}

impl Labels<'_, '_> {
    /// Where a box's places are, for the writing `edited` has.
    pub fn spots(&self, text_box: Entity, edited: &Edited) -> Option<Spots> {
        let (_, info, block, node, transform, height) =
            self.labels.iter().find(|(label, ..)| label.0 == text_box)?;
        let units = edited.units();
        let newlines: Vec<bool> = units.iter().map(|unit| unit.c == '\n').collect();
        let scale = node.inverse_scale_factor;
        let runs = runs(info, block, scale, height.0, &self.chars);
        Some(Spots {
            corner: corner_of(node, transform),
            width: node.size().x * scale,
            line_height: height.0,
            places: spots_of(runs, &newlines, units.len()),
        })
    }

    /// The place in a box nearest to `at` on the screen.
    pub fn place_at(&self, text_box: Entity, edited: &Edited, at: Vec2) -> usize {
        self.spots(text_box, edited)
            .map_or(edited.len(), |spots| spots.nearest(at))
    }
}

/// Where a node's top left corner is, in interface pixels.
pub(super) fn corner_of(node: &ComputedNode, transform: &UiGlobalTransform) -> Vec2 {
    (transform.translation - node.size() / 2.0) * node.inverse_scale_factor
}

#[cfg(test)]
mod tests;
