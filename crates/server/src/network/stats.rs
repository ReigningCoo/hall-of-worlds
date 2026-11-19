//! `/_stats`: how the standalone server is doing, as JSON for dashboards such
//! as Homepage's customapi widget: players online and in all, the process's
//! memory and CPU, and how long it has run. Only with the key (`StatsKey`), in
//! `X-API-Key`, `Authorization: Bearer` or `?key=`; without it, a plain 404,
//! as for a path that isn't there.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use axum::Router;
use axum::extract::RawQuery;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use bevy::prelude::*;
use serde_json::json;

use crate::clock::Repeating;
use crate::connections::Connections;

mod key;
mod sample;

pub use key::StatsKey;
#[cfg(test)]
pub(super) use sample::machine_share;

/// Where the figures are served.
pub(crate) const STATS_PATH: &str = "/_stats";

/// The figures, written by the game and the sampler and read by the network
/// thread. Cloning it gives another handle on the same.
#[derive(Resource, Clone)]
pub(super) struct Stats {
    key: StatsKey,
    started: Instant,
    figures: Arc<Mutex<Figures>>,
}

#[derive(Clone, Copy, Default)]
struct Figures {
    online: usize,
    total: usize,
    memory_bytes: u64,
    cpu_percent: f64,
    cores: usize,
}

/// Keeps the process sampled while the server runs.
#[derive(Resource)]
pub(super) struct Sampling {
    _repeating: Option<Repeating>,
}

impl Stats {
    pub(super) fn new(key: StatsKey) -> Self {
        Self {
            key,
            started: Instant::now(),
            figures: Arc::default(),
        }
    }

    /// Starts sampling the process, and counting players in `app`.
    pub(super) fn track(&self, app: &mut App) {
        app.insert_resource(self.clone())
            .insert_resource(Sampling {
                _repeating: sample::start(self.clone()),
            })
            .add_systems(Last, count_players);
    }

    fn figures(&self) -> std::sync::MutexGuard<'_, Figures> {
        self.figures.lock().expect("never poisoned")
    }

    pub(super) fn counted(&self, online: usize, total: usize) {
        let mut figures = self.figures();
        figures.online = online;
        figures.total = total;
    }

    pub(super) fn sampled(&self, memory_bytes: u64, cpu_percent: f64, cores: usize) {
        let mut figures = self.figures();
        figures.memory_bytes = memory_bytes;
        figures.cpu_percent = cpu_percent;
        figures.cores = cores;
    }

    /// The answer, as JSON.
    pub(super) fn json(&self) -> String {
        let figures = *self.figures();
        json!({
            "players": { "online": figures.online, "total": figures.total },
            "memory": { "bytes": figures.memory_bytes },
            "cpu": { "percent": figures.cpu_percent, "cores": figures.cores },
            "uptime_seconds": self.started.elapsed().as_secs(),
        })
        .to_string()
    }

    /// Whether the request carries the key, in any of the ways it may.
    fn allows(&self, headers: &HeaderMap, query: Option<&str>) -> bool {
        let header = |name| headers.get(name).and_then(|value| value.to_str().ok());
        let bearer =
            header(header::AUTHORIZATION.as_str()).and_then(|value| value.strip_prefix("Bearer "));
        let queried = query.into_iter().flat_map(|query| {
            url::form_urlencoded::parse(query.as_bytes())
                .filter(|(name, _)| name == "key")
                .map(|(_, value)| value.into_owned())
        });
        [header("x-api-key"), bearer]
            .into_iter()
            .flatten()
            .map(str::to_owned)
            .chain(queried)
            .any(|given| self.key.matches(given.trim()))
    }
}

/// Counts the players online and characters in all, every tick.
fn count_players(stats: Res<Stats>, connections: Res<Connections>) {
    let (online, total) = connections.player_counts();
    stats.counted(online, total);
}

/// The stats route.
pub(super) fn stats_routes(stats: Stats) -> Router {
    Router::new().route(
        STATS_PATH,
        get(
            move |headers: HeaderMap, RawQuery(query): RawQuery| async move {
                answer(&stats, &headers, query.as_deref())
            },
        ),
    )
}

fn answer(stats: &Stats, headers: &HeaderMap, query: Option<&str>) -> Response {
    if !stats.allows(headers, query) {
        return StatusCode::NOT_FOUND.into_response();
    }
    (
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::CACHE_CONTROL, "no-store"),
        ],
        stats.json(),
    )
        .into_response()
}
