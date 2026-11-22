//! What a piece of decor (or candles on a table) is drawn as, and where its
//! model stands.

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use shared::decor::{ChandelierStyle, Decor, Hanging, Rug};
use shared::door::Side;
use shared::dungeon::{CELL_UNITS, WALL_HALF_UNITS};
use shared::furniture::Candles;
use shared::rng::Seed;

use super::super::CELL_SIZE;
use super::model::Model;
use super::{boards, candles, pegs, rugs, trophies};

/// How far a wall's face stands from the middle of its cell.
const WALL_FACE: f32 = WALL_HALF_UNITS as f32 / CELL_UNITS as f32 * CELL_SIZE;

/// How many variations a hanging has (colours, sizes), from where it hangs.
const VARIANTS: u64 = 10;

/// What a model is built from: one is built per look.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum DecorLook {
    Hanging(Hanging, u8),
    Chandelier(ChandelierStyle),
    Candles(Candles),
    Rug((u8, u8), Rug),
}

impl DecorLook {
    pub(super) fn of(decor: &Decor) -> Self {
        match *decor {
            Decor::Wall {
                wall,
                face,
                hanging,
                ..
            } => {
                let seed = Seed::new(0).derive("decor look");
                let variant = seed.at(&[wall.0, wall.1, face as i64]) % VARIANTS;
                Self::Hanging(hanging, variant as u8)
            }
            Decor::Chandelier { style, .. } => Self::Chandelier(style),
            Decor::Rug { size, rug, .. } => Self::Rug(size, rug),
        }
    }

    /// Its model, in its own frame.
    pub(super) fn build(self) -> Model {
        let mut model = Model::default();
        match self {
            Self::Hanging(hanging, variant) => {
                let m = &mut model;
                match hanging {
                    Hanging::Antlers => trophies::antlers(m, variant),
                    Hanging::BoarHead => trophies::boar_head(m, variant),
                    Hanging::Shield => trophies::shield(m, variant),
                    Hanging::CrossedWeapons => trophies::crossed_weapons(m, variant),
                    Hanging::HerbBundles => pegs::herb_bundles(m, variant),
                    Hanging::Hams => pegs::hams(m, variant),
                    Hanging::Onions => pegs::onions(m, variant),
                    Hanging::CoatPegs => pegs::coat_pegs(m, variant),
                    Hanging::Tapestry => boards::tapestry(m, variant),
                    Hanging::Sign => boards::sign(m, variant),
                    Hanging::NoticeBoard => boards::notice_board(m, variant),
                    Hanging::Sconce => boards::sconce(m),
                }
            }
            Self::Chandelier(style) => candles::chandelier(&mut model, style),
            Self::Candles(kind) => candles::candles(&mut model, kind),
            Self::Rug(size, rug) => rugs::rug(&mut model, size, rug),
        }
        model
    }
}

/// The turn taking a model facing south (`+z`) to face `side`.
pub(super) fn turn_to(side: Side) -> Quat {
    Quat::from_rotation_y(match side {
        Side::South => 0.0,
        Side::East => FRAC_PI_2,
        Side::North => PI,
        Side::West => -FRAC_PI_2,
    })
}

/// Where a piece of decor's frame stands, over the ground at `ground` (world
/// units), and how it is turned.
pub(super) fn placed(decor: &Decor, ground: f32) -> Transform {
    let middle = |(x, y): (i64, i64)| Vec3::new(x as f32, 0.0, y as f32) * CELL_SIZE;
    match *decor {
        Decor::Wall { wall, face, .. } => {
            let (dx, dy) = face.beside((0, 0));
            let out = Vec3::new(dx as f32, 0.0, dy as f32);
            let at = middle(wall) + out * WALL_FACE + Vec3::Y * ground;
            Transform::from_translation(at).with_rotation(turn_to(face))
        }
        Decor::Chandelier { at, .. } => Transform::from_translation(middle(at) + Vec3::Y * ground),
        Decor::Rug { at, .. } => {
            let corner = middle(at) - Vec3::new(0.5, 0.0, 0.5) * CELL_SIZE;
            Transform::from_translation(corner + Vec3::Y * ground)
        }
    }
}
