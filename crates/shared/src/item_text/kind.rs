//! A kind of item written into text, rather than one item: `${kind:<kind>[:<option>…]}`.
//! For instructions such as a recipe's ("an egg", "a carrot", "a knife"), which no
//! real item stands behind. Each part is lower-case letters, digits and `_`.
//!
//! - A food as cooking wants it (`cooking::Wanted`): its source alone for any of
//!   it (`${kind:egg}`), or with a key for one (`${kind:vegetable:carrot}`).
//!   Sources: `herb`, `fruit`, `vegetable`, `fungus`, `meat`, `milk`, `egg`,
//!   `honey`, `part`, `pantry`.
//! - A cooking tool: `${kind:knife}`, `pan`, `cauldron`, `bowl`, `oven`, `sieve`,
//!   `mortar`.
//! - Water in a bottle, any (`${kind:bottle:water}`) or one
//!   (`${kind:bottle:water:rain}`): `plain`, `cave`, `moon`, `salt`, `rain`.

use crate::cooking::{Source, Tool, Wanted};
use crate::potion::Water;

/// What starts a kind of item in text.
pub const KIND_START: &str = "${kind:";
/// The longest a kind's writing between `${kind:` and `}` may be.
const MOST_LENGTH: usize = 80;

/// A kind of item, as written in text.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KindToken {
    /// A food: any of a source, or one by its key.
    Food(Wanted),
    /// A cooking tool.
    Tool(Tool),
    /// Water in a bottle: any, or one.
    Water(Option<Water>),
}

const SOURCES: [(Source, &str); 10] = [
    (Source::Herb, "herb"),
    (Source::Fruit, "fruit"),
    (Source::Vegetable, "vegetable"),
    (Source::Fungus, "fungus"),
    (Source::Meat, "meat"),
    (Source::Milk, "milk"),
    (Source::Egg, "egg"),
    (Source::Honey, "honey"),
    (Source::Part, "part"),
    (Source::Pantry, "pantry"),
];

const TOOLS: [(Tool, &str); 7] = [
    (Tool::Knife, "knife"),
    (Tool::Pan, "pan"),
    (Tool::Cauldron, "cauldron"),
    (Tool::Bowl, "bowl"),
    (Tool::Oven, "oven"),
    (Tool::Sieve, "sieve"),
    (Tool::Mortar, "mortar"),
];

const WATERS: [(Water, &str); 5] = [
    (Water::Plain, "plain"),
    (Water::Cave, "cave"),
    (Water::Moon, "moon"),
    (Water::Salt, "salt"),
    (Water::Rain, "rain"),
];

/// The word for `thing` in a list of pairs.
fn word_of<T: PartialEq + Copy>(list: &[(T, &'static str)], thing: T) -> &'static str {
    list.iter()
        .find(|(t, _)| *t == thing)
        .map_or("", |(_, w)| w)
}

/// The thing a word is in a list of pairs.
fn of_word<T: Copy>(list: &[(T, &'static str)], word: &str) -> Option<T> {
    list.iter().find(|(_, w)| *w == word).map(|(t, _)| *t)
}

/// Whether a part may be written: lower-case letters, digits and `_`, not empty.
fn is_part(part: &str) -> bool {
    !part.is_empty()
        && part
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

impl KindToken {
    /// How it is written into text. A food's key that can't be written (not
    /// lower-case letters, digits and `_`) is left out: any of its source.
    pub fn token(&self) -> String {
        let parts = match self {
            Self::Food(Wanted::Any(source)) => word_of(&SOURCES, *source).to_owned(),
            Self::Food(Wanted::One(source, key)) => match is_part(key) {
                true => format!("{}:{key}", word_of(&SOURCES, *source)),
                false => word_of(&SOURCES, *source).to_owned(),
            },
            Self::Tool(tool) => word_of(&TOOLS, *tool).to_owned(),
            Self::Water(None) => "bottle:water".to_owned(),
            Self::Water(Some(water)) => format!("bottle:water:{}", word_of(&WATERS, *water)),
        };
        format!("{KIND_START}{parts}}}")
    }

    /// The kind written between `${kind:` and `}`, split at its `:`s.
    fn read(parts: &[&str]) -> Option<Self> {
        if !parts.iter().all(|part| is_part(part)) {
            return None;
        }
        if let Some(source) = of_word(&SOURCES, parts[0]) {
            return match parts {
                [_] => Some(Self::Food(Wanted::Any(source))),
                [_, key] => Some(Self::Food(Wanted::One(source, (*key).to_owned()))),
                _ => None,
            };
        }
        match parts {
            [tool] => of_word(&TOOLS, tool).map(Self::Tool),
            ["bottle", "water"] => Some(Self::Water(None)),
            ["bottle", "water", water] => of_word(&WATERS, water).map(|w| Self::Water(Some(w))),
            _ => None,
        }
    }
}

/// The kind of item written at the start of `text`, and how many bytes it takes.
pub fn kind_at(text: &str) -> Option<(KindToken, usize)> {
    let rest = text.strip_prefix(KIND_START)?;
    let end = rest.find('}').filter(|&end| end <= MOST_LENGTH)?;
    let parts: Vec<&str> = rest[..end].split(':').collect();
    let kind = KindToken::read(&parts)?;
    Some((kind, KIND_START.len() + end + 1))
}
