//! What a field's ground is, and how many stages a vegetable plant goes
//! through.

use shared::herb::STAGES;
use shared::site::FieldLook;

/// What a field's ground is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Bare earth, in furrows running north to south: the ground of every
    /// field of crops, its plants standing on it.
    Ploughed,
    /// Flat green.
    Grass,
    /// Grazed grass.
    Short,
}

impl Stage {
    const PASTURE: [Self; 2] = [Self::Grass, Self::Short];

    /// The stage of a field's ground. Pasture is only ever kept grazed, an
    /// orchard is grass under its fruit plants, and a field of crops is
    /// ploughed, its plants standing on it as they grow.
    pub fn of(field: &FieldLook) -> Self {
        if field.orchard {
            Self::Grass
        } else if field.livestock {
            Self::PASTURE[field.variety as usize % Self::PASTURE.len()]
        } else {
            Self::Ploughed
        }
    }
}

/// How many stages a vegetable plant goes through, dug up included (what
/// `common::CropGrowthShift` goes round).
pub const CROP_STAGES: usize = STAGES as usize + 1;
