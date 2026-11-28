use std::fs;
use std::path::PathBuf;

use shared::behaviour::Temper;
use shared::creature::CreatureKind;

use super::*;

/// A folder of its own for this test, emptied.
fn folder(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join("how-behaviour-mods").join(name);
    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).unwrap();
    folder
}

fn written(defs: &[shared::behaviour::BehaviourDef]) -> String {
    ron::ser::to_string_pretty(&defs.to_vec(), Default::default()).unwrap()
}

#[test]
fn without_mods_the_games_behaviours_are_all_there_are() {
    let empty = folder("none");
    assert_eq!(with_mods(&empty.join("behaviours")), Behaviours::built_in());
    assert_eq!(with_mods(&empty), Behaviours::built_in());
}

#[test]
fn mods_replace_races_and_wrong_files_are_left_out() {
    let folder = folder("some");
    let game = Behaviours::built_in();
    let mut tame = game.of(CreatureKind::Wolf).clone();
    tame.temper = Temper::Friendly;
    fs::write(folder.join("a.ron"), written(&[tame])).unwrap();
    let mut wrong = game.of(CreatureKind::Orc).clone();
    wrong.temper = Temper::Friendly;
    wrong.drives.courage = 200;
    fs::write(folder.join("b.ron"), written(&[wrong])).unwrap();
    fs::write(folder.join("c.ron"), "not a list").unwrap();
    fs::write(folder.join("notes.txt"), "not a mod").unwrap();
    let behaviours = with_mods(&folder);
    assert_eq!(behaviours.len(), game.len());
    assert_eq!(behaviours.of(CreatureKind::Wolf).temper, Temper::Friendly);
    assert_eq!(behaviours.of(CreatureKind::Orc).temper, Temper::Hostile);
}
