//! `cargo xtask test`'s test step, skipping test binaries that already passed.
//!
//! Rust builds all of a crate's tests into one program, its test binary. If
//! that file is byte for byte the same as one that passed before, and the
//! files its tests read while running are too, running it again gives the
//! same result, so it is skipped. Cargo rebuilds a binary whenever its code
//! (or anything it depends on) changes, so an unchanged file means unchanged
//! code.
//!
//! The cache is for local development only: with `--all`, or whenever `CI` is
//! set (GitHub Actions sets it), every binary runs.

mod binaries;
mod key;
mod store;

#[cfg(test)]
mod tests;

use crate::target_dir::target_dir;
use binaries::TestBinary;
use std::env;
use std::path::Path;
use std::process::{Command, ExitCode};
use std::time::Instant;
use store::Cache;

/// Builds every test binary, then runs those not already known to pass.
pub(crate) fn run(root: &Path, all: bool) -> ExitCode {
    println!("==> Tests: cargo {}", binaries::BUILD.join(" "));
    let started = Instant::now();
    let Some(binaries) = binaries::build(root) else {
        eprintln!("==> FAILED: Tests (building)");
        return ExitCode::FAILURE;
    };
    println!(
        "==> Tests built in {:.1} s",
        started.elapsed().as_secs_f64()
    );

    let mut cache = match off_reason(all, env::var_os("CI").is_some()) {
        Some(reason) => {
            println!("==> Running every test binary: {reason}");
            Cache::off()
        }
        None => Cache::load(&target_dir(root)),
    };

    let started = Instant::now();
    let mut failed = Vec::new();
    for binary in &binaries {
        let name = binary.name();
        // Worked out before running, so the key is of what was tested.
        let key = cache.is_on().then(|| key::of(root, binary)).flatten();
        if key.as_ref().is_some_and(|key| cache.has(key)) {
            println!("==> skipped {name} (unchanged since it passed)");
            continue;
        }
        println!("==> Running {name}");
        if run_binary(binary) {
            if let Some(key) = key {
                cache.record(&key);
            }
        } else {
            failed.push(name);
        }
    }
    println!("==> Tests ran in {:.1} s", started.elapsed().as_secs_f64());

    if failed.is_empty() {
        println!("==> All tests passed");
        ExitCode::SUCCESS
    } else {
        eprintln!("==> FAILED: Tests ({})", failed.join(", "));
        ExitCode::FAILURE
    }
}

/// Why the cache is not used this time, if it isn't: `--all` was asked for,
/// or this is CI, which must always run everything (the owner's decision).
fn off_reason(all: bool, ci: bool) -> Option<&'static str> {
    if ci {
        Some("CI is set, and CI always runs every test")
    } else if all {
        Some("--all ignores the cache of tests that passed")
    } else {
        None
    }
}

/// Runs one test binary as `cargo test` would: from its package's folder,
/// with no arguments, and with `CARGO_MANIFEST_DIR` set as cargo sets it.
fn run_binary(binary: &TestBinary) -> bool {
    Command::new(&binary.path)
        .current_dir(&binary.package_dir)
        .env("CARGO_MANIFEST_DIR", &binary.package_dir)
        .status()
        .is_ok_and(|status| status.success())
}
