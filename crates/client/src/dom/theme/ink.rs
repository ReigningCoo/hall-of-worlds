//! Writing in other inks: the reader's own hand (`p.hand`, `span.hand`),
//! blue-black iron-gall ink, as an own recipe's "Your recipe" and the quill
//! beside it; and faded grey (`p.off`), what doesn't count for the reader
//! now, as set pieces not worn in an item's popup.

use bevy::prelude::*;

use super::{Look, ROW, Theme};
use crate::dom::tag::Tag;

/// Blue-black iron-gall ink.
const HAND_INK: Color = Color::srgb(0.13, 0.15, 0.3);
/// Faded, grey ink.
const OFF_INK: Color = Color::srgb(0.5, 0.48, 0.45);

pub(super) fn add(theme: &mut Theme) {
    let look = Look {
        gap: ROW,
        colour: Some(HAND_INK),
        ..default()
    };
    theme.set(Tag::P, "hand", look);
    theme.set(Tag::Span, "hand", look);
    let off = Look {
        colour: Some(OFF_INK),
        ..look
    };
    theme.set(Tag::P, "off", off);
}
