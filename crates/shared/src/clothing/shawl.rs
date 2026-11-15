//! Over the shoulders: shawls, partlets, fichus, the dupatta, capelets and tippets. On
//! the Shoulders; one with a collar covers the Neck too, and one without leaves it to
//! a hood, so a hood and a shawl go together (owner, 2026-11-15).

use serde::{Deserialize, Serialize};

use super::Pattern;
use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShawlForm {
    /// Draped.
    Shawl,
    /// Fitted to the neck and shoulders (16th century on).
    Partlet,
    /// A folded triangle tucked in (17th century).
    Fichu,
    /// Long, over one shoulder or both.
    Dupatta,
    /// Short, to the elbows.
    Capelet,
    /// A long hanging strip (late medieval).
    Tippet,
}

/// How far it hangs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ShawlLength {
    Chest,
    Waist,
    Knee,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShawlCollar {
    None,
    Standing,
    Ruffled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Shawl {
    pub form: ShawlForm,
    pub length: ShawlLength,
    pub collar: ShawlCollar,
}

const FORMS: [(ShawlForm, u64); 6] = [
    (ShawlForm::Shawl, 35),
    (ShawlForm::Partlet, 12),
    (ShawlForm::Fichu, 10),
    (ShawlForm::Dupatta, 15),
    (ShawlForm::Capelet, 18),
    (ShawlForm::Tippet, 10),
];

fn lengths(form: ShawlForm) -> [(ShawlLength, u64); 3] {
    use ShawlForm::{Capelet, Dupatta, Fichu, Partlet, Shawl, Tippet};
    [
        (
            ShawlLength::Chest,
            when(matches!(form, Shawl | Partlet | Fichu | Capelet), 40),
        ),
        (
            ShawlLength::Waist,
            when(matches!(form, Shawl | Dupatta | Tippet), 40),
        ),
        (
            ShawlLength::Knee,
            when(matches!(form, Shawl | Dupatta | Tippet), 20),
        ),
    ]
}

fn collars(form: ShawlForm) -> [(ShawlCollar, u64); 3] {
    use ShawlForm::{Capelet, Fichu, Partlet};
    [
        (ShawlCollar::None, when(form != Partlet, 70)),
        (ShawlCollar::Standing, when(form == Partlet, 50)),
        (
            ShawlCollar::Ruffled,
            when(matches!(form, Partlet | Fichu | Capelet), 25),
        ),
    ]
}

impl Shawl {
    pub(super) fn roll(rng: &mut Rng) -> Self {
        let form = rng.pick_weighted(&FORMS);
        Self {
            form,
            length: rng.pick_weighted(&lengths(form)),
            collar: rng.pick_weighted(&collars(form)),
        }
    }

    pub(super) fn allowed(&self) -> bool {
        has(&FORMS, self.form)
            && has(&lengths(self.form), self.length)
            && has(&collars(self.form), self.collar)
    }

    /// The materials it can be made of, the usual one first.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        use Material::{Cotton, Fur, Linen, Silk, Wool};
        match self.form {
            ShawlForm::Shawl => vec![(Wool, 60), (Silk, 15), (Cotton, 15), (Linen, 10)],
            ShawlForm::Partlet => vec![(Linen, 60), (Silk, 40)],
            ShawlForm::Fichu => vec![(Linen, 50), (Cotton, 30), (Silk, 20)],
            ShawlForm::Dupatta => vec![(Cotton, 50), (Silk, 40), (Linen, 10)],
            ShawlForm::Capelet => vec![(Wool, 70), (Silk, 10), (Linen, 10), (Fur, 10)],
            ShawlForm::Tippet => vec![(Fur, 50), (Silk, 25), (Wool, 25)],
        }
    }

    /// Whether it may be striped or checked.
    pub(super) fn patterned(&self) -> bool {
        matches!(self.form, ShawlForm::Shawl | ShawlForm::Dupatta)
    }

    /// Its name from the research's signatures: a long cotton shawl, often striped, is
    /// a Rebozo.
    pub(super) fn name(&self, material: Material, pattern: Pattern) -> &'static str {
        match self.form {
            ShawlForm::Shawl if self.rebozo(material, pattern) => "Rebozo",
            ShawlForm::Shawl => "Shawl",
            ShawlForm::Partlet => "Partlet",
            ShawlForm::Fichu => "Fichu",
            ShawlForm::Dupatta => "Dupatta",
            ShawlForm::Capelet => "Capelet",
            ShawlForm::Tippet => "Tippet",
        }
    }

    fn rebozo(&self, material: Material, pattern: Pattern) -> bool {
        material == Material::Cotton
            && self.length >= ShawlLength::Waist
            && matches!(pattern, Pattern::Plain | Pattern::Striped)
    }

    /// Grams in plain linen at an ordinary size.
    pub(super) fn base_grams(&self) -> u32 {
        let cloth = match (self.form, self.length) {
            (ShawlForm::Shawl, ShawlLength::Chest) => 150,
            (ShawlForm::Shawl, ShawlLength::Waist) => 300,
            (ShawlForm::Shawl, ShawlLength::Knee) => 450,
            (ShawlForm::Dupatta, ShawlLength::Knee) => 350,
            (ShawlForm::Dupatta, _) => 250,
            (ShawlForm::Partlet, _) => 100,
            (ShawlForm::Fichu, _) => 60,
            (ShawlForm::Capelet, _) => 220,
            (ShawlForm::Tippet, ShawlLength::Knee) => 150,
            (ShawlForm::Tippet, _) => 100,
        };
        let collar = match self.collar {
            ShawlCollar::None => 0,
            ShawlCollar::Standing => 20,
            ShawlCollar::Ruffled => 30,
        };
        cloth + collar
    }

    /// Whether it leaves the neck bare: one without a collar.
    pub(super) fn neck_bare(&self) -> bool {
        self.collar == ShawlCollar::None
    }
}
