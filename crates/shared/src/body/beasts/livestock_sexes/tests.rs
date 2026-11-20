use crate::body::{Beast, Coat, Horns, Layout};
use crate::creature::CreatureKind;
use crate::gender::Gender;

fn body(kind: CreatureKind, gender: Gender) -> Layout {
    Layout::of_gender(kind, gender).unwrap()
}

/// How high its hips are.
fn hips(layout: &Layout) -> i32 {
    layout.spine.base[1]
}

#[test]
fn every_kept_animal_is_drawn_with_both_sexes() {
    use CreatureKind::*;
    for kind in [Cattle, Sheep, Goat, Pig, WaterBuffalo, Yak] {
        let beast = Beast::of(kind).unwrap_or_else(|| panic!("{kind:?}"));
        assert!(beast.sexes.is_some(), "{kind:?}");
        let (male, female) = (body(kind, Gender::Male), body(kind, Gender::Female));
        assert!(hips(&male) > hips(&female), "{kind:?}");
        assert_eq!(male.parts.udder, 0, "{kind:?}");
        assert!(female.parts.udder > 0, "{kind:?}");
    }
}

#[test]
fn rams_coil_and_ewes_have_no_horns() {
    let ram = body(CreatureKind::Sheep, Gender::Male);
    assert_eq!(ram.parts.horns, Horns::Spiral);
    assert_eq!(
        body(CreatureKind::Sheep, Gender::Female).parts.horns,
        Horns::None
    );
    assert_eq!(ram.parts.coat_shape, Coat::Fleece);
}

#[test]
fn bucks_have_longer_horns_and_beards() {
    let (buck, doe) = (
        body(CreatureKind::Goat, Gender::Male),
        body(CreatureKind::Goat, Gender::Female),
    );
    assert!(buck.parts.horn_length > doe.parts.horn_length * 2);
    assert!(buck.parts.beard > doe.parts.beard && doe.parts.beard > 0);
}

#[test]
fn only_boars_have_tusks() {
    assert!(body(CreatureKind::Pig, Gender::Male).parts.tusks > 0);
    assert_eq!(body(CreatureKind::Pig, Gender::Female).parts.tusks, 0);
    assert_eq!(body(CreatureKind::Pig, Gender::Neutral).parts.tusks, 0);
}

#[test]
fn yak_bulls_are_far_bigger_with_a_hump_over_the_shoulders_and_a_skirt() {
    let (bull, cow) = (
        body(CreatureKind::Yak, Gender::Male),
        body(CreatureKind::Yak, Gender::Female),
    );
    // About 125 cm and 111 cm at the withers.
    let ratio = hips(&bull) * 100 / hips(&cow);
    assert!((120..=130).contains(&ratio), "{ratio}");
    assert!(bull.parts.hump > cow.parts.hump * 2);
    assert_eq!(bull.parts.humps, 1);
    assert!(bull.parts.hump_along > 700);
    assert_eq!(bull.parts.coat_shape, Coat::Skirt);
    assert!(bull.parts.coat_length > cow.parts.coat_length);
    assert_eq!(bull.parts.horns, Horns::Bull);
    assert_eq!(cow.parts.horns, Horns::Upturned);
}

#[test]
fn buffalo_horns_sweep_back() {
    for gender in [Gender::Male, Gender::Female, Gender::Neutral] {
        let buffalo = body(CreatureKind::WaterBuffalo, gender);
        assert_eq!(buffalo.parts.horns, Horns::Sweep);
    }
}
