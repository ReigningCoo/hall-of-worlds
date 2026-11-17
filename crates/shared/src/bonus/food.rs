//! What each food is to eat: how much Health it gives back and for how long,
//! and which kinds of creature it suits. What eating one does is `eating.rs`.

use crate::creature::CreatureKind;
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::item::ItemKind;
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
/// on the action bar. Fruit is, eaten by its own rules (`fruit::eating`).
pub fn is_food(kind: ItemKind) -> bool {
    food(kind).is_some() || kind == ItemKind::Fruit
}

/// Whether it can be eaten at all: food, or a herb (eaten raw, for a weak dose
/// of its first effect: `herb::raw_dose`). Herbs aren't food: they never go on
/// the action bar (`is_food`).
pub fn edible(kind: ItemKind) -> bool {
    is_food(kind) || kind == ItemKind::Herb
}

/// Whether a creature of this kind may eat it: a herb or fruit (anyone may),
/// or food not refused to it.
pub fn may_eat(kind: ItemKind, eater: CreatureKind) -> bool {
    matches!(kind, ItemKind::Herb | ItemKind::Fruit)
        || food(kind).is_some_and(|food| food.suits(eater) || food.others != Others::Refused)
}
