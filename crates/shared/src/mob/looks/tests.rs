use super::*;

#[test]
fn weighted_rolls_land_by_their_weights() {
    let weights = [60, 22, 6, 6, 3, 3];
    let mut counts = [0u32; 6];
    for rolled in 0..100u64 {
        counts[usize::from(weighted(rolled, &weights))] += 1;
    }
    assert_eq!(counts, [60, 22, 6, 6, 3, 3]);
}

#[test]
fn coos_are_mostly_red_brown_and_others_even() {
    let mut red = 0;
    let mut rolled = 0u64;
    for n in 0..2000u64 {
        let roll = |what: i64| n.wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ what as u64;
        let looks = looks(&roll, CreatureKind::Coo);
        let coat = looks.choice(CreatureKind::Coo, Trait::CoatColour).unwrap();
        red += u32::from(coat.name == "Red-brown");
        rolled += 1;
    }
    let share = red as u64 * 100 / rolled;
    assert!((50..=70).contains(&share), "{share}% red-brown");
    assert_eq!(weights(CreatureKind::Cattle, Trait::CoatColour), None);
    let coat = Trait::CoatColour.choices_for(CreatureKind::Coo).len();
    assert_eq!(
        weights(CreatureKind::Coo, Trait::CoatColour).unwrap().len(),
        coat
    );
}
