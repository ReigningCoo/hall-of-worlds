//! Claiming a networked server that has no Owner: a code made at start,
//! written to `owner-claim.txt` and the log, which whoever runs the server
//! types as a new character's name.

use std::fs;
use std::path::Path;

pub(super) use crate::secret_code::new_code;

pub(super) fn file(data: &Path) -> std::path::PathBuf {
    data.join("owner-claim.txt")
}

pub(super) fn write_file(data: &Path, code: &str) -> Result<(), String> {
    let text = format!(
        "{code}\n\nMake a new character with this as its name to become this server's Owner.\n\
         It works once, and a new code is made each time the server starts until then.\n"
    );
    let path = file(data);
    fs::write(&path, text).map_err(|error| format!("can't write {}: {error}", path.display()))
}

pub(super) fn remove_file(data: &Path) {
    let _ = fs::remove_file(file(data));
}
