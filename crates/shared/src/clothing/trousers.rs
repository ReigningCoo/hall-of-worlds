//! Trousers: two legs from the waist to the knee, calf or ankle: braccae, trews,
//! breeches, the wide salvar and sharovary, the hakama. On the Legs. Leg wraps wound
//! from ankle to knee are an option, not an item of their own (the research's
//! puttees and winingas).

use serde::{Deserialize, Serialize};

use super::Pattern;
use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrouserLength {
    Knee,
    Calf,
    Ankle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fullness {
    /// Close to the leg, cut on the bias (trews).
    Fitted,
    Loose,
    /// Very full and gathered (salvar, sharovary).
    VeryFull,
    /// Pleated and divided, to the ankle (the hakama).
    Pleated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrouserWaist {
    Drawstring,
    Buttons,
    /// Tied to a doublet's points (late medieval on).
    Points,
    /// The hakama's four straps.
    Ties,
}

/// How the legs end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegEnds {
    Open,
    GatheredAtAnkle,
    /// Buttoned or tied at the knee (breeches).
    AtKnee,
    /// A strap under each foot.
    Stirrups,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrouserFront {
    Plain,
    /// A buttoned flap (16th century on).
    FallFront,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Trousers {
    pub length: TrouserLength,
    pub fullness: Fullness,
    pub waist: TrouserWaist,
    pub ends: LegEnds,
    pub front: TrouserFront,
    /// Cloth strips wound from ankle to knee over them (early medieval).
    pub leg_wraps: bool,
}

const LENGTHS: [(TrouserLength, u64); 3] = [
    (TrouserLength::Knee, 20),
    (TrouserLength::Calf, 15),
    (TrouserLength::Ankle, 65),
];

fn fullnesses(length: TrouserLength) -> [(Fullness, u64); 4] {
    [
        (Fullness::Fitted, 30),
        (Fullness::Loose, 40),
        (Fullness::VeryFull, 18),
        (Fullness::Pleated, when(length == TrouserLength::Ankle, 12)),
    ]
}

fn waists(fullness: Fullness) -> [(TrouserWaist, u64); 4] {
    let pleated = fullness == Fullness::Pleated;
    [
        (TrouserWaist::Drawstring, when(!pleated, 50)),
        (TrouserWaist::Buttons, when(!pleated, 25)),
        (
            TrouserWaist::Points,
            when(!pleated && fullness != Fullness::VeryFull, 10),
        ),
        (TrouserWaist::Ties, when(pleated, 1)),
    ]
}

fn ends(length: TrouserLength, fullness: Fullness) -> [(LegEnds, u64); 4] {
    let ankle = length == TrouserLength::Ankle;
    let pleated = fullness == Fullness::Pleated;
    [
        (LegEnds::Open, 40),
        (LegEnds::GatheredAtAnkle, when(ankle && !pleated, 30)),
        (LegEnds::AtKnee, when(length == TrouserLength::Knee, 40)),
        (
            LegEnds::Stirrups,
            when(ankle && fullness == Fullness::Fitted, 10),
        ),
    ]
}

fn fronts(waist: TrouserWaist) -> [(TrouserFront, u64); 2] {
    [
        (TrouserFront::Plain, 85),
        (
            TrouserFront::FallFront,
            when(waist == TrouserWaist::Buttons, 15),
        ),
    ]
}

fn wraps(length: TrouserLength, fullness: Fullness) -> [(bool, u64); 2] {
    let slim = matches!(fullness, Fullness::Fitted | Fullness::Loose);
    [
        (false, 85),
        (true, when(slim && length != TrouserLength::Knee, 15)),
    ]
}

impl Trousers {
    pub(super) fn roll(rng: &mut Rng) -> Self {
        let length = rng.pick_weighted(&LENGTHS);
        let fullness = rng.pick_weighted(&fullnesses(length));
        let waist = rng.pick_weighted(&waists(fullness));
        let ends = rng.pick_weighted(&ends(length, fullness));
        let front = rng.pick_weighted(&fronts(waist));
        let leg_wraps = rng.pick_weighted(&wraps(length, fullness));
        Self {
            length,
            fullness,
            waist,
            ends,
            front,
            leg_wraps,
        }
    }

    pub(super) fn allowed(&self) -> bool {
        has(&LENGTHS, self.length)
            && has(&fullnesses(self.length), self.fullness)
            && has(&waists(self.fullness), self.waist)
            && has(&ends(self.length, self.fullness), self.ends)
            && has(&fronts(self.waist), self.front)
            && has(&wraps(self.length, self.fullness), self.leg_wraps)
    }

    /// The materials they can be made of, the usual one first.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        use Material::{Cotton, Leather, Linen, Silk, Wool};
        let leather = when(self.length == TrouserLength::Knee, 25) + 3;
        vec![
            (Wool, 45),
            (Linen, 25),
            (Cotton, 10),
            (Silk, 8),
            (Leather, leather),
        ]
    }

    /// Whether they are cut close enough for tartan trews.
    pub(super) fn fitted(&self) -> bool {
        self.fullness == Fullness::Fitted
    }

    /// Their name from the research's signatures; "Trousers" when none fits.
    pub(super) fn name(&self, material: Material, pattern: Pattern) -> &'static str {
        use Fullness::{Fitted, Loose, Pleated, VeryFull};
        let gathered =
            self.waist == TrouserWaist::Drawstring && self.ends == LegEnds::GatheredAtAnkle;
        match self.fullness {
            Pleated => "Hakama",
            Fitted if self.ends == LegEnds::Stirrups => "Thorsberg Trousers",
            _ if self.leather_shorts(material) => "Lederhosen",
            Fitted if self.length == TrouserLength::Ankle && pattern == Pattern::Tartan => "Trews",
            VeryFull if gathered => "Salvar",
            VeryFull => "Sharovary",
            Loose if gathered => "Braccae",
            _ if self.ends == LegEnds::AtKnee => "Breeches",
            _ => "Trousers",
        }
    }

    /// Leather to the knee with a fall front: lederhosen, whose name says leather.
    pub(super) fn leather_shorts(&self, material: Material) -> bool {
        material == Material::Leather
            && self.length == TrouserLength::Knee
            && self.front == TrouserFront::FallFront
    }

    /// Grams in plain linen at an ordinary size.
    pub(super) fn base_grams(&self) -> u32 {
        let legs = match self.length {
            TrouserLength::Knee => 300,
            TrouserLength::Calf => 400,
            TrouserLength::Ankle => 500,
        };
        let fullness = match self.fullness {
            Fullness::Fitted | Fullness::Loose => 0,
            Fullness::VeryFull => 250,
            Fullness::Pleated => 350,
        };
        legs + fullness + if self.leg_wraps { 80 } else { 0 }
    }
}
