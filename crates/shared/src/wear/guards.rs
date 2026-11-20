//! What worn things keep off (fungi plan PR 9, decision 13): anything on the
//! hands keeps a fungus's touch off the skin, and something over the mouth
//! keeps out its spores and dust.

use crate::armour::neck::Front;
use crate::clothing::{Garment, NeckwearForm};
use crate::creature::Location;
use crate::item::{Inventory, Item};
use crate::item_form::ItemForm;

impl Item {
    /// Whether it covers the mouth when worn: a litham (a headcloth over the
    /// face), a bandana, a closed helmet (a helm: faceplate, slit, visor or
    /// mask), an aventail closed over the lower face or with its ventail, or a
    /// set with such a helmet or aventail.
    pub fn covers_mouth(&self) -> bool {
        let closed_front = |front: Front| matches!(front, Front::Closed | Front::Ventail);
        match &self.form {
            Some(ItemForm::Clothing(clothing)) => match &clothing.garment {
                Garment::Headcloth(cloth) => cloth.face_covered,
                Garment::Neckwear(neckwear) => neckwear.form == NeckwearForm::Bandana,
                _ => false,
            },
            Some(ItemForm::Head(head)) => head.face.closed(),
            Some(ItemForm::Neck(neck)) => closed_front(neck.front),
            Some(ItemForm::Set(set)) => {
                set.head.is_some_and(|head| head.face.closed())
                    || set.neck.is_some_and(|neck| closed_front(neck.front))
            }
            _ => false,
        }
    }
}

impl Inventory {
    /// Whether anything worn covers the hands (gloves, gauntlets, a set's).
    pub fn hands_covered(&self) -> bool {
        (self.worn.iter()).any(|item| item.worn_locations().contains(&Location::Hands))
    }

    /// Whether anything worn covers the mouth (`Item::covers_mouth`).
    pub fn mouth_covered(&self) -> bool {
        self.worn.iter().any(Item::covers_mouth)
    }
}

#[cfg(test)]
#[path = "guards_tests.rs"]
mod tests;
