//! Cloth draped, tied or wound round the head: veils, kerchiefs, turbans, head ties,
//! the nemes and the square headcloth held by a cord ring. On the Head, and the Neck
//! too when it reaches the shoulders. Turban and veil forms keep plain names, never
//! dastar, niqab or burqa, and the corded cloth is a Headcloth, not a keffiyeh (owner,
//! 2026-11-15).

use serde::{Deserialize, Serialize};

use crate::material::Material;
use crate::rng::Rng;
use crate::underclothes::table::{has, when};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeadclothForm {
    /// Draped and pinned.
    Veil,
    /// A square or triangle, tied.
    Kerchief,
    /// Wound.
    Turban,
    /// Folded and pinned high.
    HeadTie,
    /// The striped lappeted headcloth (ancient).
    Nemes,
    /// A square cloth held by a cord ring.
    Corded,
}

/// How far it hangs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Reach {
    Head,
    Shoulders,
    /// To the waist (veils).
    Waist,
}

/// How much cloth a turban is wound from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TurbanSize {
    Small,
    /// One to four metres.
    Full,
    /// Up to ten metres.
    Great,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Headcloth {
    pub form: HeadclothForm,
    pub reach: Reach,
    /// The mouth and nose covered too (the litham).
    pub face_covered: bool,
    /// A turban's size; `None` for the rest.
    pub turban: Option<TurbanSize>,
    /// Of lace (a veil, 16th century on: the mantilla).
    pub lace: bool,
}

const FORMS: [(HeadclothForm, u64); 6] = [
    (HeadclothForm::Veil, 30),
    (HeadclothForm::Kerchief, 25),
    (HeadclothForm::Turban, 20),
    (HeadclothForm::HeadTie, 10),
    (HeadclothForm::Nemes, 5),
    (HeadclothForm::Corded, 10),
];

fn reaches(form: HeadclothForm) -> [(Reach, u64); 3] {
    use HeadclothForm::{Corded, HeadTie, Kerchief, Nemes, Turban, Veil};
    [
        (
            Reach::Head,
            when(matches!(form, Veil | Kerchief | Turban | HeadTie), 50),
        ),
        (
            Reach::Shoulders,
            when(matches!(form, Veil | Kerchief | Nemes | Corded), 40),
        ),
        (Reach::Waist, when(form == Veil, 15)),
    ]
}

fn faces(form: HeadclothForm) -> [(bool, u64); 2] {
    use HeadclothForm::{Corded, Turban, Veil};
    [
        (false, 85),
        (true, when(matches!(form, Veil | Turban | Corded), 15)),
    ]
}

fn turbans(form: HeadclothForm) -> [(Option<TurbanSize>, u64); 4] {
    let turban = form == HeadclothForm::Turban;
    [
        (None, when(!turban, 1)),
        (Some(TurbanSize::Small), when(turban, 30)),
        (Some(TurbanSize::Full), when(turban, 55)),
        (Some(TurbanSize::Great), when(turban, 15)),
    ]
}

fn laces(form: HeadclothForm, reach: Reach, face_covered: bool) -> [(bool, u64); 2] {
    let veil = form == HeadclothForm::Veil && reach >= Reach::Shoulders && !face_covered;
    [(false, 90), (true, when(veil, 10))]
}

impl Headcloth {
    pub(super) fn roll(rng: &mut Rng) -> Self {
        let form = rng.pick_weighted(&FORMS);
        let reach = rng.pick_weighted(&reaches(form));
        let face_covered = rng.pick_weighted(&faces(form));
        Self {
            form,
            reach,
            face_covered,
            turban: rng.pick_weighted(&turbans(form)),
            lace: rng.pick_weighted(&laces(form, reach, face_covered)),
        }
    }

    pub(super) fn allowed(&self) -> bool {
        has(&FORMS, self.form)
            && has(&reaches(self.form), self.reach)
            && has(&faces(self.form), self.face_covered)
            && has(&turbans(self.form), self.turban)
            && has(&laces(self.form, self.reach, self.face_covered), self.lace)
    }

    /// The materials it can be made of, the usual one first. Lace is silk or linen.
    pub(super) fn materials(&self) -> Vec<(Material, u64)> {
        use Material::{Cotton, Linen, Silk, Wool};
        match self.form {
            _ if self.lace => vec![(Silk, 70), (Linen, 30)],
            HeadclothForm::Veil => vec![(Linen, 50), (Silk, 35), (Cotton, 15)],
            HeadclothForm::Kerchief => vec![(Linen, 50), (Cotton, 30), (Silk, 10), (Wool, 10)],
            HeadclothForm::Turban => vec![(Cotton, 50), (Silk, 25), (Linen, 15), (Wool, 10)],
            HeadclothForm::HeadTie => vec![(Cotton, 70), (Silk, 30)],
            HeadclothForm::Nemes => vec![(Linen, 1)],
            HeadclothForm::Corded => vec![(Cotton, 60), (Wool, 25), (Linen, 15)],
        }
    }

    /// Whether it may be striped or checked.
    pub(super) fn patterned(&self) -> bool {
        use HeadclothForm::{Corded, HeadTie, Kerchief, Nemes, Turban};
        !self.lace && matches!(self.form, Kerchief | Turban | HeadTie | Nemes | Corded)
    }

    /// Its name from the research's signatures.
    pub(super) fn name(&self) -> &'static str {
        match self.form {
            _ if self.face_covered => "Litham",
            HeadclothForm::Veil if self.lace => "Mantilla",
            HeadclothForm::Veil => "Veil",
            HeadclothForm::Kerchief => "Kerchief",
            HeadclothForm::Turban => "Turban",
            HeadclothForm::HeadTie => "Head Tie",
            HeadclothForm::Nemes => "Nemes",
            HeadclothForm::Corded => "Headcloth",
        }
    }

    /// Grams in plain linen at an ordinary size.
    pub(super) fn base_grams(&self) -> u32 {
        let reach = match self.reach {
            Reach::Head => 0,
            Reach::Shoulders => 40,
            Reach::Waist => 110,
        };
        let cloth = match (self.form, self.turban) {
            (_, Some(TurbanSize::Small)) => 120,
            (_, Some(TurbanSize::Full)) => 250,
            (_, Some(TurbanSize::Great)) => 550,
            (HeadclothForm::HeadTie, _) => 100,
            (HeadclothForm::Nemes | HeadclothForm::Corded, _) => 110,
            _ => 40,
        };
        let face = if self.face_covered { 20 } else { 0 };
        cloth + reach + face
    }

    /// Turbans take two cells.
    pub(super) fn wide(&self) -> bool {
        self.form == HeadclothForm::Turban
    }

    /// Whether it leaves the neck bare: one that stays on the head.
    pub(super) fn neck_bare(&self) -> bool {
        self.reach == Reach::Head
    }
}
