//! Bodices: a fitted upper body to the waist, laced, often stiffened, with sleeves
//! laced on or none: the bodice, the choli, the pair of bodies. On the Torso, and the
//! Arms only with sleeves.

use serde::{Deserialize, Serialize};

use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Lacing {
    Front,
    Side,
    Back,
    /// Tied at the back (the choli's).
    Ties,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BodiceNeck {
    Square,
    Round,
    High,
    /// Low, for a partlet or fichu over it.
    Low,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BodiceSleeves {
    /// Shoulder straps: the arms are bare.
    None,
    /// Short (the choli's).
    Short,
    /// Laced on at the shoulder (1500s on).
    LacedOn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stiffening {
    None,
    Lined,
    /// Boned with reed or whalebone (1500s on).
    Boned,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BodiceLength {
    /// Just below the breasts (the choli's).
    Chest,
    Waist,
    /// Below the waist in tabs (1500s on).
    Tabs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bodice {
    pub length: BodiceLength,
    pub lacing: Lacing,
    pub neck: BodiceNeck,
    pub sleeves: BodiceSleeves,
    pub stiffening: Stiffening,
    /// A stomacher panel pinned in at the front (1500s on).
    pub stomacher: bool,
}

const LENGTHS: [(BodiceLength, u64); 3] = [
    (BodiceLength::Chest, 12),
    (BodiceLength::Waist, 75),
    (BodiceLength::Tabs, 13),
];

fn lacings(length: BodiceLength) -> [(Lacing, u64); 4] {
    let choli = length == BodiceLength::Chest;
    [
        (Lacing::Front, when(!choli, 45)),
        (Lacing::Side, when(!choli, 20)),
        (Lacing::Back, when(!choli, 25)),
        (Lacing::Ties, when(choli, 1)),
    ]
}

fn necks(length: BodiceLength) -> [(BodiceNeck, u64); 4] {
    let choli = length == BodiceLength::Chest;
    [
        (BodiceNeck::Square, when(!choli, 35)),
        (BodiceNeck::Round, 30),
        (BodiceNeck::High, when(!choli, 10)),
        (BodiceNeck::Low, 25),
    ]
}

fn sleeves(length: BodiceLength) -> [(BodiceSleeves, u64); 3] {
    let choli = length == BodiceLength::Chest;
    [
        (BodiceSleeves::None, when(!choli, 55)),
        (BodiceSleeves::Short, when(choli, 1)),
        (BodiceSleeves::LacedOn, when(!choli, 45)),
    ]
}

fn stiffenings(length: BodiceLength) -> [(Stiffening, u64); 3] {
    let choli = length == BodiceLength::Chest;
    [
        (Stiffening::None, 30),
        (Stiffening::Lined, 50),
        (Stiffening::Boned, when(!choli, 20)),
    ]
}

impl Bodice {
    fn stomacher_allowed(&self) -> bool {
        self.lacing == Lacing::Front && self.length != BodiceLength::Chest
    }

    pub(super) fn roll(rng: &mut Rng) -> Self {
        let length = rng.pick_weighted(&LENGTHS);
        let lacing = rng.pick_weighted(&lacings(length));
        let neck = rng.pick_weighted(&necks(length));
        let sleeves = rng.pick_weighted(&sleeves(length));
        let stiffening = rng.pick_weighted(&stiffenings(length));
        let mut bodice = Self {
            length,
            lacing,
            neck,
            sleeves,
            stiffening,
            stomacher: false,
        };
        bodice.stomacher = bodice.stomacher_allowed() && rng.range_u64(0, 4) == 0;
        bodice
    }

    pub(super) fn allowed(&self) -> bool {
        has(&LENGTHS, self.length)
            && has(&lacings(self.length), self.lacing)
            && has(&necks(self.length), self.neck)
            && has(&sleeves(self.length), self.sleeves)
            && has(&stiffenings(self.length), self.stiffening)
            && (!self.stomacher || self.stomacher_allowed())
    }

    /// Its name from the research's signatures; "Bodice" when none fits.
    pub(super) fn name(&self) -> &'static str {
        match (self.length, self.stiffening) {
            (BodiceLength::Chest, _) => "Choli",
            (_, Stiffening::Boned) => "Pair of Bodies",
            _ => "Bodice",
        }
    }

    /// Grams in plain linen at an ordinary size.
    pub(super) fn base_grams(&self) -> u32 {
        let body = match self.length {
            BodiceLength::Chest => 120,
            BodiceLength::Waist => 220,
            BodiceLength::Tabs => 260,
        };
        let sleeves = match self.sleeves {
            BodiceSleeves::None => 0,
            BodiceSleeves::Short => 30,
            BodiceSleeves::LacedOn => 100,
        };
        let stiffening = match self.stiffening {
            Stiffening::None => 0,
            Stiffening::Lined => 100,
            Stiffening::Boned => 350,
        };
        let stomacher = if self.stomacher { 60 } else { 0 };
        body + sleeves + stiffening + stomacher
    }
}
