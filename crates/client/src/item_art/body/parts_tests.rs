//! The livestock's parts (livestock plan PR 2): each drawn where it belongs, in the
//! right group, standing clear of the ground and inside its picture.

use super::*;
use shared::body::{Coat, Ears, Feet, Horns, Snout, Tail};
use shared::creature::CreatureKind;

fn cattle() -> Layout {
    Layout::of(CreatureKind::Cattle).unwrap()
}

fn hen() -> Layout {
    Layout::of(CreatureKind::Chicken).unwrap()
}

/// More triangles in `group` of the body's pieces with the change than without.
fn adds(before: Layout, change: impl Fn(&mut Layout), group: usize) -> Layout {
    let mut after = before;
    change(&mut after);
    let count = |layout: &Layout| morph::pieces(layout, Hair::NONE, None)[group].len();
    assert!(
        count(&after) > count(&before),
        "group {group}: {:?}",
        after.parts
    );
    fits(&after);
    after
}

/// Inside its picture, every point finite.
fn fits(layout: &Layout) {
    let (wide, long) = cells(layout);
    let per_cell = VIEW_WIDTH / (wide.max(long) as f32);
    for corner in fit::fitted(layout, 1750, Hair::NONE, None)
        .iter()
        .flatten()
        .flatten()
    {
        assert!(corner.is_finite());
        assert!(corner.x.abs() <= wide as f32 * per_cell / 2.0);
        assert!(corner.y.abs() <= long as f32 * per_cell / 2.0);
    }
}

const SKIN: usize = 0;
const FEATHERS: usize = 3;
const MANE: usize = 5;
const FLESH: usize = 6;

#[test]
fn antlers_rise_above_the_head_with_their_tines() {
    let plain = adds(cattle(), |l| l.parts.antlers = 700, SKIN);
    let tined = adds(plain, |l| l.parts.tines = 4, SKIN);
    let top = |layout: &Layout| {
        build(layout)
            .iter()
            .flatten()
            .map(|corner| corner.y)
            .fold(f32::MIN, f32::max)
    };
    let mut bare = cattle();
    bare.parts.horns = Horns::None;
    // Well above the withers, which are higher than a cow's lowered head.
    assert!(top(&tined) > top(&bare) + 300.0);
}

#[test]
fn a_ram_has_coiled_horns_beside_its_face() {
    let mut ram = cattle();
    ram.parts.horns = Horns::Spiral;
    ram.parts.horn_length = 700;
    let paths = horns_of(&ram);
    assert_eq!(paths.len(), 2);
    // Coiled: its tip comes back near its root, below it.
    let path = &paths[0];
    let (root, tip) = (path[0].0, path[path.len() - 1].0);
    let length: f32 = path.windows(2).map(|p| (p[1].0 - p[0].0).length()).sum();
    assert!((tip - root).length() < length * 0.5);
    assert!(tip.y < root.y);
    fits(&ram);
}

#[test]
fn humps_fleece_and_long_ears_are_skin() {
    adds(cattle(), |l| (l.parts.humps, l.parts.hump) = (1, 250), SKIN);
    let one = {
        let mut one = cattle();
        (one.parts.humps, one.parts.hump) = (1, 250);
        build(&one).len()
    };
    let mut two = cattle();
    (two.parts.humps, two.parts.hump) = (2, 250);
    assert!(build(&two).len() > one);
    adds(
        cattle(),
        |l| (l.parts.coat_shape, l.parts.coat_length) = (Coat::Fleece, 60),
        SKIN,
    );
    let mut long = cattle();
    long.parts.ear_shape = Ears::Long;
    long.parts.ears = 300;
    fits(&long);
}

#[test]
fn a_beard_and_a_skirt_are_long_hair_off_the_ground() {
    adds(cattle(), |l| l.parts.beard = 150, MANE);
    let yak = adds(
        cattle(),
        |l| (l.parts.coat_shape, l.parts.coat_length) = (Coat::Skirt, 2000),
        MANE,
    );
    assert!(
        coat_hair(&yak)
            .iter()
            .flatten()
            .all(|corner| corner.y > 0.0)
    );
}

#[test]
fn combs_wattles_snoods_and_caruncles_are_bare_red_skin() {
    // A hen's comb and wattles are no longer in her feathers' colour.
    assert!(!flesh(&hen()).is_empty());
    let mut bare = hen();
    (bare.parts.comb, bare.parts.wattles) = (0, 0);
    assert!(flesh(&bare).is_empty());
    adds(bare, |l| l.parts.snood = 60, FLESH);
    adds(bare, |l| l.parts.caruncles = 6, FLESH);
    adds(bare, |l| l.parts.wattles = 40, FLESH);
}

#[test]
fn a_bill_tusks_and_webbed_feet_are_drawn() {
    // A beak is in the head's rings; a bill is drawn past them.
    adds(hen(), |l| l.parts.snout = Snout::Bill, SKIN);
    adds(cattle(), |l| l.parts.tusks = 120, SKIN);
    // Three toes and webs between, in place of four toes.
    let mut webbed = hen();
    webbed.parts.feet = Feet::Webbed;
    assert_ne!(build(&webbed), build(&hen()));
    assert!(
        build(&webbed)
            .iter()
            .flatten()
            .all(|corner| corner.y >= 0.0)
    );
    fits(&webbed);
}

#[test]
fn sickles_are_feathers_rising_behind_and_spurs_are_skin() {
    let mut tailless = hen();
    tailless.parts.tail = 0;
    let cock = adds(
        tailless,
        |l| (l.parts.tail_shape, l.parts.tail) = (Tail::Sickle, 250),
        FEATHERS,
    );
    let rump = cock.spine.rings[0];
    let top = wings(&cock)
        .iter()
        .flatten()
        .map(|c| c.y)
        .fold(f32::MIN, f32::max);
    assert!(top > cock.spine.base[1] as f32 + rump.back as f32);
    adds(cock, |l| l.parts.spurs = 20, SKIN);
}

#[test]
fn a_knob_sits_on_a_bill() {
    let mut billed = hen();
    billed.parts.snout = Snout::Bill;
    adds(billed, |l| l.parts.knob = 20, SKIN);
}

#[test]
fn a_fanned_tail_is_feathers_standing_up_behind() {
    let mut tailless = hen();
    tailless.parts.tail = 0;
    let turkey = adds(
        tailless,
        |l| (l.parts.tail_shape, l.parts.tail) = (Tail::Fan, 200),
        FEATHERS,
    );
    let fan = wings(&turkey);
    let rump = turkey.spine.rings[0];
    let top = fan.iter().flatten().map(|c| c.y).fold(f32::MIN, f32::max);
    assert!(top > turkey.spine.base[1] as f32 + rump.back as f32);
}
