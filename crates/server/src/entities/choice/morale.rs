//! Morale: how afraid a mob is of the enemies it sees, by how hurt it is, its
//! courage, and how many friends stand with it.

/// How much braver each friend in sight makes it, up to `MOST_FRIENDS`.
const PER_FRIEND: i32 = 10;
const MOST_FRIENDS: i32 = 3;
/// How much more afraid each enemy past one more than its side makes it.
const PER_EXTRA_ENEMY: i32 = 20;

/// Its fear, 0 up: what fleeing from its nearest enemy scores. `hurt` is the
/// share of its Health lost, 0 to 100; `courage` its race's drive; `enemies`
/// those it sees who are against it; `friends` its band and allies in sight.
/// Nothing without enemies. Hurt counts more the less courage it has (all of
/// it at 60, twice at 20); each enemy beyond its side's number adds fear, each
/// friend takes some away.
pub(in super::super) fn fear(courage: u8, hurt: i32, enemies: i32, friends: i32) -> i32 {
    if enemies <= 0 {
        return 0;
    }
    let from_hurt = hurt.clamp(0, 100) * (100 - i32::from(courage.min(100))) / 40;
    let outnumbered = (enemies - 1 - friends).max(0) * PER_EXTRA_ENEMY;
    let braver = friends.clamp(0, MOST_FRIENDS) * PER_FRIEND;
    (from_hurt + outnumbered - braver).max(0)
}
