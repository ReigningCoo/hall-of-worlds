//! `Garment`: which piece of clothing something is and how that one is cut, passing
//! each question on to its own file.

use serde::{Deserialize, Serialize};

use super::{
    Belt, Bodice, BodiceSleeves, Cap, ClothingPiece, Doublet, DoubletDecoration, Fingers, Gloves,
    Hat, Headcloth, Headdress, Hood, Neckwear, Robe, RobeDecoration, Shawl, Skirt, Trousers, Tunic,
    Vest, VestDecoration,
};
use crate::creature::Location;
use crate::item::Quality;
use crate::rng::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Garment {
    Tunic(Tunic),
    Robe(Robe),
    Doublet(Doublet),
    Bodice(Bodice),
    Vest(Vest),
    Trousers(Trousers),
    Skirt(Skirt),
    Belt(Belt),
    Gloves(Gloves),
    Hood(Hood),
    Cap(Cap),
    Headcloth(Headcloth),
    Headdress(Headdress),
    Neckwear(Neckwear),
    Shawl(Shawl),
    Hat(Hat),
}

impl Garment {
    pub fn piece(&self) -> ClothingPiece {
        match self {
            Self::Tunic(_) => ClothingPiece::Tunic,
            Self::Robe(_) => ClothingPiece::Robe,
            Self::Doublet(_) => ClothingPiece::Doublet,
            Self::Bodice(_) => ClothingPiece::Bodice,
            Self::Vest(_) => ClothingPiece::Vest,
            Self::Trousers(_) => ClothingPiece::Trousers,
            Self::Skirt(_) => ClothingPiece::Skirt,
            Self::Belt(_) => ClothingPiece::Belt,
            Self::Gloves(_) => ClothingPiece::Gloves,
            Self::Hood(_) => ClothingPiece::Hood,
            Self::Cap(_) => ClothingPiece::Cap,
            Self::Headcloth(_) => ClothingPiece::Headcloth,
            Self::Headdress(_) => ClothingPiece::Headdress,
            Self::Neckwear(_) => ClothingPiece::Neckwear,
            Self::Shawl(_) => ClothingPiece::Shawl,
            Self::Hat(_) => ClothingPiece::Hat,
        }
    }

    pub(super) fn roll(rng: &mut Rng, piece: ClothingPiece, quality: Quality) -> Self {
        match piece {
            ClothingPiece::Tunic => Self::Tunic(Tunic::roll(rng, quality)),
            ClothingPiece::Robe => Self::Robe(Robe::roll(rng)),
            ClothingPiece::Doublet => Self::Doublet(Doublet::roll(rng)),
            ClothingPiece::Bodice => Self::Bodice(Bodice::roll(rng)),
            ClothingPiece::Vest => Self::Vest(Vest::roll(rng)),
            ClothingPiece::Trousers => Self::Trousers(Trousers::roll(rng)),
            ClothingPiece::Skirt => Self::Skirt(Skirt::roll(rng)),
            ClothingPiece::Belt => Self::Belt(Belt::roll(rng, quality)),
            ClothingPiece::Gloves => Self::Gloves(Gloves::roll(rng, quality)),
            ClothingPiece::Hood => Self::Hood(Hood::roll(rng, quality)),
            ClothingPiece::Cap => Self::Cap(Cap::roll(rng)),
            ClothingPiece::Headcloth => Self::Headcloth(Headcloth::roll(rng)),
            ClothingPiece::Headdress => Self::Headdress(Headdress::roll(rng)),
            ClothingPiece::Neckwear => Self::Neckwear(Neckwear::roll(rng, quality)),
            ClothingPiece::Shawl => Self::Shawl(Shawl::roll(rng)),
            ClothingPiece::Hat => Self::Hat(Hat::roll(rng)),
        }
    }

    pub(super) fn allowed(&self) -> bool {
        match self {
            Self::Tunic(tunic) => tunic.allowed(),
            Self::Robe(robe) => robe.allowed(),
            Self::Doublet(doublet) => doublet.allowed(),
            Self::Bodice(bodice) => bodice.allowed(),
            Self::Vest(vest) => vest.allowed(),
            Self::Trousers(trousers) => trousers.allowed(),
            Self::Skirt(skirt) => skirt.allowed(),
            Self::Belt(belt) => belt.allowed(),
            Self::Gloves(gloves) => gloves.allowed(),
            Self::Hood(hood) => hood.allowed(),
            Self::Cap(cap) => cap.allowed(),
            Self::Headcloth(cloth) => cloth.allowed(),
            Self::Headdress(headdress) => headdress.allowed(),
            Self::Neckwear(neckwear) => neckwear.allowed(),
            Self::Shawl(shawl) => shawl.allowed(),
            Self::Hat(hat) => hat.allowed(),
        }
    }

    /// Locations of its kind it leaves bare: a sleeveless tunic's or bodice's arms,
    /// fingerless gloves' fingers, the neck of a headcloth on the head only or of a
    /// shawl without a collar. Worn and clashing go by what is left.
    pub fn bare(&self) -> &'static [Location] {
        match self {
            Self::Tunic(tunic) if tunic.sleeves.bare() => &[Location::Arms],
            Self::Bodice(bodice) if bodice.sleeves == BodiceSleeves::None => &[Location::Arms],
            Self::Gloves(gloves) if gloves.fingers == Fingers::Fingerless => &[Location::Fingers],
            Self::Headcloth(cloth) if cloth.neck_bare() => &[Location::Neck],
            Self::Shawl(shawl) if shawl.neck_bare() => &[Location::Neck],
            _ => &[],
        }
    }

    /// Whether it has a second colour of its own: contrasting edges or a lining.
    pub(super) fn contrasting(&self) -> bool {
        match self {
            Self::Robe(robe) => robe.decoration == RobeDecoration::ContrastEdges,
            Self::Doublet(doublet) => doublet.decoration == DoubletDecoration::ContrastLining,
            Self::Vest(vest) => vest.decoration == VestDecoration::ContrastEdging,
            _ => false,
        }
    }
}
