//! Stiffened headdresses of the late Middle Ages and on: the hennin, escoffion, French
//! and gable hoods, and the kokoshnik. On the Head. Buckram, paste board and wire
//! stiffen them inside, so they are not materials of their own.

use serde::{Deserialize, Serialize};

use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeaddressForm {
    /// A steeple, up to 46 cm.
    Steeple,
    /// A truncated cone.
    Flowerpot,
    /// Horned rolls at the sides.
    Horned,
    /// A crescent band on a cap (16th century on).
    FrenchHood,
    /// A pointed gable over the face (16th century on).
    Gable,
    /// A crest rising over the brow.
    Kokoshnik,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeaddressVeil {
    None,
    /// Hanging from the top.
    FromTop,
    Behind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeaddressDecoration {
    None,
    Pearls,
    /// A jewelled band (billaments).
    Billaments,
    GoldNet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Headdress {
    pub form: HeaddressForm,
    pub veil: HeaddressVeil,
    pub decoration: HeaddressDecoration,
}

const FORMS: [(HeaddressForm, u64); 6] = [
    (HeaddressForm::Steeple, 20),
    (HeaddressForm::Flowerpot, 15),
    (HeaddressForm::Horned, 20),
    (HeaddressForm::FrenchHood, 12),
    (HeaddressForm::Gable, 10),
    (HeaddressForm::Kokoshnik, 15),
];

fn veils(form: HeaddressForm) -> [(HeaddressVeil, u64); 3] {
    use HeaddressForm::{Flowerpot, FrenchHood, Gable, Steeple};
    let hood = matches!(form, FrenchHood | Gable);
    [
        (HeaddressVeil::None, when(!hood, 30)),
        (
            HeaddressVeil::FromTop,
            when(matches!(form, Steeple | Flowerpot), 50),
        ),
        (HeaddressVeil::Behind, when(form != Steeple, 40)),
    ]
}

fn decorations(form: HeaddressForm) -> [(HeaddressDecoration, u64); 4] {
    use HeaddressForm::{FrenchHood, Gable, Horned, Kokoshnik};
    [
        (HeaddressDecoration::None, 50),
        (
            HeaddressDecoration::Pearls,
            20 + when(form == Kokoshnik, 30),
        ),
        (
            HeaddressDecoration::Billaments,
            when(matches!(form, FrenchHood | Gable), 30),
        ),
        (HeaddressDecoration::GoldNet, when(form == Horned, 30)),
    ]
}

impl Headdress {
    pub(super) fn roll(rng: &mut Rng) -> Self {
        let form = rng.pick_weighted(&FORMS);
        Self {
            form,
            veil: rng.pick_weighted(&veils(form)),
            decoration: rng.pick_weighted(&decorations(form)),
        }
    }

    pub(super) fn allowed(&self) -> bool {
        has(&FORMS, self.form)
            && has(&veils(self.form), self.veil)
            && has(&decorations(self.form), self.decoration)
    }

    /// The materials its cover can be made of, the usual one first.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        use Material::{Linen, Silk, Wool};
        vec![(Silk, 55), (Linen, 30), (Wool, 15)]
    }

    /// Its name from the research's signatures.
    pub(super) fn name(&self) -> &'static str {
        match self.form {
            HeaddressForm::Steeple | HeaddressForm::Flowerpot => "Hennin",
            HeaddressForm::Horned => "Escoffion",
            HeaddressForm::FrenchHood => "French Hood",
            HeaddressForm::Gable => "Gable Hood",
            HeaddressForm::Kokoshnik => "Kokoshnik",
        }
    }

    /// Grams in plain linen at an ordinary size, its stiffening included, its
    /// decoration left out.
    pub(super) fn base_grams(&self) -> u32 {
        let form = match self.form {
            HeaddressForm::Steeple => 300,
            HeaddressForm::Flowerpot => 250,
            HeaddressForm::Horned => 350,
            HeaddressForm::FrenchHood => 200,
            HeaddressForm::Gable => 400,
            HeaddressForm::Kokoshnik => 300,
        };
        let veil = match self.veil {
            HeaddressVeil::None => 0,
            HeaddressVeil::FromTop | HeaddressVeil::Behind => 60,
        };
        form + veil
    }

    /// Grams its pearls, jewels or gold add.
    pub(super) fn decoration_grams(&self) -> u32 {
        match self.decoration {
            HeaddressDecoration::None => 0,
            HeaddressDecoration::Pearls => 40,
            HeaddressDecoration::Billaments => 60,
            HeaddressDecoration::GoldNet => 30,
        }
    }
}
