//! The character's personal map marks on this planet, as the server says
//! (`map_marks` keeps them; `minimap` draws them).

use bevy::prelude::*;
use shared::lang::{Args, Lang};
use shared::map_mark::{MapMark, MarkIcon};
use shared::wilderness::SCALE;

/// Every mark the character has on the planet they are on (`shared::map_mark`).
#[derive(Resource, Default, Debug, PartialEq)]
pub struct MapMarks(pub Vec<MapMark>);

impl MapMarks {
    /// The mark nearest `at` (world map points) within `reach` points of it,
    /// on a planet `width` points round.
    pub fn near(&self, at: (f64, f64), reach: f64, width: i64) -> Option<&MapMark> {
        let distance = |mark: &MapMark| {
            let (x, y) = in_points(mark.at);
            let across = near_way_round(x - at.0, width);
            across.abs().max((y - at.1).abs())
        };
        (self.0.iter())
            .map(|mark| (distance(mark), mark))
            .filter(|(distance, _)| *distance <= reach)
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, mark)| mark)
    }
}

/// A mark's place in world map points: its cell's middle, where point `p` spans
/// `p - 0.5` to `p + 0.5` and its cells `p * SCALE` onwards.
pub fn in_points(at: (i64, i64)) -> (f64, f64) {
    let scale = SCALE as f64;
    let along = |cell: i64| (cell as f64 + 0.5) / scale - 0.5;
    (along(at.0), along(at.1))
}

/// The world map point a place in points is on.
pub fn point_at(at: (f64, f64)) -> (i64, i64) {
    ((at.0 + 0.5).floor() as i64, (at.1 + 0.5).floor() as i64)
}

/// `across` (points east) brought the near way round a planet `width` round.
pub fn near_way_round(across: f64, width: i64) -> f64 {
    if width <= 0 {
        return across;
    }
    let width = width as f64;
    across - (across / width).round() * width
}

/// A mark's name, for the full map's hover box and its popup: "Your mark: Camp".
pub fn mark_name(lang: &Lang, icon: MarkIcon) -> String {
    let args = Args::new().text("icon", icon.key());
    super::written(lang.render("map-mark-name", &args))
}
