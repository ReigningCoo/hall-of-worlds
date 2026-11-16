//! The insides of a box of several lines (`TextBox::multiline`): its writing
//! wraps to the box's width on a page in a scroll area, with the selection's
//! bands and the caret on the page too, so they scroll with it. While the box
//! has the keyboard, the area follows the caret whenever it moves.

use bevy::prelude::*;
use bevy::text::LineBreak;

use super::keys::current;
use super::layout::Labels;
use super::selection::TextBoxSelection;
use super::{BORDER, Cursor, TextBox, TextBoxCaret, TextBoxLabel, TextFocus, caret, line_height};
use crate::widgets::item_text::InlineHeight;
use crate::widgets::scroll::furthest;
use crate::widgets::{INK, TextSize, spawn_filling_scroll_area};

/// Room above and below the lines, inside the edge.
pub(super) const END_PADDING: f32 = 4.0;

/// On a box of several lines: the scroll area its page is in, and when the
/// caret last moved and how far the area could go when it last followed it.
#[derive(Component)]
pub(in crate::widgets) struct Lines {
    pub area: Entity,
    followed: (f32, f32),
}

/// How tall a box of `rows` lines of writing this size is, edge and all.
pub fn text_area_height(size: TextSize, rows: usize) -> f32 {
    rows.max(1) as f32 * line_height(size) + (END_PADDING + BORDER) * 2.0
}

/// Fills a box of several lines: a scroll area with the page in it.
pub(super) fn fill(holder: &mut EntityCommands, size: TextSize) {
    let entity = holder.id();
    let line = line_height(size);
    let mut area = Entity::PLACEHOLDER;
    holder.with_children(|inside| {
        area = spawn_filling_scroll_area(inside);
        // No width of its own: it fills the box, and never widens it (or
        // what holds it) to its writing laid out on one line.
        let mut commands = inside.commands();
        let mut made = commands.entity(area);
        made.entry::<Node>().and_modify(|mut node| {
            node.width = px(0);
            node.min_width = px(0);
            node.flex_basis = px(0);
        });
        made.with_children(|area| {
            let page = Node {
                width: percent(100),
                flex_shrink: 0.0,
                ..default()
            };
            area.spawn(page).with_children(|page| {
                // First, so behind the writing.
                for part in 0..3 {
                    page.spawn((
                        TextBoxSelection(entity, part),
                        Node {
                            position_type: PositionType::Absolute,
                            height: px(line),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        Visibility::Hidden,
                    ));
                }
                page.spawn((
                    TextBoxLabel(entity),
                    InlineHeight(line),
                    Text::new(""),
                    // A word longer than the line (a code) breaks anywhere.
                    TextLayout {
                        linebreak: LineBreak::WordOrCharacter,
                        ..default()
                    },
                    size.font(),
                    TextColor(INK),
                    Node {
                        width: percent(100),
                        ..default()
                    },
                ));
                page.spawn((
                    TextBoxCaret(entity),
                    Node {
                        position_type: PositionType::Absolute,
                        width: px(caret::WIDTH),
                        height: px(line),
                        ..default()
                    },
                    BackgroundColor(INK),
                    Visibility::Hidden,
                ));
            });
        });
    });
    holder.insert(Lines {
        area,
        followed: (-1.0, -1.0),
    });
}

/// Where an area `height` tall, scrolled to `at`, goes to show the line from
/// `top` to `bottom`: as little as it must, and never past `furthest`.
pub(super) fn follow(at: f32, top: f32, bottom: f32, height: f32, furthest: f32) -> f32 {
    let mut to = at;
    if bottom > to + height {
        to = bottom - height;
    }
    if top < to {
        to = top;
    }
    to.clamp(0.0, furthest.max(0.0))
}

/// Scrolls the box with the keyboard to its caret when the caret moves, or
/// the writing's height changes. After `show_carets`.
pub(in crate::widgets) fn follow_carets(
    focus: Res<TextFocus>,
    mut boxes: Query<(&TextBox, &Cursor, &mut Lines)>,
    labels: Labels,
    mut areas: Query<(&ComputedNode, &mut ScrollPosition)>,
) {
    let Some(entity) = focus.0 else {
        return;
    };
    let Ok((text_box, cursor, mut lines)) = boxes.get_mut(entity) else {
        return;
    };
    let Ok((node, mut position)) = areas.get_mut(lines.area) else {
        return;
    };
    let most = furthest(node);
    if lines.followed == (cursor.moved_at, most) {
        return;
    }
    lines.followed = (cursor.moved_at, most);
    let edited = current(text_box, cursor);
    let Some(spots) = labels.spots(entity, &edited) else {
        return;
    };
    let top = spots.top(edited.caret);
    let height = node.size.y * node.inverse_scale_factor;
    let to = follow(position.y, top, top + spots.line_height, height, most);
    if (to - position.y).abs() > 0.5 {
        position.y = to;
    }
}

#[cfg(test)]
mod tests;
