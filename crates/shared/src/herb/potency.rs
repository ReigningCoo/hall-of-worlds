//! How much of a herb's quality its user draws out (owner: quality counts only
//! as far as the user's Herbalism reaches): for eating herbs raw (`raw_dose`)
//! and, later, making potions.

use super::quality::FULL;
use crate::item::Quality;

/// The skill that draws out a herb's quality.
pub const HERBALISM: &str = "gathering/herbalism";

/// How strong a herb of this quality is in the best hands, as a percentage of
/// a common one's. First guesses.
pub const fn quality_percent(quality: Quality) -> u32 {
    match quality {
        Quality::Poor => 80,
        Quality::Common => 100,
        Quality::Uncommon => 115,
        Quality::Rare => 130,
        Quality::Epic => 150,
        Quality::Legendary => 175,
        Quality::Artifact => 200,
    }
}

/// The strongest a user with this much Herbalism (hundredths of a percent,
/// `skill_value`) can make any herb, as a percentage: a poor one's at none, an
/// artifact's at 100%, in a straight line between.
pub fn skill_cap(herbalism: u32) -> u32 {
    let low = quality_percent(Quality::Poor);
    let high = quality_percent(Quality::Artifact);
    low + (high - low) * herbalism.min(FULL) / FULL
}

/// How strong a herb of this quality is for a user with this much Herbalism, as
/// a percentage of a common one's.
pub fn potency(quality: Quality, herbalism: u32) -> u32 {
    quality_percent(quality).min(skill_cap(herbalism))
}
