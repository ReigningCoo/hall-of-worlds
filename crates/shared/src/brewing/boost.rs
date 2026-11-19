//! Strengthening a dose by a quarter (decisions 2 and 3: Cave and Moon Water,
//! an oil's row), and which row a stat and an oil belong to.

use crate::herb::Stat;
use crate::potion::Oil;
use crate::sheet::Row;

/// A dose made a quarter stronger, at least one step, whichever way it goes:
/// 4 → 5, -2 → -3, 0 stays 0.
pub fn boosted(step: i32) -> i32 {
    let more = (step.abs() / 4).max(1);
    match step {
        0 => 0,
        s if s > 0 => s + more,
        s => s - more,
    }
}

/// The row a stat counts in (decision 3): an attribute's own, Health with
/// Body and Mana with Mind.
pub fn stat_row(stat: Stat) -> Row {
    match stat {
        Stat::Health => Row::Body,
        Stat::Mana => Row::Mind,
        other => other.attribute().map_or(Row::Body, |a| a.row()),
    }
}

/// The row an oil boosts (decision 3, Alchemy plan): Grain Body, Nut Mind,
/// Seed Spirit; Neutral none.
pub fn oil_suits(oil: Oil) -> Option<Row> {
    match oil {
        Oil::Neutral => None,
        Oil::Grain => Some(Row::Body),
        Oil::Nut => Some(Row::Mind),
        Oil::Seed => Some(Row::Spirit),
    }
}
