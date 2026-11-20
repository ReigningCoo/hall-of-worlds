//! `Stages`: an animal's life stages, their names by sex, and when the young
//! and juvenile stages end.

use serde::{Deserialize, Serialize};

/// One real month of an animal's growth is one game day (owner, decision 2),
/// so a game hour is a real month's 24th.
pub const GAME_HOURS_A_MONTH: u32 = 24;

/// Where an animal is in its life.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Stage {
    Young,
    Juvenile,
    Adult,
}

/// A name for each sex; the same name twice when they share one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sexed {
    pub female: String,
    pub male: String,
}

/// The grown animal's names: a castrated male's where people name it (ox,
/// wether, gelding, capon), and any others (a hive's queen).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Adult {
    pub female: String,
    pub male: String,
    #[serde(default)]
    pub castrate: Option<String>,
    #[serde(default)]
    pub others: Vec<String>,
}

/// The names of its stages and when the first two end.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stages {
    /// A calf, lamb or chick.
    pub young: String,
    pub juvenile: Sexed,
    pub adult: Adult,
    /// Its age in game hours when it stops being young (a real month's growth
    /// a game day: `GAME_HOURS_A_MONTH`).
    pub young_until: u32,
    /// Its age in game hours when it is grown, after `young_until`.
    pub juvenile_until: u32,
}

impl Stages {
    /// Its stage at this age, in game hours since birth.
    pub fn at(&self, age_hours: u32) -> Stage {
        if age_hours < self.young_until {
            Stage::Young
        } else if age_hours < self.juvenile_until {
            Stage::Juvenile
        } else {
            Stage::Adult
        }
    }

    pub(super) fn check(&self) -> Result<(), String> {
        let names = [&self.young, &self.juvenile.female, &self.juvenile.male];
        let adult = [&self.adult.female, &self.adult.male];
        if names
            .iter()
            .chain(&adult)
            .any(|name| name.trim().is_empty())
        {
            return Err("a stage has no name".to_owned());
        }
        if self.young_until == 0 || self.juvenile_until <= self.young_until {
            return Err("its stages end out of order".to_owned());
        }
        Ok(())
    }
}
