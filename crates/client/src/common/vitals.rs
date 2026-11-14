//! The player's Health and Mana as the server last told them, and the room the
//! interface keeps at the bottom of the window.

use bevy::prelude::*;
use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::vitals::{DRAIN_FLOOR, PER_POINT, VitalNow};

/// The player's own Health and Mana (`ServerMessage::Vitals` with no `who`),
/// and when they were told, by `Time` (which stops while single player is
/// paused, as the server's clock does). `vitals` writes it; `None` until told.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct OwnVitals(pub Option<ToldVitals>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToldVitals {
    pub health: VitalNow,
    pub mana: VitalNow,
    /// `Time::elapsed_secs_f64` when they came.
    pub at: f64,
}

impl ToldVitals {
    /// Health and Mana `now` (by `Time`), refilled as the server would, with
    /// the part of a point refilled so far, for drawing smoothly.
    pub fn now(&self, now: f64) -> (f32, f32) {
        let ticks = ((now - self.at).max(0.0) * ACTION_TICKS_PER_SECOND) as f32;
        (refilled(self.health, ticks), refilled(self.mana, ticks))
    }

    /// Whether the player is fighting `now` (by `Time`), as the server would
    /// say: Health is still waiting to refill. No changing floors then.
    pub fn fighting(&self, now: f64) -> bool {
        let ticks = (now - self.at).max(0.0) * ACTION_TICKS_PER_SECOND;
        ticks < f64::from(self.health.refills_in)
    }
}

impl OwnVitals {
    /// `ToldVitals::fighting`, `false` until told.
    pub fn fighting(&self, now: f64) -> bool {
        self.0.is_some_and(|told| told.fighting(now))
    }
}

/// `VitalNow::after` with fractions (`vitals::course`): refilling after any
/// wait, what bonuses give back on top while refilling, and draining (poison)
/// all the while, never below 1, until they stop.
fn refilled(told: VitalNow, ticks: f32) -> f32 {
    let most = told.most as f32;
    let wait = told.refills_in as f32;
    let stop = told.restore_ends_in as f32;
    let per_tick = PER_POINT as f32 * ACTION_TICKS_PER_SECOND as f32;
    let refill = most / told.refill.max(1) as f32;
    let restore = told.restore as f32 / per_tick;
    let drain = told.drain as f32 / per_tick;
    let floor = DRAIN_FLOOR as f32 / PER_POINT as f32;
    // The rates only change at `wait` and `stop`.
    let mut cuts = [wait.min(ticks), stop.min(ticks), ticks];
    cuts.sort_by(f32::total_cmp);
    let (mut value, mut from) = (told.value.as_f32().min(most), 0.0);
    for to in cuts {
        if to <= from {
            continue;
        }
        let middle = (from + to) / 2.0;
        let refilling = if middle >= wait { refill } else { 0.0 };
        let restoring = if middle >= wait && middle < stop {
            restore
        } else {
            0.0
        };
        let draining = if middle < stop { drain } else { 0.0 };
        let change = (refilling + restoring - draining) * (to - from);
        value = if change >= 0.0 {
            (value + change).min(most.max(value))
        } else {
            (value + change).max(value.min(floor))
        };
        from = to;
    }
    value
}

/// How much of the window's bottom, in interface pixels, the interface keeps
/// across its middle (the Health and Mana circle): boxes naming what is
/// hovered stand above it.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct BottomHud(pub f32);

/// Where the Health and Mana circle is drawn, in interface pixels, at the bottom
/// middle of the window: how far across (0 while hidden) and how far its bottom
/// stands above the window's. The action bar stands either side of it.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct VitalsCircle {
    pub across: f32,
    /// Its bottom edge, above the window's.
    pub bottom: f32,
}

#[cfg(test)]
mod tests;
