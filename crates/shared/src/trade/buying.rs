//! The Buying box of a shop trade (trading tweaks, owner): what is being
//! bought lies in it like a grid 8 wide, as each side of a trade with a
//! character does, at least 4 rows high, a row more whenever something added
//! wouldn't fit. Nothing bought is in the inventory before completing.

use serde::{Deserialize, Serialize};

use super::{Offer, Refusal, SIDE_CELLS, ShopTrade};
use crate::id::Id;
use crate::item::{Item, Rotation, Shape};

/// How wide the Buying box is.
pub const BUYING_COLUMNS: u8 = SIDE_CELLS;

/// How few rows it has: as many as it is when empty.
pub const BUYING_ROWS: u8 = 4;

/// Where in the Buying box something is put.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoxPlace {
    pub x: u8,
    pub y: u8,
    pub rotation: Rotation,
}

impl ShopTrade {
    /// How many rows the Buying box has: `BUYING_ROWS`, or down to its
    /// lowest thing. `stock` is the shop's, things being bought included.
    pub fn buying_rows(&self, stock: &[Item]) -> u8 {
        rows(&taken(&self.buying, stock))
    }

    /// `item` from the shop's `stock` put in the Buying box at `at` (moved,
    /// if it is there already), or with no place in the first free one,
    /// growing the box by rows if need be. A place given must be free and in
    /// the box as it is, unless nothing like it fits anywhere there: then it
    /// goes in the first place below.
    pub fn buy(&mut self, item: Id, at: Option<BoxPlace>, stock: &[Item]) -> Result<(), Refusal> {
        if self.selling.contains(&item) {
            return Err(Refusal::AlreadyIn);
        }
        let found = stock.iter().find(|one| one.id == item);
        let shape = found.ok_or(Refusal::NotStocked)?.shape();
        let rest: Vec<Offer> = (self.buying.iter())
            .filter(|offer| offer.item != item)
            .copied()
            .collect();
        let cells = taken(&rest, stock);
        let rows = rows(&cells);
        let place = match at {
            Some(at) if fits(&cells, shape.turned(at.rotation), at.x, at.y, rows) => at,
            Some(at) if first_free(&cells, shape.turned(at.rotation), rows).is_some() => {
                return Err(Refusal::NoRoom);
            }
            Some(at) => below(&cells, shape, at.rotation)?,
            None => below(&cells, shape, Rotation::None)?,
        };
        self.buying = rest;
        self.buying.push(Offer {
            item,
            x: place.x,
            y: place.y,
            rotation: place.rotation,
        });
        Ok(())
    }

    /// Something being bought put back in the shop's stock.
    pub fn unbuy(&mut self, item: Id) {
        self.buying.retain(|offer| offer.item != item);
    }
}

/// The cells the things in a box take up.
fn taken(offers: &[Offer], stock: &[Item]) -> Vec<(u8, u8)> {
    let shaped = offers.iter().filter_map(|offer| {
        let item = stock.iter().find(|item| item.id == offer.item)?;
        Some((item.shape().turned(offer.rotation), offer.x, offer.y))
    });
    let cells = shaped.flat_map(|(shape, left, top)| {
        (shape.cells()).map(move |(x, y)| (left.saturating_add(x), top.saturating_add(y)))
    });
    cells.collect()
}

/// The rows a box with these cells taken shows.
fn rows(cells: &[(u8, u8)]) -> u8 {
    let lowest = cells.iter().map(|&(_, y)| y.saturating_add(1)).max();
    lowest.unwrap_or(0).max(BUYING_ROWS)
}

/// Whether a shape fits with its corner at (`left`, `top`), within `rows`.
fn fits(cells: &[(u8, u8)], shape: Shape, left: u8, top: u8, rows: u8) -> bool {
    left as u32 + shape.width as u32 <= BUYING_COLUMNS as u32
        && top as u32 + shape.height as u32 <= rows as u32
        && (shape.cells()).all(|(x, y)| !cells.contains(&(left + x, top + y)))
}

/// The first free place for a shape within `rows`, along each row from the
/// left.
fn first_free(cells: &[(u8, u8)], shape: Shape, rows: u8) -> Option<(u8, u8)> {
    let mut places = (0..rows).flat_map(|y| (0..BUYING_COLUMNS).map(move |x| (x, y)));
    places.find(|&(x, y)| fits(cells, shape, x, y, rows))
}

/// The first free place, growing the box as far as need be: turned a quarter
/// if it is too wide as it is, refused only if it is too wide either way.
fn below(cells: &[(u8, u8)], shape: Shape, rotation: Rotation) -> Result<BoxPlace, Refusal> {
    let turns = [rotation, rotation.next()];
    let wide_enough = |&rotation: &Rotation| shape.turned(rotation).width <= BUYING_COLUMNS;
    let rotation = turns.into_iter().find(wide_enough).ok_or(Refusal::NoRoom)?;
    let shape = shape.turned(rotation);
    // Always room below everything.
    let rows = rows(cells).saturating_add(shape.height);
    let (x, y) = first_free(cells, shape, rows).ok_or(Refusal::NoRoom)?;
    Ok(BoxPlace { x, y, rotation })
}
