//! The window's two grids: the own side (what is being sold, let go there from
//! the inventory) and the shop's (its stock, dragged into the inventory to
//! buy). Each is a `widgets::StockGrid`: placed here, drawn again when what is
//! in it changes, with green where something held would land.

use bevy::prelude::*;
use bevy::ui::UiGlobalTransform;
use shared::id::Id;

use super::state::TradeWindow;
use crate::common::{GaveItem, HeldOverGifts, HeldStock, own_side};
use crate::dom::GridShown;
use crate::inventory::Carried;
use crate::widgets::{StockGridNode, StockOf, UiId, show_stock_aim};

/// Notes each grid once built, to be drawn in.
pub(super) fn bind_grids(
    new: Query<(Entity, &UiId), Added<UiId>>,
    mut window: ResMut<TradeWindow>,
) {
    for (entity, id) in &new {
        match id.parts() {
            ("trade", "own") => {
                window.own_holder = Some(entity);
                window.own.changed = true;
            }
            ("trade", "stock") => {
                window.their_holder = Some(entity);
                if let Some(grid) = &mut window.theirs {
                    grid.changed = true;
                }
            }
            _ => {}
        }
    }
}

/// Notes where things were let go on either grid, for when they come back.
pub(super) fn note_gifts(
    mut gave: MessageReader<GaveItem>,
    grids: Query<(&StockGridNode, &ComputedNode, &UiGlobalTransform)>,
    mut window: ResMut<TradeWindow>,
) {
    for GaveItem(gift) in gave.read() {
        let Some((_, node, transform)) = grids.iter().find(|(grid, ..)| grid.0 == gift.to) else {
            continue;
        };
        if gift.to == own_side() {
            window.own.gave(gift, node, transform);
        } else if let Some(grid) = (window.theirs.as_mut()).filter(|grid| grid.with == gift.to) {
            grid.gave(gift, node, transform);
        }
    }
}

/// Draws the grids again when what is in them changes, or something is picked
/// up from them or put back. What is held isn't drawn where it was.
pub(super) fn draw_grids(
    held: Res<HeldStock>,
    mut window: ResMut<TradeWindow>,
    mut commands: Commands,
    (mut drawn_without, mut drawn_empty): (Local<Option<Id>>, Local<Option<Entity>>),
) {
    let picked_up = *drawn_without != held.0;
    *drawn_without = held.0;
    let window = &mut *window;
    if let Some(holder) = window.own_holder
        && (window.own.changed || picked_up)
    {
        window.own.changed = false;
        window.own.show(&mut commands, holder, held.0);
    }
    let Some(holder) = window.their_holder else {
        return;
    };
    match &mut window.theirs {
        Some(grid) if grid.changed || picked_up => {
            grid.changed = false;
            *drawn_empty = None;
            grid.show(&mut commands, holder, held.0);
        }
        Some(_) => {}
        // No one to trade with: an empty grid.
        None if *drawn_empty != Some(holder) => {
            *drawn_empty = Some(holder);
            commands
                .entity(holder)
                .remove::<StockOf>()
                .try_insert(GridShown {
                    columns: super::markup::STOCK_COLUMNS,
                    rows: 4,
                    ..default()
                });
        }
        None => {}
    }
}

/// Green where something held over either grid would land.
pub(super) fn show_aim(
    held: Res<HeldOverGifts>,
    carried: Res<Carried>,
    window: Res<TradeWindow>,
    grids: Query<(&StockGridNode, &ComputedNode, &UiGlobalTransform)>,
    mut shown: Query<(&StockOf, &mut GridShown)>,
) {
    let theirs = window.theirs.as_ref().map(|grid| grid.with);
    let ours = |with: Id| with == own_side() || Some(with) == theirs;
    let aim = held.0.filter(|gift| ours(gift.to)).and_then(|gift| {
        let (_, node, transform) = grids.iter().find(|(grid, ..)| grid.0 == gift.to)?;
        let grid = match gift.to == own_side() {
            true => &window.own,
            false => window.theirs.as_ref()?,
        };
        grid.aim(&gift, &carried.0, node, transform)
    });
    show_stock_aim(shown.iter_mut().filter(|(of, _)| ours(of.0)), aim);
}
