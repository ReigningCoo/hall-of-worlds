//! Burning by playtime (owner): only a lantern held in a hand or tied to the
//! belt burns and gives light; anywhere else (in a bag, on the ground) it uses
//! no oil. The server counts playtime, so it is paused while logged out.

use crate::creature::Location;
use crate::item::{Inventory, Item, ItemKind};

use super::form::{LanternForm, SECONDS_PER_ML};

/// What burning did, for telling the player and viewers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Burnt {
    /// Some lantern used a millilitre or more: its fount changed.
    pub used: bool,
    /// Some lantern ran dry, so the carrier's light may have shrunk.
    pub went_out: bool,
}

impl LanternForm {
    /// Burns `secs` of playtime: a millilitre each `SECONDS_PER_ML`, stopping
    /// when dry.
    pub fn burn(&mut self, secs: u32) -> Burnt {
        if !self.has_oil() {
            return Burnt::default();
        }
        let total = u64::from(self.burnt) + u64::from(secs);
        let per = u64::from(SECONDS_PER_ML);
        let used = (total / per).min(u64::from(self.ml)) as u32;
        self.ml -= used;
        self.burnt = match self.ml {
            0 => 0,
            _ => (total % per) as u32,
        };
        Burnt {
            used: used > 0,
            went_out: self.ml == 0,
        }
    }
}

impl Inventory {
    /// The lanterns where they burn: held in a hand or tied to the belt, with
    /// oil or without.
    pub fn burning_places(&self) -> impl Iterator<Item = &Item> {
        let held = self.worn.iter().filter(|item| is_held(item));
        (held.chain(&self.bag_slots)).filter(|item| item.kind == ItemKind::Lantern)
    }

    /// Whether a lantern held or tied has oil, so its carrier gives light
    /// (`light_reach`) and is a light source for everyone who can see.
    pub fn lantern_lit(&self) -> bool {
        self.burning_places()
            .any(|item| item.lantern().is_some_and(|lantern| lantern.has_oil()))
    }

    /// Lets `secs` of playtime pass for every lantern held or tied: each burns
    /// its own oil.
    pub fn burn_lanterns(&mut self, secs: u32) -> Burnt {
        let mut burnt = Burnt::default();
        let held = self.worn.iter_mut().filter(|item| is_held(item));
        for item in held.chain(&mut self.bag_slots) {
            if !item.lantern().is_some_and(|lantern| lantern.has_oil()) {
                continue;
            }
            if let Some(fount) = item.lantern_mut() {
                let done = fount.burn(secs);
                burnt.used |= done.used;
                burnt.went_out |= done.went_out;
            }
        }
        burnt
    }
}

/// Whether a worn item is held in a hand.
fn is_held(item: &Item) -> bool {
    (item.kind.def().wear).is_some_and(|wear| wear.locations.contains(&Location::Held))
}
