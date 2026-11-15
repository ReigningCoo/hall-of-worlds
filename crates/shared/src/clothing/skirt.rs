//! Skirts: a skirt, kilt or wrapped cloth from the waist: the kilt, fustanella and
//! shendyt, the qun and chima, sarong, lungi and longyi, the dhoti, the lavalava and
//! the grass skirt. On the Legs, so they clash with trousers but not belts, and leg
//! armour goes over them (owner, 2026-11-15).

use serde::{Deserialize, Serialize};

use super::Pattern;
use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkirtLength {
    Knee,
    Calf,
    Ankle,
    Floor,
}

/// How the cloth is made up.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkirtMake {
    Gathered,
    /// Knife or box pleats.
    Pleated,
    /// Many narrow pleats (the fustanella), to the knee.
    ManyPleats,
    Gored,
    /// A tube sewn shut (the sarong).
    Tube,
    /// Wrapped round and tucked in.
    Wrapped,
    /// Wrapped and passed between the legs (the dhoti).
    BetweenLegs,
    /// Fibres hanging from a band: grass, or sheepskin tufts (the kaunakes).
    Fibres,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkirtWaist {
    Drawstring,
    /// A waistband with ties.
    Ties,
    /// A tucked roll.
    Tucked,
    /// Held by a belt of its own.
    Belt,
    /// Up to the chest (the chima), to the floor.
    Chest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkirtFront {
    Closed,
    /// A flat apron front (the kilt, the mamianqun).
    Apron,
    /// Overlapping at the side, as wraps do.
    SideOverlap,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Skirt {
    pub length: SkirtLength,
    pub make: SkirtMake,
    pub waist: SkirtWaist,
    pub front: SkirtFront,
}

const LENGTHS: [(SkirtLength, u64); 4] = [
    (SkirtLength::Knee, 25),
    (SkirtLength::Calf, 25),
    (SkirtLength::Ankle, 35),
    (SkirtLength::Floor, 15),
];

fn makes(length: SkirtLength) -> [(SkirtMake, u64); 8] {
    use SkirtLength::{Ankle, Calf, Knee};
    let middling = matches!(length, Calf | Ankle);
    [
        (SkirtMake::Gathered, 30),
        (SkirtMake::Pleated, 18),
        (SkirtMake::ManyPleats, when(length == Knee, 8)),
        (SkirtMake::Gored, 12),
        (SkirtMake::Tube, when(middling, 12)),
        (SkirtMake::Wrapped, when(length != SkirtLength::Floor, 15)),
        (SkirtMake::BetweenLegs, when(middling, 8)),
        (SkirtMake::Fibres, when(matches!(length, Knee | Calf), 6)),
    ]
}

fn waists(length: SkirtLength, make: SkirtMake) -> [(SkirtWaist, u64); 5] {
    use SkirtMake::*;
    let full = matches!(make, Gathered | Pleated);
    let (drawstring, ties, tucked, belt) = match make {
        Tube => (20, 0, 50, 0),
        Wrapped => (0, 30, 30, 20),
        BetweenLegs => (0, 0, 1, 0),
        Fibres => (0, 50, 0, 50),
        _ => (30, 30, 0, 20),
    };
    [
        (SkirtWaist::Drawstring, drawstring),
        (SkirtWaist::Ties, ties),
        (SkirtWaist::Tucked, tucked),
        (SkirtWaist::Belt, belt),
        (
            SkirtWaist::Chest,
            when(full && length == SkirtLength::Floor, 15),
        ),
    ]
}

fn fronts(make: SkirtMake, waist: SkirtWaist) -> [(SkirtFront, u64); 3] {
    let wrapped = make == SkirtMake::Wrapped;
    let apron =
        matches!(make, SkirtMake::Gathered | SkirtMake::Pleated) && waist != SkirtWaist::Chest;
    [
        (SkirtFront::Closed, when(!wrapped, 75)),
        (SkirtFront::Apron, when(apron, 30)),
        (SkirtFront::SideOverlap, when(wrapped, 1)),
    ]
}

impl Skirt {
    pub(super) fn roll(rng: &mut Rng) -> Self {
        let length = rng.pick_weighted(&LENGTHS);
        let make = rng.pick_weighted(&makes(length));
        let waist = rng.pick_weighted(&waists(length, make));
        let front = rng.pick_weighted(&fronts(make, waist));
        Self {
            length,
            make,
            waist,
            front,
        }
    }

    pub(super) fn allowed(&self) -> bool {
        has(&LENGTHS, self.length)
            && has(&makes(self.length), self.make)
            && has(&waists(self.length, self.make), self.waist)
            && has(&fronts(self.make, self.waist), self.front)
    }

    /// The materials it can be made of, the usual one first.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        use Material::{Cotton, Fur, Grass, Linen, Silk, Wool};
        match self.make {
            SkirtMake::Fibres => vec![(Grass, 70), (Fur, 30)],
            _ if self.kilted() => vec![(Wool, 85), (Linen, 15)],
            SkirtMake::BetweenLegs => vec![(Cotton, 60), (Linen, 25), (Silk, 15)],
            SkirtMake::Tube => vec![(Cotton, 50), (Silk, 25), (Linen, 25)],
            SkirtMake::Wrapped => vec![(Linen, 35), (Cotton, 35), (Wool, 15), (Silk, 15)],
            _ => vec![(Wool, 40), (Linen, 25), (Silk, 20), (Cotton, 15)],
        }
    }

    /// Pleated with an apron front to the knee: a kilt, mostly tartan.
    pub(super) fn kilted(&self) -> bool {
        self.make == SkirtMake::Pleated
            && self.front == SkirtFront::Apron
            && self.length == SkirtLength::Knee
    }

    /// Whether its name says what it is made of (grass, sheepskin).
    pub(super) fn named_for_material(&self) -> bool {
        self.make == SkirtMake::Fibres
    }

    /// Its name from the research's signatures; "Skirt" when none fits.
    pub(super) fn name(&self, material: Material, pattern: Pattern) -> &'static str {
        use SkirtLength::{Ankle, Knee};
        use SkirtMake::*;
        match (self.make, self.length) {
            (Fibres, _) if material == Material::Grass => "Grass Skirt",
            (Fibres, _) => "Kaunakes",
            (BetweenLegs, _) => "Dhoti",
            _ if self.waist == SkirtWaist::Chest => "Chima",
            _ if self.kilted() => "Kilt",
            (Pleated, Ankle) => "Qun",
            (Gathered, Ankle) if self.front == SkirtFront::Apron => "Qun",
            (ManyPleats, _) => "Fustanella",
            (Wrapped, Knee) if material == Material::Linen => "Shendyt",
            (Wrapped, Knee) => "Lavalava",
            (Wrapped, Ankle) => "Lungi",
            (Tube, Ankle) if pattern == Pattern::Checked => "Longyi",
            (Tube, Ankle) => "Sarong",
            _ => "Skirt",
        }
    }

    /// Grams in plain linen at an ordinary size.
    pub(super) fn base_grams(&self) -> u32 {
        let length = match self.length {
            SkirtLength::Knee => 250,
            SkirtLength::Calf => 350,
            SkirtLength::Ankle => 450,
            SkirtLength::Floor => 550,
        };
        match self.make {
            SkirtMake::Gathered | SkirtMake::Pleated => length * 3 / 2,
            // A fustanella takes many metres of cloth.
            SkirtMake::ManyPleats => length * 4,
            SkirtMake::BetweenLegs => 400,
            SkirtMake::Fibres => 400,
            SkirtMake::Gored | SkirtMake::Tube | SkirtMake::Wrapped => length,
        }
    }

    /// Whether it is long or full enough to take three cells.
    pub(super) fn long(&self) -> bool {
        matches!(self.length, SkirtLength::Ankle | SkirtLength::Floor)
    }
}
