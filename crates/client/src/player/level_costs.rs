//! What moves on a level cost the player, as the server prices them: the
//! smooth ground's slopes, wading, terrain, jumps and squeezes
//! (`shared::movement::LevelWay`), and a sneaking pace while sneaking.

use bevy::math::DVec2;
use shared::clearance::{Other, crowd_per_mille};
use shared::creature::Size;
use shared::dungeon::Level;
use shared::energy::{level_move_energy, site_move_cost, sneak_move_energy};
use shared::movement::{Gait, LevelWay, move_on_level};
use shared::relief::Relief;
use shared::spot::{CELL_SPAN, Spot};

use crate::common::WildCellHeights;

/// How far ahead of the player the squeeze is looked at when pacing a walk,
/// in units: about a frame's walk at most, and one of `Stride`'s pieces.
const LOOK_AHEAD: f64 = 40.0;

/// Everything a move's cost depends on.
pub(super) struct LevelCosts<'a> {
    pub level: &'a Level,
    /// The wilderness's terrain, by cell.
    pub heights: &'a WildCellHeights,
    /// The level's ground, as drawn (`common::DrawnGround`).
    pub relief: Option<&'a Relief>,
    pub gait: Gait,
    pub sneaking: bool,
    /// The player's speed, which a sneaking pace is worked out from.
    pub speed: u32,
    /// How big the player is, and who else is about: squeezing through gaps
    /// and past others is slower (`shared::clearance`).
    pub size: Size,
    pub others: &'a [Other],
}

impl LevelCosts<'_> {
    /// The way a move landing at `to` goes (its terrain is the landing's).
    pub(super) fn way(&self, to: Spot) -> LevelWay<'_> {
        LevelWay {
            level: self.level,
            relief: self.relief,
            terrain: self.heights.terrain(to.cell()),
            gait: self.gait,
        }
    }

    /// The energy a move takes before squeezing: no quicker than a sneaking
    /// pace while sneaking.
    fn time(&self, from: Spot, to: Spot, jumped: bool) -> u32 {
        let cost = level_move_energy(self.way(to).cost(from, to, jumped));
        if self.sneaking {
            sneak_move_energy(cost, from.distance(to), self.speed)
        } else {
            cost
        }
    }

    /// The energy a move in units takes, as the server prices it, slower for
    /// squeezing.
    pub(super) fn move_time(&self, from: Spot, to: Spot) -> u32 {
        let landing = move_on_level(self.level, self.size, self.gait, from, to);
        let jumped = landing.is_some_and(|landing| landing.jumped());
        let squeeze = u64::from(self.squeeze(from, to));
        (u64::from(self.time(from, to, jumped)) * squeeze / 1000) as u32
    }

    /// How much slower squeezing makes a move, in thousandths (1000: not at
    /// all): through gaps and past others, the slower, as `move_time` charges.
    pub(super) fn squeeze(&self, from: Spot, to: Spot) -> u32 {
        let landing = move_on_level(self.level, self.size, self.gait, from, to);
        let gap = landing.map_or(1000, |landing| landing.squeeze_per_mille());
        let crowd = crowd_per_mille(self.size, from, to, self.others).unwrap_or(1000);
        gap.max(crowd)
    }

    /// How much longer than on flat dry ground walking on from `at` going
    /// `way` (a unit vector) takes: `slowness` a cell ahead, times the squeeze
    /// the move under way (from `start`, where the stretch began, to just
    /// ahead) will be charged. So squeezing is a slow walk that keeps pace
    /// with the energy, rather than one that runs ahead and stutters.
    pub(super) fn pace(&self, start: Spot, at: Spot, way: DVec2) -> f64 {
        let ahead = |length: f64| {
            at.offset(
                (way.x * length).round() as i64,
                (way.y * length).round() as i64,
            )
        };
        let squeeze = self.squeeze(start, ahead(LOOK_AHEAD));
        self.slowness(at, ahead(CELL_SPAN as f64)) * f64::from(squeeze) / 1000.0
    }

    /// How much longer than on flat dry ground walking from `from` to `to`
    /// takes (less than 1 when a gait makes it quicker): slopes, wading,
    /// terrain and sneaking. Free movement is paced by distance, with this.
    pub(super) fn slowness(&self, from: Spot, to: Spot) -> f64 {
        if from == to {
            return 1.0;
        }
        let flat = site_move_cost(from, to).max(1);
        f64::from(self.time(from, to, false)) / f64::from(flat)
    }
}

#[cfg(test)]
mod tests;
