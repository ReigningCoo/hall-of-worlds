//! What clothes weigh and take up. Weights are worked out from cloth areas for plain
//! linen, as for underclothes, changed by the material and weave, plus any fur
//! lining; no article gave one.

use super::{ClothingForm, Garment, Length, Weave};
use crate::creature::Size;
use crate::item::Shape;
use crate::material::Material;

impl ClothingForm {
    fn base(&self) -> u32 {
        match &self.garment {
            Garment::Tunic(tunic) => tunic.base_grams(),
            Garment::Robe(robe) => robe.base_grams(),
            Garment::Doublet(doublet) => doublet.base_grams(),
            Garment::Bodice(bodice) => bodice.base_grams(),
            Garment::Vest(vest) => vest.base_grams(),
        }
    }

    /// Hundredths of linen's weight its material and weave make it.
    fn factor(&self) -> u32 {
        let material = match self.material {
            Material::Linen | Material::Ramie => 100,
            Material::Hemp => 110,
            Material::Cotton => 90,
            Material::Silk => 60,
            Material::Wool => 130,
            Material::Suede => 150,
            Material::Leather => 170,
            Material::Felt => 220,
            // Thick oiled oxhide.
            Material::BuffLeather => 450,
            _ => 100,
        };
        let weave = match self.weave {
            Weave::Plain => 100,
            Weave::Velvet => 160,
            Weave::Brocade => 140,
            Weave::Damask => 110,
        };
        material * weave / 100
    }

    fn fur(&self) -> u32 {
        match &self.garment {
            Garment::Robe(robe) => robe.fur_grams(),
            Garment::Vest(vest) => vest.fur_grams(),
            _ => 0,
        }
    }

    /// What it weighs at an ordinary size, in grams.
    pub fn grams(&self) -> u32 {
        (self.base() * self.factor() / 100 + self.fur()).max(5)
    }

    /// What it weighs made at this size: bigger across, as thick, so by the square of
    /// its size (as underclothes).
    pub fn grams_at(&self, size: Size) -> u32 {
        let quarters = size.quarters() as u64;
        (self.grams() as u64 * quarters * quarters / 16).max(1) as u32
    }

    /// The cells it takes up at an ordinary size, wide and long: 2 × 2, or 2 × 3 for
    /// robes and tunics below the hip.
    pub fn shape(&self) -> Shape {
        let (width, height) = match &self.garment {
            Garment::Tunic(tunic) if tunic.length > Length::Hip => (2, 3),
            Garment::Robe(_) => (2, 3),
            _ => (2, 2),
        };
        Shape::rectangle(width, height)
    }
}
