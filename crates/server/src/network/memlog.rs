//! `/_memlog`: the web page's memory log (`web/memlog.js`, with `?memlog` in
//! the page's address), a short JSON line a second from each page that has it
//! on, logged at `info` with the target `memlog`. Only with
//! `HOW_DEBUG_MEMLOG=true`; otherwise a plain 404, as for a path that isn't
//! there. For finding what grows on a browser that kills the page (iOS
//! Safari), whose own tools can't be opened.

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::extract::connect_info::Connected;
use axum::extract::{ConnectInfo, Request, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::serve::IncomingStream;
use bevy::prelude::*;

use super::TlsListener;
use crate::environment::option_on;

/// Where the page sends its lines.
pub(super) const MEMLOG_PATH: &str = "/_memlog";
/// The variable that turns it on.
const VARIABLE: &str = "HOW_DEBUG_MEMLOG";
/// The most a line may be: the page's are about 400 bytes.
const MOST_BYTES: usize = 2048;
/// The most lines an address may send a second: a page sends one, and one
/// more on leaving or an error, and a few pages may share an address.
const MOST_A_SECOND: u32 = 8;
/// Addresses kept for the limit before the old ones are forgotten.
const ADDRESSES_KEPT: usize = 1024;

/// The route, if `HOW_DEBUG_MEMLOG` turns it on; otherwise nothing.
pub(super) fn memlog_routes() -> Router {
    routes(option_on(VARIABLE, false))
}

pub(super) fn routes(on: bool) -> Router {
    if !on {
        return Router::new();
    }
    Router::new()
        .route(MEMLOG_PATH, post(take_line))
        .with_state(Arc::new(Limit::default()))
}

/// Logs a line, if it is JSON, small enough, and its address hasn't sent too many.
async fn take_line(State(limit): State<Arc<Limit>>, request: Request<Body>) -> StatusCode {
    let address = request
        .extensions()
        .get::<ConnectInfo<Peer>>()
        .map(|info| info.0.0.ip())
        .or_else(|| {
            let plain = request.extensions().get::<ConnectInfo<SocketAddr>>();
            plain.map(|info| info.0.ip())
        });
    if !limit.allows(address, Instant::now()) {
        return StatusCode::TOO_MANY_REQUESTS;
    }
    let Ok(body) = to_bytes(request.into_body(), MOST_BYTES).await else {
        return StatusCode::PAYLOAD_TOO_LARGE;
    };
    match line(&body) {
        Some(line) => {
            info!(target: "memlog", "{line}");
            StatusCode::NO_CONTENT
        }
        None => StatusCode::BAD_REQUEST,
    }
}

/// The line to log: a JSON object, written on one line, or `None`.
pub(super) fn line(body: &[u8]) -> Option<String> {
    match serde_json::from_slice::<serde_json::Value>(body) {
        Ok(value) if value.is_object() => Some(value.to_string()),
        _ => None,
    }
}

/// Who asked on the secure port, for the limit by address (the plain port
/// gives a `SocketAddr` itself, which `TlsListener` can't be given by this
/// crate: `routes::with_addresses`).
#[derive(Clone, Copy, Debug)]
pub(super) struct Peer(SocketAddr);

impl Connected<IncomingStream<'_, TlsListener>> for Peer {
    fn connect_info(stream: IncomingStream<'_, TlsListener>) -> Self {
        Self(*stream.remote_addr())
    }
}

/// How many lines each address has sent in its current second.
#[derive(Default)]
pub(super) struct Limit(Mutex<HashMap<Option<IpAddr>, (Instant, u32)>>);

impl Limit {
    /// Whether `address` may send another line `now`, counting it if so.
    pub(super) fn allows(&self, address: Option<IpAddr>, now: Instant) -> bool {
        let mut seen = self.0.lock().expect("never poisoned");
        if seen.len() >= ADDRESSES_KEPT {
            seen.retain(|_, (since, _)| now.duration_since(*since) < Duration::from_secs(1));
        }
        let (since, count) = seen.entry(address).or_insert((now, 0));
        if now.duration_since(*since) >= Duration::from_secs(1) {
            *since = now;
            *count = 0;
        }
        *count += 1;
        *count <= MOST_A_SECOND
    }
}

#[cfg(test)]
mod tests;
