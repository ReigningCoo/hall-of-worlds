//! How Health or Mana runs over time: refilling (after any wait), restoring
//! from bonuses on top while refilling, and draining (poison) all the while,
//! never below 1 point. Worked out the same for the server's `Track::at` and
//! the client's `VitalNow::after`.

use super::TICKS;
use super::points::{PER_POINT, Points};
use super::track::Rate;

/// Draining never takes it below this, in thousandths: 1 point (owner,
/// 2026-11-14: poison never kills). Harm can, and so can a deadly poison
/// (`Rate::deadly`, fungi plan decision 8), draining to 0.
pub const DRAIN_FLOOR: u32 = PER_POINT;

/// What `value` comes to after `ticks`, coming back at `rate`: refilling
/// starts after `wait` ticks, and restoring and draining stop after `stop`
/// ticks (`u64::MAX` for never).
///
/// The rates only change at `wait` and `stop`, so the time is cut there into
/// up to three stretches, each going only up or only down: going up stops at
/// the most, going down at `DRAIN_FLOOR`. Each stretch's change is the
/// difference between running totals counted from the start, so the whole
/// adds up exactly as one sum would.
pub fn course(value: Points, rate: Rate, wait: u64, stop: u64, ticks: u64) -> Points {
    let most = i64::from(Points::whole(rate.most).milli());
    let floor = if rate.deadly {
        0
    } else {
        i64::from(DRAIN_FLOOR)
    };
    let mut value = i64::from(value.milli()).min(most);
    let mut from = 0;
    let mut cuts = [wait.min(ticks), stop.min(ticks), ticks];
    cuts.sort_unstable();
    for to in cuts {
        if to <= from {
            continue;
        }
        let change = net(rate, wait, stop, to) - net(rate, wait, stop, from);
        value = if change >= 0 {
            (value + change).min(most.max(value))
        } else {
            (value + change).max(value.min(floor))
        };
        from = to;
    }
    Points(value.clamp(0, i64::from(u32::MAX)) as u32)
}

/// Everything gained less everything drained in the first `ticks`, in
/// thousandths, not stopped at the most or the floor.
fn net(rate: Rate, wait: u64, stop: u64, ticks: u64) -> i64 {
    let most = u64::from(rate.most) * u64::from(PER_POINT);
    let refilling = ticks.saturating_sub(wait);
    let restoring = ticks.min(stop).saturating_sub(wait);
    let draining = ticks.min(stop);
    let refilled = refilling.saturating_mul(most) / rate.refill.max(1);
    let restored = restoring.saturating_mul(rate.restore.into()) / TICKS;
    let drained = draining.saturating_mul(rate.drain.into()) / TICKS;
    to_i64(refilled) + to_i64(restored) - to_i64(drained)
}

fn to_i64(value: u64) -> i64 {
    value.min(i64::MAX as u64 / 4) as i64
}
