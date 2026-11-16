//! What the program is and where it runs, and the real date: the parts of a
//! report that come from the computer, not the game.

use shared::debug_report::{Build, Platform};
use shared::protocol::PROTOCOL_VERSION;

/// This program.
pub(super) fn build() -> Build {
    Build {
        version: env!("CARGO_PKG_VERSION").into(),
        protocol: PROTOCOL_VERSION,
        platform: if cfg!(target_arch = "wasm32") {
            Platform::Web
        } else {
            Platform::Native
        },
        system: system(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn system() -> String {
    std::env::consts::OS.into()
}

/// The browser's description of itself.
#[cfg(target_arch = "wasm32")]
fn system() -> String {
    web_sys::window()
        .and_then(|window| window.navigator().user_agent().ok())
        .unwrap_or_else(|| "browser".into())
}

/// Seconds since 1970, UTC, by this computer's clock.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn utc_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs() as i64)
}

/// Seconds since 1970, UTC, by the browser's clock.
#[cfg(target_arch = "wasm32")]
pub(super) fn utc_now() -> i64 {
    (js_sys::Date::now() / 1000.0) as i64
}
