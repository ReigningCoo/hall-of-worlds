//! `ItemSet`: one set of the pool, whole.

use super::bonuses::{SetBonus, roll_bonuses};
use super::roll;
use crate::bonus::Bonus;
use crate::item::{ItemKind, Quality};

/// One set: everything about it comes from its number.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemSet {
    /// Its number in the pool.
    pub number: u16,
    /// A placeholder name, like artifacts' (`item::artifact_name`).
    pub name: String,
    /// Every piece's quality.
    pub quality: Quality,
    /// Its pieces' kinds, in order (`SetPiece::piece` counts along them).
    pub pieces: Vec<ItemKind>,
    /// Its bonuses, each with how many pieces must be worn for it, fewest
    /// first.
    pub bonuses: Vec<SetBonus>,
}

impl ItemSet {
    /// Set `number` of the pool (any number is taken round the pool).
    pub fn of(number: u16) -> Self {
        let number = number % super::POOL;
        let (quality, pieces) = roll::pieces(number);
        Self {
            number,
            name: name(number),
            quality,
            bonuses: roll_bonuses(number, &pieces),
            pieces,
        }
    }

    /// The bonuses given while `worn` of its pieces are worn.
    pub fn bonuses_at(&self, worn: usize) -> impl Iterator<Item = &Bonus> {
        (self.bonuses.iter())
            .filter(move |each| usize::from(each.worn) <= worn)
            .map(|each| &each.bonus)
    }
}

/// A placeholder name for a set (owner: names later).
fn name(number: u16) -> String {
    const WORDS: [&str; 16] = [
        "Ashen",
        "Dawning",
        "Endless",
        "Forgotten",
        "Gilded",
        "Hollow",
        "Iron",
        "Last",
        "Moonlit",
        "Quiet",
        "Sunken",
        "Thousandth",
        "Wandering",
        "Whispering",
        "Winter",
        "Wild",
    ];
    const NOUNS: [&str; 8] = [
        "Array",
        "Garb",
        "Harness",
        "Panoply",
        "Raiment",
        "Regalia",
        "Trappings",
        "Vestments",
    ];
    let mut rng = roll::seed(number, "name").rng();
    let word = WORDS[rng.range_u64(0, WORDS.len() as u64) as usize];
    let noun = NOUNS[rng.range_u64(0, NOUNS.len() as u64) as usize];
    format!("The {word} {noun}")
}
