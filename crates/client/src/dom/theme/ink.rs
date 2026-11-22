//! Writing in the reader's own hand (`p.hand`, `span.hand`): blue-black
//! iron-gall ink, as an own recipe's "Your recipe" and the quill beside it.

use bevy::prelude::*;

use super::{Look, ROW, Theme};
use crate::dom::tag::Tag;

/// Blue-black iron-gall ink.
const HAND_INK: Color = Color::srgb(0.13, 0.15, 0.3);

pub(super) fn add(theme: &mut Theme) {
    let look = Look {
        gap: ROW,
        colour: Some(HAND_INK),
        ..default()
    };
    theme.set(Tag::P, "hand", look);
    theme.set(Tag::Span, "hand", look);
}
