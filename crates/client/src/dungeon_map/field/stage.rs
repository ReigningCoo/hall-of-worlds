//! How far a field or a vegetable plant has grown, and what stands there at
//! each stage.

use shared::herb::STAGES;
use shared::site::FieldLook;

/// How far a field or a vegetable plant has grown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Bare earth, in furrows running north to south: the ground of every
    /// field of crops, with nothing standing on it for a plant just sown or
    /// dug up.
    Ploughed,
    /// Flat green.
    Grass,
    /// Green, and 6 pixels tall.
    Short,
    /// Turning yellow-green, and 12 pixels tall.
    Tall,
    /// Yellow and ready to harvest, 18 pixels tall.
    Ripe,
}

impl Stage {
    const PASTURE: [Self; 2] = [Self::Grass, Self::Short];

    /// The stage of a field's ground. Pasture is only ever kept grazed, an
    /// orchard is grass under its fruit plants, and a field of crops is
    /// ploughed, its crops standing on it by their own growth (`of_plant`).
    pub fn of(field: &FieldLook) -> Self {
        if field.orchard {
            Self::Grass
        } else if field.livestock {
            Self::PASTURE[field.variety as usize % Self::PASTURE.len()]
        } else {
            Self::Ploughed
        }
    }

    /// What stands for a vegetable plant at `stage` (`VegetablePlant::stage`,
    /// 0 for dug up to `herb::STAGES` full grown), until plants have models
    /// of their own: nothing just sown, then short, tall and ripe crops.
    pub fn of_plant(stage: u8) -> Self {
        match stage {
            0 | 1 => Self::Ploughed,
            2 => Self::Short,
            STAGES => Self::Ripe,
            _ => Self::Tall,
        }
    }

    /// What stands in the field: which tile of the crops' picture, and how
    /// many pixels tall.
    pub(super) fn crop(self) -> Option<(f32, f32)> {
        match self {
            Self::Ploughed | Self::Grass => None,
            Self::Short => Some((0.0, 6.0)),
            Self::Tall => Some((1.0, 12.0)),
            Self::Ripe => Some((2.0, 18.0)),
        }
    }
}

/// How many stages a vegetable plant goes through, dug up included (what
/// `common::CropGrowthShift` goes round).
pub const CROP_STAGES: usize = STAGES as usize + 1;
