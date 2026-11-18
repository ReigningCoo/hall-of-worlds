//! Whether a wilderness cell is the sea rather than a river: for the water a
//! bottle is filled with there (`potion::Source::sea`).

use super::Wilderness;
use super::samples::Samples;

impl Wilderness {
    /// Whether cell `(x, y)` lies in the sea (open water below the lowest tide,
    /// not frozen), as `block` makes it. Rivers and land aren't.
    pub fn is_sea(&self, x: i64, y: i64) -> bool {
        let samples = Samples::new(self, (x, y, 1, 1));
        samples.height(x, y) < self.low_water && !samples.frozen(x, y)
    }
}
