//! Green where something held over a stock grid would land.

use bevy::prelude::*;

use super::{StockAimAt, StockOf};
use crate::dom::{GridAim, GridShown};

/// Green where something held would go if let go now, in the grids given.
pub fn show_stock_aim<'a>(
    grids: impl Iterator<Item = (&'a StockOf, Mut<'a, GridShown>)>,
    aim: Option<StockAimAt>,
) {
    for (of, mut shown) in grids {
        let now = aim.filter(|aim| aim.0 == of.0).map(|(_, (x, y), shape)| {
            let cells = shape.cells();
            GridAim {
                cells: cells
                    .map(|(cell_x, cell_y)| ((x + cell_x) as i32, (y + cell_y) as i32))
                    .collect(),
                allowed: true,
            }
        });
        if shown.aim != now {
            shown.aim = now;
        }
    }
}
