//! How a beast's males and females differ: their horns, an udder, and how big
//! they are. Only for looks: a creature's `Size`, weight and footprint are its
//! kind's whatever its gender.

use super::super::Layout;
use super::super::blend::WHOLE;
use super::super::parts::Horns;
use super::Beast;
use crate::creature::CreatureKind;
use crate::gender::Gender;

/// What one sex has in place of the neutral body's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sex {
    pub horns: Horns,
    pub horn_length: i32,
    pub udder: i32,
    /// How big it is, in thousandths of the neutral body.
    pub size: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sexes {
    pub male: Sex,
    pub female: Sex,
}

/// Highland cattle (owner): bulls about 125 cm at the withers and cows about 105
/// (the neutral body 115); cows' horns longer and sweeping up at the tips,
/// bulls' thicker and pointing forwards.
pub(super) const COO: Sexes = Sexes {
    male: Sex {
        horns: Horns::Forward,
        horn_length: 450,
        udder: 0,
        size: 1087,
    },
    female: Sex {
        horns: Horns::Upturned,
        horn_length: 580,
        udder: 140,
        size: 913,
    },
};

impl Layout {
    /// A kind's body as `Layout::of`, for a creature of this gender: a beast
    /// whose sexes differ has its sex's horns and size.
    pub fn of_gender(kind: CreatureKind, gender: Gender) -> Option<Layout> {
        let layout = Layout::of(kind)?;
        let Some(sexes) = Beast::of(kind).and_then(|beast| beast.sexes) else {
            return Some(layout);
        };
        let sex = match gender {
            Gender::Male => sexes.male,
            Gender::Female => sexes.female,
            Gender::Neutral => return Some(layout),
        };
        let mut sexed = layout;
        sexed.parts.horns = sex.horns;
        sexed.parts.horn_length = sex.horn_length;
        sexed.parts.udder = sex.udder;
        Some(match sex.size {
            WHOLE => sexed,
            size => sexed.scaled(size),
        })
    }
}

#[cfg(test)]
mod tests;
