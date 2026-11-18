use super::*;
use crate::item::ItemKind;
use crate::rng::Seed;
use crate::site::BuildingKind;

const fn miller(quality: u16) -> Shop {
    Shop {
        kind: BuildingKind::Miller,
        quality,
        trades: 100,
        of_kind: 2,
    }
}

const DAY: u64 = MINUTES_PER_DAY;

/// He sells only flour, at most ten sacks, made at two to three a day by
/// his quality, and never tops up.
#[test]
fn a_miller_keeps_a_small_stock_of_flour() {
    assert_eq!(sells(BuildingKind::Miller), &[(ItemKind::Flour, 1)]);
    for quality in [0, 500, 1000] {
        let shop = miller(quality);
        assert_eq!(shop.target(), 10);
        assert!(!shop.tops_up());
        let a_day = DAY / shop.make_minutes(crate::item::Quality::Common, 100);
        assert!((2..=3).contains(&a_day), "{quality}: {a_day} a day");
        let full = catch_up(Seed::new(3), &shop, &[], None, None, 100 * DAY);
        assert!(full.new.len() <= 10);
        assert!(full.new.iter().all(|new| new.kind == ItemKind::Flour));
    }
    assert!(miller(0).make_minutes(crate::item::Quality::Common, 100) == DAY / 2);
    assert!(miller(1000).make_minutes(crate::item::Quality::Common, 100) == DAY / 3);
}

/// Bought out and left a week, he has only what he milled since, not a
/// top-up.
#[test]
fn a_miller_left_a_while_has_only_what_he_milled() {
    let shop = miller(0);
    let first = catch_up(Seed::new(5), &shop, &[], None, None, 100 * DAY);
    let later = 100 * DAY + 2 * DAY;
    let back = catch_up(
        Seed::new(5),
        &shop,
        &[],
        first.last_made,
        Some(100 * DAY),
        later,
    );
    assert!(back.new.len() <= 5, "{}", back.new.len());
    assert!(back.new.len() >= 3, "{}", back.new.len());
}
