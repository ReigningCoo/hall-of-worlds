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
            Garment::Trousers(trousers) => trousers.name(self.material, self.pattern),
            Garment::Skirt(skirt) => skirt.name(self.material, self.pattern),
            Garment::Belt(belt) => belt.name(),
            Garment::Gloves(gloves) => gloves.name(),
            Garment::Hood(hood) => hood.name(),
            Garment::Cap(cap) => cap.name(),
            Garment::Headcloth(cloth) => cloth.name(),
            Garment::Headdress(headdress) => headdress.name(),
            Garment::Neckwear(neckwear) => neckwear.name(),
            Garment::Shawl(shawl) => shawl.name(self.material, self.pattern),
            Garment::Hat(hat) => hat.name(),
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
    /// leather, lederhosen too, a grass skirt grass, tartan is wool).
    fn cloth_word(&self) -> Option<&'static str> {
        if let Some(weave) = self.weave.name() {
            return Some(weave);
        }
        let usual = match &self.garment {
            // Each make has its own material, but only felt hats and wool caps are usual.
            Garment::Hat(_) => self.material == Material::Felt,
            Garment::Cap(_) => self.material == Material::Wool,
            _ => materials(&self.garment)
                .first()
                .is_some_and(|&(first, _)| first == self.material),
        };
        let in_name = match &self.garment {
            Garment::Vest(vest) => vest.make == VestMake::Buff || self.specific_name() == "Jerkin",
            Garment::Trousers(trousers) => trousers.leather_shorts(self.material),
            Garment::Skirt(skirt) => skirt.named_for_material(),
            Garment::Hat(hat) => hat.named_for_material(),
            Garment::Cap(_) => matches!(
                self.specific_name(),
                "Ushanka" | "Papakha" | "Fez" | "Kalpak" | "Pileus"
            ),
            Garment::Shawl(_) => self.specific_name() == "Rebozo",
            _ => false,
        };
        let tartan = self.pattern == Pattern::Tartan && self.material == Material::Wool;
        (!(usual || in_name || tartan)).then(|| self.material.name())
    }
}
