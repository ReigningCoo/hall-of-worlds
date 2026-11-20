//! `Meat`: an animal's meat by quality, with several cut names per quality
//! chosen by hash (owner, decision 4), a young animal's meat (decision 5) and
//! the carcass's shares by quality (decision 6).

use serde::{Deserialize, Serialize};

use crate::item::Quality;

/// Each quality's share of a carcass's meat, in percent, from beef's cut
/// tables (fillet about 3%, sirloin and rib about 16%): each portion's odds
/// before Butchery moves share up from Poor (decision 6).
pub const SHARES: [(Quality, u32); 6] = [
    (Quality::Poor, 12),
    (Quality::Common, 15),
    (Quality::Uncommon, 30),
    (Quality::Rare, 24),
    (Quality::Epic, 16),
    (Quality::Legendary, 3),
];

/// A young animal gives this percent of a grown one's meat (decision 5: much
/// less).
pub const YOUNG_YIELD_PERCENT: u32 = 25;

/// Names for each quality, Poor to Legendary. A quality with none is one the
/// animal never gives ("-" in the research).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ByQuality {
    pub poor: Vec<String>,
    pub common: Vec<String>,
    pub uncommon: Vec<String>,
    pub rare: Vec<String>,
    pub epic: Vec<String>,
    pub legendary: Vec<String>,
}

impl ByQuality {
    /// The names for this quality; none for Artifact.
    pub fn get(&self, quality: Quality) -> &[String] {
        match quality {
            Quality::Poor => &self.poor,
            Quality::Common => &self.common,
            Quality::Uncommon => &self.uncommon,
            Quality::Rare => &self.rare,
            Quality::Epic => &self.epic,
            Quality::Legendary => &self.legendary,
            Quality::Artifact => &[],
        }
    }

    /// The qualities it has names for, worst first.
    pub fn qualities(&self) -> impl Iterator<Item = Quality> + '_ {
        SHARES
            .iter()
            .map(|&(quality, _)| quality)
            .filter(|&quality| !self.get(quality).is_empty())
    }
}

/// An animal's meat.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meat {
    /// The meat as a whole ("beef", "mutton").
    pub name: String,
    /// A young animal's meat ("veal", "lamb", "suckling pig").
    pub young: String,
    pub cuts: ByQuality,
}

impl Meat {
    /// One cut's name at this quality, chosen by `hash`; `None` for a quality
    /// it never gives.
    pub fn cut_name(&self, quality: Quality, hash: u64) -> Option<&str> {
        let names = self.cuts.get(quality);
        let at = usize::try_from(hash % names.len().max(1) as u64).ok()?;
        names.get(at).map(String::as_str)
    }

    /// The worst quality a young animal's meat comes in: a step above the
    /// grown animal's worst, unless that is its best (decision 5).
    pub fn young_lowest(&self) -> Option<Quality> {
        let mut qualities = self.cuts.qualities();
        let lowest = qualities.next()?;
        Some(qualities.next().unwrap_or(lowest))
    }

    /// Each quality it gives with its percent of a carcass, summing to 100:
    /// `SHARES` kept for the qualities it has and scaled up, the rounding left
    /// over going to the largest. A young animal's starts at `young_lowest`.
    pub fn shares(&self, young: bool) -> Vec<(Quality, u32)> {
        let lowest = if young { self.young_lowest() } else { None };
        let kept: Vec<(Quality, u32)> = SHARES
            .iter()
            .copied()
            .filter(|&(quality, _)| !self.cuts.get(quality).is_empty())
            .filter(|&(quality, _)| lowest.is_none_or(|lowest| quality >= lowest))
            .collect();
        let total: u32 = kept.iter().map(|&(_, share)| share).sum();
        if total == 0 {
            return Vec::new();
        }
        let mut scaled: Vec<(Quality, u32)> = kept
            .iter()
            .map(|&(quality, share)| (quality, share * 100 / total))
            .collect();
        let left = 100 - scaled.iter().map(|&(_, share)| share).sum::<u32>();
        if let Some(largest) = scaled.iter_mut().max_by_key(|(_, share)| *share) {
            largest.1 += left;
        }
        scaled
    }

    pub(super) fn check(&self) -> Result<(), String> {
        let names = SHARES
            .iter()
            .flat_map(|&(quality, _)| self.cuts.get(quality));
        if self.name.trim().is_empty() || self.young.trim().is_empty() {
            return Err("its meat has no name".to_owned());
        }
        if names
            .clone()
            .any(|name| name.trim().is_empty() || name == "-")
        {
            return Err(format!("{}: a cut has no name", self.name));
        }
        if self.cuts.qualities().next().is_none() {
            return Err(format!("{}: it has no cuts", self.name));
        }
        Ok(())
    }
}
