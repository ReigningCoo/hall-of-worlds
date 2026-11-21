//! One item, or kind of item, among words: room in the writing a line tall and
//! square, with a little gap either side. What fills it is its frame (`frame`).

use bevy::prelude::*;
use bevy::text::{InlineBox, InlineBoxKind};
use shared::id::Id;
use shared::item_text::Token;

use super::look::Shows;
use crate::common::KnownItems;

/// The room left either side of an item, so even items side by side stand
/// apart, in interface pixels.
pub(super) const GAP: f32 = 2.0;

/// How tall the items in some writing are, in interface pixels: on the writing.
#[derive(Component, Clone, Copy, Debug)]
#[require(crate::dom::Solid)]
pub(in crate::widgets) struct InlineHeight(pub f32);

/// What is written: one item, asked after, or a kind of item, drawn from a
/// stand-in made here once (`KindToken::stand_in`) and named by its kind.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Inline {
    Item(Id),
    Kind { shows: Shows, name: String },
}

/// An item in writing: a child of its text, in among its spans. `index`
/// counts the items and kinds in the writing from 0, for marking any missing.
#[derive(Component, Clone, Debug)]
pub(in crate::widgets) struct InlineItem {
    pub(super) what: Inline,
    pub(super) index: usize,
}

/// An item to go among a text's spans: a square `side` across, and its gaps.
pub(super) fn inline_item(token: &Token, index: usize, side: f32) -> impl Bundle {
    let what = match token {
        Token::Item(id) => Inline::Item(*id),
        Token::Kind(kind) => Inline::Kind {
            shows: Shows::of_kind(kind),
            name: kind.name(),
        },
    };
    (
        InlineItem { what, index },
        // So whether it is on show can be told.
        Visibility::Inherited,
        InlineBox {
            kind: InlineBoxKind::InFlow,
            size: Vec2::new(side + 2.0 * GAP, side),
        },
    )
}

/// Says which items are on show, to be asked after if need be.
pub(in crate::widgets) fn want_inline_items(
    mut known: ResMut<KnownItems>,
    time: Res<Time<Real>>,
    items: Query<(&InlineItem, &InheritedVisibility)>,
) {
    let now = time.elapsed_secs_f64();
    for (inline, visible) in &items {
        if let Inline::Item(id) = inline.what
            && visible.get()
        {
            // Not a change to what is known: only noted for asking.
            known.bypass_change_detection().want(id, now);
        }
    }
}
