//! What one item is physically like, rolled once when the server makes it and saved
//! on the item from then on. Changing the rolls never changes items already made.
//! See `docs/context/shared/item.md`.
//!
//! Each kind rolls its choices in order, each seeing those before it. A choice can
//! lean on an earlier one by changing weights, or limit it with a weight of 0.

use serde::{Deserialize, Serialize};

use crate::armour::arms::ArmsForm;
use crate::armour::feet::FeetForm;
use crate::armour::hands::HandsForm;
use crate::armour::head::HeadForm;
use crate::armour::legs::LegsForm;
use crate::armour::neck::NeckForm;
use crate::armour::set::SetForm;
use crate::armour::shoulders::ShouldersForm;
use crate::armour::torso::TorsoForm;
use crate::armour::waist::WaistForm;
use crate::blade::BladeForm;
use crate::clothing::ClothingForm;
use crate::footwear::FootwearForm;
use crate::item::Item;
use crate::underclothes::UnderForm;

mod armour;
mod armour_ask;
mod armour_choices;
mod armour_parts;
mod choices;
mod gem;
mod give;
mod give_to;
mod key;
mod limits;
mod make;
mod measure;
mod necklace;
mod necklace_ask;
mod necklace_name;
mod necklace_roll;
mod necklace_weight;
mod request;
mod ring;
mod ring_ask;
mod ring_roll;
mod words;

pub use choices::{Choices, choices};
pub use gem::{Cut, GemForm};
pub use give::{give_blade_within, give_commission, give_form, give_form_from};
pub use give_to::give_form_to;
pub use key::{KeyForm, KeyFound, KeyPlace, KeyUse, Turned};
pub use limits::Limits;
pub use make::{ItemOrder, batch_seed, make_item, try_make_item};
pub use necklace::{
    Charm, CordStyle, Link, Medallion, NecklaceForm, NecklaceLength, Ornament, Strand, Thickness,
    TorcEnd,
};
pub(crate) use request::keep_only;
pub use request::{Field, Refused, RollRequest, fields};
pub use ring::{BandWidth, Mount, Pattern, Profile, RingForm, Setting};

/// What an item is like, for kinds that have this.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemForm {
    Gem(GemForm),
    Ring(Box<RingForm>),
    /// A cord, chain or torc, and anything hung from it.
    Necklace(Box<NecklaceForm>),
    Blade(Box<BladeForm>),
    /// Gauntlets and other hand pieces.
    Hands(HandsForm),
    /// A pair of pauldrons or other shoulder pieces.
    Shoulders(ShouldersForm),
    /// A cuirass or other torso armour, or an arming vest.
    Torso(TorsoForm),
    /// A helmet, coif or arming cap.
    Head(HeadForm),
    /// Vambraces, sleeves or other arm pieces, for one arm or both.
    Arms(ArmsForm),
    /// Greaves, chausses or other leg pieces, for both legs or one.
    Legs(LegsForm),
    /// A fauld, tassets, a skirt or another waist piece.
    Waist(WaistForm),
    /// Sabatons, mail shoes or foot guards, a pair.
    Feet(FeetForm),
    /// A collar, gorget, bevor, aventail or mantle.
    Neck(NeckForm),
    /// A pair of shoes, boots, sandals, clogs or slippers.
    Footwear(FootwearForm),
    /// Underclothes: a shirt, braies, hose, a coif and the rest.
    Under(UnderForm),
    /// Clothes on the Middle layer: a tunic, robe, doublet, bodice or vest.
    Clothing(ClothingForm),
    /// An armour set: several locations' pieces as one item.
    Set(Box<SetForm>),
    /// A key: the lock it opens (`door::Lock::id`) and what it remembers.
    Key(Box<KeyForm>),
    /// A herb (`shared::herb`): which, how it looks and what one weighs.
    Herb(crate::herb::HerbForm),
    /// A bottle (`shared::potion`): what it holds and how it looks.
    Bottle(crate::potion::BottleForm),
    /// A fruit or nut (`shared::fruit`): which, a serving or not, how it looks
    /// and what one weighs.
    Fruit(crate::fruit::FruitForm),
    /// A vegetable, pulse or grain (`shared::vegetable`): which, a serving or
    /// not, how it looks and what one weighs.
    Vegetable(crate::vegetable::VegetableForm),
    /// A mushroom, fungus or lichen (`shared::fungus`): which, a serving or
    /// not, how it looks and what one weighs.
    Fungus(crate::fungus::FungusForm),
    /// An ointment (`shared::brewing`): the doses it holds.
    Ointment(crate::brewing::OintmentForm),
    /// A paste (`shared::alchemy`): the doses it holds, as an ointment's.
    Paste(crate::brewing::OintmentForm),
    /// A powder (`shared::alchemy`): the doses it holds, as a paste's.
    Powder(crate::brewing::OintmentForm),
    /// A cask (`shared::cask`): its size, wood and what is in it.
    Cask(crate::cask::CaskForm),
    /// A sack of flour (`shared::flour`): which grain it was milled from.
    Flour(crate::flour::FlourForm),
    /// An egg (`shared::farmyard`): which animal laid it.
    Egg(crate::farmyard::EggForm),
    /// A log (`shared::woodcutting`): its wood.
    Log(crate::woodcutting::LogForm),
    /// Kindling (`shared::woodcutting`): its wood.
    Kindling(crate::woodcutting::KindlingForm),
    /// A vessel (`shared::vessel`): which one and what is in it.
    Vessel(crate::vessel::VesselForm),
    /// A yeast (`shared::yeast`): its clock and progress.
    Yeast(crate::yeast::YeastForm),
}

