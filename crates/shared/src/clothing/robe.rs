//! Robes: open in front and wrapped, tied, hooked or buttoned shut, usually with a
//! sash: the kosode and kimono, kaftan, deel, qaba and shenyi. On the Torso and Arms;
//! how long it is, is a property (owner, 2026-11-15).

use serde::{Deserialize, Serialize};

use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RobeLength {
    Knee,
    Calf,
    Ankle,
}

/// How the front closes over the body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Front {
    /// The left side over the right, as the living wear the kimono.
    WrapLeft,
    WrapRight,
    /// The edges meeting at the centre.
    Meeting,
    /// A curved flap overlapping to the right side (the deel's).
    CurvedFlap,
}

impl Front {
    pub fn wrapped(self) -> bool {
        matches!(self, Self::WrapLeft | Self::WrapRight)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fastening {
    /// Held by the sash alone.
    Sash,
    Ties,
    /// Buttons and loops.
    Buttons,
    /// Hooks or clasps at the side (the deel's).
    SideHooks,
    /// Knotted cord frogs.
    Frogs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RobeSleeves {
    /// Narrow to the wrist.
    Narrow,
    Wide,
    /// Square and hanging (the kimono's, 1600s).
    SquareHanging,
    /// Long and hanging, worn thrown back (1600s).
    LongHanging,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Collar {
    /// A wrapped band down the front edges (kimono, hanfu).
    Band,
    Standing,
    Round,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Lining {
    Unlined,
    Lined,
    /// Padded with silk floss or cotton for winter.
    Padded,
    /// Lined with fur (the deel's).
    Fur,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RobeDecoration {
    None,
    /// Cuffs and edges in a second colour.
    ContrastEdges,
    Embroidery,
    /// Woven or printed all over.
    AllOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Robe {
    pub length: RobeLength,
    pub front: Front,
    pub fastening: Fastening,
    pub sleeves: RobeSleeves,
    pub collar: Collar,
    pub lining: Lining,
    pub decoration: RobeDecoration,
}

const LENGTHS: [(RobeLength, u64); 3] = [
    (RobeLength::Knee, 20),
    (RobeLength::Calf, 35),
    (RobeLength::Ankle, 45),
];

const FRONTS: [(Front, u64); 4] = [
    (Front::WrapLeft, 30),
    (Front::WrapRight, 25),
    (Front::Meeting, 30),
    (Front::CurvedFlap, 15),
];

fn fastenings(front: Front) -> [(Fastening, u64); 5] {
    let flap = front == Front::CurvedFlap;
    [
        (Fastening::Sash, 30),
        (Fastening::Ties, when(!flap, 25)),
        (Fastening::Buttons, when(!flap, 25)),
        (Fastening::SideHooks, when(flap, 40)),
        (Fastening::Frogs, 10),
    ]
}

fn sleeves(front: Front) -> [(RobeSleeves, u64); 4] {
    [
        (RobeSleeves::Narrow, 45),
        (RobeSleeves::Wide, 35),
        (RobeSleeves::SquareHanging, when(front.wrapped(), 10)),
        (RobeSleeves::LongHanging, when(front == Front::Meeting, 4)),
    ]
}

fn collars(front: Front, sleeves: RobeSleeves) -> [(Collar, u64); 4] {
    let kimono = sleeves == RobeSleeves::SquareHanging;
    [
        (Collar::Band, when(front.wrapped(), 40)),
        (Collar::Standing, when(!kimono, 20)),
        (Collar::Round, when(!kimono, 20)),
        (Collar::None, when(!kimono, 20)),
    ]
}

fn linings(front: Front) -> [(Lining, u64); 4] {
    [
        (Lining::Unlined, 40),
        (Lining::Lined, 35),
        (Lining::Padded, 15),
        (Lining::Fur, when(front == Front::CurvedFlap, 25) + 3),
    ]
}

const DECORATIONS: [(RobeDecoration, u64); 4] = [
    (RobeDecoration::None, 45),
    (RobeDecoration::ContrastEdges, 25),
    (RobeDecoration::Embroidery, 20),
    (RobeDecoration::AllOver, 10),
];

impl Robe {
    pub(super) fn roll(rng: &mut Rng) -> Self {
        let length = rng.pick_weighted(&LENGTHS);
        let front = rng.pick_weighted(&FRONTS);
        let fastening = rng.pick_weighted(&fastenings(front));
        let sleeves = rng.pick_weighted(&sleeves(front));
        let collar = rng.pick_weighted(&collars(front, sleeves));
        let lining = rng.pick_weighted(&linings(front));
        let decoration = rng.pick_weighted(&DECORATIONS);
        Self {
            length,
            front,
            fastening,
            sleeves,
            collar,
            lining,
            decoration,
        }
    }

    pub(super) fn allowed(&self) -> bool {
        has(&LENGTHS, self.length)
            && has(&FRONTS, self.front)
            && has(&fastenings(self.front), self.fastening)
            && has(&sleeves(self.front), self.sleeves)
            && has(&collars(self.front, self.sleeves), self.collar)
            && has(&linings(self.front), self.lining)
            && has(&DECORATIONS, self.decoration)
    }

    /// Its name from the research's signatures; "Robe" when none fits.
    pub(super) fn name(&self) -> &'static str {
        let (length, sleeves, fastening) = (self.length, self.sleeves, self.fastening);
        if self.front == Front::CurvedFlap {
            return "Deel";
        }
        if sleeves == RobeSleeves::SquareHanging {
            return "Kimono";
        }
        if self.front.wrapped() {
            return match (length, sleeves, fastening) {
                (RobeLength::Ankle, RobeSleeves::Narrow, _)
                    if self.front == Front::WrapLeft && self.collar == Collar::Band =>
                {
                    "Kosode"
                }
                (RobeLength::Ankle, RobeSleeves::Wide, _) => "Shenyi",
                (RobeLength::Knee, _, Fastening::Ties) => "Angarkha",
                (RobeLength::Calf, _, Fastening::Buttons) => "Qaba",
                _ => "Robe",
            };
        }
        match fastening {
            Fastening::Buttons if length == RobeLength::Ankle => "Kaftan",
            Fastening::Sash if length == RobeLength::Ankle => "Khalat",
            _ => "Robe",
        }
    }

    /// Grams in plain linen at an ordinary size, any fur lining left out.
    pub(super) fn base_grams(&self) -> u32 {
        let body = match self.length {
            RobeLength::Knee => 700,
            RobeLength::Calf => 850,
            RobeLength::Ankle => 1000,
        };
        let sleeves = match self.sleeves {
            RobeSleeves::Narrow => 0,
            RobeSleeves::Wide => 150,
            RobeSleeves::SquareHanging => 200,
            RobeSleeves::LongHanging => 250,
        };
        let lining = match self.lining {
            Lining::Unlined | Lining::Fur => 0,
            Lining::Lined => 300,
            Lining::Padded => 800,
        };
        body + sleeves + lining
    }

    /// Grams a fur lining adds.
    pub(super) fn fur_grams(&self) -> u32 {
        match self.lining {
            Lining::Fur => match self.length {
                RobeLength::Knee => 1000,
                RobeLength::Calf => 1200,
                RobeLength::Ankle => 1400,
            },
            _ => 0,
        }
    }
}
