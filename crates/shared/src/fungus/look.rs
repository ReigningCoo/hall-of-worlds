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
