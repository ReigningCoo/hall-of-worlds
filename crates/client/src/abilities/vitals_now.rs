//! What the player's Health and Mana allow now, as the client refills them:
//! whether an automatic heal is due (a spell's Mana: `common::affords`).
//! The server decides; this only keeps from asking for what it would refuse.

use shared::magic::needs_healing;
use shared::vitals::{PER_POINT, Points};

use crate::common::OwnVitals;

/// Whether the player's Health is low enough for an automatic heal.
pub(super) fn wants_healing(vitals: &OwnVitals, now: f64) -> bool {
    vitals.0.is_some_and(|told| {
        let health = Points((told.now(now).0 * PER_POINT as f32) as u32);
        needs_healing(health, told.health.most)
    })
}
