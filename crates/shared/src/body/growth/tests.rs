use super::super::Layout;
use super::super::blend::WHOLE;
use crate::creature::CreatureKind;

fn cow() -> Layout {
    Layout::of(CreatureKind::Cattle).expect("a body")
}

#[test]
fn a_whole_share_changes_nothing() {
    let cow = cow();
    assert_eq!(cow.with_legs(WHOLE), cow);
    assert_eq!(cow.with_girth(WHOLE), cow);
    assert_eq!(cow.with_back(WHOLE), cow);
    assert_eq!(cow.with_neck(WHOLE), cow);
    assert_eq!(cow.with_head(WHOLE), cow);
    assert_eq!(cow.with_fluff(0), cow);
}

#[test]
fn longer_legs_raise_the_body_by_what_they_gain() {
    let cow = cow();
    let leggy = cow.with_legs(1200);
    let length = |layout: &Layout| -> i32 {
        let leg = layout.limbs.iter().flatten().next().expect("a leg");
        leg.bones.iter().flatten().map(|bone| bone.span).sum()
    };
    let gained = length(&leggy) - length(&cow);
    assert!(gained > 0);
    assert_eq!(leggy.spine.base[1] - cow.spine.base[1], gained);
}

#[test]
fn a_bigger_head_is_bigger() {
    let cow = cow();
    assert!(cow.with_head(1300).head.length > cow.head.length);
    assert!(cow.with_girth(800).spine.chest.half.0 < cow.spine.chest.half.0);
}
