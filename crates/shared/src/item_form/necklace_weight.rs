//! What a necklace weighs and the cells it takes up, from its form: first guesses.

use crate::creature::Size;
use crate::item::Shape;
use crate::item_form::necklace::{
    CordStyle, NecklaceForm, NecklaceLength, Ornament, Strand, Thickness,
};
use crate::material::Material;

impl NecklaceForm {
    /// Its own weight at an ordinary size, in grams, without its gems.
    pub fn grams(&self) -> u32 {
        let strand = match self.strand {
            Strand::Cord { leather, style } => {
                let times = match style {
                    CordStyle::Plain => 2,
                    CordStyle::Braided => 3,
                    CordStyle::Knotted => 4,
                };
                grams(self.length_mm() * 3 * times / 2, leather)
            }
            Strand::Chain { thickness, .. } => {
                // How much metal there is for each millimetre, in tenths of mm².
                let tenths = match thickness {
                    Thickness::Fine => 5,
                    Thickness::Medium => 15,
                    Thickness::Heavy => 50,
                };
                grams(self.length_mm() * tenths / 10, self.metal)
            }
            // A twisted bar some 6 mm across.
            Strand::Torc { .. } => grams(self.length_mm() * 30, self.metal),
        };
        let ornament = match self.ornament {
            Ornament::None => 0,
            Ornament::Pendant => 200,
            Ornament::Amulet(_) => 1500,
            Ornament::Stations => 100 * self.settings.len() as u32,
            Ornament::Charm(_) => 400,
        };
        (strand + grams(ornament, self.metal)).max(1)
    }

    /// Its own weight made at this size, larger necks taking longer strands.
    pub fn grams_at(&self, size: Size) -> u32 {
        let quarters = size.quarters() as u64;
        (self.grams() as u64 * quarters * quarters / 16).max(1) as u32
    }

    /// The cells it takes up at an ordinary size: a torc 2 × 2, anything else lies
    /// coiled in one.
    pub fn shape(&self) -> Shape {
        match self.strand {
            Strand::Torc { .. } => Shape::rectangle(2, 2),
            _ => Shape::rectangle(1, 1),
        }
    }

    /// How long its strand is, round a neck of ordinary size.
    fn length_mm(&self) -> u32 {
        match self.length {
            NecklaceLength::Choker => 350,
            NecklaceLength::Collarbone => 450,
            NecklaceLength::Chest => 550,
            NecklaceLength::Long => 750,
        }
    }
}

/// What this many cubic millimetres of a material weigh, in grams.
fn grams(mm3: u32, material: Material) -> u32 {
    mm3 * material.density() / 10_000
}
