//! Tunics: everything pulled on over the head, from the draped chiton to the fitted
//! kirtle, the kurta and the thawb. On the Torso and Arms (the Torso alone without
//! sleeves); how long it is, is a property, never more locations (owner, 2026-11-15).

use serde::{Deserialize, Serialize};

mod names;
mod tables;

use crate::item::Quality;
use crate::rng::Rng;
use crate::underclothes::table::has;
use tables::{LENGTHS, closures, cuts, decorations, necks, sleeves};

/// Where the hem falls on a standing body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Length {
    Hip,
    /// Halfway from the hip to the knee.
    Thigh,
    Knee,
    /// Halfway from the knee to the ankle.
    Calf,
    Ankle,
    /// Trailing on the ground: late medieval, rolled at Rare and better.
    Floor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Sleeves {
    /// Sleeveless: the arms are bare.
    None,
    Short,
    Elbow,
    Wrist,
    /// Wide and open (ancient, and elsewhere).
    Wide,
    /// Hanging long from the elbow: the bliaut's (high medieval).
    Hanging,
    /// Laced on at the shoulder (late medieval on).
    LacedOn,
    /// Made by pinning the draped cloth along the arms (the chiton).
    Pinned,
    /// Draped over one shoulder, the other and both arms bare (the exomis).
    OneShoulder,
}

impl Sleeves {
    /// Whether the arms are bare.
    pub fn bare(self) -> bool {
        matches!(self, Self::None | Self::OneShoulder)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cut {
    /// Rectangles with gussets under the arms.
    Rectangles,
    /// Widened with gores in the skirt.
    Gored,
    /// The body fitted and the skirt pleated in (high medieval on).
    Fitted,
    /// A rectangle draped and pinned at the shoulders (ancient).
    Draped,
}

/// How it is closed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Closure {
    /// Pulled on over the head, loose.
    PulledOn,
    LacedFront,
    LacedSide,
    LacedBack,
    /// Buttoned down the front (late medieval).
    Buttoned,
}

impl Closure {
    pub fn laced(self) -> bool {
        matches!(self, Self::LacedFront | Self::LacedSide | Self::LacedBack)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Neck {
    /// A keyhole slit at the front.
    Slit,
    /// A slit closed with buttons (the kurta's).
    ButtonedSlit,
    Round,
    Square,
    V,
    /// A slit off to one side (the kosovorotka's).
    OffCentre,
    /// Pinned at the shoulders: draped ones only.
    Pinned,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TunicDecoration {
    None,
    /// Tablet-woven bands at the neck, cuffs and hem.
    Bands,
    /// Two stripes from the shoulders down (ancient).
    Clavi,
    Embroidery,
    /// Smocking across the chest (the smock-frock's, 1600s on).
    Smocking,
    /// Woven panels (the huipil's).
    BrocadedPanels,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tunic {
    pub length: Length,
    pub cut: Cut,
    pub sleeves: Sleeves,
    pub closure: Closure,
    pub neck: Neck,
    /// Draped ones only: folded over to the waist (the apoptygma).
    pub overfold: bool,
    pub decoration: TunicDecoration,
}

impl Tunic {
    /// A new one; trailing to the floor only at Rare and better.
    pub(super) fn roll(rng: &mut Rng, quality: Quality) -> Self {
        let mut lengths = LENGTHS;
        if quality < Quality::Rare {
            lengths[5].1 = 0;
        }
        let length = rng.pick_weighted(&lengths);
        let cut = rng.pick_weighted(&cuts(length));
        let sleeves = rng.pick_weighted(&sleeves(length, cut));
        let closure = rng.pick_weighted(&closures(length, cut, sleeves));
        let neck = rng.pick_weighted(&necks(cut, closure));
        let overfold = cut == Cut::Draped && sleeves == Sleeves::None && rng.range_u64(0, 2) == 0;
        let mut tunic = Self {
            length,
            cut,
            sleeves,
            closure,
            neck,
            overfold,
            decoration: TunicDecoration::None,
        };
        tunic.decoration = rng.pick_weighted(&decorations(&tunic));
        tunic
    }

    pub(super) fn allowed(&self) -> bool {
        has(&LENGTHS, self.length)
            && has(&cuts(self.length), self.cut)
            && has(&sleeves(self.length, self.cut), self.sleeves)
            && has(&closures(self.length, self.cut, self.sleeves), self.closure)
            && has(&necks(self.cut, self.closure), self.neck)
            && (!self.overfold || (self.cut == Cut::Draped && self.sleeves == Sleeves::None))
            && has(&decorations(self), self.decoration)
    }
}
