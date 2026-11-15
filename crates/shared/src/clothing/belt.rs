//! Belts: strap belts, cord girdles, sashes and the obi, one kind rolled into forms
//! (owner, 2026-11-15). On the Waist; every form has the belt's bag slots. The
//! baldric waits for sheaths.

use serde::{Deserialize, Serialize};

use crate::item::Quality;
use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeltForm {
    /// A strap, buckled or tied.
    Strap,
    /// A cord, knotted or wound twice (the girdle).
    Cord,
    /// Cloth wound round, the ends hanging.
    Sash,
    /// Wide, wound and tied behind.
    Obi,
    /// Metal plates on a strap (high and late medieval, for rank): rolled at Rare and
    /// better.
    Plates,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeltWidth {
    Narrow,
    Wide,
    /// Sashes and the obi.
    VeryWide,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeltEnd {
    Short,
    /// A long tongue hanging to the knee (late medieval).
    Long,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fittings {
    None,
    Studs,
    /// Metal plates along it: the plated belt's.
    Plates,
    Chain,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeltFastening {
    Buckle,
    Tied,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Belt {
    pub form: BeltForm,
    pub width: BeltWidth,
    pub end: BeltEnd,
    pub fittings: Fittings,
    pub fastening: BeltFastening,
    /// A pleated sash (the cummerbund, 16th century on).
    pub pleated: bool,
}

fn forms(quality: Quality) -> [(BeltForm, u64); 5] {
    [
        (BeltForm::Strap, 45),
        (BeltForm::Cord, 20),
        (BeltForm::Sash, 20),
        (BeltForm::Obi, 8),
        (BeltForm::Plates, when(quality >= Quality::Rare, 10)),
    ]
}

/// Every form, for the grid: plated belts are allowed at any quality.
const ANY_FORM: [(BeltForm, u64); 5] = [
    (BeltForm::Strap, 1),
    (BeltForm::Cord, 1),
    (BeltForm::Sash, 1),
    (BeltForm::Obi, 1),
    (BeltForm::Plates, 1),
];

fn widths(form: BeltForm) -> [(BeltWidth, u64); 3] {
    use BeltForm::*;
    let (narrow, wide, very) = match form {
        Strap => (60, 40, 0),
        Cord => (1, 0, 0),
        Sash => (0, 60, 40),
        Obi => (0, 0, 1),
        Plates => (0, 1, 0),
    };
    [
        (BeltWidth::Narrow, narrow),
        (BeltWidth::Wide, wide),
        (BeltWidth::VeryWide, very),
    ]
}

fn ends(form: BeltForm) -> [(BeltEnd, u64); 2] {
    let long = match form {
        BeltForm::Strap | BeltForm::Plates => 25,
        BeltForm::Cord => 50,
        BeltForm::Sash | BeltForm::Obi => 0,
    };
    [(BeltEnd::Short, 100 - long), (BeltEnd::Long, long)]
}

fn fittings(form: BeltForm) -> [(Fittings, u64); 4] {
    let strap = form == BeltForm::Strap;
    let plates = form == BeltForm::Plates;
    [
        (Fittings::None, when(!plates, 60)),
        (Fittings::Studs, when(strap, 25)),
        (Fittings::Plates, when(plates, 1)),
        (Fittings::Chain, when(strap, 15)),
    ]
}

fn fastenings(form: BeltForm) -> [(BeltFastening, u64); 2] {
    let (buckle, tied) = match form {
        BeltForm::Strap => (80, 20),
        BeltForm::Plates => (1, 0),
        BeltForm::Cord | BeltForm::Sash | BeltForm::Obi => (0, 1),
    };
    [(BeltFastening::Buckle, buckle), (BeltFastening::Tied, tied)]
}

fn pleats(form: BeltForm) -> [(bool, u64); 2] {
    [(false, 85), (true, when(form == BeltForm::Sash, 15))]
}

impl Belt {
    pub(super) fn roll(rng: &mut Rng, quality: Quality) -> Self {
        let form = rng.pick_weighted(&forms(quality));
        Self {
            form,
            width: rng.pick_weighted(&widths(form)),
            end: rng.pick_weighted(&ends(form)),
            fittings: rng.pick_weighted(&fittings(form)),
            fastening: rng.pick_weighted(&fastenings(form)),
            pleated: rng.pick_weighted(&pleats(form)),
        }
    }

    pub(super) fn allowed(&self) -> bool {
        let form = self.form;
        has(&ANY_FORM, form)
            && has(&widths(form), self.width)
            && has(&ends(form), self.end)
            && has(&fittings(form), self.fittings)
            && has(&fastenings(form), self.fastening)
            && has(&pleats(form), self.pleated)
    }

    /// The materials it can be made of, the usual one first.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        use Material::{Cotton, Hemp, Leather, Linen, Silk, Suede, Wool};
        match self.form {
            BeltForm::Strap | BeltForm::Plates => vec![(Leather, 80), (Suede, 20)],
            BeltForm::Cord => vec![(Wool, 40), (Linen, 30), (Silk, 20), (Hemp, 10)],
            BeltForm::Sash => vec![(Silk, 40), (Wool, 30), (Cotton, 20), (Linen, 10)],
            BeltForm::Obi => vec![(Silk, 70), (Cotton, 30)],
        }
    }

    /// Whether it is of cloth wide enough to be woven fine (brocade obi).
    pub(super) fn cloth(&self) -> bool {
        matches!(self.form, BeltForm::Sash | BeltForm::Obi)
    }

    /// Its name from the research's signatures; "Belt" when none fits.
    pub(super) fn name(&self) -> &'static str {
        match self.form {
            BeltForm::Cord => "Girdle",
            BeltForm::Sash if self.pleated => "Cummerbund",
            BeltForm::Sash => "Sash",
            BeltForm::Obi => "Obi",
            BeltForm::Strap if self.fastening == BeltFastening::Tied => "Zoster",
            BeltForm::Strap | BeltForm::Plates => "Belt",
        }
    }

    /// Grams in plain linen at an ordinary size, metal fittings left out.
    pub(super) fn base_grams(&self) -> u32 {
        let band = match (self.form, self.width) {
            (BeltForm::Cord, _) => 60,
            (BeltForm::Obi, _) => 350,
            (_, BeltWidth::Narrow) => 60,
            (_, BeltWidth::Wide) => 100,
            (_, BeltWidth::VeryWide) => 180,
        };
        band + if self.end == BeltEnd::Long { 30 } else { 0 }
    }

    /// Grams its metal adds: a buckle, studs, plates or chain.
    pub(super) fn metal_grams(&self) -> u32 {
        let buckle = if self.fastening == BeltFastening::Buckle {
            40
        } else {
            0
        };
        buckle
            + match self.fittings {
                Fittings::None => 0,
                Fittings::Studs => 40,
                Fittings::Plates => 400,
                Fittings::Chain => 150,
            }
    }
}
