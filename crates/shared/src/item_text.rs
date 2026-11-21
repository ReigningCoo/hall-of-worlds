//! Items written into text, such as a chat line or a note: `${item:<uuid>}`, the
//! item's id in the usual text form of a UUID. Copied and pasted, it stays the same
//! item; what it is called and looks like is asked of the server
//! (`ClientMessage::LookUpItems`), so the text itself gives nothing away.
//!
//! A kind of item rather than one (`${kind:egg}`, `${kind:vegetable:carrot}`) is
//! written by `KindToken` (`item_text/kind.rs`).
//!
//! Only those exact shapes count: anything else, a misspelt one included, is plain
//! text. `@name` is kept for mentioning players later.

mod kind;
mod kind_item;

use uuid::Uuid;

use crate::id::Id;

pub use kind::{KIND_START, KindToken, kind_at};

/// What starts an item in text.
pub const ITEM_START: &str = "${item:";
/// What ends one.
pub const ITEM_END: char = '}';
/// How long the id between them is: the hyphenated form of a UUID.
const ID_LENGTH: usize = 36;

/// A part of some text: plain words, an item, or a kind of item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Piece<'a> {
    Text(&'a str),
    Item(Id),
    Kind(KindToken),
}

/// What is written in text as one whole: an item, or a kind of item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Item(Id),
    Kind(KindToken),
}

impl Token {
    /// How it is written.
    pub fn token(&self) -> String {
        match self {
            Self::Item(id) => item_token(*id),
            Self::Kind(kind) => kind.token(),
        }
    }
}

/// How an item is written into text.
pub fn item_token(id: Id) -> String {
    format!("{ITEM_START}{}{ITEM_END}", id.0.hyphenated())
}

/// The item written at the start of `text`, and how many bytes it takes.
pub fn item_at(text: &str) -> Option<(Id, usize)> {
    let rest = text.strip_prefix(ITEM_START)?;
    let id = rest.get(..ID_LENGTH)?;
    rest[ID_LENGTH..].strip_prefix(ITEM_END)?;
    // `parse_str` also takes other forms (simple, braced, urn): only hyphenated here.
    let hyphenated = id.char_indices().all(|(at, c)| match at {
        8 | 13 | 18 | 23 => c == '-',
        _ => c.is_ascii_hexdigit(),
    });
    let uuid = Uuid::parse_str(id).ok().filter(|_| hyphenated)?;
    Some((Id(uuid), ITEM_START.len() + ID_LENGTH + 1))
}

/// The item or kind of item written at the start of `text`, and how many bytes
/// it takes.
pub fn token_at(text: &str) -> Option<(Token, usize)> {
    if let Some((id, length)) = item_at(text) {
        return Some((Token::Item(id), length));
    }
    kind_at(text).map(|(kind, length)| (Token::Kind(kind), length))
}

/// The text in order as plain words, items and kinds of item. Neighbouring
/// words are one piece, and there are no empty ones.
pub fn pieces(text: &str) -> Vec<Piece<'_>> {
    let mut pieces = Vec::new();
    let (mut start, mut at) = (0, 0);
    while let Some(found) = text[at..].find("${") {
        at += found;
        match token_at(&text[at..]) {
            Some((token, length)) => {
                if start < at {
                    pieces.push(Piece::Text(&text[start..at]));
                }
                pieces.push(match token {
                    Token::Item(id) => Piece::Item(id),
                    Token::Kind(kind) => Piece::Kind(kind),
                });
                at += length;
                start = at;
            }
            None => at += 2,
        }
    }
    if start < text.len() {
        pieces.push(Piece::Text(&text[start..]));
    }
    pieces
}

/// Whether anything is written in the text as a whole: an item or a kind.
pub fn has_tokens(text: &str) -> bool {
    (pieces(text).iter()).any(|piece| !matches!(piece, Piece::Text(_)))
}

/// Every item written in the text, each once, in the order first written.
pub fn items_in(text: &str) -> Vec<Id> {
    let mut items = Vec::new();
    for piece in pieces(text) {
        if let Piece::Item(id) = piece
            && !items.contains(&id)
        {
            items.push(id);
        }
    }
    items
}

#[cfg(test)]
mod kind_tests;
#[cfg(test)]
mod tests;
