//! Every piece of clothing that may be made: the grid rolls keep to, checked by the
//! tests.

use super::ClothingForm;
use super::cloth::{contrasts, dyes, materials, patterns, weaves};
use super::roll::two_coloured;
use crate::underclothes::table::has;

impl ClothingForm {
    /// Whether these choices go together. A tunic trailing to the floor is allowed at
    /// any quality, though only rolled at Rare and better.
    pub fn allowed(&self) -> bool {
        let garment = &self.garment;
        let material = self.material;
        let coloured = match self.contrast {
            Some(second) => {
                two_coloured(garment, self.pattern) && has(&contrasts(self.dye), second)
            }
            None => !two_coloured(garment, self.pattern),
        };
        garment.allowed()
            && has(&materials(garment), material)
            && has(&weaves(garment, material), self.weave)
            && has(&patterns(garment, material, self.weave), self.pattern)
            && has(&dyes(material), self.dye)
            && coloured
    }
}
