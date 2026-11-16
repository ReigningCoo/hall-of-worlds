//! Debug reports: where a player was and what they saw, written by the client's
//! Debug Info panel as one string to paste into a bug report, and read back by
//! staff to go there. The string (`encode`, `decode`) is `HOW1.` then the
//! report as JSON, deflated in zlib's wrapping (whose Adler-32 checksum finds
//! damage) and written in URL-safe base64. Squeezing it only shortens and
//! obscures it: anything going where a report says is checked by the server.
//!
//! A report never holds the player's email address, the server's address or
//! any account's name (`CLAUDE.md`'s email rule).

mod code;
mod report;

pub use code::{DecodeError, MOST_LETTERS, PREFIX, decode, encode};
pub use report::{
    Build, Camera, DebugFlags, DebugReport, DevPlace, Platform, ReportClock, Where, Who,
};

#[cfg(test)]
mod tests;
