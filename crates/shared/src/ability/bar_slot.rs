//! What one slot of the action bar holds: an ability, or a kind of item that can
//! be used from there (food, eaten), and whether it is used by itself.
//!
//! Saves (human-readable, RON) write a slot as `(ability: Attack, auto: false)`,
//! as before items could go on the bar, or `(item: Apple)`; messages (postcard)
//! carry `BarSlot` as it is.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Ability;
use crate::bonus::is_food;
use crate::item::ItemKind;

/// What a slot holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BarThing {
    Ability(Ability),
    /// A kind of item: the slot stands for every one of it carried, and using
    /// it uses one of them (owner, 2026-11-15).
    Item(ItemKind),
}

impl BarThing {
    /// The ability, if it is one.
    pub fn ability(self) -> Option<Ability> {
        match self {
            Self::Ability(ability) => Some(ability),
            Self::Item(_) => None,
        }
    }

    /// The kind of item, if it is one.
    pub fn item(self) -> Option<ItemKind> {
        match self {
            Self::Item(kind) => Some(kind),
            Self::Ability(_) => None,
        }
    }

    /// Whether it may be used by itself: only some abilities; items never, as
    /// using them up by themselves would be a surprise (owner, 2026-11-15).
    pub fn can_be_automatic(self) -> bool {
        self.ability().is_some_and(Ability::can_be_automatic)
    }
}

impl From<Ability> for BarThing {
    fn from(ability: Ability) -> Self {
        Self::Ability(ability)
    }
}

impl From<ItemKind> for BarThing {
    fn from(kind: ItemKind) -> Self {
        Self::Item(kind)
    }
}

/// Whether a kind of item can go on the bar: it has something to do when used
/// (food, eaten), even if the player's kind shouldn't (owner, 2026-11-15).
pub fn usable_from_bar(kind: ItemKind) -> bool {
    is_food(kind)
}

/// One slot's ability or item, and whether it is used by itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BarSlot {
    pub thing: BarThing,
    /// Used by itself rather than when its key is pressed (right-click or a long
    /// press toggles it). Forgotten when it leaves the bar; never for items.
    pub auto: bool,
}

impl BarSlot {
    /// An ability or item, not used by itself.
    pub fn new(thing: impl Into<BarThing>) -> Self {
        Self {
            thing: thing.into(),
            auto: false,
        }
    }

    /// The ability, if it holds one.
    pub fn ability(self) -> Option<Ability> {
        self.thing.ability()
    }
}

/// A slot in messages.
#[derive(Serialize, Deserialize)]
struct SentSlot {
    thing: BarThing,
    auto: bool,
}

/// A slot in saves: untagged, so saves from before items keep loading.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum SavedSlot {
    Ability {
        ability: Ability,
        #[serde(default)]
        auto: bool,
    },
    Item {
        item: ItemKind,
    },
}

impl Serialize for BarSlot {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if !serializer.is_human_readable() {
            let sent = SentSlot {
                thing: self.thing,
                auto: self.auto,
            };
            return sent.serialize(serializer);
        }
        let saved = match self.thing {
            BarThing::Ability(ability) => SavedSlot::Ability {
                ability,
                auto: self.auto,
            },
            BarThing::Item(item) => SavedSlot::Item { item },
        };
        saved.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BarSlot {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if !deserializer.is_human_readable() {
            let SentSlot { thing, auto } = SentSlot::deserialize(deserializer)?;
            return Ok(Self { thing, auto });
        }
        Ok(match SavedSlot::deserialize(deserializer)? {
            SavedSlot::Ability { ability, auto } => Self {
                thing: BarThing::Ability(ability),
                auto,
            },
            SavedSlot::Item { item } => Self {
                thing: BarThing::Item(item),
                auto: false,
            },
        })
    }
}

#[cfg(test)]
mod tests;
