//! Neckwear: scarves and kerchiefs at the throat, the cravat, the ruff, the falling
//! band, the jabot, and the bandana tied up over the mouth and nose. On the Neck, over
//! any necklace (which is on the Skin).

use serde::{Deserialize, Serialize};

use crate::item::Quality;
use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NeckwearForm {
    /// Wound round the neck.
    Scarf,
    /// A triangle, knotted.
    Neckerchief,
    /// A long strip, tied (17th century).
    Cravat,
    /// Figure-eight pleats (16th century on).
    Ruff,
    /// A flat collar (17th century).
    FallingBand,
    /// A lace fall (17th century).
    Jabot,
    /// A neckerchief tied up over the mouth and nose, knotted behind: it keeps out
    /// spores and dust (`Item::covers_mouth`).
    Bandana,
}

/// How wide a ruff stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuffSize {
    Narrow,
    Wide,
    /// On a wire frame (rank): rolled at Rare and better.
    Cartwheel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Neckwear {
    pub form: NeckwearForm,
    /// A ruff's size; `None` for the rest.
    pub ruff: Option<RuffSize>,
}

/// Later pieces rarer.
const FORMS: [(NeckwearForm, u64); 7] = [
    (NeckwearForm::Scarf, 35),
    (NeckwearForm::Neckerchief, 30),
    (NeckwearForm::Cravat, 8),
    (NeckwearForm::Ruff, 12),
    (NeckwearForm::FallingBand, 8),
    (NeckwearForm::Jabot, 7),
    (NeckwearForm::Bandana, 10),
];

/// A ruff's size; `rank` allows the cartwheel, only rolled at Rare and better.
fn ruffs(form: NeckwearForm, rank: bool) -> [(Option<RuffSize>, u64); 4] {
    let ruff = form == NeckwearForm::Ruff;
    [
        (None, when(!ruff, 1)),
        (Some(RuffSize::Narrow), when(ruff, 55)),
        (Some(RuffSize::Wide), when(ruff, 35)),
        (Some(RuffSize::Cartwheel), when(ruff && rank, 15)),
    ]
}

impl Neckwear {
    pub(super) fn roll(rng: &mut Rng, quality: Quality) -> Self {
        let form = rng.pick_weighted(&FORMS);
        Self {
            form,
            ruff: rng.pick_weighted(&ruffs(form, quality >= Quality::Rare)),
        }
    }

    /// A cartwheel ruff is allowed at any quality, though only rolled at Rare and better.
    pub(super) fn allowed(&self) -> bool {
        has(&FORMS, self.form) && has(&ruffs(self.form, true), self.ruff)
    }

    /// The materials it can be made of, the usual one first.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        use Material::{Cotton, Linen, Silk, Wool};
        match self.form {
            NeckwearForm::Scarf => vec![(Wool, 50), (Silk, 25), (Linen, 15), (Cotton, 10)],
            NeckwearForm::Neckerchief => vec![(Cotton, 40), (Linen, 30), (Silk, 30)],
            NeckwearForm::Bandana => vec![(Cotton, 60), (Linen, 25), (Silk, 15)],
            NeckwearForm::Cravat | NeckwearForm::Jabot => vec![(Linen, 60), (Silk, 40)],
            NeckwearForm::Ruff | NeckwearForm::FallingBand => vec![(Linen, 1)],
        }
    }

    /// Whether it may be striped or checked.
    pub(super) fn patterned(&self) -> bool {
        matches!(
            self.form,
            NeckwearForm::Scarf | NeckwearForm::Neckerchief | NeckwearForm::Bandana
        )
    }

    /// Its name from the research's signatures.
    pub(super) fn name(&self) -> &'static str {
        match self.form {
            NeckwearForm::Scarf => "Scarf",
            NeckwearForm::Neckerchief => "Neckerchief",
            NeckwearForm::Cravat => "Cravat",
            NeckwearForm::Ruff => "Ruff",
            NeckwearForm::FallingBand => "Falling Band",
            NeckwearForm::Jabot => "Jabot",
            NeckwearForm::Bandana => "Bandana",
        }
    }

    /// Grams in plain linen at an ordinary size, a cartwheel's wire included.
    pub(super) fn base_grams(&self) -> u32 {
        match (self.form, self.ruff) {
            (_, Some(RuffSize::Narrow)) => 60,
            (_, Some(RuffSize::Wide)) => 120,
            (_, Some(RuffSize::Cartwheel)) => 250,
            (NeckwearForm::Scarf, _) => 120,
            (NeckwearForm::Cravat, _) => 50,
            _ => 40,
        }
    }
}
