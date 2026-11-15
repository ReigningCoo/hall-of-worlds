//! The key `/_stats` asks for: `HOW_STATS_SECRET` (or `HOW_STATS_SECRET_FILE`,
//! such as a Docker secret); otherwise the one kept in the data folder's
//! `stats-key.txt`, made on the first start. Only its hash is kept in memory,
//! and neither the key nor its hash is ever logged or shown.

use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use bevy::log::info;
use ring::digest::{SHA256, digest};

use crate::environment::secret;
use crate::secret_code::new_code;

/// Where the key is kept, in the data folder, when the environment gives none.
const KEY_FILE: &str = "stats-key.txt";

/// The variable the key can be given in (or its `_FILE`).
const KEY_VARIABLE: &str = "HOW_STATS_SECRET";

/// The key's SHA-256 hash. Deliberately not `Debug`.
#[derive(Clone)]
pub struct StatsKey([u8; 32]);

impl StatsKey {
    /// The key from the environment, or kept in (or made for) `data`.
    pub fn from_environment(data: &Path) -> Result<Self, String> {
        Self::choose(secret(KEY_VARIABLE)?, data)
    }

    /// `given` (from the environment) if there is one; otherwise the key kept
    /// in `data`, made and kept now if there is none.
    pub(crate) fn choose(given: Option<String>, data: &Path) -> Result<Self, String> {
        if let Some(given) = given {
            info!("The stats key is {KEY_VARIABLE}'s");
            return Ok(Self::new(&given));
        }
        let file = data.join(KEY_FILE);
        match fs::read_to_string(&file) {
            Ok(text) if !text.trim().is_empty() => {
                info!("The stats key is kept in {}", file.display());
                return Ok(Self::new(text.trim()));
            }
            Ok(_) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(format!("Can't read {}: {error}", file.display())),
        }
        let key = new_code()?;
        fs::create_dir_all(data)
            .and_then(|()| fs::write(&file, format!("{key}\n")))
            .map_err(|error| format!("Can't keep the stats key in {}: {error}", file.display()))?;
        info!("Made a stats key, kept in {}", file.display());
        Ok(Self::new(&key))
    }

    fn new(key: &str) -> Self {
        Self(hash(key))
    }

    /// Whether `given` is the key: both sides compared as hashes, so the time
    /// taken says nothing about how much of it was right.
    pub(crate) fn matches(&self, given: &str) -> bool {
        hash(given) == self.0
    }
}

fn hash(text: &str) -> [u8; 32] {
    let mut hashed = [0; 32];
    hashed.copy_from_slice(digest(&SHA256, text.as_bytes()).as_ref());
    hashed
}
