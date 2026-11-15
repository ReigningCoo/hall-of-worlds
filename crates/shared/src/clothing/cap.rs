//! Soft brimless caps: the Phrygian cap, pileus, beret and Tudor bonnet, the knitted
//! Monmouth cap, the fez and kalpak, fur caps. On the Head. Skullcaps are a form of
//! the Linen Coif (Skin), so a turban goes over them (owner, 2026-11-15).

use serde::{Deserialize, Serialize};

use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapShape {
    Round,
    /// Flat and wide (the beret).
    Flat,
    Cone,
    /// A cone bent forward.
    Phrygian,
    /// A short cylinder (fez, kalpak).
    Cylinder,
    Tall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapMake {
    Sewn,
    Felt,
    Knitted,
    Fur,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapBrim {
    None,
    /// A turned-up band.
    Band,
    /// A small stiff peak.
    Peak,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapTop {
    Plain,
    Button,
    Tassel,
    /// A feather (the bonnet, 16th century on).
    Feather,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cap {
    pub shape: CapShape,
    pub make: CapMake,
    /// Flaps that tie under the chin.
    pub ear_flaps: bool,
    pub brim: CapBrim,
    pub top: CapTop,
}

const SHAPES: [(CapShape, u64); 6] = [
    (CapShape::Round, 35),
    (CapShape::Flat, 20),
    (CapShape::Cone, 15),
    (CapShape::Phrygian, 10),
    (CapShape::Cylinder, 12),
    (CapShape::Tall, 8),
];

fn makes(shape: CapShape) -> [(CapMake, u64); 4] {
    use CapShape::{Cylinder, Tall};
    [
        (CapMake::Sewn, when(shape != Tall, 40)),
        (CapMake::Felt, 30),
        (
            CapMake::Knitted,
            when(!matches!(shape, Cylinder | Tall), 20),
        ),
        (
            CapMake::Fur,
            when(matches!(shape, CapShape::Round | Cylinder | Tall), 15),
        ),
    ]
}

fn flaps(shape: CapShape, make: CapMake) -> [(bool, u64); 2] {
    let shaped = matches!(shape, CapShape::Round | CapShape::Cone);
    let warm = matches!(make, CapMake::Fur | CapMake::Knitted | CapMake::Felt);
    [(false, 80), (true, when(shaped && warm, 20))]
}

fn brims(shape: CapShape, make: CapMake, ear_flaps: bool) -> [(CapBrim, u64); 3] {
    use CapShape::{Cylinder, Flat, Round, Tall};
    let free = !ear_flaps;
    let stiff = matches!(make, CapMake::Sewn | CapMake::Felt);
    [
        (CapBrim::None, 65),
        (
            CapBrim::Band,
            when(free && matches!(shape, Round | Flat | Cylinder | Tall), 25),
        ),
        (
            CapBrim::Peak,
            when(free && stiff && matches!(shape, Round | Flat), 10),
        ),
    ]
}

fn tops(shape: CapShape, make: CapMake) -> [(CapTop, u64); 4] {
    let tassel = matches!(shape, CapShape::Cylinder | CapShape::Cone | CapShape::Round);
    let feather = shape == CapShape::Flat && matches!(make, CapMake::Sewn | CapMake::Felt);
    [
        (CapTop::Plain, 60),
        (CapTop::Button, when(shape != CapShape::Tall, 15)),
        (CapTop::Tassel, when(tassel || make == CapMake::Knitted, 15)),
        (CapTop::Feather, when(feather, 10)),
    ]
}

impl Cap {
    pub(super) fn roll(rng: &mut Rng) -> Self {
        let shape = rng.pick_weighted(&SHAPES);
        let make = rng.pick_weighted(&makes(shape));
        let ear_flaps = rng.pick_weighted(&flaps(shape, make));
        Self {
            shape,
            make,
            ear_flaps,
            brim: rng.pick_weighted(&brims(shape, make, ear_flaps)),
            top: rng.pick_weighted(&tops(shape, make)),
        }
    }

    pub(super) fn allowed(&self) -> bool {
        has(&SHAPES, self.shape)
            && has(&makes(self.shape), self.make)
            && has(&flaps(self.shape, self.make), self.ear_flaps)
            && has(&brims(self.shape, self.make, self.ear_flaps), self.brim)
            && has(&tops(self.shape, self.make), self.top)
    }

    /// The materials it can be made of, the usual one first.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        use Material::{Cotton, Felt, Fur, Leather, Linen, Silk, Wool};
        match self.make {
            CapMake::Sewn => vec![
                (Wool, 50),
                (Linen, 20),
                (Silk, 15),
                (Cotton, 10),
                (Leather, 5),
            ],
            CapMake::Felt => vec![(Felt, 1)],
            CapMake::Knitted => vec![(Wool, 80), (Silk, 10), (Cotton, 10)],
            CapMake::Fur => vec![(Fur, 1)],
        }
    }

    /// Its name from the research's signatures; "Cap" when none fits.
    pub(super) fn name(&self) -> &'static str {
        use CapMake::{Felt, Fur, Knitted};
        match (self.shape, self.make) {
            _ if self.ear_flaps && self.make == Fur => "Ushanka",
            _ if self.ear_flaps && self.make == Knitted => "Chullo",
            (CapShape::Tall, Fur) => "Papakha",
            (CapShape::Cylinder, Felt) if self.top == CapTop::Tassel => "Fez",
            (CapShape::Cylinder, Felt) => "Kalpak",
            (CapShape::Flat, _) if self.brim == CapBrim::Band && self.top == CapTop::Feather => {
                "Tudor Bonnet"
            }
            (CapShape::Flat, _) => "Beret",
            (CapShape::Phrygian, _) => "Phrygian Cap",
            (CapShape::Cone, Felt) => "Pileus",
            (_, Knitted) if self.top == CapTop::Button => "Monmouth Cap",
            _ => "Cap",
        }
    }

    /// Grams in plain linen at an ordinary size.
    pub(super) fn base_grams(&self) -> u32 {
        let shape = match self.shape {
            CapShape::Round | CapShape::Cone => 60,
            CapShape::Flat | CapShape::Phrygian | CapShape::Cylinder => 70,
            CapShape::Tall => 100,
        };
        let flaps = if self.ear_flaps { 30 } else { 0 };
        let brim = match self.brim {
            CapBrim::None => 0,
            CapBrim::Band => 15,
            CapBrim::Peak => 20,
        };
        shape + flaps + brim
    }

    /// Tall caps and fezzes take two cells.
    pub(super) fn tall(&self) -> bool {
        matches!(self.shape, CapShape::Tall | CapShape::Cylinder)
    }
}
