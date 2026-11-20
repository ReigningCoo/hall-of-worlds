use crate::body::{Layout, Parts, Tail};
use crate::creature::CreatureKind;
use crate::gender::Gender;

fn parts(kind: CreatureKind, gender: Gender) -> Parts {
    Layout::of_gender(kind, gender).expect("a body").parts
}

fn hips(kind: CreatureKind, gender: Gender) -> i32 {
    Layout::of_gender(kind, gender).expect("a body").spine.base[1]
}

#[test]
fn cocks_have_big_combs_spurs_and_sickles_and_hens_small_combs() {
    let cock = parts(CreatureKind::Chicken, Gender::Male);
    let hen = parts(CreatureKind::Chicken, Gender::Female);
    assert!(cock.comb > hen.comb * 2 && hen.comb > 0);
    assert!(cock.wattles > hen.wattles);
    assert!(cock.spurs > 0 && hen.spurs == 0);
    assert_eq!(cock.tail_shape, Tail::Sickle);
    assert_eq!(hen.tail_shape, Tail::Fan);
}

#[test]
fn toms_fan_their_tails_and_have_the_bigger_snood_and_a_beard() {
    let tom = parts(CreatureKind::Turkey, Gender::Male);
    let hen = parts(CreatureKind::Turkey, Gender::Female);
    assert_eq!(tom.tail_shape, Tail::Fan);
    assert!(tom.tail > hen.tail * 2 && hen.tail > 0);
    assert!(tom.snood > hen.snood * 2 && hen.snood > 0);
    assert!(tom.beard > 0 && hen.beard == 0);
    assert!(hips(CreatureKind::Turkey, Gender::Male) > hips(CreatureKind::Turkey, Gender::Female));
}

#[test]
fn muscovy_drakes_alone_have_caruncles_and_a_knob() {
    let drake = parts(CreatureKind::MuscovyDuck, Gender::Male);
    let duck = parts(CreatureKind::MuscovyDuck, Gender::Female);
    assert!(drake.caruncles > 0 && drake.knob > 0);
    assert!(duck.caruncles == 0 && duck.knob == 0);
}

#[test]
fn water_birds_have_bills_and_webbed_feet() {
    for kind in [
        CreatureKind::Duck,
        CreatureKind::MuscovyDuck,
        CreatureKind::Goose,
    ] {
        let parts = parts(kind, Gender::Neutral);
        assert_eq!(parts.snout, crate::body::Snout::Bill, "{kind:?}");
        assert_eq!(parts.feet, crate::body::Feet::Webbed, "{kind:?}");
    }
    let gander = parts(CreatureKind::Goose, Gender::Male);
    assert!(gander.knob > parts(CreatureKind::Goose, Gender::Female).knob);
}
