//! The border round a poi's cutout: the wilderness cells within `BORDER` of
//! it, hatched on the client, where a click or tap leads back to the world
//! map (walking over them does nothing). It follows the cutout's own shape,
//! so a cave mouth's border runs round its hill. Nothing grows on it (owner):
//! no trees, no herbs (`ChunkPlan::keeps_clear`).

use super::Cutout;

/// How far round a cutout its border reaches, in cells (owner).
pub const BORDER: i64 = 2;

impl Cutout {
    /// The cells of its border, in wilderness cells: not its own, but within
    /// `BORDER` of one of them each way (diagonals too), row by row.
    pub fn border(&self) -> Vec<(i64, i64)> {
        let (min_x, min_y, width, height) = self.level().bounds();
        (min_y - BORDER..min_y + height + BORDER)
            .flat_map(|y| (min_x - BORDER..min_x + width + BORDER).map(move |x| (x, y)))
            .filter(|&(x, y)| self.on_border(x, y))
            .collect()
    }

    /// Whether a wilderness cell is on its border (`border`).
    pub fn on_border(&self, x: i64, y: i64) -> bool {
        self.distance_squared(x, y) <= 2 * BORDER * BORDER
            && !self.contains(x, y)
            && (y - BORDER..=y + BORDER)
                .any(|ny| (x - BORDER..=x + BORDER).any(|nx| self.contains(nx, ny)))
    }
}

#[cfg(test)]
mod tests;
