//! `Stored`: what lies in a chest or wardrobe, placed in its grid.

use serde::{Deserialize, Serialize};

use super::Storage;
use crate::crafting_bench::NotTaken;
use crate::id::Id;
use crate::item::{Item, Placed, Rotation, Shape};

/// What lies in a chest or wardrobe: each thing at a place in its grid, as
/// in a bag. The server's to keep, whole in itself.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stored {
    pub storage: Storage,
    pub items: Vec<Placed>,
}

impl Stored {
    /// Nothing in it.
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
            items: Vec::new(),
        }
    }

    /// Everything in it, in the order put in.
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.items.iter().map(|placed| &placed.item)
    }

    /// Everything in it, to change in place (food resting, secrets hidden).
    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut Item> {
        self.items.iter_mut().map(|placed| &mut placed.item)
    }

    /// Puts `item` in the first place it fits (rows from the top, then the
    /// next quarter turn), if it takes its kind; else the one `NotTaken`,
    /// never saying why (it won't take it, or there's no room).
    pub fn put(&mut self, item: Item) -> Result<(), NotTaken> {
        if !self.storage.takes(item.kind) {
            return Err(NotTaken);
        }
        let (x, y, rotation) = self.first_room(item.shape()).ok_or(NotTaken)?;
        self.items.push(Placed {
            item,
            x,
            y,
            rotation,
        });
        Ok(())
    }

    /// Takes `id` out, if it is in it.
    pub fn take(&mut self, id: Id) -> Option<Item> {
        let index = self.items.iter().position(|placed| placed.item.id == id)?;
        Some(self.items.remove(index).item)
    }

    /// The first place a `shape` fits, unturned first.
    fn first_room(&self, shape: Shape) -> Option<(u8, u8, Rotation)> {
        let (width, height) = self.storage.grid();
        for rotation in Rotation::ALL {
            let turned = shape.turned(rotation);
            for y in 0..height {
                for x in 0..width {
                    if self.has_room(turned, x, y) {
                        return Some((x, y, rotation));
                    }
                }
            }
        }
        None
    }

    /// Whether `shape` lies inside the grid at `(x, y)` on no one's cells.
    fn has_room(&self, shape: Shape, x: u8, y: u8) -> bool {
        let (width, height) = self.storage.grid();
        let taken = |cell: (u8, u8)| (self.items.iter()).any(|p| p.cells().any(|c| c == cell));
        (shape.cells()).all(|(dx, dy)| {
            let (cx, cy) = (x + dx, y + dy);
            cx < width && cy < height && !taken((cx, cy))
        })
    }
}
