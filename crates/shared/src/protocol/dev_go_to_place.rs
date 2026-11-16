//! Developer tools: going to the place a debug report was written at (Load in
//! Debug Info, `shared::debug_report`). The client reads the string; the
//! server checks the place and does the whole trip.

use serde::{Deserialize, Serialize};

use crate::debug_report::{DevPlace, ReportClock, Where};
use crate::facing::Heading;

/// Where to go (`ClientMessage::DevGoToPlace`): the report's place, standing on
/// its spot (or the nearest with room) facing its way. Its generated place, not
/// another server's saved state of it.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevGoToPlace {
    /// Where, with the world's terrain seed: the server finds the world among
    /// those it has open, or by `planet`.
    pub place: Where,
    /// The world's seed when it is one of the Hall's planets (they are public),
    /// which the server checks gives the terrain seed.
    pub planet: Option<u32>,
    /// The site a debug tool made (Generate POI, Dev Area), made again.
    pub dev: Option<DevPlace>,
    /// The world's time to set too: only with World Time's permission.
    pub clock: Option<ReportClock>,
}

/// How a `DevGoToPlace` went (`ServerMessage::DevPlace`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceAnswer {
    /// There, facing this way.
    Arrived {
        heading: Heading,
    },
    Refused(PlaceRefusal),
}

/// Why a `DevGoToPlace` was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceRefusal {
    /// No world this server has open, nor a Hall planet, has that terrain seed.
    UnknownWorld,
    /// The world has no poi in that chunk, or one of another kind.
    NoSuchPoi,
}
