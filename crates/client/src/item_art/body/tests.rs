use super::*;
use shared::body::{BEASTS, Beard, FIRST_BIPEDS, HairStyle};
use shared::creature::CreatureKind;
use shared::gender::Gender;

#[test]
fn every_body_stays_inside_its_picture_with_room_for_its_edge() {
    let kinds = CreatureKind::ALL
        .into_iter()
        .filter_map(Layout::of)
        .map(|layout| (layout, 1750));
    let coming = FIRST_BIPEDS.map(|biped| {
        let layout = Layout::biped(&biped.proportions.frame(), biped.parts);
        (layout, biped.height)
    });
    let beasts = BEASTS.map(|beast| (beast.layout(), 1750));
    let sexed = [Gender::Male, Gender::Female]
        .map(|gender| (Layout::of_gender(CreatureKind::Coo, gender).unwrap(), 1750));
    for (layout, height) in kinds.chain(coming).chain(beasts).chain(sexed) {
        let (wide, long) = cells(&layout);
        let (wide, long) = (wide as f32, long as f32);
        let per_cell = VIEW_WIDTH / wide.max(long);
        let edge = OUTLINE_PIXELS / PIXELS_PER_CELL * per_cell;
        for styled in hairs() {
            let parts = fitted(&layout, height, styled, None);
            for corner in parts.iter().flatten().flatten() {
                assert!(corner.is_finite());
                assert!(corner.x.abs() + edge <= wide * per_cell / 2.0, "{layout:?}");
                assert!(corner.y.abs() + edge <= long * per_cell / 2.0, "{layout:?}");
            }
        }
    }
}

/// No hair, and every style and beard at least once, the largest together.
fn hairs() -> [Hair; 5] {
    let with = |style, beard| Hair { style, beard };
    [
        Hair::NONE,
        with(HairStyle::Long, Beard::Long),
        with(HairStyle::Braids, Beard::Braided),
        with(HairStyle::Curly, Beard::Forked),
        with(HairStyle::TiedBack, Beard::Full),
    ]
}

#[test]
fn horses_have_manes_and_tails_of_hair() {
    let drawn = |kind| coat_hair(&Layout::of(kind).expect("drawn"));
    let horse = drawn(CreatureKind::Horse);
    assert!(!horse.is_empty());
    // A centaur keeps the horse's tail but has no horse's neck for a mane, so less.
    let centaur = drawn(CreatureKind::Centaur);
    assert!(!centaur.is_empty() && centaur.len() < horse.len());
    assert!(!drawn(CreatureKind::Cattle).is_empty());
    assert!(drawn(CreatureKind::Human).is_empty());
    assert!(drawn(CreatureKind::Wolf).is_empty());
    // Hair hangs off the ground.
    assert!(horse.iter().flatten().all(|corner| corner.y > 0.0));
}

#[test]
fn coo_bulls_horns_point_forwards_and_are_thicker_than_cows() {
    let horns = |gender| horns_of(&Layout::of_gender(CreatureKind::Coo, gender).unwrap());
    let (bull, cow) = (horns(Gender::Male), horns(Gender::Female));
    assert_eq!((bull.len(), cow.len()), (2, 2));
    let reach = |path: &Vec<(Vec3, f32)>| path[path.len() - 1].0 - path[0].0;
    // A bull's tip is well forward of its root; a cow's turns up more than forwards.
    assert!(reach(&bull[0]).z > reach(&bull[0]).y);
    assert!(reach(&cow[0]).y > reach(&cow[0]).z);
    assert!(bull[0][0].1 > cow[0][0].1 * 1.3);
    // The two sides mirror each other.
    assert!((reach(&cow[0]).x + reach(&cow[1]).x).abs() < 1.0);
}

#[test]
fn a_coo_has_a_fringe_and_long_hair_on_its_tail() {
    let layout = Layout::of(CreatureKind::Coo).unwrap();
    let mut without = layout;
    without.parts.fringe = 0;
    assert!(build(&layout).len() > build(&without).len());
    assert!(!coat_hair(&layout).is_empty());
}
