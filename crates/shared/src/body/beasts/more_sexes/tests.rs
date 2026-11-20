use crate::body::{Layout, Parts};
use crate::creature::CreatureKind;
use crate::gender::Gender;

fn parts(kind: CreatureKind, gender: Gender) -> Parts {
    Layout::of_gender(kind, gender).expect("a body").parts
}

fn withers(kind: CreatureKind, gender: Gender) -> i32 {
    let layout = Layout::of_gender(kind, gender).expect("a body");
    layout.spine.base[1]
}

#[test]
fn reindeer_of_both_sexes_have_antlers_the_bulls_far_bigger() {
    let bull = parts(CreatureKind::Reindeer, Gender::Male);
    let cow = parts(CreatureKind::Reindeer, Gender::Female);
    assert!(cow.antlers > 0 && cow.tines > 0);
    assert!(bull.antlers > cow.antlers * 2);
    assert!(bull.tines > cow.tines);
}

#[test]
fn camels_keep_their_humps_and_bulls_are_bigger() {
    for kind in [CreatureKind::Dromedary, CreatureKind::BactrianCamel] {
        for gender in [Gender::Male, Gender::Female, Gender::Neutral] {
            assert!(parts(kind, gender).hump > 0, "{kind:?} {gender:?}");
        }
        assert!(withers(kind, Gender::Male) > withers(kind, Gender::Female));
    }
    assert_eq!(parts(CreatureKind::Dromedary, Gender::Neutral).humps, 1);
    assert_eq!(parts(CreatureKind::BactrianCamel, Gender::Neutral).humps, 2);
}

#[test]
fn bactrian_bulls_have_the_heavier_mane_and_beard() {
    let bull = parts(CreatureKind::BactrianCamel, Gender::Male);
    let cow = parts(CreatureKind::BactrianCamel, Gender::Female);
    let neutral = parts(CreatureKind::BactrianCamel, Gender::Neutral);
    assert!(bull.mane > neutral.mane && neutral.mane > cow.mane && cow.mane > 0);
    assert!(bull.beard > cow.beard);
}

#[test]
fn rabbit_does_have_a_dewlap() {
    assert!(parts(CreatureKind::Rabbit, Gender::Female).dewlap > 0);
    assert_eq!(parts(CreatureKind::Rabbit, Gender::Male).dewlap, 0);
}

#[test]
fn mules_llamas_and_alpacas_have_one_model() {
    for kind in [
        CreatureKind::Mule,
        CreatureKind::Llama,
        CreatureKind::Alpaca,
    ] {
        let neutral = Layout::of_gender(kind, Gender::Neutral);
        assert_eq!(Layout::of_gender(kind, Gender::Male), neutral);
        assert_eq!(Layout::of_gender(kind, Gender::Female), neutral);
    }
}

#[test]
fn cattle_keep_their_dewlaps_in_both_sexes() {
    for gender in [Gender::Male, Gender::Female] {
        assert!(parts(CreatureKind::Cattle, gender).dewlap > 0);
    }
}
