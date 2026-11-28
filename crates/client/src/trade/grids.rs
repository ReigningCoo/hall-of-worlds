//! The window's grids: the own side (what is being sold, let go there from
//! the inventory), the Buying box (the shop's stock let go there is bought
//! where it lands, moved about in it bought there again), the shop's (its
//! stock), and the receive box (taken out into the inventory). Each is a
//! `widgets::StockGrid`: placed here, drawn again when what is in it changes,
//! with green where something held would land. In a trade with a character,
//! what is let go on the own side is offered where it lands; their side is
//! `sides`'.

use bevy::prelude::*;
use bevy::ui::UiGlobalTransform;
use shared::id::Id;
use shared::protocol::{ClientMessage, TradeAsk};
use shared::trade::{BoxPlace, Offer};

use super::state::TradeWindow;
use crate::common::{GaveItem, HeldOverGifts, HeldStock, Trading, buying_box, own_side};
use crate::connection::ToServer;
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
            ("trade", "buying") => {
                window.buying_holder = Some(entity);
                window.buying.changed = true;
            }
            ("trade", "stock") => {
                window.their_holder = Some(entity);
                if let Some(grid) = &mut window.theirs {
                    grid.changed = true;
                }
            }
            ("trade", "theirs") => {
                window.between_holder = Some(entity);
                window.dirty = true;
            }
            ("trade", "received") => {
                window.receive_holder = Some(entity);
                window.receive.changed = true;
            }
            _ => {}
        }
    }
}

/// Notes where things were let go on either grid, for when they come back;
/// in a trade with a character, offers what was let go on the own side where
/// it lands.
pub(super) fn note_gifts(
    mut gave: MessageReader<GaveItem>,
    grids: Query<(&StockGridNode, &ComputedNode, &UiGlobalTransform)>,
    mut window: ResMut<TradeWindow>,
    (trading, carried): (Res<Trading>, Res<Carried>),
    mut to_server: MessageWriter<ToServer>,
) {
    for GaveItem(gift) in gave.read() {
        let Some((_, node, transform)) = grids.iter().find(|(grid, ..)| grid.0 == gift.to) else {
            continue;
        };
        if gift.to == buying_box() {
            // Bought, or moved, where it lands; the server says where it lies.
            let stock = window.stock.iter().find(|item| item.id == gift.item);
            if let Some((_, (x, y), _)) = window.buying.aim(gift, stock, node, transform) {
                let at = Some(BoxPlace {
                    x,
                    y,
                    rotation: gift.rotation,
                });
                let ask = TradeAsk::Buy {
                    item: gift.item,
                    at,
                };
                to_server.write(ToServer(ClientMessage::Trade(ask)));
            }
        } else if gift.to == own_side() && trading.between.is_some() {
            if let Some((_, (x, y), _)) =
                window
                    .own
                    .aim(gift, carried.0.find(gift.item), node, transform)
            {
                let offer = Offer {
                    item: gift.item,
                    x,
                    y,
                    rotation: gift.rotation,
                };
                to_server.write(ToServer(ClientMessage::Trade(TradeAsk::Offer(offer))));
            }
            window.own.gave(gift, node, transform);
        } else if gift.to == own_side() {
            window.own.gave(gift, node, transform);
        } else if let Some(grid) = (window.theirs.as_mut()).filter(|grid| grid.with == gift.to) {
            grid.gave(gift, node, transform);
        }
    }
}

/// Draws the grids again when what is in them changes, or something is picked
/// up from them or put back. What is held isn't drawn where it was.
pub(super) fn draw_grids(
    (held, trading): (Res<HeldStock>, Res<Trading>),
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
    if let Some(holder) = window.buying_holder
        && (window.buying.changed || picked_up)
    {
        window.buying.changed = false;
        window.buying.show(&mut commands, holder, held.0);
    }
    if let Some(holder) = window.receive_holder
        && (window.receive.changed || picked_up)
    {
        window.receive.changed = false;
        window.receive.show(&mut commands, holder, held.0);
    }
    let Some(holder) = window.their_holder else {
        return;
    };
    // Something being bought is held from the Buying box: its copy in the
    // stock stays drawn.
    let from_stock = held.0.filter(|&id| trading.bought(id).is_none());
    match &mut window.theirs {
        Some(grid) if grid.changed || picked_up => {
            grid.changed = false;
            *drawn_empty = None;
            grid.show(&mut commands, holder, from_stock);
        }
        Some(_) => {}
        // No one to trade with: no stock, and no empty frames.
        None if *drawn_empty != Some(holder) => {
            *drawn_empty = Some(holder);
            commands
                .entity(holder)
                .remove::<StockOf>()
                .try_insert(GridShown {
                    columns: super::markup::STOCK_COLUMNS,
                    rows: 0,
                    ..default()
                });
        }
        None => {}
    }
}

/// Green where something held over a grid would land: none over the Buying
/// box for something new that can't be paid for.
pub(super) fn show_aim(
    held: Res<HeldOverGifts>,
    (carried, trading): (Res<Carried>, Res<Trading>),
    window: Res<TradeWindow>,
    grids: Query<(&StockGridNode, &ComputedNode, &UiGlobalTransform)>,
    mut shown: Query<(&StockOf, &mut GridShown)>,
) {
    let theirs = window.theirs.as_ref().map(|grid| grid.with);
    let ours = |with: Id| with == own_side() || with == buying_box() || Some(with) == theirs;
    let aim = held.0.filter(|gift| ours(gift.to)).and_then(|gift| {
        let (_, node, transform) = grids.iter().find(|(grid, ..)| grid.0 == gift.to)?;
        let grid = match gift.to {
            to if to == own_side() => &window.own,
            to if to == buying_box() => &window.buying,
            _ => window.theirs.as_ref()?,
        };
        let from = carried.0.find(gift.item);
        let stock = window.stock.iter().find(|item| item.id == gift.item);
        let new = gift.to == buying_box() && trading.bought(gift.item).is_none();
        let coins = carried.0.coins_carried();
        if new && stock.is_some_and(|item| !trading.affords(item, coins)) {
            return None;
        }
        grid.aim(&gift, from.or(stock), node, transform)
    });
    show_stock_aim(shown.iter_mut().filter(|(of, _)| ours(of.0)), aim);
}
