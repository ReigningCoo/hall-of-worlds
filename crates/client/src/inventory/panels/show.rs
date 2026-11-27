//! Showing what is in each open grid, and how much it weighs (`widgets::weight_words`).

use bevy::prelude::*;
use shared::id::Id;
use shared::item::{Holder, Inventory, Placed};

use super::super::filter::{GridFilter, chosen, dims};
use super::super::{Carried, Holding};
use super::shown::shown_size;
use super::{GridOwner, WeightText};
use crate::common::{CarryLimit, Trading};
use crate::dom::{GridShown, GridThing};
use crate::widgets::{Dropdown, ItemMark, weight_words};

/// Says what is in each open grid again when it changes, or its filter does, and
/// how big it is. What is held isn't shown where it was, though it is still there
/// until dropped. A trade marks what is being sold, and draws what is being
/// bought where it is to go, unpaid (`common::Trading`).
pub(in crate::inventory) fn show_grids(
    carried: Res<Carried>,
    trading: Res<Trading>,
    holding: Res<Holding>,
    new_grids: Query<(), Added<GridOwner>>,
    mut grids: Query<(&GridOwner, &mut GridShown)>,
    filters: Query<(&GridFilter, Ref<Dropdown>)>,
    mut drawn_without: Local<Option<Id>>,
) {
    let held = holding.held.map(|held| held.item.id);
    let filtered = filters.iter().any(|(_, dropdown)| dropdown.is_changed());
    let unchanged = !carried.is_changed() && !trading.is_changed();
    if unchanged && new_grids.is_empty() && *drawn_without == held && !filtered {
        return;
    }
    *drawn_without = held;
    for (owner, mut shown) in &mut grids {
        let holder = owner.holder;
        let filter = filters
            .iter()
            .find(|(filter, _)| filter.0 == Some(holder))
            .and_then(|(_, dropdown)| chosen(&dropdown));
        let placed = carried
            .0
            .grid(holder)
            .into_iter()
            .flat_map(|grid| grid.iter());
        let mark = |placed: &Placed| match trading.is_selling(placed.item.id) {
            true => ItemMark::Selling,
            false => ItemMark::None,
        };
        let unpaid = (trading.buying.iter()).filter(|(buy, _)| buy.to == holder);
        let unpaid = unpaid.map(|(buy, item)| Placed {
            item: item.clone(),
            x: buy.x,
            y: buy.y,
            rotation: buy.rotation,
        });
        let unpaid: Vec<Placed> = unpaid.collect();
        let things = placed
            .filter(|placed| Some(placed.item.id) != held)
            .map(|placed| thing(placed, dims(filter, &placed.item), mark(placed)))
            .chain(
                unpaid
                    .iter()
                    .filter(|placed| Some(placed.item.id) != held)
                    .map(|placed| thing(placed, dims(filter, &placed.item), ItemMark::Unpaid)),
            )
            .collect();
        let (columns, rows) = shown_size(&carried.0, holder).unwrap_or((shown.columns, shown.rows));
        let now = GridShown {
            columns,
            rows,
            things,
            aim: shown.aim.clone(),
            takes_gifts: None,
            cells: short_rows(&carried.0, holder, columns, rows),
        };
        // (Only when it differs: the grid draws itself again when changed.)
        if *shown != now {
            *shown = now;
        }
    }
}

/// Writes each open grid's weight under it again when what is carried, or the most
/// that can be, changes.
pub(in crate::inventory) fn show_weights(
    carried: Res<Carried>,
    limit: Res<CarryLimit>,
    new_weights: Query<(), Added<WeightText>>,
    mut weights: Query<(&WeightText, &mut Text)>,
) {
    if !carried.is_changed() && !limit.is_changed() && new_weights.is_empty() {
        return;
    }
    for (weight, mut text) in &mut weights {
        **text = grid_weight(&carried.0, weight.holder, limit.0);
    }
}

/// How many cells the own grid has when bags tied to the belt leave its last row
/// short (`Inventory::grid_cells`); `None` for a full grid.
fn short_rows(carried: &Inventory, holder: Holder, columns: u8, rows: u8) -> Option<u32> {
    let cells = match holder {
        Holder::Inventory => carried.grid_cells(),
        Holder::Inside(_) => return None,
    };
    (cells < columns as u32 * rows as u32).then_some(cells)
}

/// A placed item, as a grid shows it.
fn thing(placed: &Placed, dimmed: bool, mark: ItemMark) -> GridThing {
    GridThing {
        item: placed.item.clone(),
        x: placed.x,
        y: placed.y,
        rotation: placed.rotation,
        dimmed,
        mark,
    }
}

pub(super) fn grid_weight(carried: &Inventory, holder: Holder, limit: u32) -> String {
    match holder {
        Holder::Inventory => weight_words(carried.total_weight(), Some(limit)),
        Holder::Inside(id) => {
            let Some(item) = carried.find(id) else {
                return String::new();
            };
            let held: u32 = item
                .contents
                .iter()
                .map(|placed| placed.item.total_weight())
                .sum();
            weight_words(
                held,
                item.kind.def().container.and_then(|bag| bag.max_total),
            )
        }
    }
}
