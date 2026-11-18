//! A fruit for Give Item's juice and cordial, the same on both sides from the
//! same seed.

use super::pressing::juice_fruits;
use crate::rng::Rng;

/// A juicable fruit's key, rolled.
pub fn juice_fruit(rng: &mut Rng) -> String {
    let fruits: Vec<_> = juice_fruits().collect();
    let at = rng.range_u64(0, fruits.len() as u64) as usize;
    fruits[at].key.clone()
}
