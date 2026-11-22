//! Furniture on a level (`shared::furniture`): kept as doors are, by the
//! cells it covers. It never blocks; it only makes moves over it dearer.
//! Pieces standing on other furniture (`Furniture::stands_on_furniture`: a
//! spirit rack on a table) are kept apart, on top, and go with what they
//! stand on.

use super::Level;
use crate::furniture::Piece;

impl Level {
    /// The piece of furniture standing on the floor at `(x, y)`, if any.
    pub fn furniture_at(&self, x: i64, y: i64) -> Option<&Piece> {
        let at = self.furniture_cells.get(&(x, y))?;
        self.furniture.get(at)
    }

    /// The piece standing on other furniture at `(x, y)`, if any.
    pub fn furniture_on_top_at(&self, x: i64, y: i64) -> Option<&Piece> {
        let at = self.on_top_cells.get(&(x, y))?;
        self.on_top.get(at)
    }

    /// Every piece of furniture on the floor, by its north-west cell.
    pub fn furniture(&self) -> impl Iterator<Item = &Piece> + '_ {
        self.furniture.values()
    }

    /// Every piece standing on other furniture, by its north-west cell.
    pub fn furniture_on_top(&self) -> impl Iterator<Item = &Piece> + '_ {
        self.on_top.values()
    }

    /// Whether `piece`, one that stands on furniture, has somewhere to stand:
    /// one piece that holds furniture (a table) under all its cells.
    pub fn can_stand_on_furniture(&self, piece: &Piece) -> bool {
        let mut under = piece.cells().map(|(x, y)| self.furniture_at(x, y));
        let Some(Some(first)) = under.next() else {
            return false;
        };
        first.furniture.holds_furniture() && under.all(|other| other == Some(first))
    }

    /// Puts a piece on the level, replacing any it overlaps on its own layer.
    /// One standing on furniture goes only where it can
    /// (`can_stand_on_furniture`); a piece on the floor takes away whatever
    /// stood on what it replaces.
    pub fn place_furniture(&mut self, piece: Piece) {
        if piece.furniture.stands_on_furniture() {
            if !self.can_stand_on_furniture(&piece) {
                return;
            }
            for cell in piece.cells() {
                if let Some(&other) = self.on_top_cells.get(&cell) {
                    self.remove_on_top(other);
                }
            }
            for cell in piece.cells() {
                self.on_top_cells.insert(cell, piece.at);
            }
            self.on_top.insert(piece.at, piece);
            return;
        }
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

    /// Takes away the piece on the floor whose north-west cell is `at`, if
    /// there is one, and anything standing on it.
    pub fn remove_furniture(&mut self, at: (i64, i64)) {
        let Some(piece) = self.furniture.remove(&at) else {
            return;
        };
        for cell in piece.cells() {
            self.furniture_cells.remove(&cell);
            if let Some(&top) = self.on_top_cells.get(&cell) {
                self.remove_on_top(top);
            }
        }
    }

    /// Takes away the piece standing on furniture whose north-west cell is
    /// `at`, if there is one.
    pub fn remove_on_top(&mut self, at: (i64, i64)) {
        if let Some(piece) = self.on_top.remove(&at) {
            for cell in piece.cells() {
                self.on_top_cells.remove(&cell);
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
