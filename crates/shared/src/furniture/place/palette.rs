//! The materials a site's furniture is made of: one or two woods for the
//! whole site and one stone, and which of them each room uses.

use super::super::material::weighted;
use super::super::{Material, Stone, Wood};
use crate::rng::Seed;

/// What a site's furniture may be made of.
pub(super) struct Palette {
    woods: Vec<Wood>,
    stone: Stone,
    stone_percent: u32,
}

impl Palette {
    /// From the site's furniture seed (the same on every level).
    pub(super) fn new(seed: Seed, stone_percent: u32) -> Self {
        let roll = |what: i64| seed.at(&[-1, what]);
        let first = weighted(roll(0), &Wood::WEIGHTS);
        let second = weighted(roll(1), &Wood::WEIGHTS);
        let mut woods = vec![first];
        if roll(2).is_multiple_of(2) && second != first {
            woods.push(second);
        }
        Self {
            woods,
            stone: weighted(roll(3), &Stone::WEIGHTS),
            stone_percent,
        }
    }

    /// A room's material, by `roll`.
    pub(super) fn material(&self, roll: u64) -> Material {
        if (roll % 100) < u64::from(self.stone_percent) {
            Material::Stone(self.stone)
        } else {
            Material::Wood(self.woods[(roll / 100) as usize % self.woods.len()])
        }
    }
}
