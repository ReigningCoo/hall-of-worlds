//! Coins (`Look::Coin`, 1 × 1): one picture per metal, whatever the count. A
//! disc with a raised rim and a reeded edge (`disc.rs`), the owner's "Reigning
//! Coo" embossed on its face in place of a ruler's head (`coo.rs`, raised by
//! `outline.rs` and `face.rs`), tilted back to show its edge (`parts.rs`).

use shared::material::Material;

// Split into files (`cargo xtask lint` keeps files small).
mod coo;
mod disc;
mod face;
mod mesh;
mod outline;
mod parts;

pub(super) use parts::coin_parts;

/// Copper's radius and thickness, against the picture's `VIEW_WIDTH` of 2.5:
/// about 90% of the cell across, less the black edge's room.
const COPPER_RADIUS: f32 = 1.06;
const COPPER_THICKNESS: f32 = 0.17;

/// How big a coin is, in the picture's units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Coin {
    pub(super) radius: f32,
    pub(super) thickness: f32,
}

impl Coin {
    /// Set by the owner, not by weight: copper the largest, gold half its width
    /// and thickness, silver between (70% of the cell, ¾ of copper's thickness).
    pub(super) fn of(metal: Material) -> Self {
        let (wide, thick) = match metal {
            Material::Silver => (7.0 / 9.0, 0.75),
            Material::Gold => (0.5, 0.5),
            _ => (1.0, 1.0),
        };
        Self {
            radius: COPPER_RADIUS * wide,
            thickness: COPPER_THICKNESS * thick,
        }
    }
}

#[cfg(test)]
mod tests;
