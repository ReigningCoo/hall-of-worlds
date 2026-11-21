//! What each food is to eat: how much Health it gives back and for how long,
//! and which kinds of creature it suits. What eating one does is `eating.rs`.

use crate::creature::CreatureKind;
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::item::{Item, ItemKind};
use crate::vitals::PER_POINT;

/// What happens to a creature of a kind a food doesn't suit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Others {
    /// It won't eat it at all (Dwarven Bread).
    Refused,
    /// It eats it, gets no healing and is poisoned (`bonus::poison`): Raw Meat.
    Poisoned,
}

/// One food: Health given back a second (thousandths of a point) for so many
/// action ticks, ended by moving or being hurt, for the kinds it suits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Food {
    pub restore: u32,
    pub ticks: u64,
    /// The kinds it suits: empty for anyone.
    pub kinds: &'static [CreatureKind],
    /// What it does to other kinds.
    pub others: Others,
}

impl Food {
    /// Whether it suits a creature of this kind.
    pub fn suits(self, eater: CreatureKind) -> bool {
        self.kinds.is_empty() || self.kinds.contains(&eater)
    }
}

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

/// What eating this is like, if it can be eaten (owner, 2026-11-14): bread
/// gives back 1 Health a second for 45 s, Dwarven Bread for
/// 60 s to dwarves only, Raw Meat 2 a second for 60 s to orcs and goblins and
/// poisons anyone else.
pub fn food(kind: ItemKind) -> Option<Food> {
    let anyone = |restore, ticks| Food {
        restore,
        ticks,
        kinds: &[],
        others: Others::Refused,
    };
    Some(match kind {
        ItemKind::Bread => anyone(PER_POINT, 45 * SECOND),
        ItemKind::DwarvenBread => Food {
            kinds: &[CreatureKind::Dwarf],
            ..anyone(PER_POINT, 60 * SECOND)
        },
        ItemKind::RawMeat => Food {
            kinds: &[CreatureKind::Orc, CreatureKind::Goblin],
            others: Others::Poisoned,
            ..anyone(2 * PER_POINT, 60 * SECOND)
        },
        _ => return None,
    })
}

/// Whether it can be eaten by someone: what can be dropped on the body and go
/// on the action bar. Fruit, vegetables and fungi are, eaten by their own
/// rules (`fruit::eating`, `vegetable::eating`, `fungus::eating`), grain too,
/// though it heals nothing.
pub fn is_food(kind: ItemKind) -> bool {
    food(kind).is_some()
        || matches!(
            kind,
            ItemKind::Fruit | ItemKind::Vegetable | ItemKind::Fungus
        )
}

/// Whether it can be eaten at all: food, or a herb (eaten raw, for a weak dose
/// of its first effect: `herb::raw_dose`). Herbs aren't food: they never go on
/// the action bar (`is_food`). Ointment counts too: used the same way (let go
/// of on the body), it is rubbed on rather than eaten (`brewing::skin_effect`).
/// A Dish too, for Well Fed (`food::eating`), though not on the bar yet.
pub fn edible(kind: ItemKind) -> bool {
    is_food(kind) || matches!(kind, ItemKind::Herb | ItemKind::Ointment | ItemKind::Dish)
}

/// Whether a creature of this kind may eat it: a herb, fruit, vegetable,
/// fungus or ointment (anyone may), or food not refused to it.
pub fn may_eat(kind: ItemKind, eater: CreatureKind) -> bool {
    matches!(
        kind,
        ItemKind::Herb
            | ItemKind::Fruit
            | ItemKind::Vegetable
            | ItemKind::Fungus
            | ItemKind::Ointment
            | ItemKind::Dish
    ) || food(kind).is_some_and(|food| food.suits(eater) || food.others != Others::Refused)
}

/// Whether this one can be used on oneself by letting go of it on the body:
/// `edible` by its kind, a bottle holding a potion, drunk (brewing PR 8), or
/// a vessel holding water, drunk a serving at a time (drinks PR 2).
pub fn usable_on_self(item: &Item) -> bool {
    edible(item.kind)
        || item.is_potion()
        || item.is_drinkable_vessel()
        || item.is_drinkable_bottle()
}

/// Whether a creature of this kind may use this one on itself: anyone may
/// drink a potion, or a drink from a vessel or bottle; otherwise `may_eat`.
pub fn may_use_on_self(item: &Item, eater: CreatureKind) -> bool {
    item.is_potion()
        || item.is_drinkable_vessel()
        || item.is_drinkable_bottle()
        || may_eat(item.kind, eater)
}
