//! Gloves: gloves, mittens and fingerless gloves, one kind rolled into forms (owner,
//! 2026-11-15). On the Hands and Fingers; fingerless ones leave the fingers bare, so
//! rings stay free. Padded gloves stay with the armour.

use serde::{Deserialize, Serialize};

use crate::item::Quality;
use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fingers {
    Five,
    /// Mittens: a thumb and one bag for the fingers.
    Mitten,
    /// The fingers bare.
    Fingerless,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cuff {
    Wrist,
    /// A flared gauntlet cuff (16th century on).
    Gauntlet,
    /// To the elbow (rank): rolled at Rare and better.
    Elbow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GloveMake {
    Sewn,
    Knitted,
    /// Made with one needle, in loops (mittens).
    Nalbound,
    Felted,
    Fur,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GloveDecoration {
    None,
    EmbroideredCuff,
    FringedCuff,
    /// Perfumed leather (16th century on, rank): rolled at Rare and better.
    Perfumed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Gloves {
    pub fingers: Fingers,
    pub cuff: Cuff,
    pub make: GloveMake,
    pub decoration: GloveDecoration,
    pub fur_lined: bool,
}

const FINGERS: [(Fingers, u64); 3] = [
    (Fingers::Five, 55),
    (Fingers::Mitten, 30),
    (Fingers::Fingerless, 15),
];

fn makes(fingers: Fingers) -> [(GloveMake, u64); 5] {
    let mitten = fingers == Fingers::Mitten;
    [
        (GloveMake::Sewn, 50),
        (GloveMake::Knitted, 20),
        (GloveMake::Nalbound, when(mitten, 15)),
        (GloveMake::Felted, when(mitten, 10)),
        (GloveMake::Fur, when(fingers != Fingers::Fingerless, 8)),
    ]
}

/// Its cuffs; `rank` allows those for rank, only rolled at Rare and better.
fn cuffs(fingers: Fingers, make: GloveMake, rank: bool) -> [(Cuff, u64); 3] {
    let five = fingers == Fingers::Five;
    let sewn = make == GloveMake::Sewn;
    [
        (Cuff::Wrist, 75),
        (Cuff::Gauntlet, when(five && sewn, 20)),
        (
            Cuff::Elbow,
            when(rank && five && (sewn || make == GloveMake::Knitted), 8),
        ),
    ]
}

fn decorations(fingers: Fingers, make: GloveMake, rank: bool) -> [(GloveDecoration, u64); 4] {
    let sewn = make == GloveMake::Sewn;
    [
        (GloveDecoration::None, 70),
        (
            GloveDecoration::EmbroideredCuff,
            when(sewn || make == GloveMake::Knitted, 15),
        ),
        (GloveDecoration::FringedCuff, when(sewn, 10)),
        (
            GloveDecoration::Perfumed,
            when(rank && sewn && fingers == Fingers::Five, 8),
        ),
    ]
}

fn linings(fingers: Fingers, make: GloveMake) -> [(bool, u64); 2] {
    let can = make != GloveMake::Fur && fingers != Fingers::Fingerless;
    [(false, 80), (true, when(can, 20))]
}

impl Gloves {
    pub(super) fn roll(rng: &mut Rng, quality: Quality) -> Self {
        let rank = quality >= Quality::Rare;
        let fingers = rng.pick_weighted(&FINGERS);
        let make = rng.pick_weighted(&makes(fingers));
        Self {
            fingers,
            make,
            cuff: rng.pick_weighted(&cuffs(fingers, make, rank)),
            decoration: rng.pick_weighted(&decorations(fingers, make, rank)),
            fur_lined: rng.pick_weighted(&linings(fingers, make)),
        }
    }

    /// Cuffs and decoration for rank are allowed at any quality, though only rolled at
    /// Rare and better.
    pub(super) fn allowed(&self) -> bool {
        has(&FINGERS, self.fingers)
            && has(&makes(self.fingers), self.make)
            && has(&cuffs(self.fingers, self.make, true), self.cuff)
            && has(&decorations(self.fingers, self.make, true), self.decoration)
            && has(&linings(self.fingers, self.make), self.fur_lined)
    }

    /// The materials they can be made of, the usual one first.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        use Material::{Cotton, Felt, Fur, Leather, Silk, Suede, Wool};
        match self.make {
            GloveMake::Sewn => vec![(Leather, 70), (Suede, 30)],
            GloveMake::Knitted => vec![(Wool, 70), (Silk, 15), (Cotton, 15)],
            GloveMake::Nalbound => vec![(Wool, 1)],
            GloveMake::Felted => vec![(Felt, 1)],
            GloveMake::Fur => vec![(Fur, 1)],
        }
    }

    /// Their name from the research's signatures; "Gloves" when none fits.
    pub(super) fn name(&self) -> &'static str {
        match self.fingers {
            Fingers::Five if self.cuff == Cuff::Gauntlet => "Gauntlet Gloves",
            Fingers::Five => "Gloves",
            Fingers::Mitten if self.make == GloveMake::Nalbound => "Nålbound Mittens",
            Fingers::Mitten => "Mittens",
            Fingers::Fingerless => "Fingerless Gloves",
        }
    }

    /// Grams in plain linen at an ordinary size, a pair, any fur lining left out.
    pub(super) fn base_grams(&self) -> u32 {
        let hands = match self.fingers {
            Fingers::Five => 50,
            Fingers::Mitten => 60,
            Fingers::Fingerless => 35,
        };
        hands
            + match self.cuff {
                Cuff::Wrist => 0,
                Cuff::Gauntlet => 20,
                Cuff::Elbow => 40,
            }
    }

    /// Grams a fur lining adds.
    pub(super) fn fur_grams(&self) -> u32 {
        if self.fur_lined { 100 } else { 0 }
    }
}
