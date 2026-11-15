//! Clothes on the Middle layer: everything worn over underclothes and under cloaks and
//! armour, footwear apart. So far: tunics, robes, doublets, bodices and vests on the
//! torso; trousers and skirts on the legs; belts; gloves; hoods, caps, headcloths,
//! headdresses, neckwear and shawls; and hats, on the Outer layer. Medieval and earlier
//! first, later pieces (to 1700) rarer. What one is like
//! is rolled once and saved; each choice comes from what the ones before allow, so
//! nothing is thrown away. See `docs/context/shared/clothing.md` and the research in
//! `docs/context/research-clothing.md`.

mod belt;
mod bodice;
mod cap;
mod cloth;
mod doublet;
mod garment;
mod gloves;
mod grid;
mod hat;
mod headcloth;
mod headdress;
mod hood;
mod names;
mod neckwear;
mod piece;
mod robe;
mod roll;
mod shawl;
mod skirt;
mod trousers;
mod tunic;
mod vest;
mod weight;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

pub use belt::{Belt, BeltEnd, BeltFastening, BeltForm, BeltWidth, Fittings};
pub use bodice::{Bodice, BodiceLength, BodiceNeck, BodiceSleeves, Lacing, Stiffening};
pub use cap::{Cap, CapBrim, CapMake, CapShape, CapTop};
pub use cloth::{Pattern, Weave};
pub use doublet::{
    Doublet, DoubletCollar, DoubletDecoration, DoubletFastening, DoubletLength, DoubletSleeves,
    Padding,
};
pub use garment::Garment;
pub use gloves::{Cuff, Fingers, GloveDecoration, GloveMake, Gloves};
pub use hat::{Crown, Hat, HatBrim, HatDecoration, HatMake};
pub use headcloth::{Headcloth, HeadclothForm, Reach, TurbanSize};
pub use headdress::{Headdress, HeaddressDecoration, HeaddressForm, HeaddressVeil};
pub use hood::{Edge, Hood, HoodCape, HoodFace, Tail};
pub use neckwear::{Neckwear, NeckwearForm, RuffSize};
pub use piece::ClothingPiece;
pub use robe::{Collar, Fastening, Front, Lining, Robe, RobeDecoration, RobeLength, RobeSleeves};
pub use roll::roll;
pub use shawl::{Shawl, ShawlCollar, ShawlForm, ShawlLength};
pub use skirt::{Skirt, SkirtFront, SkirtLength, SkirtMake, SkirtWaist};
pub use trousers::{Fullness, LegEnds, TrouserFront, TrouserLength, TrouserWaist, Trousers};
pub use tunic::{Closure, Cut, Length, Neck, Sleeves, Tunic, TunicDecoration};
pub use vest::{Vest, VestDecoration, VestFastening, VestLength, VestMake};

use crate::armour::Dye;
use crate::material::Material;

/// One piece of clothing: everything about it that is rolled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClothingForm {
    /// Its shape: which garment it is and how that is cut.
    pub garment: Garment,
    pub material: Material,
    /// Velvet, brocade or damask instead of plain cloth.
    pub weave: Weave,
    pub pattern: Pattern,
    pub dye: Dye,
    /// A second colour: for a pattern, contrasting edges or a lining.
    pub contrast: Option<Dye>,
}

impl ClothingForm {
    /// Which kind of clothing it is.
    pub fn piece(&self) -> ClothingPiece {
        self.garment.piece()
    }
}
