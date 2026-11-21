//! Keeps `ItemText`s on show: its words as spans, and its items and kinds of
//! item as pictures among them, made again whenever the writing changes.

use bevy::prelude::*;
use shared::item_text::{Piece, Token, pieces};

use super::{InlineHeight, ItemText, spawn_inline_item};

pub(in crate::widgets) fn show_item_texts(
    mut commands: Commands,
    texts: Query<(Entity, &ItemText, &TextFont, &TextColor, &InlineHeight), Changed<ItemText>>,
) {
    for (entity, text, font, colour, height) in &texts {
        let mut shown = commands.entity(entity);
        shown.despawn_related::<Children>();
        shown.with_children(|spans| {
            let mut index = 0;
            for piece in pieces(&text.0) {
                let token = match piece {
                    Piece::Text(words) => {
                        spans.spawn((TextSpan::new(words), font.clone(), *colour));
                        continue;
                    }
                    Piece::Item(id) => Token::Item(id),
                    Piece::Kind(kind) => Token::Kind(kind),
                };
                spawn_inline_item(spans, (&token, index), height.0, ());
                index += 1;
            }
        });
    }
}
