//! Furniture on a level (`shared::furniture`): kept as doors are, by the
//! cells it covers. It never blocks; it only makes moves over it dearer.

use super::Level;
use crate::furniture::Piece;

impl Level {
    /// The piece of furniture covering `(x, y)`, if any.
    pub fn furniture_at(&self, x: i64, y: i64) -> Option<&Piece> {
        let at = self.furniture_cells.get(&(x, y))?;
        self.furniture.get(at)
    }

    /// Every piece of furniture, by its north-west cell.
    pub fn furniture(&self) -> impl Iterator<Item = &Piece> + '_ {
        self.furniture.values()
    }

    /// Puts a piece on the level, replacing any it overlaps.
    pub fn place_furniture(&mut self, piece: Piece) {
        for cell in piece.cells() {
            if let Some(&other) = self.furniture_cells.get(&cell) {
                self.remove_furniture(other);
            }
        }
        for cell in piece.cells() {
            self.furniture_cells.insert(cell, piece.at);
        }
        self.furniture.insert(piece.at, piece);
    }

    /// Takes away the piece whose north-west cell is `at`, if there is one.
    pub fn remove_furniture(&mut self, at: (i64, i64)) {
        if let Some(piece) = self.furniture.remove(&at) {
            for cell in piece.cells() {
                self.furniture_cells.remove(&cell);
            }
        }
    }

    /// How much longer a move over `(x, y)` takes than over open floor, in
    /// thousandths: 1000 without furniture.
    pub fn furniture_cost_per_mille(&self, x: i64, y: i64) -> u32 {
        self.furniture_at(x, y)
            .map_or(1000, |piece| piece.furniture.cost_per_mille())
    }
}
