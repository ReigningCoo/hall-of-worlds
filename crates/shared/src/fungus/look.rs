//! `FungusLook`: what one fungus looks like, from the research's descriptions,
//! for the client's pictures and models (fungi plan PRs 2 and 4).

use serde::{Deserialize, Serialize};

use crate::herb::Rgb;

/// The most colours a look may have: its body, underneath, its stem.
pub const MOST_COLOURS: usize = 3;

/// The shape its picture and model are built from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Form {
    /// A cap on a stem.
    CapAndStem,
    /// A funnel or trumpet (chanterelle, black trumpet).
    Funnel,
    /// A shelf, fan or hoof on wood.
    Bracket,
    /// A ball (puffballs, earthballs, lion's mane).
    Ball,
    /// Branches, frills or ribbons (cauliflower fungus, coral).
    Coral,
    Cup,
    /// A blob, ear or brain of jelly.
    Jelly,
    /// A pitted or wrinkled head on a stem (morels, false morel).
    Honeycomb,
    /// Clubs or fingers standing up.
    Club,
    /// A buried lump.
    Truffle,
    /// A spreading crust or plate.
    Crust,
    /// Tufts of lichen.
    LichenTufts,
    /// Hanging beards of lichen.
    LichenBeard,
    /// Flat plates or lobes of lichen.
    LichenPlates,
}

/// What one fungus looks like.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FungusLook {
    pub form: Form,
    /// 1 to `MOST_COLOURS`: its body (a cap, a bracket's top), then what is
    /// underneath (gills, pores, the inside of a cup), then its stem.
    pub colours: Vec<Rgb>,
    /// Across one, from small to large, in millimetres.
    pub smallest: u16,
    pub largest: u16,
    /// A cap's shape (a cap and stem's, a funnel's rim).
    #[serde(default)]
    pub cap: Cap,
    /// What is under a cap or bracket.
    #[serde(default)]
    pub under: Under,
    /// A cap's stem.
    #[serde(default)]
    pub stem: Stem,
    /// A skirt round the stem.
    #[serde(default)]
    pub ring: bool,
    /// A cup round the stem's foot.
    #[serde(default)]
    pub volva: bool,
    /// Pale patches on the cap (fly agaric's).
    #[serde(default)]
    pub warts: bool,
}

/// A cap's shape.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cap {
    /// Domed.
    #[default]
    Convex,
    /// Opened out flat, perhaps with a bump in the middle.
    Flat,
    /// Bell-shaped, taller than wide.
    Bell,
    /// A pointed cone.
    Conical,
    /// Dipped in the middle, its rim rolled or raised.
    Depressed,
}

/// What is under a cap or bracket.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Under {
    #[default]
    Gills,
    /// A sponge of tubes (boletes, polypores).
    Pores,
    /// Hanging teeth (hedgehog mushroom, lion's mane).
    Spines,
    /// Blunt, forking folds running down the stem (chanterelles).
    Ridges,
    Smooth,
}

/// A cap's stem.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stem {
    #[default]
    Slender,
    /// Tall and thin (parasol, shaggy ink cap).
    Tall,
    /// Thick, often swollen at the foot (boletes).
    Stout,
}

impl FungusLook {
    /// Whether it makes sense: an error saying what is wrong.
    pub fn check(&self) -> Result<(), String> {
        if self.colours.is_empty() || self.colours.len() > MOST_COLOURS {
            return Err(format!("a look has 1 to {MOST_COLOURS} colours"));
        }
        if self.smallest == 0 || self.smallest > self.largest {
            return Err("a look's sizes go from small to large".to_owned());
        }
        Ok(())
    }
}
