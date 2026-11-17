//! Herbs picked before their quality was their plant's rarity (owner,
//! 2026-11-17) had a rolled one: put right when a character loads, and the
//! stacks that now look alike joined, so old saves carry one stack per herb.

use super::Herbs;
use crate::item::{Inventory, Item, Placed, StackLimits};

/// Gives every herb carried (in any grid, bag or the bin) its plant's rarity
/// as its quality, then joins stacks that now look alike within each grid, as
/// far as `limits` allow. Herbs this server's table doesn't have are left as
/// they are. How many herb items changed quality.
pub fn settle_herbs(inventory: &mut Inventory, herbs: &Herbs, limits: StackLimits) -> usize {
    let mut changed = settle_grid(&mut inventory.items, herbs, limits);
    for item in inventory.worn.iter_mut().chain(&mut inventory.bag_slots) {
        changed += settle_item(item, herbs, limits);
    }
    if let Some(item) = &mut inventory.bin {
        changed += settle_item(item, herbs, limits);
    }
    changed
}

/// One item and everything in it.
fn settle_item(item: &mut Item, herbs: &Herbs, limits: StackLimits) -> usize {
    let mut changed = 0;
    if let Some(def) = item.herb().and_then(|herb| herbs.get(&herb.herb)) {
        let quality = Some(def.abundance.quality());
        if item.quality != quality {
            item.quality = quality;
            changed += 1;
        }
    }
    changed + settle_grid(&mut item.contents, herbs, limits)
}

/// A grid's items, then its herb stacks joined onto the first like them.
fn settle_grid(grid: &mut Vec<Placed>, herbs: &Herbs, limits: StackLimits) -> usize {
    let changed = grid
        .iter_mut()
        .map(|placed| settle_item(&mut placed.item, herbs, limits))
        .sum();
    let mut at = 0;
    while at < grid.len() {
        if grid[at].item.herb().is_some() && join_earlier(grid, at, limits) {
            grid.remove(at);
        } else {
            at += 1;
        }
    }
    changed
}

/// Moves as many of the herbs at `at` as fit onto earlier stacks like them.
/// Whether none are left there.
fn join_earlier(grid: &mut [Placed], at: usize, limits: StackLimits) -> bool {
    for earlier in 0..at {
        let room = limits.room(&grid[earlier].item, &grid[at].item);
        if room == 0 {
            continue;
        }
        // Counted: moving some is changing two numbers.
        grid[earlier].item.count += room;
        grid[at].item.count -= room;
        if grid[at].item.count == 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests;
