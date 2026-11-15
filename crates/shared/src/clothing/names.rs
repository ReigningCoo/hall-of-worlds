//! What a piece of clothing is called: the garment's name from the research's
//! signatures, with its pattern and an unusual material or weave in front ("Striped
//! Silk Robe", "Velvet Doublet").

use super::cloth::materials;
use super::{ClothingForm, Garment, Pattern, VestMake};
use crate::material::Material;

impl ClothingForm {
    /// The garment's own name, without its material.
    pub fn specific_name(&self) -> &'static str {
        match &self.garment {
            Garment::Tunic(tunic) => tunic.name(),
            Garment::Robe(robe) => robe.name(),
            Garment::Doublet(doublet) => doublet.name(),
            Garment::Bodice(bodice) => bodice.name(),
            Garment::Vest(vest) => vest.name(),
        }
    }

    /// Its full name.
    pub fn name(&self) -> String {
        let pattern = match self.pattern {
            Pattern::Plain => None,
            Pattern::Striped => Some("Striped"),
            Pattern::Checked => Some("Checked"),
            Pattern::Tartan => Some("Tartan"),
        };
        let words: Vec<&str> = [pattern, self.cloth_word(), Some(self.specific_name())]
            .into_iter()
            .flatten()
            .collect();
        words.join(" ")
    }

    /// The word for its cloth, when it is worth saying: a weave always, and a material
    /// other than its garment's usual one, unless its name says so already (a jerkin is
    /// leather, tartan is wool).
    fn cloth_word(&self) -> Option<&'static str> {
        if let Some(weave) = self.weave.name() {
            return Some(weave);
        }
        let usual = materials(&self.garment)
            .first()
            .is_some_and(|&(first, _)| first == self.material);
        let in_name = match &self.garment {
            Garment::Vest(vest) => vest.make == VestMake::Buff || self.specific_name() == "Jerkin",
            _ => false,
        };
        let tartan = self.pattern == Pattern::Tartan && self.material == Material::Wool;
        (!(usual || in_name || tartan)).then(|| self.material.name())
    }
}
