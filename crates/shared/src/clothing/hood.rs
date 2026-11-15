//! Hoods: a hood with a short cape and perhaps a tail (hood, chaperon, gugel), and the
//! bashlyk with its long lappets. On the Head and Neck; the cape's length is a
//! property the model drapes over the shoulders, so a hood and a shawl go together
//! (owner, 2026-11-15).

use serde::{Deserialize, Serialize};

use crate::item::Quality;
use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

/// How far its cape reaches.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HoodCape {
    None,
    Neck,
    Shoulders,
    Chest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tail {
    None,
    /// A short point at the back.
    Point,
    /// A liripipe to the waist (late medieval).
    Liripipe,
    /// A liripipe to the ankle (rank): rolled at Rare and better.
    LongLiripipe,
}

impl Tail {
    pub fn long(self) -> bool {
        matches!(self, Self::Liripipe | Self::LongLiripipe)
    }
}

/// How it is closed round the face.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HoodFace {
    Open,
    Buttoned,
    /// Pulled close round the face.
    Close,
}

/// Its cape's edge: plain, or cut into dags (late medieval).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Edge {
    Plain,
    Dagged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hood {
    pub cape: HoodCape,
    pub tail: Tail,
    pub face: HoodFace,
    pub edge: Edge,
    /// Long lappets wound round the neck (the bashlyk).
    pub lappets: bool,
    /// Worn rolled up as a hat on a padded ring (the chaperon, late medieval).
    pub chaperon: bool,
}

const LAPPETS: [(bool, u64); 2] = [(false, 88), (true, 12)];

fn chaperons(lappets: bool) -> [(bool, u64); 2] {
    [(false, 85), (true, when(!lappets, 15))]
}

fn capes(lappets: bool, chaperon: bool) -> [(HoodCape, u64); 4] {
    [
        (HoodCape::None, when(!chaperon, 10)),
        (HoodCape::Neck, when(!chaperon, 20)),
        (HoodCape::Shoulders, 50),
        (HoodCape::Chest, when(!lappets, 20)),
    ]
}

/// Its tail; `rank` allows one to the ankle, only rolled at Rare and better.
fn tails(lappets: bool, rank: bool) -> [(Tail, u64); 4] {
    [
        (Tail::None, 45),
        (Tail::Point, 30),
        (Tail::Liripipe, when(!lappets, 20)),
        (Tail::LongLiripipe, when(rank && !lappets, 6)),
    ]
}

fn faces(lappets: bool, chaperon: bool) -> [(HoodFace, u64); 3] {
    [
        (HoodFace::Open, 55),
        (HoodFace::Buttoned, when(!lappets && !chaperon, 25)),
        (HoodFace::Close, when(!chaperon, 20)),
    ]
}

fn edges(cape: HoodCape) -> [(Edge, u64); 2] {
    [
        (Edge::Plain, 80),
        (Edge::Dagged, when(cape >= HoodCape::Shoulders, 20)),
    ]
}

impl Hood {
    pub(super) fn roll(rng: &mut Rng, quality: Quality) -> Self {
        let rank = quality >= Quality::Rare;
        let lappets = rng.pick_weighted(&LAPPETS);
        let chaperon = rng.pick_weighted(&chaperons(lappets));
        let cape = rng.pick_weighted(&capes(lappets, chaperon));
        Self {
            cape,
            tail: rng.pick_weighted(&tails(lappets, rank)),
            face: rng.pick_weighted(&faces(lappets, chaperon)),
            edge: rng.pick_weighted(&edges(cape)),
            lappets,
            chaperon,
        }
    }

    /// A tail to the ankle is allowed at any quality, though only rolled at Rare and
    /// better.
    pub(super) fn allowed(&self) -> bool {
        has(&LAPPETS, self.lappets)
            && has(&chaperons(self.lappets), self.chaperon)
            && has(&capes(self.lappets, self.chaperon), self.cape)
            && has(&tails(self.lappets, true), self.tail)
            && has(&faces(self.lappets, self.chaperon), self.face)
            && has(&edges(self.cape), self.edge)
    }

    /// The materials it can be made of, the usual one first.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        use Material::{Cotton, Felt, Linen, Silk, Wool};
        if self.lappets {
            return vec![(Wool, 60), (Felt, 40)];
        }
        vec![(Wool, 60), (Linen, 15), (Felt, 10), (Silk, 10), (Cotton, 5)]
    }

    /// Its name from the research's signatures; "Hood" when none fits.
    pub(super) fn name(&self) -> &'static str {
        if self.lappets {
            "Bashlyk"
        } else if self.chaperon {
            "Chaperon"
        } else if self.cape >= HoodCape::Shoulders && self.tail.long() {
            "Gugel"
        } else {
            "Hood"
        }
    }

    /// Grams in plain linen at an ordinary size.
    pub(super) fn base_grams(&self) -> u32 {
        let cape = match self.cape {
            HoodCape::None => 0,
            HoodCape::Neck => 50,
            HoodCape::Shoulders => 150,
            HoodCape::Chest => 300,
        };
        let tail = match self.tail {
            Tail::None => 0,
            Tail::Point => 10,
            Tail::Liripipe => 60,
            Tail::LongLiripipe => 150,
        };
        let lappets = if self.lappets { 120 } else { 0 };
        let roll = if self.chaperon { 60 } else { 0 };
        150 + cape + tail + lappets + roll
    }
}
