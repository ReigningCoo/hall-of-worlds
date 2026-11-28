//! Scoring a mob's options and picking one: plain numbers, so tests can check
//! them without a level.

use shared::behaviour::Drives;

use super::super::timid::ENEMY_APART;
use super::morale::fear;
use super::seen::{Seen, Wants};
use super::{Choice, Scored};

/// How much better a new choice must score than what it is doing, so it
/// doesn't flip between two.
pub(in super::super) const MARGIN: i32 = 10;
/// Keeping away from someone too near (timid mobs, or one under Fear).
const KEEP_AWAY: i32 = 60;
const APPROACH: i32 = 40;

/// What scoring needs to know of the mob itself.
#[derive(Clone, Copy, Debug)]
pub(in super::super) struct Mind {
    pub drives: Drives,
    /// Share of its Health lost, 0 to 100.
    pub hurt: i32,
    /// Its band and allies in sight.
    pub friends: i32,
    pub fears_fire: bool,
}

/// Attacking scores 50 to 100 by aggression.
pub(in super::super) fn attack_score(drives: Drives) -> i32 {
    50 + i32::from(drives.aggression.min(100)) / 2
}

/// Guarding scores 45 to 70 by its guarding drive.
fn guard_score(drives: Drives) -> i32 {
    45 + i32::from(drives.guarding.min(100)) / 4
}

/// Every option `seen` gives, in order, and its fear: fleeing from its
/// nearest enemy within `ENEMY_APART` scores that. Someone by an open fire is no option
/// to go up to for one that fears fire.
pub(in super::super) fn score(mind: &Mind, seen: &[Seen]) -> (Vec<Scored>, i32) {
    let mut options = Vec::new();
    for one in seen {
        let by_fire = mind.fears_fire && one.near_fire;
        let option = match one.wants {
            Wants::Nothing => None,
            Wants::Attack if by_fire => None,
            Wants::Approach if by_fire => None,
            Wants::Attack => Some((Choice::Attack(one.id), attack_score(mind.drives))),
            Wants::Approach => Some((Choice::Approach(one.id), APPROACH)),
            Wants::Guard => Some((Choice::Guard(one.id), guard_score(mind.drives))),
            Wants::KeepAway => Some((Choice::Flee(one.id), KEEP_AWAY)),
        };
        if let Some((choice, score)) = option {
            options.push(Scored { choice, score });
        }
    }
    let enemies = seen.iter().filter(|one| one.enemy).count() as i32;
    let fear = fear(mind.drives.courage, mind.hurt, enemies, mind.friends);
    let nearest = (seen.iter())
        .filter(|one| one.enemy && one.apart < ENEMY_APART)
        .min_by_key(|one| one.apart);
    if let Some(enemy) = nearest.filter(|_| fear > 0) {
        let choice = Choice::Flee(enemy.id);
        match options.iter_mut().find(|option| option.choice == choice) {
            Some(option) => option.score = option.score.max(fear),
            None => options.push(Scored {
                choice,
                score: fear,
            }),
        }
    }
    (options, fear)
}

/// The best option (the first of equals), unless what it is doing now
/// (`current`) is among them and scores within `MARGIN` of it: then that.
/// `None` when there are no options.
pub(in super::super) fn pick(options: &[Scored], current: Option<Choice>) -> Option<Choice> {
    let best = options
        .iter()
        .fold(None::<&Scored>, |best, option| match best {
            Some(best) if best.score >= option.score => Some(best),
            _ => Some(option),
        })?;
    let now = options.iter().find(|option| Some(option.choice) == current);
    match now {
        Some(now) if best.score <= now.score + MARGIN => Some(now.choice),
        _ => Some(best.choice),
    }
}
