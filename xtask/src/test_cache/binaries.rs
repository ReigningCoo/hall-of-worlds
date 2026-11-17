//! Building the test binaries, and finding out from cargo where they are.

use crate::DEBUG_API;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// What builds every test binary, with the debug API so its tests run too.
/// `--no-run` builds without running; the JSON on stdout says what was
/// built, while errors are still written for people on stderr.
pub(crate) const BUILD: &[&str] = &[
    "test",
    "--workspace",
    "--features",
    DEBUG_API,
    "--no-run",
    "--message-format=json-render-diagnostics",
];

/// One crate's test binary.
#[derive(Debug, PartialEq)]
pub(crate) struct TestBinary {
    /// The program itself, in cargo's target folder.
    pub(crate) path: PathBuf,
    /// The crate's folder (where its `Cargo.toml` is): tests run from here.
    pub(crate) package_dir: PathBuf,
}

impl TestBinary {
    /// The program's file name, to name it in what is printed.
    pub(crate) fn name(&self) -> String {
        self.path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default()
    }
}

/// Builds the test binaries and lists them, or `None` if the build failed.
pub(crate) fn build(root: &Path) -> Option<Vec<TestBinary>> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let output = Command::new(cargo)
        .args(BUILD)
        .current_dir(root)
        .stderr(Stdio::inherit())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| parse(&String::from_utf8_lossy(&output.stdout)))
}

/// The test binaries in cargo's JSON messages, one message a line: each
/// `compiler-artifact` built for tests (`profile.test`) that is a program
/// (has an `executable`).
pub(crate) fn parse(messages: &str) -> Vec<TestBinary> {
    messages
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|message| message["reason"] == "compiler-artifact")
        .filter(|message| message["profile"]["test"] == true)
        .filter_map(|message| {
            let path = message["executable"].as_str()?;
            let manifest = Path::new(message["manifest_path"].as_str()?);
            Some(TestBinary {
                path: PathBuf::from(path),
                package_dir: manifest.parent()?.to_path_buf(),
            })
        })
        .collect()
}
