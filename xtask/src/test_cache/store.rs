//! The keys of test binaries that passed, a line each in a file in cargo's
//! target folder. Keys come from contents, never from paths, so every
//! worktree sharing that folder can share the file safely.

use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// The file's name in cargo's target folder.
const FILE: &str = "xtask-passed-tests.txt";

/// The keys known to pass, and where to add more (`None`: the cache is off).
pub(crate) struct Cache {
    keys: HashSet<String>,
    file: Option<PathBuf>,
}

impl Cache {
    /// Not used: nothing is skipped or recorded.
    pub(crate) fn off() -> Self {
        Self {
            keys: HashSet::new(),
            file: None,
        }
    }

    /// The keys in `target`'s file (none if there is no file yet).
    pub(crate) fn load(target: &Path) -> Self {
        let file = target.join(FILE);
        let text = fs::read_to_string(&file).unwrap_or_default();
        Self {
            keys: text.lines().map(str::to_owned).collect(),
            file: Some(file),
        }
    }

    pub(crate) fn is_on(&self) -> bool {
        self.file.is_some()
    }

    pub(crate) fn has(&self, key: &str) -> bool {
        self.keys.contains(key)
    }

    /// Notes that the binary with this key passed. Failing to write only
    /// means it runs again next time, so that is just a warning.
    pub(crate) fn record(&mut self, key: &str) {
        let Some(file) = &self.file else { return };
        let written = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file)
            .and_then(|mut out| writeln!(out, "{key}"));
        if let Err(error) = written {
            eprintln!(
                "warning: couldn't note a passed test in {}: {error}",
                file.display()
            );
        }
        self.keys.insert(key.to_owned());
    }
}
