//! `world.ron` and `server.ron`: a world's and the server's own clocks.

use serde::{Deserialize, Serialize};
use shared::world::{ClockRate, WorldSeed};

/// `world.ron`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldSave {
    /// The `SAVE_FORMAT` it was written with.
    pub format: u32,
    /// Secret: never sent to a client.
    pub seed: WorldSeed,
    /// The world map generator it was made with (`WORLD_MAP_VERSION`).
    pub world_map_version: u32,
    /// Its time when saved: game minutes since it began.
    pub clock: f64,
    /// The server's clock (`ServerSave::clock`) when `clock` was saved: the
    /// world's time has run on since, at `rate`. `None` from before worlds went
    /// by the server's clock, when it stopped while the server was off.
    #[serde(default)]
    pub server_clock: Option<f64>,
    /// How fast its time runs against the server's clock.
    #[serde(default)]
    pub rate: ClockRate,
    /// The action clock when saved, from before the server had one of its own
    /// (`ServerSave::action_ticks`): only read if that isn't saved yet.
    #[serde(default)]
    pub action_ticks: u64,
}

/// `server.ron`: the server's own clocks, which every world goes by, and
/// whether it is locked down.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServerSave {
    /// The `SAVE_FORMAT` it was written with.
    pub format: u32,
    /// Real seconds the server has run, not counting pauses (`clock::ServerClock`).
    pub clock: f64,
    /// Ticks of action time (`clock::ActionClock`).
    pub action_ticks: u64,
    /// Whether only accounts with Server Options may play (lockdown), kept
    /// across restarts.
    #[serde(default)]
    pub lockdown: bool,
}
