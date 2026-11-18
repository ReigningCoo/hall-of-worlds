//! The Pour rule: what pouring one container into another does, or why not.

use serde::{Deserialize, Serialize};

use super::held::{BOTTLE_ML, Held, holding};
use crate::energy::handling_cost;
use crate::item::{Item, ItemKind};
use crate::juice::{Mixed, mix};
use crate::potion::Liquid;

/// Why something can't be poured.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PourRefused {
    /// One of them isn't a cask, a vessel or a bottle.
    NotContainers,
    /// Into itself.
    Itself,
    /// Nothing is in it.
    Empty,
    /// Potions stay in their bottles.
    Potion,
    /// The other holds something else: never two liquids together.
    Different,
    /// The other has no room (a bottle is only ever emptied whole).
    Full,
    /// Drawing from a cask needs a tap carried.
    NeedsTap,
    /// A bottle is only ever filled whole: there isn't a bottle's worth.
    TooLittle,
    /// Cordial takes at least as much juice as water.
    TooWatery,
    // New refusals go at the end: messages carry one by its place in this list.
}

impl PourRefused {
    /// Why, in a few words, for logs and the red line under the clothes.
    pub fn words(self) -> &'static str {
        match self {
            Self::NotContainers => "Only casks, vessels and bottles can be poured.",
            Self::Itself => "That is the same one.",
            Self::Empty => "There is nothing in it to pour.",
            Self::Potion => "Potions stay in their bottles.",
            Self::Different => "It holds something else.",
            Self::Full => "There is no room for it.",
            Self::NeedsTap => "Drawing from a cask needs a tap.",
            Self::TooLittle => "There isn't enough to fill a bottle.",
            Self::TooWatery => "A cordial needs at least as much juice as water.",
        }
    }
}

/// Both containers after pouring, and how many millilitres went.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poured {
    pub from: Item,
    pub into: Item,
    pub ml: u32,
}

/// Pours `from` into `into` (one each, not stacks): as much as there is, up
/// to what `into` takes. Juice and water make cordial (`juice::mix`): water
/// poured into juice goes in only up to the juice's amount, and juice poured
/// into water must be at least as much as the water. `tap`: whether the one pouring carries a tap, which
/// drawing from a cask needs. `into` is sealed (if it can be) when it was
/// empty and filled from a cask; everything poured from or into opens.
pub fn pour(from: &Item, into: &Item, tap: bool) -> Result<Poured, PourRefused> {
    let (Some(source), Some(target)) = (Held::of(from), Held::of(into)) else {
        return Err(PourRefused::NotContainers);
    };
    if from.id == into.id {
        return Err(PourRefused::Itself);
    }
    let Some(liquid) = source.liquid else {
        return Err(PourRefused::Empty);
    };
    if matches!(liquid, Liquid::Potion(_)) {
        return Err(PourRefused::Potion);
    }
    let from_cask = from.kind == ItemKind::Cask;
    let mut ml = source.ml.min(target.room);
    let mut made = liquid.clone();
    if let Some(held) = target.liquid.as_ref().filter(|held| **held != liquid) {
        match mix(&liquid, held) {
            // No more water than the juice already there.
            Some(Mixed::WaterIntoJuice(cordial)) => {
                ml = ml.min(target.ml);
                made = cordial;
            }
            Some(Mixed::JuiceIntoWater(_)) if ml < target.ml => {
                return Err(PourRefused::TooWatery);
            }
            Some(Mixed::JuiceIntoWater(cordial)) => made = cordial,
            None => return Err(PourRefused::Different),
        }
    }
    if from_cask && !tap {
        return Err(PourRefused::NeedsTap);
    }
    if ml == 0 || (from.kind == ItemKind::Bottle && ml < source.ml) {
        return Err(PourRefused::Full);
    }
    if into.kind == ItemKind::Bottle && ml < BOTTLE_ML {
        return Err(PourRefused::TooLittle);
    }
    let sealed = from_cask && target.ml == 0;
    Ok(Poured {
        from: holding(from, &liquid, source.ml - ml, false),
        into: holding(into, &made, target.ml + ml, sealed),
        ml,
    })
}

/// What pouring this many millilitres costs: handling their weight, a gram
/// each.
pub fn pour_cost(ml: u32) -> u32 {
    handling_cost(ml)
}
