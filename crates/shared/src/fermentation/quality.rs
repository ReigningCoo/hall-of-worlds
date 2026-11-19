//! A fermented drink's quality (owner: from the yeast, the ingredients and
//! the cask's wood), and moving a quality by steps.

use crate::item::Quality;
use crate::material::Material;

use super::MashSource;

/// The best a drink can be made or aged to: Artifacts are never brewed.
const BEST: Quality = Quality::Legendary;

pub(super) fn rank(quality: Quality) -> i32 {
    Quality::ALL.iter().position(|&q| q == quality).unwrap_or(1) as i32
}

/// `quality` moved `steps` up (or down, negative), kept from Poor to
/// Legendary.
pub(super) fn stepped(quality: Quality, steps: i32) -> Quality {
    let rank = (rank(quality) + steps).clamp(0, rank(BEST));
    Quality::ALL[rank as usize]
}

/// The yeasts' quality, averaged by rank and rounded down (a Poor yeast
/// pulls a good one down); Poor for none (wild).
pub fn mean_quality(yeasts: &[Quality]) -> Quality {
    if yeasts.is_empty() {
        return Quality::Poor;
    }
    let sum: i32 = yeasts.iter().map(|&q| rank(q)).sum();
    Quality::ALL[(sum / yeasts.len() as i32) as usize]
}

/// Steps the mash's making moves quality: juice and grain none, cordial
/// (watered) and fruit mashed whole in water one down.
fn source_steps(source: MashSource) -> i32 {
    match source {
        MashSource::Juice | MashSource::Grain => 0,
        MashSource::Cordial | MashSource::FruitInWater => -1,
    }
}

/// Steps the cask's wood moves quality while fermenting: pine taints (one
/// down), the rest none (ageing is `aged`'s).
fn wood_steps(wood: Material) -> i32 {
    match wood {
        Material::Pine => -1,
        _ => 0,
    }
}

/// A drink's quality when it has fermented: the yeasts' `mean_quality`, moved
/// by how the mash was made and the cask's wood, from Poor to Legendary.
/// Wild fermenting (no yeast) is always Poor.
pub fn drink_quality(yeasts: &[Quality], source: MashSource, wood: Material) -> Quality {
    if yeasts.is_empty() {
        return Quality::Poor;
    }
    let steps = source_steps(source) + wood_steps(wood);
    stepped(mean_quality(yeasts), steps)
}
