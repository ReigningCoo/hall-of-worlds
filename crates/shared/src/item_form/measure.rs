//! What forms say about the size and weight of their items.

use super::ItemForm;
use crate::creature::Size;
use crate::item::Shape;

impl ItemForm {
    /// The cells its item takes up at an ordinary size, for forms that decide it.
    pub fn shape(&self) -> Option<Shape> {
        match self {
            Self::Blade(blade) => Some(blade.shape()),
            Self::Necklace(necklace) => Some(necklace.shape()),
            Self::Shoulders(shoulders) => Some(shoulders.shape()),
            Self::Torso(torso) => Some(torso.shape()),
            Self::Head(head) => Some(head.shape()),
            Self::Arms(arms) => Some(arms.shape()),
            Self::Legs(legs) => Some(legs.shape()),
            Self::Waist(waist) => Some(waist.shape()),
            Self::Feet(feet) => Some(feet.shape()),
            Self::Neck(neck) => Some(neck.shape()),
            Self::Footwear(footwear) => Some(footwear.shape()),
            Self::Under(under) => Some(under.shape()),
            Self::Clothing(clothing) => Some(clothing.shape()),
            Self::Fruit(fruit) => Some(fruit.shape()),
            Self::Vegetable(vegetable) => Some(vegetable.shape()),
            Self::Fungus(fungus) => Some(fungus.shape()),
            Self::Cask(cask) => Some(cask.size.shape()),
            Self::Vessel(vessel) => Some(vessel.vessel.shape()),
            Self::Flour(flour) => Some(flour.shape()),
            Self::Egg(egg) => Some(egg.shape()),
            Self::Food(food) => Some(food.shape()),
            Self::Log(log) => Some(log.shape()),
            Self::Kindling(kindling) => Some(kindling.shape()),
            Self::Gem(_)
            | Self::Ring(_)
            | Self::Hands(_)
            | Self::Set(_)
            | Self::Key(_)
            | Self::Herb(_)
            | Self::Bottle(_)
            | Self::Yeast(_)
            | Self::Ointment(_)
            | Self::Paste(_)
            | Self::Powder(_) => None,
        }
    }

    /// What its item weighs made at this size, without anything in it, for forms that
    /// decide it.
    pub fn grams_at(&self, size: Size) -> Option<u32> {
        match self {
            Self::Blade(blade) => Some(blade.grams_at(size)),
            Self::Necklace(necklace) => Some(necklace.grams_at(size)),
            Self::Hands(hands) => Some(hands.grams_at(size)),
            Self::Shoulders(shoulders) => Some(shoulders.grams_at(size)),
            Self::Torso(torso) => Some(torso.grams_at(size)),
            Self::Head(head) => Some(head.grams_at(size)),
            Self::Arms(arms) => Some(arms.grams_at(size)),
            Self::Legs(legs) => Some(legs.grams_at(size)),
            Self::Waist(waist) => Some(waist.grams_at(size)),
            Self::Feet(feet) => Some(feet.grams_at(size)),
            Self::Neck(neck) => Some(neck.grams_at(size)),
            Self::Footwear(footwear) => Some(footwear.grams_at(size)),
            Self::Under(under) => Some(under.grams_at(size)),
            Self::Clothing(clothing) => Some(clothing.grams_at(size)),
            Self::Set(set) => Some(set.grams_at(size)),
            Self::Herb(herb) => Some(herb.grams),
            Self::Bottle(bottle) => Some(bottle.grams()),
            Self::Ointment(_) => Some(crate::brewing::OINTMENT_GRAMS),
            Self::Paste(_) => Some(crate::alchemy::PASTE_GRAMS),
            Self::Powder(_) => Some(crate::alchemy::POWDER_GRAMS),
            Self::Fruit(fruit) => Some(fruit.grams),
            Self::Vegetable(vegetable) => Some(vegetable.grams),
            Self::Fungus(fungus) => Some(fungus.grams),
            Self::Cask(cask) => Some(cask.grams()),
            Self::Vessel(vessel) => Some(vessel.grams()),
            Self::Flour(flour) => Some(flour.grams),
            Self::Egg(egg) => Some(egg.grams),
            Self::Food(food) => Some(food.grams()),
            Self::Log(log) => Some(log.grams()),
            Self::Kindling(_) => Some(crate::woodcutting::KINDLING_GRAMS),
            Self::Gem(_) | Self::Ring(_) | Self::Key(_) | Self::Yeast(_) => None,
        }
    }
}
