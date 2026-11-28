//! The panel's grid of tools, as a shop's stock is drawn (`widgets::StockGrid`):
//! its node once built, things let go on it or taken off it, drawing it
//! again, and green where something held would land. What it is told and
//! what is asked of the server is `inventory`'s, through the bench's id
//! (`common::Benches::id_of`).

use bevy::prelude::*;
use bevy::ui::UiGlobalTransform;
use shared::id::Id;

use super::BenchPanel;
use crate::common::{GaveItem, HeldOverGifts, HeldStock, TookItem};
use crate::dom::GridShown;
use crate::inventory::Carried;
use crate::widgets::{StockGridNode, StockOf, UiId, show_stock_aim};

/// Gives the grid, once built, to the open panel.
pub(super) fn bind_grid(new: Query<(Entity, &UiId), Added<UiId>>, mut panel: ResMut<BenchPanel>) {
    for (entity, id) in &new {
        if id.parts() != ("bench", "tools") {
            continue;
        }
        if let Some(open) = &mut panel.0 {
            open.holder = Some(entity);
            open.grid.changed = true;
        }
    }
}

/// Notes where tools put on were let go, and takes away those taken off.
pub(super) fn note_gifts(
    mut gave: MessageReader<GaveItem>,
    mut took: MessageReader<TookItem>,
    grids: Query<(&StockGridNode, &ComputedNode, &UiGlobalTransform)>,
    mut panel: ResMut<BenchPanel>,
) {
    let Some(open) = &mut panel.0 else {
        gave.clear();
        took.clear();
        return;
    };
    let with = open.grid.with;
    for GaveItem(gift) in gave.read().filter(|gave| gave.0.to == with) {
        let grid = grids.iter().find(|(grid, ..)| grid.0 == with);
        if let Some((_, node, transform)) = grid {
            open.grid.gave(gift, node, transform);
        }
    }
    for took in took.read().filter(|took| took.with == with) {
        open.grid.took(took.item);
    }
}

/// Draws the grid again when what is on it changes, or a tool is picked up
/// from it or put back. What is held isn't drawn where it was.
pub(super) fn draw_grid(
    held: Res<HeldStock>,
    mut panel: ResMut<BenchPanel>,
    mut commands: Commands,
    mut drawn_without: Local<Option<Id>>,
) {
    let picked_up = *drawn_without != held.0;
    *drawn_without = held.0;
    let Some(open) = &mut panel.0 else {
        return;
    };
    if let Some(holder) = open.holder
        && (open.grid.changed || picked_up)
    {
        open.grid.changed = false;
        open.grid.show(&mut commands, holder, held.0);
    }
}

/// Green where something held would land: anywhere free, whatever it is, so
/// it never tells what the bench takes.
pub(super) fn show_gift_aim(
    held: Res<HeldOverGifts>,
    carried: Res<Carried>,
    panel: Res<BenchPanel>,
    grids: Query<(&StockGridNode, &ComputedNode, &UiGlobalTransform)>,
    mut shown: Query<(&StockOf, &mut GridShown)>,
) {
    let Some(open) = &panel.0 else {
        return;
    };
    let with = open.grid.with;
    let aim = held.0.filter(|gift| gift.to == with).and_then(|gift| {
        let (_, node, transform) = grids.iter().find(|(grid, ..)| grid.0 == with)?;
        open.grid
            .aim(&gift, carried.0.find(gift.item), node, transform)
    });
    let ours = shown.iter_mut().filter(|(of, _)| of.0 == with);
    show_stock_aim(ours, aim);
}
