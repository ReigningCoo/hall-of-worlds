//! What one slot of the action bar holds: an ability, a kind of item that can
//! be used from there (food, eaten), a particular potion (drunk), or a spell
//! from the spellbook (cast), and whether it is used by itself.
//!
//! Saves (human-readable, RON) write a slot as `(ability: Attack, auto: false)`,
//! as before items could go on the bar, `(item: Apple)`, `(potion: [...])` or
//! `(spell: 123, auto: false)`; the Heal ability of older saves loads as the
//! Heal spell. Messages (postcard) carry `BarSlot` as it is.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Ability;
use crate::bonus::is_food;
use crate::item::ItemKind;
use crate::potion::PotionKey;
use crate::spell::{AutoCast, SpellKey, Spellbook, auto_cast, heal_spell};

/// What a slot holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BarThing {
    Ability(Ability),
    /// A kind of item: the slot stands for every one of it carried, and using
    /// it uses one of them (owner, 2026-11-15).
    Item(ItemKind),
    /// A particular potion, by its contents (brewing decision 22): the slot
    /// stands for every bottle of it carried, and using it drinks one.
    Potion(PotionKey),
    /// A spell in the spellbook, by its design's key (magic plan PR 5): cast
    /// when used. One no longer in the book stays, only it can't be cast.
    Spell(SpellKey),
}

impl BarThing {
    /// The ability, if it is one.
    pub fn ability(self) -> Option<Ability> {
        match self {
            Self::Ability(ability) => Some(ability),
            _ => None,
        }
    }

    /// The kind of item, if it is one.
    pub fn item(self) -> Option<ItemKind> {
        match self {
            Self::Item(kind) => Some(kind),
            _ => None,
        }
    }

    /// The potion, if it is one.
    pub fn potion(self) -> Option<PotionKey> {
        match self {
            Self::Potion(potion) => Some(potion),
            _ => None,
        }
    }

    /// The spell's key, if it is one.
    pub fn spell(self) -> Option<SpellKey> {
        match self {
            Self::Spell(key) => Some(key),
            _ => None,
        }
    }

    /// Whether it may be used by itself: some abilities, and spells in the
    /// book whose effects say when (`spell::auto_cast`: not by hand only);
    /// items never, as using them up by themselves would be a surprise
    /// (owner, 2026-11-15).
    pub fn can_be_automatic(self, book: &Spellbook) -> bool {
        match self {
            Self::Ability(ability) => ability.can_be_automatic(),
            Self::Spell(key) => book
                .by_key(key)
                .is_some_and(|kept| auto_cast(&kept.spell) != AutoCast::ByHand),
            Self::Item(_) | Self::Potion(_) => false,
        }
    }

    /// The Heal spell (`spell::heal_spell`), as bars hold it.
    pub fn heal() -> Self {
        Self::Spell(SpellKey::of(&heal_spell()))
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

impl From<SpellKey> for BarThing {
    fn from(spell: SpellKey) -> Self {
        Self::Spell(spell)
    }
}

impl From<PotionKey> for BarThing {
    fn from(potion: PotionKey) -> Self {
        Self::Potion(potion)
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
    Potion {
        potion: PotionKey,
    },
    Spell {
        spell: SpellKey,
        #[serde(default)]
        auto: bool,
    },
    /// The Heal ability of saves from before spells (magic plan PR 5).
    OldHeal {
        ability: OldAbility,
        #[serde(default)]
        auto: bool,
    },
}

/// Abilities saves may still name that are no longer abilities.
#[derive(Serialize, Deserialize)]
enum OldAbility {
    Heal,
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
            BarThing::Potion(potion) => SavedSlot::Potion { potion },
            BarThing::Spell(spell) => SavedSlot::Spell {
                spell,
                auto: self.auto,
            },
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
            SavedSlot::Potion { potion } => Self {
                thing: BarThing::Potion(potion),
                auto: false,
            },
            SavedSlot::Spell { spell, auto } => Self {
                thing: BarThing::Spell(spell),
                auto,
            },
            SavedSlot::OldHeal {
                ability: OldAbility::Heal,
                auto,
            } => Self {
                thing: BarThing::heal(),
                auto,
            },
        })
    }
}

#[cfg(test)]
mod tests;
