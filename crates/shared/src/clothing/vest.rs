//! Vests: sleeveless and fitted, buttoned: the jerkin worn alone, the buff jerkin,
//! the waistcoat, the yelek. On the Torso alone.

use serde::{Deserialize, Serialize};

use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VestLength {
    Waist,
    Hip,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VestFastening {
    Buttons,
    Lacing,
    Hooks,
    /// Worn open.
    Open,
}

/// What it is made of, which its material keeps to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VestMake {
    Cloth,
    Leather,
    /// Thick oiled oxhide (1600s).
    Buff,
    /// Cloth or leather lined with fur.
    FurLined,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VestDecoration {
    None,
    Embroidery,
    Braid,
    /// Edged in a second colour.
    ContrastEdging,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vest {
    pub length: VestLength,
    pub make: VestMake,
    pub fastening: VestFastening,
    pub decoration: VestDecoration,
}

const LENGTHS: [(VestLength, u64); 2] = [(VestLength::Waist, 50), (VestLength::Hip, 50)];

const MAKES: [(VestMake, u64); 4] = [
    (VestMake::Cloth, 45),
    (VestMake::Leather, 35),
    (VestMake::Buff, 6),
    (VestMake::FurLined, 14),
];

fn decorations(make: VestMake) -> [(VestDecoration, u64); 4] {
    let cloth = make == VestMake::Cloth;
    [
        (VestDecoration::None, 55),
        (VestDecoration::Embroidery, when(cloth, 20)),
        (VestDecoration::Braid, 15),
        (VestDecoration::ContrastEdging, 15),
    ]
}

const FASTENINGS: [(VestFastening, u64); 4] = [
    (VestFastening::Buttons, 45),
    (VestFastening::Lacing, 25),
    (VestFastening::Hooks, 15),
    (VestFastening::Open, 15),
];

impl Vest {
    pub(super) fn roll(rng: &mut Rng) -> Self {
        let length = rng.pick_weighted(&LENGTHS);
        let make = rng.pick_weighted(&MAKES);
        let fastening = rng.pick_weighted(&FASTENINGS);
        let decoration = rng.pick_weighted(&decorations(make));
        Self {
            length,
            make,
            fastening,
            decoration,
        }
    }

    pub(super) fn allowed(&self) -> bool {
        has(&LENGTHS, self.length)
            && has(&MAKES, self.make)
            && has(&FASTENINGS, self.fastening)
            && has(&decorations(self.make), self.decoration)
    }

    /// Its name from the research's signatures; "Vest" when none fits.
    pub(super) fn name(&self) -> &'static str {
        match self.make {
            VestMake::Buff => "Buff Jerkin",
            VestMake::Leather if self.length == VestLength::Hip => "Jerkin",
            VestMake::Cloth if self.decoration == VestDecoration::Embroidery => "Yelek",
            VestMake::Cloth if self.fastening == VestFastening::Buttons => "Waistcoat",
            _ => "Vest",
        }
    }

    /// Grams in plain linen at an ordinary size, any fur lining left out.
    pub(super) fn base_grams(&self) -> u32 {
        match self.length {
            VestLength::Waist => 220,
            VestLength::Hip => 280,
        }
    }

    /// Grams a fur lining adds.
    pub(super) fn fur_grams(&self) -> u32 {
        match self.make {
            VestMake::FurLined => 500,
            _ => 0,
        }
    }
}
