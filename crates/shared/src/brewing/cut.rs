//! Herbalism cutting unwanted effects (decision 13).

use crate::herb::Stat;

/// Takes `steps` off the doses going against the wanted way, one step at a
/// time, round robin from the largest. The wanted way is the sign of the
/// largest dose; a tie between a helpful and a harmful one counts as helpful
/// (Claude's choice). Doses cut to 0 stay in the list, at 0.
pub fn cut_unwanted(doses: &mut [(Stat, i32)], steps: u32) {
    let Some(largest) = doses.iter().map(|(_, s)| s.abs()).max() else {
        return;
    };
    let wanted_up = doses.iter().any(|&(_, s)| s == largest);
    let unwanted = |s: i32| s != 0 && (s > 0) != wanted_up;
    // The unwanted ones, largest first (ties in `Stat`'s order, as `doses` is).
    let mut order: Vec<usize> = (0..doses.len()).filter(|&i| unwanted(doses[i].1)).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(doses[i].1.abs()));
    let mut left = steps;
    while left > 0 && order.iter().any(|&i| doses[i].1 != 0) {
        for &i in &order {
            if left == 0 {
                break;
            }
            let step = &mut doses[i].1;
            if *step != 0 {
                *step -= step.signum();
                left -= 1;
            }
        }
    }
}
