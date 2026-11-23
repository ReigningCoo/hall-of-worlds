//! The Way Inn as a site: the server keeps it, and the client draws it, as
//! any other site's levels, made by `site::Generator::Inn` from `level`.

use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::{Generator, INN_VERSION, Site};

use super::{GROUND, own_room_at, spots};
use crate::furniture::{Holder, Piece};

/// What the inn counts as wherever a site's kind of poi is asked for (its
/// stairs' styles, the look of its walls and floors): a castle's stone.
pub const KIND: PoiKind = PoiKind::Castle;

/// The inn as a site: both its floors, the stairs between them.
pub fn site() -> Site {
    Site::with_generator(KIND, Seed::new(0), INN_VERSION, Generator::Inn)
        .expect("the inn's version runs")
}

/// Whether `(x, y)` on floor `index` is the way out: the entrance door.
/// Stepping onto it leaves the inn, back to the Hall.
pub fn is_way_out(index: u32, (x, y): (i64, i64)) -> bool {
    index == GROUND && (x, y) == spots::ENTRANCE
}

/// Whether `piece` on floor `index` is only to look at, never opened:
/// chests and wardrobes anywhere but players' own rooms (the foyer's, the
/// treasury's of gold, the locked bedrooms').
pub fn look_only(index: u32, piece: &Piece) -> bool {
    let store = matches!(Holder::of(piece.furniture), Some(Holder::Store(_)));
    store && own_room_at(index, piece.at).is_none()
}
