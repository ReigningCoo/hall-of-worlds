//! Prey near a pack's den (mob behaviour plan PR 5): a block whose pack hunts
//! (its race's `hunts`, from the game's own behaviours, so mods never change
//! what is made) often has a herd or warren of its prey not far off, within
//! the hunters' sight, if any of its prey may live there. Otherwise one kind to
//! a block, and blocks' dens anywhere, hunters would seldom meet their prey.

use super::wild::{TRIES, open, pick};
use super::wild_shares::Resident;
use crate::behaviour::Behaviours;
use crate::creature::CreatureKind;
use crate::dungeon::Level;

/// Where the prey's members come in the block's numbering: after the pack's.
pub(super) const PREY_MEMBERS: i64 = 50;
/// How many packs out of 100 that could have prey near have it.
const PREY_PERCENT: u64 = 60;
/// How far from the pack's den the prey's is, in cells each way: nearest.
const NEAREST: i64 = 10;
/// And furthest: within a wolf's sight.
const FURTHEST: i64 = 16;
/// Which of a block's rolls are for its prey.
const PREY_ROLL: i64 = 200;

/// The prey living near a pack of `hunter` round `den`, and its den, if any:
/// one of `lives_here` the hunter hunts, by their weights.
pub(super) fn prey_near(
    hunter: CreatureKind,
    den: (i64, i64),
    lives_here: &[(Resident, u64)],
    level: &Level,
    roll: &impl Fn(i64) -> u64,
) -> Option<(Resident, (i64, i64))> {
    let hunts = &Behaviours::built_in_ref().of(hunter).hunts;
    if hunts.is_empty() || roll(PREY_ROLL) % 100 >= PREY_PERCENT {
        return None;
    }
    let prey: Vec<(Resident, u64)> = (lives_here.iter())
        .filter(|(resident, _)| hunts.contains(&resident.kind))
        .copied()
        .collect();
    if prey.is_empty() {
        return None;
    }
    let resident = pick(roll(PREY_ROLL + 1), &prey);
    let span = (2 * FURTHEST + 1) as u64;
    let at = (0..TRIES).find_map(|attempt| {
        let dx = (roll(PREY_ROLL + 2 + attempt * 2) % span) as i64 - FURTHEST;
        let dy = (roll(PREY_ROLL + 3 + attempt * 2) % span) as i64 - FURTHEST;
        let (x, y) = (den.0 + dx, den.1 + dy);
        let far_enough = dx.abs().max(dy.abs()) >= NEAREST;
        (far_enough && level.contains(x, y) && open(level, x, y)).then_some((x, y))
    })?;
    Some((resident, at))
}
