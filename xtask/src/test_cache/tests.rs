use super::binaries::{TestBinary, parse};
use super::key;
use super::off_reason;
use super::store::Cache;
use std::fs;
use std::path::{Path, PathBuf};

/// An empty scratch folder for one test, named after it.
fn scratch(test: &str) -> PathBuf {
    let folder =
        std::env::temp_dir().join(format!("xtask-test-cache-{test}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).unwrap();
    folder
}

/// A workspace with a pretend xtask test binary and its data.
fn workspace(test: &str) -> (PathBuf, TestBinary) {
    let root = scratch(test);
    fs::create_dir_all(root.join("xtask")).unwrap();
    fs::create_dir_all(root.join("scenarios")).unwrap();
    fs::write(root.join("Dockerfile"), "FROM a").unwrap();
    fs::write(root.join("scenarios/one.ron"), "(steps: [])").unwrap();
    let path = root.join("xtask-1234.exe");
    fs::write(&path, b"program").unwrap();
    let binary = TestBinary {
        path,
        package_dir: root.join("xtask"),
    };
    (root, binary)
}

#[test]
fn the_key_changes_when_a_data_file_changes() {
    let (root, binary) = workspace("data");
    let before = key::of(&root, &binary).unwrap();
    assert_eq!(
        key::of(&root, &binary).unwrap(),
        before,
        "the same files, the same key"
    );
    fs::write(root.join("scenarios/one.ron"), "(steps: [Wait])").unwrap();
    assert_ne!(key::of(&root, &binary).unwrap(), before);
}

#[test]
fn the_key_changes_when_the_binary_changes() {
    let (root, binary) = workspace("binary");
    let before = key::of(&root, &binary).unwrap();
    fs::write(&binary.path, b"program, rebuilt").unwrap();
    assert_ne!(key::of(&root, &binary).unwrap(), before);
}

#[test]
fn a_file_added_to_a_data_folder_changes_the_key() {
    let (root, binary) = workspace("added");
    let before = key::of(&root, &binary).unwrap();
    fs::write(root.join("scenarios/two.ron"), "(steps: [])").unwrap();
    assert_ne!(key::of(&root, &binary).unwrap(), before);
}

#[test]
fn a_crate_not_listed_has_no_key_so_always_runs() {
    let (root, mut binary) = workspace("unlisted");
    binary.package_dir = root.join("crates/new_crate");
    assert_eq!(key::of(&root, &binary), None);
}

#[test]
fn an_unchanged_binary_is_skipped_once_it_has_passed() {
    let (root, binary) = workspace("skipped");
    let target = root.join("target");
    fs::create_dir_all(&target).unwrap();
    let key = key::of(&root, &binary).unwrap();

    assert!(!Cache::load(&target).has(&key), "nothing has passed yet");
    Cache::load(&target).record(&key);
    assert!(Cache::load(&target).has(&key), "read back from the file");

    fs::write(&binary.path, b"program, rebuilt").unwrap();
    let rebuilt = key::of(&root, &binary).unwrap();
    assert!(!Cache::load(&target).has(&rebuilt), "a rebuilt binary runs");
}

#[test]
fn all_and_ci_run_everything() {
    assert_eq!(off_reason(false, false), None);
    assert!(off_reason(true, false).is_some(), "--all");
    assert!(off_reason(false, true).is_some(), "CI");
    assert!(off_reason(true, true).is_some());
    assert!(!Cache::off().is_on());
}

#[test]
fn test_binaries_are_found_in_cargos_messages() {
    let messages = [
        r#"{"reason":"compiler-artifact","manifest_path":"C:\\game\\crates\\shared\\Cargo.toml","profile":{"test":true},"executable":"D:\\target\\debug\\deps\\shared-1.exe"}"#,
        // A library built for others to use, not for tests.
        r#"{"reason":"compiler-artifact","manifest_path":"C:\\game\\crates\\shared\\Cargo.toml","profile":{"test":false},"executable":null}"#,
        r#"{"reason":"build-finished","success":true}"#,
        "not JSON",
    ]
    .join("\n");
    assert_eq!(
        parse(&messages),
        vec![TestBinary {
            path: PathBuf::from(r"D:\target\debug\deps\shared-1.exe"),
            package_dir: Path::new(r"C:\game\crates\shared").to_path_buf(),
        }]
    );
}
