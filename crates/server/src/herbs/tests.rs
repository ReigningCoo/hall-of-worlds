use std::fs;

use super::*;

/// A folder of its own for this test, emptied.
fn folder(name: &str) -> PathBuf {
    let folder = env::temp_dir().join("how-herb-mods").join(name);
    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).unwrap();
    folder
}

fn written(herbs: &[shared::herb::HerbDef]) -> String {
    ron::ser::to_string_pretty(&herbs.to_vec(), Default::default()).unwrap()
}

#[test]
fn without_mods_the_games_herbs_are_all_there_are() {
    let empty = folder("none");
    assert_eq!(with_mods(&empty.join("herbs")), Herbs::built_in());
    assert_eq!(with_mods(&empty), Herbs::built_in());
}

#[test]
fn mods_add_and_replace_herbs_and_wrong_files_are_left_out() {
    let folder = folder("some");
    let game = Herbs::built_in();
    let mut renamed = game.iter().next().unwrap().clone();
    renamed.name = "Rose Root".into();
    let mut new = renamed.clone();
    new.key = "not_a_real_herb".into();
    fs::write(folder.join("a.ron"), written(&[renamed])).unwrap();
    fs::write(folder.join("b.ron"), written(&[new])).unwrap();
    let mut wrong = game.iter().nth(1).unwrap().clone();
    wrong.name = "Never Read".into();
    wrong.grams = 0;
    fs::write(folder.join("c.ron"), written(&[wrong.clone()])).unwrap();
    fs::write(folder.join("d.ron"), "not a list").unwrap();
    fs::write(folder.join("notes.txt"), "not a mod").unwrap();
    let herbs = with_mods(&folder);
    assert_eq!(herbs.len(), game.len() + 1);
    assert_eq!(herbs.iter().next().unwrap().name, "Rose Root");
    assert!(herbs.get("not_a_real_herb").is_some());
    assert_ne!(herbs.get(&wrong.key).unwrap().name, "Never Read");
}