impl ItemForm {
    /// What something like this is called, without its size.
    pub fn name(&self) -> String {
        match self {
            Self::Gem(gem) => format!("{} {}", gem.cut.name(), gem.material.name()),
            Self::Ring(ring) => ring.name(),
            Self::Necklace(necklace) => necklace.name(),
            Self::Blade(blade) => blade.name().to_owned(),
            Self::Hands(hands) => hands.name().to_owned(),
            Self::Shoulders(shoulders) => shoulders.name().to_owned(),
            Self::Torso(torso) => torso.name().to_owned(),
            Self::Head(head) => head.name(),
            Self::Arms(arms) => arms.name(),
            Self::Legs(legs) => legs.name(),
            Self::Waist(waist) => waist.name(),
            Self::Feet(feet) => feet.name(),
            Self::Neck(neck) => neck.name(),
            Self::Footwear(footwear) => footwear.name(),
            Self::Under(under) => under.name(),
            Self::Clothing(clothing) => clothing.name(),
            Self::Set(set) => set.name(),
            Self::Key(_) => "Key".to_owned(),
            // Its herb's name is in the herb table, which only the client's
            // panels have, going by what the character knows.
            Self::Herb(_) => "Herb".to_owned(),
            Self::Bottle(bottle) => bottle.name(),
            Self::Fruit(fruit) => fruit.name(),
            Self::Vegetable(vegetable) => vegetable.name(),
            Self::Fungus(fungus) => fungus.name(),
            Self::Ointment(_) => "Ointment".to_owned(),
            Self::Paste(_) => "Paste".to_owned(),
            Self::Powder(_) => "Powder".to_owned(),
            Self::Cask(cask) => cask.name(),
            Self::Flour(flour) => flour.name(),
            Self::Egg(egg) => egg.name(),
            Self::Log(log) => log.name(),
            Self::Kindling(kindling) => kindling.name(),
            Self::Vessel(vessel) => vessel.name(),
            Self::Yeast(_) => "Yeast".to_owned(),
        }
    }

    /// A new key to `lock`, which remembers nothing yet.
    pub fn key(lock: crate::id::Id) -> Self {
        Self::Key(Box::new(KeyForm::new(lock)))
    }

    /// The lock it opens, if it is a key.
    pub fn lock(&self) -> Option<crate::id::Id> {
        match self {
            Self::Key(key) => Some(key.lock),
            _ => None,
        }
    }

    /// The gems set in it.
    pub fn gems(&self) -> impl Iterator<Item = &Item> {
        let settings = match self {
            Self::Ring(ring) => ring.settings.as_slice(),
            Self::Necklace(necklace) => necklace.settings.as_slice(),
            Self::Gem(_)
            | Self::Blade(_)
            | Self::Hands(_)
            | Self::Shoulders(_)
            | Self::Torso(_)
            | Self::Head(_)
            | Self::Arms(_)
            | Self::Legs(_)
            | Self::Waist(_)
            | Self::Feet(_)
            | Self::Neck(_)
            | Self::Footwear(_)
            | Self::Under(_)
            | Self::Clothing(_)
            | Self::Set(_)
            | Self::Key(_)
            | Self::Herb(_)
            | Self::Bottle(_)
            | Self::Fruit(_)
            | Self::Vegetable(_)
            | Self::Fungus(_)
            | Self::Ointment(_)
            | Self::Paste(_)
            | Self::Powder(_)
            | Self::Cask(_)
            | Self::Flour(_)
            | Self::Egg(_)
            | Self::Log(_)
            | Self::Kindling(_)
            | Self::Vessel(_)
            | Self::Yeast(_) => &[],
        };
        settings.iter().filter_map(|setting| setting.gem.as_ref())
    }
}

#[cfg(test)]
mod make_tests;
#[cfg(test)]
mod necklace_tests;
#[cfg(test)]
mod request_tests;
#[cfg(test)]
mod tests;
