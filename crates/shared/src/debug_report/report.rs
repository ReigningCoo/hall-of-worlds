//! What a debug report holds. Every part the client may not know (no
//! character yet, no camera) is optional; positions are as the game keeps them,
//! so going back there is exact.

use serde::{Deserialize, Serialize};

use crate::creature::CreatureKind;
use crate::difficulty::Difficulty;
use crate::facing::Heading;
use crate::gender::Gender;
use crate::poi::{ChunkPos, PoiKind};
use crate::protocol::CharacterId;
use crate::rng::Seed;
use crate::site::DevArea;
use crate::spot::Spot;
use crate::world::ClockRate;

/// Everything a bug report needs to find the place again.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DebugReport {
    /// The program that wrote it.
    pub build: Build,
    /// When it was written, in seconds since 1970 (UTC), by the player's clock.
    pub utc: i64,
    /// Who the player is playing, if anyone yet.
    pub who: Option<Who>,
    /// Where they are, once in a world.
    pub place: Option<Where>,
    /// The world's seed when it is one of the Hall's planets (public, as their
    /// gates' places give it), so staff can go there: `place` has only its
    /// terrain seed. Missing from older strings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planet: Option<u32>,
    /// The world's time, once in a world.
    pub clock: Option<ReportClock>,
    /// How the view is turned.
    pub camera: Option<Camera>,
    /// The debug switches that were on.
    pub flags: DebugFlags,
    /// The site made by a debug tool the player is in, if any: the poi of
    /// `place` stands for nothing on the world map then.
    pub dev: Option<DevPlace>,
    /// The player's own words about what went wrong.
    pub problem: String,
}

/// The program a report came from.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Build {
    /// The game's version, such as `0.1.0`.
    pub version: String,
    /// Its `protocol::PROTOCOL_VERSION`.
    pub protocol: u32,
    pub platform: Platform,
    /// The operating system natively (`windows`, `linux`, `macos`), or the
    /// browser's own description of itself on the web.
    pub system: String,
}

/// The kind of build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Native,
    Web,
}

/// The character played (never their account).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Who {
    /// `None` while the client hasn't been told it.
    pub id: Option<CharacterId>,
    pub kind: CreatureKind,
    pub gender: Gender,
    pub difficulty: Option<Difficulty>,
}

/// Where in a world the player is. `seed` is the world's public terrain seed
/// (`WorldParams::seed`), which finds its planet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Where {
    /// On the world map, at this grid point.
    WorldMap { seed: Seed, x: i64, y: i64 },
    /// In the wilderness, at this spot of its cells; `poi`: the kind whose
    /// cutout they stand in, if any.
    Wilderness {
        seed: Seed,
        at: Spot,
        heading: Heading,
        poi: Option<PoiKind>,
    },
    /// In a site: the world map point it was entered from (and its chunk), the
    /// poi's kind, whether it is ruined, which level of how many, and the spot.
    Site {
        seed: Seed,
        x: i64,
        y: i64,
        chunk: ChunkPos,
        kind: PoiKind,
        ruined: bool,
        level: u32,
        levels: u32,
        at: Spot,
        heading: Heading,
    },
    /// In the Hall of Worlds: `origin` is where its `x` counts from.
    Hall {
        origin: i64,
        at: Spot,
        heading: Heading,
    },
    /// In The Way Inn (`shared::inn`), on this floor: the same from any world.
    Inn {
        level: u32,
        at: Spot,
        heading: Heading,
    },
}

/// The world's time when the report was written.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportClock {
    /// Game minutes since the world began.
    pub minutes: f64,
    pub rate: ClockRate,
}

/// The camera: `turn` and `tilt` in radians, `zoom` as its distance.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Camera {
    pub turn: f32,
    pub tilt: f32,
    pub zoom: f32,
}

/// Debug switches that change what happens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugFlags {
    pub god_mode: bool,
    pub truesight: bool,
    pub freeze_mobs: bool,
}

/// A site made by a debug tool, as it was asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DevPlace {
    /// Generate POI: a poi of this kind from this seed, this ruined.
    Site { kind: PoiKind, seed: u64, ruin: u8 },
    /// Dev Area.
    Area(DevArea),
}
