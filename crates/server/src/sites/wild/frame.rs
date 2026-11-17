//! Where a wilderness window lies round the planet, for placing cells and
//! cutouts in it.

/// Where the player is in the planet, and the window's cells.
pub(super) struct Frame {
    pub(super) shift: i64,
    pub(super) width: i64,
    pub(super) bounds: (i64, i64, i64, i64),
}

impl Frame {
    /// Where a cell round the planet is in the window, if it is in it.
    pub(super) fn place(&self, x: i64, y: i64) -> Option<i64> {
        let (min_x, min_y, columns, rows) = self.bounds;
        if !(min_y..min_y + rows).contains(&y) {
            return None;
        }
        [-self.width, 0, self.width]
            .into_iter()
            .map(|turn| x + self.shift + turn)
            .find(|x| (min_x..min_x + columns).contains(x))
    }
}

/// How far along a cutout at `bounds` round the planet is in the window's
/// frame, if it lies in the window at all. Cutouts are kept inside their
/// chunk, which never lies across the seam.
pub(super) fn turn_of(
    frame: &Frame,
    (min_x, min_y, columns, rows): (i64, i64, i64, i64),
) -> Option<i64> {
    let (wx, wy, wc, wr) = frame.bounds;
    [-frame.width, 0, frame.width]
        .into_iter()
        .map(|turn| turn + frame.shift)
        .find(|&turn| {
            min_x + turn < wx + wc
                && min_x + columns + turn > wx
                && min_y < wy + wr
                && min_y + rows > wy
        })
}
