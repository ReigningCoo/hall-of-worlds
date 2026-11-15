//! Rolling a piece of clothing, each choice from what the ones before allow, so
//! nothing is thrown away.

use super::cloth::{contrasts, dyes, materials, patterns, weaves};
use super::{ClothingForm, ClothingPiece, Garment, Pattern};
use crate::item::Quality;
use crate::rng::Rng;

/// Rolls one of this kind: its garment, material, weave, pattern and colours. A
/// tunic trailing to the floor only comes at Rare and better.
pub fn roll(rng: &mut Rng, piece: ClothingPiece, quality: Quality) -> ClothingForm {
    let garment = Garment::roll(rng, piece, quality);
    let material = rng.pick_weighted(&materials(&garment));
    let weave = rng.pick_weighted(&weaves(&garment, material));
    let pattern = rng.pick_weighted(&patterns(&garment, material, weave));
    let dye = rng.pick_weighted(&dyes(material));
    let contrast = two_coloured(&garment, pattern).then(|| rng.pick_weighted(&contrasts(dye)));
    ClothingForm {
        garment,
        material,
        weave,
        pattern,
        dye,
        contrast,
    }
}

/// Whether it has a second colour: a pattern, contrasting edges or a lining.
pub(super) fn two_coloured(garment: &Garment, pattern: Pattern) -> bool {
    pattern != Pattern::Plain || garment.contrasting()
}
