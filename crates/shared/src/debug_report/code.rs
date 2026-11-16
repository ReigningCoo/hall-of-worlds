//! A report as a string and back: `PREFIX`, then the JSON deflated with
//! zlib's wrapping (its Adler-32 checksum finds a damaged string) in URL-safe
//! base64 without padding. Reading one trusts nothing in it: sizes are
//! limited before and after inflating.

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

use super::DebugReport;

/// What every string of this version starts with. A later version changes
/// the number, so older games can say they can't read it.
pub const PREFIX: &str = "HOW1.";

/// The longest string read, in letters (spaces and line breaks aside).
pub const MOST_LETTERS: usize = 20_000;

/// The most bytes a report's JSON may inflate to.
const MOST_JSON: usize = 64 * 1024;

/// How squeezed: the most there is.
const LEVEL: u8 = 10;

/// Why a string isn't a report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// It doesn't start like one.
    NotAReport,
    /// A report from another version of the game, which this one can't read.
    UnknownVersion(String),
    /// Longer than any report.
    TooLong,
    /// Cut short, mistyped or changed.
    Damaged,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAReport => write!(f, "That isn't a debug string."),
            Self::UnknownVersion(version) => {
                write!(f, "That debug string is from another version ({version}).")
            }
            Self::TooLong => write!(f, "That is too long to be a debug string."),
            Self::Damaged => write!(f, "That debug string is damaged."),
        }
    }
}

/// The report as a string to paste.
pub fn encode(report: &DebugReport) -> String {
    let json = serde_json::to_vec(report).expect("a report's fields always serialise");
    let squeezed = miniz_oxide::deflate::compress_to_vec_zlib(&json, LEVEL);
    format!("{PREFIX}{}", URL_SAFE_NO_PAD.encode(squeezed))
}

/// The report a string holds. Spaces and line breaks in it are ignored, as
/// pasting may add them.
pub fn decode(text: &str) -> Result<DebugReport, DecodeError> {
    let text: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    if text.len() > MOST_LETTERS {
        return Err(DecodeError::TooLong);
    }
    let Some(rest) = text.strip_prefix("HOW") else {
        return Err(DecodeError::NotAReport);
    };
    let Some((version, body)) = rest.split_once('.') else {
        return Err(DecodeError::NotAReport);
    };
    if version.is_empty() || !version.chars().all(|c| c.is_ascii_digit()) {
        return Err(DecodeError::NotAReport);
    }
    if format!("HOW{version}.") != PREFIX {
        return Err(DecodeError::UnknownVersion(version.to_owned()));
    }
    let squeezed = URL_SAFE_NO_PAD
        .decode(body)
        .map_err(|_| DecodeError::Damaged)?;
    let json = miniz_oxide::inflate::decompress_to_vec_zlib_with_limit(&squeezed, MOST_JSON)
        .map_err(|_| DecodeError::Damaged)?;
    serde_json::from_slice(&json).map_err(|_| DecodeError::Damaged)
}
