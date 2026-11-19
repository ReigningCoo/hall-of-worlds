//! The character's personal map marks on this planet, as the server says
//! (`map_marks` keeps them; `minimap` draws them).

use bevy::prelude::*;
use shared::lang::{Args, Lang};
use shared::map_mark::MapMark;
use shared::spot::CELL_SPAN;
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

    /// `near` for the wilderness's maps: `at` and `reach` in its cells.
    pub fn near_cell(&self, at: (f64, f64), reach: f64, width: i64) -> Option<&MapMark> {
        self.near(cells_in_points(at), reach / SCALE as f64, width)
    }
}

/// A place in wilderness cells (cell `c` from `c - 0.5` to `c + 0.5`) in world
/// map points.
fn cells_in_points(at: (f64, f64)) -> (f64, f64) {
    let scale = SCALE as f64;
    let along = |cell: f64| (cell + 0.5) / scale - 0.5;
    (along(at.0), along(at.1))
}

/// Where a mark is drawn on the wilderness's maps, in its cells: exactly on
/// its pile, if it has one, or in the middle of its cell.
pub fn in_cells(mark: &MapMark) -> (f64, f64) {
    let span = CELL_SPAN as f64;
    match mark.pile {
        Some(pile) => (pile.x as f64 / span - 0.5, pile.y as f64 / span - 0.5),
        None => (mark.at.0 as f64, mark.at.1 as f64),
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

/// A mark's name, for the full map's hover box and its popup: "Your mark:
/// Camp", or an automatic one's by its pile: "Log pile", "Pile".
pub fn mark_name(lang: &Lang, mark: &MapMark) -> String {
    let args = Args::new().text("icon", mark.icon.key());
    let key = if mark.pile.is_some() {
        "map-mark-pile"
    } else {
        "map-mark-name"
    };
    super::written(lang.render(key, &args))
}
