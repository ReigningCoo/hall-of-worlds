//! What clothes are made of, how the cloth is woven, its pattern and colours: each
//! list goes by the garment rolled before it. Velvet, brocade and damask are weaves of
//! silk (velvet of wool too), and tartan a pattern like stripes (owner, 2026-11-15).

use serde::{Deserialize, Serialize};

use super::{Garment, Lining, VestMake};
use crate::armour::Dye;
use crate::material::Material;
use crate::underclothes::table::when;

/// How the cloth is woven.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Weave {
    Plain,
    /// A cut pile: silk or wool.
    Velvet,
    /// Patterned with extra threads, often gold: silk.
    Brocade,
    /// A pattern woven in the cloth's own sheen: silk.
    Damask,
}

impl Weave {
    pub fn name(self) -> Option<&'static str> {
        match self {
            Self::Plain => None,
            Self::Velvet => Some("Velvet"),
            Self::Brocade => Some("Brocade"),
            Self::Damask => Some("Damask"),
        }
    }
}

/// A pattern woven or dyed in two colours.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pattern {
    Plain,
    Striped,
    Checked,
    /// Checks of crossing stripes in wool.
    Tartan,
}

use Material::{BuffLeather, Cotton, Hemp, Leather, Linen, Silk, Suede, Wool};

/// The materials it can be made of, the usual one first.
pub(super) fn materials(garment: &Garment) -> Vec<(Material, u64)> {
    match garment {
        Garment::Tunic(_) => vec![(Wool, 45), (Linen, 30), (Silk, 10), (Cotton, 10), (Hemp, 5)],
        Garment::Robe(_) => vec![(Silk, 35), (Cotton, 25), (Wool, 25), (Linen, 10), (Hemp, 5)],
        Garment::Doublet(doublet) if doublet.arming() => {
            vec![(Linen, 50), (Cotton, 30), (Silk, 15), (Wool, 5)]
        }
        Garment::Doublet(_) => vec![
            (Wool, 35),
            (Linen, 20),
            (Silk, 25),
            (Cotton, 8),
            (Leather, 8),
            (Suede, 4),
        ],
        Garment::Bodice(_) => vec![(Wool, 40), (Linen, 30), (Silk, 20), (Cotton, 10)],
        Garment::Vest(vest) => match vest.make {
            VestMake::Cloth => vec![(Wool, 50), (Linen, 20), (Silk, 15), (Cotton, 15)],
            VestMake::Leather => vec![(Leather, 70), (Suede, 30)],
            VestMake::Buff => vec![(BuffLeather, 1)],
            VestMake::FurLined => vec![(Wool, 50), (Leather, 35), (Silk, 15)],
        },
        Garment::Trousers(trousers) => trousers.materials(),
        Garment::Skirt(skirt) => skirt.materials(),
        Garment::Belt(belt) => belt.materials(),
        Garment::Gloves(gloves) => gloves.materials(),
    }
}

/// How its cloth may be woven: velvet, brocade and damask in silk, velvet in wool.
pub(super) fn weaves(garment: &Garment, material: Material) -> Vec<(Weave, u64)> {
    let fine = match garment {
        Garment::Doublet(doublet) => !doublet.arming(),
        Garment::Robe(robe) => robe.lining != Lining::Padded || material == Silk,
        Garment::Belt(belt) => belt.cloth(),
        Garment::Gloves(_) => false,
        _ => true,
    };
    let silk = fine && material == Silk;
    let wool = fine && material == Wool;
    vec![
        (Weave::Plain, 85),
        (Weave::Velvet, when(silk, 7) + when(wool, 4)),
        (Weave::Brocade, when(silk, 5)),
        (Weave::Damask, when(silk, 5)),
    ]
}

/// Its pattern: plain cloth only, tartan only in wool, kilts and trews mostly tartan,
/// tube skirts often checked (the longyi).
pub(super) fn patterns(garment: &Garment, material: Material, weave: Weave) -> Vec<(Pattern, u64)> {
    let woven = weave == Weave::Plain && material.category() == crate::material::Category::Textile;
    let patterned = woven
        && match garment {
            Garment::Tunic(tunic) => tunic.cut != super::Cut::Draped,
            Garment::Robe(_) | Garment::Vest(_) | Garment::Trousers(_) => true,
            Garment::Skirt(skirt) => !skirt.named_for_material(),
            Garment::Belt(belt) => belt.form == super::BeltForm::Sash,
            Garment::Doublet(_) | Garment::Bodice(_) | Garment::Gloves(_) => false,
        };
    let (tartan, checked) = match garment {
        Garment::Trousers(trousers) if trousers.fitted() => (10, 0),
        Garment::Skirt(skirt) if skirt.kilted() => (100, 0),
        Garment::Skirt(skirt) if skirt.make == super::SkirtMake::Tube => (0, 15),
        _ => (0, 0),
    };
    vec![
        (Pattern::Plain, 90),
        (Pattern::Striped, when(patterned, 5)),
        (Pattern::Checked, when(patterned, 3 + checked)),
        (
            Pattern::Tartan,
            when(patterned && material == Wool, 3 + tartan),
        ),
    ]
}

/// Its colours: leather mostly its own, silk brighter, wool and linen plainer.
pub(super) fn dyes(material: Material) -> Vec<(Dye, u64)> {
    use Dye::{Black, Blue, Brown, Green, Natural, Purple, Red, White, Yellow};
    match material.category() {
        crate::material::Category::Leather => {
            vec![(Natural, 5), (Brown, 4), (Black, 3), (Red, 1)]
        }
        // Grass and leaves keep their own colours, or are dyed plainly.
        crate::material::Category::Plant => vec![(Natural, 6), (Brown, 2), (Red, 1), (Yellow, 1)],
        _ if material == Silk => vec![
            (Red, 20),
            (Blue, 15),
            (Green, 12),
            (Black, 12),
            (White, 10),
            (Yellow, 10),
            (Purple, 10),
            (Natural, 8),
            (Brown, 3),
        ],
        _ => vec![
            (Natural, 25),
            (Brown, 15),
            (Blue, 14),
            (Red, 12),
            (Green, 12),
            (Black, 8),
            (White, 6),
            (Yellow, 6),
            (Purple, 2),
        ],
    }
}

/// A second colour: any but the first.
pub(super) fn contrasts(dye: Dye) -> Vec<(Dye, u64)> {
    [
        (Dye::White, 15),
        (Dye::Red, 20),
        (Dye::Blue, 18),
        (Dye::Black, 15),
        (Dye::Green, 14),
        (Dye::Yellow, 12),
        (Dye::Natural, 6),
    ]
    .into_iter()
    .filter(|&(second, _)| second != dye)
    .collect()
}
