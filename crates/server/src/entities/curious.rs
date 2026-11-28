//! Curiosity and alarm calls (mob behaviour plan PR 8). A mob at rest that
//! has turned to a noise it still can't see may go and look (`Investigating`)
//! by its race's curiosity, to where it can see the noise's cell; it looks a
//! while, then goes home. A timid one steps away from it instead (`Shying`).
//! A hurt mob of a race with an alarm call gives it (`call_out`); its band
//! who hear it come over, timid ones keeping away, whatever they were doing
//! unless they are busy fighting, following or fleeing.

use shared::behaviour::{Behaviours, Relation};
use shared::dungeon::{Level, line_is_clear};
use shared::noise::{Noise, Source};
use shared::rng::Seed;
use shared::spot::Spot as Units;

use super::choice::Other;
use super::doubling_back::walk_towards;
use super::moves::take_step;
use super::noise::Made;
use super::timid::is_timid;
use super::{Doing, Maps, Mob, THINKING_COST, Taken, cells_apart};

/// How long it looks once it can see where the noise was: 5 seconds.
pub(super) const LOOK_TICKS: u64 = 100;
/// How long it may walk to see it before giving up: 30 seconds.
pub(super) const WALK_WITHIN: u64 = 600;
/// How far from home it goes to look, in cells.
pub(super) const FURTHEST_FROM_HOME: i64 = 24;
/// How many steps a timid one takes away from a noise.
pub(super) const SHY_STEPS: u8 = 6;

/// Whether `mob` can see the cell `at` from where it stands (turned to it).
pub(super) fn sees_cell(mob: &Mob, level: &Level, at: (i64, i64)) -> bool {
    cells_apart(mob.position, at) <= mob.kind.senses().sight_range()
        && line_is_clear(level, mob.position, at)
}

/// The nearest call heard among `made` (by index, as `heard` says) from one
/// of `mob`'s own band, if any.
pub(super) fn kin_call<'a>(
    mob: &Mob,
    made: &'a [Made],
    heard: &[usize],
    (others, behaviours): (&[Other], &Behaviours),
) -> Option<&'a Made> {
    let me = mob.member();
    (heard.iter().map(|&index| &made[index]))
        .filter(|made| made.noise.source == Source::Call)
        .filter(|made| {
            let caller = others.iter().find(|other| Some(other.id) == made.by);
            caller.is_some_and(|other| behaviours.relation(me, other.member) == Relation::Kin)
        })
        .min_by_key(|made| mob.spot.distance_squared(made.noise.at))
}

impl Mob {
    /// Whether it is free to answer a call: not fighting, following or fleeing.
    fn may_answer(&self) -> bool {
        matches!(
            self.doing,
            Doing::Idle
                | Doing::Settling { .. }
                | Doing::GoingHome { .. }
                | Doing::Investigating { .. }
                | Doing::Shying { .. }
        )
    }

    /// One of its band called from `at`: it comes over, a timid one keeps away.
    /// Whether it changed what it was doing.
    pub(super) fn answer(&mut self, at: Units, tick: u64) -> bool {
        if !self.may_answer() {
            return false;
        }
        self.doing = self.about_noise(at.cell(), tick);
        true
    }

    /// It has turned at rest to a noise at `at`: if it still can't see there,
    /// a timid one steps away, a curious one (by its curiosity, rolled) goes to
    /// look. Whether it changed what it was doing.
    pub(super) fn wonder(&mut self, level: &Level, at: Units, curiosity: u8, tick: u64) -> bool {
        let cell = at.cell();
        if self.doing != Doing::Idle || sees_cell(self, level, cell) {
            return false;
        }
        let (high, low) = self.id.0.as_u64_pair();
        let roll = Seed::new(high ^ low).at(&[tick as i64, cell.0, cell.1]) % 100;
        if !is_timid(self) && roll >= u64::from(curiosity) {
            return false;
        }
        self.doing = self.about_noise(cell, tick);
        self.doing != Doing::Idle
    }

    /// Going to look at `cell`, or shying from it if timid; staying put if it
    /// is too far from home to bother.
    fn about_noise(&self, cell: (i64, i64), tick: u64) -> Doing {
        if is_timid(self) {
            return Doing::Shying {
                from: cell,
                steps_left: SHY_STEPS,
            };
        }
        match cells_apart(self.home, cell) > FURTHEST_FROM_HOME {
            true => self.doing,
            false => Doing::Investigating {
                at: cell,
                since: tick,
                looking: None,
            },
        }
    }
}

/// A turn of a mob `Investigating` or `Shying` (`moves::act`): what it cost.
pub(super) fn look_into(
    mob: &mut Mob,
    level: &Level,
    maps: &mut Maps,
    tick: u64,
    taken: &Taken,
) -> u32 {
    let home = Doing::GoingHome { since: tick };
    match mob.doing {
        Doing::Investigating { at, since, looking } => {
            if let Some(until) = looking {
                mob.face(Units::middle_of(at));
                if tick >= until {
                    mob.doing = home;
                }
                return THINKING_COST;
            }
            if sees_cell(mob, level, at) {
                mob.face(Units::middle_of(at));
                mob.doing = Doing::Investigating {
                    at,
                    since,
                    looking: Some(tick + LOOK_TICKS),
                };
                return THINKING_COST;
            }
            if tick.saturating_sub(since) > WALK_WITHIN {
                mob.doing = home;
                return THINKING_COST;
            }
            walk_towards(mob, level, maps, at, taken).unwrap_or_else(|| {
                // No way there.
                mob.doing = home;
                THINKING_COST
            })
        }
        Doing::Shying { from, steps_left } => {
            mob.doing = match steps_left.checked_sub(1) {
                Some(steps_left) => Doing::Shying { from, steps_left },
                None => home,
            };
            let free = |cell: (i64, i64)| !taken.contains(&cell);
            let away = maps
                .to(level, mob.size(), from)
                .away(level, mob.position, free);
            take_step(mob, level, away, taken).unwrap_or(THINKING_COST)
        }
        _ => THINKING_COST,
    }
}

/// A mob just hurt gives its race's alarm call, if it has one, for the
/// level's mobs to hear when they next run.
pub(super) fn call_out(noises: &mut Vec<Made>, caller: &Mob, behaviours: &Behaviours) {
    let Some(alarm) = behaviours.of(caller.kind).alarm.as_ref() else {
        return;
    };
    noises.push(Made {
        noise: Noise::call(caller.spot, alarm.loudness),
        by: Some(caller.id),
        sneaker: None,
    });
}
