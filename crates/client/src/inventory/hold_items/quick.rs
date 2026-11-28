//! Doing a quick move (`hold::quick_target`): the same as picking it up and
//! letting go where it would go, so it is shown at once and sent as any move
//! is; or the red line under the clothes saying why it can't go.

use bevy::prelude::*;
use shared::item::StackLimits;

use super::super::hold::{QuickFrom, quick_target};
use super::super::panels::ItemCell;
use super::super::panels::OpenGrids;
use super::super::{Carried, HeldBy, Holding};
use super::Outbox;
use super::let_go::let_go_of;
use crate::common::{GROUND, GameView, HeldOverGifts, HeldStock, StockCell};
use crate::dom::Hand;

/// Moves `from` to the other side, if it can go.
#[allow(clippy::too_many_arguments)]
pub(super) fn quick_move(
    from: QuickFrom,
    open: &OpenGrids,
    view: &State<GameView>,
    limits: StackLimits,
    holding: &mut ResMut<Holding>,
    carried: &mut ResMut<Carried>,
    gifts: (&HeldOverGifts, &mut ResMut<HeldStock>),
    out: &mut Outbox,
) {
    // Only sites have ground.
    let has_ground = *view.get() == GameView::Dungeon;
    let bags = open.bags_open();
    let Some(found) = quick_target(&carried.0, &from, &bags, limits, has_ground) else {
        return;
    };
    let (held, aim) = match found {
        Ok(found) => found,
        Err(why) => {
            let now = out.wear.2.elapsed_secs_f64();
            out.wear.0.say_words(why.words(), now);
            return;
        }
    };
    holding.group = None;
    holding.counted = None;
    holding.stock = match from {
        QuickFrom::Ground(item) => Some((GROUND, *item)),
        // (Nothing in another grid gets this far.)
        QuickFrom::Carried(_) | QuickFrom::Stock { .. } => None,
    };
    let held = HeldBy {
        item: held,
        by: Hand::Mouse,
    };
    let_go_of(held, aim, holding, carried, gifts, out);
}

/// What a right-click on this would move: a carried item, one on the ground,
/// or one in another grid (a trade window's, which only `trade` moves).
pub(super) fn quick_from(
    entity: Entity,
    cells: &Query<&ItemCell>,
    stock_cells: &Query<&StockCell>,
) -> Option<QuickFrom> {
    if let Ok(cell) = cells.get(entity) {
        return Some(QuickFrom::Carried(cell.id));
    }
    let cell = stock_cells.get(entity).ok()?;
    Some(match cell.with == GROUND {
        true => QuickFrom::Ground(Box::new(cell.item.clone())),
        false => QuickFrom::Stock {
            with: cell.with,
            item: cell.item.id,
        },
    })
}
