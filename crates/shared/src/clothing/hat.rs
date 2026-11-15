//! Brimmed and stiff hats, on the Outer layer over a hood, cap or veil (owner,
//! 2026-11-15): straw hats, the petasos, bycocket, capotain, cavalier hat and
//! tricorne, the conical hat. The Hat kind was a placeholder before; old ones without
//! a form load as before.

use serde::{Deserialize, Serialize};

use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Crown {
    Low,
    Round,
    Tall,
    Flat,
    Cone,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HatBrim {
    Narrow,
    Wide,
    VeryWide,
    /// Turned up behind and peaked in front (the bycocket).
    TurnedUpBehind,
    /// Cocked on three sides (the tricorne, late).
    Cocked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HatMake {
    Felt,
    Straw,
    Leather,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HatDecoration {
    None,
    Band,
    Feather,
    /// A rosette of ribbon (late).
    Cockade,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hat {
    pub crown: Crown,
    pub brim: HatBrim,
    pub make: HatMake,
    pub chin_strap: bool,
    pub decoration: HatDecoration,
}

const CROWNS: [(Crown, u64); 5] = [
    (Crown::Low, 20),
    (Crown::Round, 35),
    (Crown::Tall, 15),
    (Crown::Flat, 15),
    (Crown::Cone, 15),
];

fn brims(crown: Crown) -> [(HatBrim, u64); 5] {
    let cone = crown == Crown::Cone;
    let low = matches!(crown, Crown::Low | Crown::Round);
    [
        (HatBrim::Narrow, when(!cone, 30)),
        (HatBrim::Wide, 35),
        (HatBrim::VeryWide, 15),
        (
            HatBrim::TurnedUpBehind,
            when(matches!(crown, Crown::Round | Crown::Tall), 12),
        ),
        (HatBrim::Cocked, when(low, 6)),
    ]
}

fn makes(crown: Crown, brim: HatBrim) -> [(HatMake, u64); 3] {
    let shaped = matches!(brim, HatBrim::TurnedUpBehind | HatBrim::Cocked);
    [
        (HatMake::Felt, when(crown != Crown::Cone, 55)),
        (HatMake::Straw, when(!shaped, 35)),
        (HatMake::Leather, when(crown != Crown::Cone && !shaped, 10)),
    ]
}

fn straps(brim: HatBrim) -> [(bool, u64); 2] {
    let broad = matches!(brim, HatBrim::Wide | HatBrim::VeryWide);
    [(false, 70), (true, when(broad, 30))]
}

fn decorations(make: HatMake, brim: HatBrim) -> [(HatDecoration, u64); 4] {
    let felt = make == HatMake::Felt;
    [
        (HatDecoration::None, 50),
        (HatDecoration::Band, 30),
        (HatDecoration::Feather, when(felt, 15)),
        (
            HatDecoration::Cockade,
            when(felt && brim == HatBrim::Cocked, 20),
        ),
    ]
}

impl Hat {
    pub(super) fn roll(rng: &mut Rng) -> Self {
        let crown = rng.pick_weighted(&CROWNS);
        let brim = rng.pick_weighted(&brims(crown));
        let make = rng.pick_weighted(&makes(crown, brim));
        Self {
            crown,
            brim,
            make,
            chin_strap: rng.pick_weighted(&straps(brim)),
            decoration: rng.pick_weighted(&decorations(make, brim)),
        }
    }

    pub(super) fn allowed(&self) -> bool {
        has(&CROWNS, self.crown)
            && has(&brims(self.crown), self.brim)
            && has(&makes(self.crown, self.brim), self.make)
            && has(&straps(self.brim), self.chin_strap)
            && has(&decorations(self.make, self.brim), self.decoration)
    }

    /// The materials it can be made of, the usual one first.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        match self.make {
            HatMake::Felt => vec![(Material::Felt, 1)],
            HatMake::Straw => vec![(Material::Straw, 1)],
            HatMake::Leather => vec![(Material::Leather, 1)],
        }
    }

    /// Its name from the research's signatures; "Hat" when none fits.
    pub(super) fn name(&self) -> &'static str {
        use HatBrim::{Narrow, VeryWide, Wide};
        let feather = self.decoration == HatDecoration::Feather;
        match (self.crown, self.brim) {
            (Crown::Cone, _) => "Conical Hat",
            (_, HatBrim::Cocked) => "Tricorne",
            (_, HatBrim::TurnedUpBehind) => "Bycocket",
            (Crown::Low, Wide) if self.chin_strap => "Petasos",
            (Crown::Tall, Narrow) => "Capotain",
            (_, Wide | VeryWide) if feather => "Cavalier Hat",
            _ if self.make == HatMake::Straw => "Straw Hat",
            _ => "Hat",
        }
    }

    /// Whether its name says its material already.
    pub(super) fn named_for_material(&self) -> bool {
        self.name() == "Straw Hat"
    }

    /// Grams in plain linen at an ordinary size.
    pub(super) fn base_grams(&self) -> u32 {
        let crown = match self.crown {
            Crown::Low | Crown::Cone => 60,
            Crown::Round | Crown::Flat => 80,
            Crown::Tall => 110,
        };
        let brim = match self.brim {
            HatBrim::Narrow => 30,
            HatBrim::Wide => 60,
            HatBrim::VeryWide => 100,
            HatBrim::TurnedUpBehind => 50,
            HatBrim::Cocked => 60,
        };
        let strap = if self.chin_strap { 5 } else { 0 };
        let decoration = match self.decoration {
            HatDecoration::None => 0,
            HatDecoration::Band => 5,
            HatDecoration::Feather | HatDecoration::Cockade => 10,
        };
        crown + brim + strap + decoration
    }

    /// Very wide hats and conical ones take a square of cells.
    pub(super) fn broad(&self) -> bool {
        self.brim == HatBrim::VeryWide || self.crown == Crown::Cone
    }
}
