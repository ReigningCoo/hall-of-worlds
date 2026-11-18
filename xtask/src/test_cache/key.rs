//! A test binary's key: a hash of the program and of the files its tests read
//! while running. The same key means the same results.

use super::binaries::TestBinary;
use std::fs;
use std::path::Path;
use web_delta::{hex, sha256};

/// The files each crate's tests read while they run, from the workspace
/// root (files a test takes in with `include_str!` are built into the binary,
/// so they need nothing here). **A test that starts reading another file or
/// folder must have it added here**, or a change to it won't rerun the test.
/// A crate missing from this list is never skipped.
const RUN_TIME_DATA: &[(&str, &[&str])] = &[
    // The abilities, entities, dungeon_map theme and skills_page icons tests.
    ("crates/client", &["crates/client/assets"]),
    // environment/tests.rs scans the server's sources for variable names.
    ("crates/server", &["crates/server/src"]),
    ("crates/shared", &[]),
    ("crates/web_delta", &[]),
    (
        "xtask",
        &[
            "Dockerfile",
            "Dockerfile.github",
            "scenarios",
            // wiki/tests.rs compares the pages and pictures made from the game's data.
            "docs/herbs",
            "docs/assets/herbs",
            "docs/fruit",
            "docs/assets/fruit",
            "docs/weapons",
            "docs/assets/weapons",
            "crates/client/assets/fonts/HallOfWorldsTitle.ttf",
        ],
    ),
];

/// The binary's key, or `None` if its crate isn't in `RUN_TIME_DATA` or the
/// binary can't be read (then it always runs).
pub(crate) fn of(root: &Path, binary: &TestBinary) -> Option<String> {
    let package = binary.package_dir.strip_prefix(root).ok()?;
    let package = package.to_string_lossy().replace('\\', "/");
    let (_, data) = RUN_TIME_DATA.iter().find(|(name, _)| *name == package)?;
    let program = fs::read(&binary.path).ok()?;
    Some(key(&program, root, data))
}

/// Hashes the program, then every data file by its path and contents.
pub(crate) fn key(program: &[u8], root: &Path, data: &[&str]) -> String {
    let mut summary = format!("program {}\n", hex(&sha256(program)));
    for path in data {
        add_files(&mut summary, root, &root.join(path));
    }
    hex(&sha256(summary.as_bytes()))
}

/// Adds a line per file (in a folder, every file in it, in name order).
fn add_files(summary: &mut String, root: &Path, path: &Path) {
    let name = path.strip_prefix(root).unwrap_or(path).to_string_lossy();
    let name = name.replace('\\', "/");
    if path.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path)
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| entry.path())
            .collect();
        entries.sort();
        for entry in entries {
            add_files(summary, root, &entry);
        }
    } else {
        let hash = fs::read(path).map(|bytes| hex(&sha256(&bytes)));
        let hash = hash.unwrap_or_else(|_| "missing".into());
        summary.push_str(&format!("{name} {hash}\n"));
    }
}
