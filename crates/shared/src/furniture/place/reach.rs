//! The check that furniture never cuts anything off: while placing, every
//! piece counts as wall, and a piece is kept only if a 4-way fill (no jumps,
//! every door open) from each way in still reaches everything it reached
//! without that piece.

use crate::dungeon::Level;

/// Fills from each way into a level, over walkable cells not yet furnished.
pub(super) struct Reach {
    min: (i64, i64),
    width: i64,
    height: i64,
    /// Walkable with every door open, and not furnished.
    open: Vec<bool>,
    starts: Vec<(i64, i64)>,
    /// For each start, the cells its fill reaches.
    reached: Vec<Vec<bool>>,
}

impl Reach {
    /// From `starts` (where people arrive: the entrance and stairs).
    pub(super) fn new(level: &Level, starts: Vec<(i64, i64)>) -> Self {
        let doors_open = level.with_all_doors_open();
        let (min_x, min_y, width, height) = level.bounds();
        let open = (min_y..min_y + height)
            .flat_map(|y| (min_x..min_x + width).map(move |x| (x, y)))
            .map(|(x, y)| doors_open.is_walkable(x, y) && level.furniture_at(x, y).is_none())
            .collect();
        let mut reach = Self {
            min: (min_x, min_y),
            width,
            height,
            open,
            starts,
            reached: Vec::new(),
        };
        reach.reached = (0..reach.starts.len()).map(|i| reach.fill(i)).collect();
        reach
    }

    fn index(&self, (x, y): (i64, i64)) -> Option<usize> {
        let (col, row) = (x - self.min.0, y - self.min.1);
        ((0..self.width).contains(&col) && (0..self.height).contains(&row))
            .then(|| (row * self.width + col) as usize)
    }

    /// The cells start `which` reaches over open cells.
    fn fill(&self, which: usize) -> Vec<bool> {
        let mut reached = vec![false; self.open.len()];
        let mut queue = vec![self.starts[which]];
        while let Some((x, y)) = queue.pop() {
            let Some(i) = self.index((x, y)) else {
                continue;
            };
            if reached[i] || !self.open[i] {
                continue;
            }
            reached[i] = true;
            queue.extend([(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]);
        }
        reached
    }

    /// Whether `cells` could all be furnished without cutting anything off:
    /// if so, they now count as wall.
    pub(super) fn take(&mut self, cells: &[(i64, i64)]) -> bool {
        let indices: Option<Vec<usize>> = cells.iter().map(|&cell| self.index(cell)).collect();
        let Some(indices) = indices else {
            return false;
        };
        if indices.iter().any(|&i| !self.open[i]) {
            return false;
        }
        for &i in &indices {
            self.open[i] = false;
        }
        let mut refilled = Vec::with_capacity(self.starts.len());
        for which in 0..self.starts.len() {
            let before = &self.reached[which];
            if !indices.iter().any(|&i| before[i]) {
                refilled.push(None);
                continue;
            }
            let after = self.fill(which);
            let kept = (before.iter().zip(&after))
                .enumerate()
                .all(|(i, (&was, &is))| !was || is || indices.contains(&i));
            if !kept {
                for &i in &indices {
                    self.open[i] = true;
                }
                return false;
            }
            refilled.push(Some(after));
        }
        for (which, after) in refilled.into_iter().enumerate() {
            if let Some(after) = after {
                self.reached[which] = after;
            }
        }
        true
    }
}
