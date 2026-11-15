//! Doublets: a fitted jacket to the waist or hip, quilted or lined, laced or buttoned
//! down the front: pourpoint, gipon, doublet, jeogori, dolman. One may be made for
//! arming, quilted with points to tie armour on, and goes under unlined plate as an
//! arming vest does (owner, 2026-11-15).

use serde::{Deserialize, Serialize};

use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DoubletLength {
    Waist,
    Hip,
    /// A short skirt below the waist to the thigh.
    Thigh,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DoubletFastening {
    /// Close-set buttons (late medieval on).
    Buttons,
    Lacing,
    Hooks,
    /// Ties (the jeogori's).
    Ties,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Padding {
    None,
    Lined,
    Quilted,
    /// Quilted, with points to tie armour on (owner, 2026-11-15).
    Arming,
    /// A padded, pointed belly (1500s).
    Peascod,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DoubletCollar {
    None,
    Standing,
    /// A wrapped band (the jeogori's).
    Band,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DoubletSleeves {
    Close,
    /// Puffed at the shoulder (1500s on).
    Puffed,
    /// Wings or rolls at the shoulder (1500s on).
    Wings,
    /// Laced on at the shoulder, to take off (1500s on).
    Detachable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DoubletDecoration {
    None,
    /// Slashed and pinked to show the lining (1500s on).
    Slashing,
    /// Braid frogging across the chest (the dolman's).
    Frogging,
    /// A lining in a second colour.
    ContrastLining,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Doublet {
    pub length: DoubletLength,
    pub padding: Padding,
    pub fastening: DoubletFastening,
    pub collar: DoubletCollar,
    pub sleeves: DoubletSleeves,
    pub decoration: DoubletDecoration,
    /// Eyelets at the waist for hose, trousers or armour to be tied to.
    pub points: bool,
}

const LENGTHS: [(DoubletLength, u64); 3] = [
    (DoubletLength::Waist, 35),
    (DoubletLength::Hip, 45),
    (DoubletLength::Thigh, 20),
];

const PADDINGS: [(Padding, u64); 5] = [
    (Padding::None, 15),
    (Padding::Lined, 35),
    (Padding::Quilted, 30),
    (Padding::Arming, 12),
    (Padding::Peascod, 4),
];

fn fastenings(padding: Padding) -> [(DoubletFastening, u64); 4] {
    let arming = padding == Padding::Arming;
    [
        (DoubletFastening::Buttons, 45),
        (DoubletFastening::Lacing, 35),
        (DoubletFastening::Hooks, when(!arming, 10)),
        (DoubletFastening::Ties, when(!arming, 10)),
    ]
}

fn collars(
    length: DoubletLength,
    padding: Padding,
    fastening: DoubletFastening,
) -> [(DoubletCollar, u64); 3] {
    let jeogori = length == DoubletLength::Waist
        && fastening == DoubletFastening::Ties
        && matches!(padding, Padding::None | Padding::Lined);
    [
        (DoubletCollar::None, 40),
        (DoubletCollar::Standing, 50),
        (DoubletCollar::Band, when(jeogori, 60)),
    ]
}

fn sleeves(padding: Padding, collar: DoubletCollar) -> [(DoubletSleeves, u64); 4] {
    // Arming doublets and the jeogori keep close sleeves.
    let fashion = padding != Padding::Arming && collar != DoubletCollar::Band;
    [
        (DoubletSleeves::Close, 70),
        (DoubletSleeves::Puffed, when(fashion, 10)),
        (DoubletSleeves::Wings, when(fashion, 10)),
        (DoubletSleeves::Detachable, when(fashion, 6)),
    ]
}

fn decorations(doublet: &Doublet) -> [(DoubletDecoration, u64); 4] {
    let fashion = doublet.padding != Padding::Arming && doublet.collar != DoubletCollar::Band;
    [
        (DoubletDecoration::None, 60),
        (DoubletDecoration::Slashing, when(fashion, 6)),
        (
            DoubletDecoration::Frogging,
            when(fashion && doublet.length == DoubletLength::Waist, 8),
        ),
        (DoubletDecoration::ContrastLining, when(fashion, 15)),
    ]
}

impl Doublet {
    /// Whether it is made to go under armour.
    pub fn arming(&self) -> bool {
        self.padding == Padding::Arming
    }

    /// Whether it may have points: always for arming, never on the jeogori.
    fn points_allowed(&self) -> bool {
        self.collar != DoubletCollar::Band
    }

    pub(super) fn roll(rng: &mut Rng) -> Self {
        let length = rng.pick_weighted(&LENGTHS);
        let padding = rng.pick_weighted(&PADDINGS);
        let fastening = rng.pick_weighted(&fastenings(padding));
        let collar = rng.pick_weighted(&collars(length, padding, fastening));
        let sleeves = rng.pick_weighted(&sleeves(padding, collar));
        let mut doublet = Self {
            length,
            padding,
            fastening,
            collar,
            sleeves,
            decoration: DoubletDecoration::None,
            points: false,
        };
        doublet.decoration = rng.pick_weighted(&decorations(&doublet));
        doublet.points = doublet.arming() || (doublet.points_allowed() && rng.range_u64(0, 3) == 0);
        doublet
    }

    pub(super) fn allowed(&self) -> bool {
        has(&LENGTHS, self.length)
            && has(&PADDINGS, self.padding)
            && has(&fastenings(self.padding), self.fastening)
            && has(
                &collars(self.length, self.padding, self.fastening),
                self.collar,
            )
            && has(&sleeves(self.padding, self.collar), self.sleeves)
            && has(&decorations(self), self.decoration)
            && (self.points || !self.arming())
            && (!self.points || self.points_allowed())
    }

    /// Its name from the research's signatures; "Doublet" when none fits.
    pub(super) fn name(&self) -> &'static str {
        match (self.padding, self.length) {
            (Padding::Arming, _) => "Arming Doublet",
            _ if self.collar == DoubletCollar::Band => "Jeogori",
            _ if self.decoration == DoubletDecoration::Frogging => "Dolman",
            (Padding::Quilted, DoubletLength::Hip) => "Pourpoint",
            (Padding::Quilted, DoubletLength::Thigh) => "Gipon",
            _ => "Doublet",
        }
    }

    /// Grams in plain linen at an ordinary size.
    pub(super) fn base_grams(&self) -> u32 {
        let body = match self.length {
            DoubletLength::Waist => 350,
            DoubletLength::Hip => 400,
            DoubletLength::Thigh => 450,
        };
        let padding = match self.padding {
            Padding::None => 0,
            Padding::Lined => 150,
            Padding::Quilted => 500,
            Padding::Arming => 700,
            Padding::Peascod => 600,
        };
        let sleeves = match self.sleeves {
            DoubletSleeves::Close => 0,
            DoubletSleeves::Puffed => 80,
            DoubletSleeves::Wings => 60,
            DoubletSleeves::Detachable => 40,
        };
        body + padding + sleeves
    }
}
