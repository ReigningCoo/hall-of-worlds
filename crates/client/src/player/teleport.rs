//! A Teleport's move (magic plan PR 13): while one lies on the player
//! (`Sheet::may_teleport`), a click on a cell they have seen, in range and in
//! a clear line, takes them there at once instead of walking
//! (`spell::may_teleport_to`, which the server checks again).

use shared::creature::Size;
use shared::dungeon::Level;
use shared::sheet::Sheet;
use shared::spell::may_teleport_to;
use shared::spot::Spot;

/// Where a click on `goal` teleports the player, standing at `at`, if it
/// does: the cell's middle.
pub(super) fn teleport_goal(
    sheet: &Sheet,
    (level, size): (&Level, Size),
    at: Spot,
    goal: (i64, i64),
    seen: impl Fn(i64, i64) -> bool,
) -> Option<Spot> {
    let to = Spot::middle_of(goal);
    let allowed =
        sheet.may_teleport() && seen(goal.0, goal.1) && may_teleport_to(level, size, at, to);
    allowed.then_some(to)
}
