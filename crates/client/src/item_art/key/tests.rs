use std::collections::HashSet;

use bevy::prelude::*;
use shared::id::Id;
use shared::item::{Item, ItemKind};
use shared::item_form::ItemForm;

use super::look::KeyLook;
use super::parts::{FIT, fitted, hung_at, pieces};

fn looks() -> Vec<KeyLook> {
    (1..=300)
        .map(|n| KeyLook::of_lock(Id::for_test(n)))
        .collect()
}

#[test]
fn every_key_to_one_lock_looks_alike() {
    let lock = Id::for_test(7);
    let key = |n| {
        let mut item = Item::new(Id::for_test(n), ItemKind::Key);
        item.form = Some(ItemForm::key(lock));
        KeyLook::of(&item)
    };
    assert_eq!(key(100), key(200));
    assert_eq!(key(100), KeyLook::of_lock(lock));
}

#[test]
fn a_key_without_a_lock_goes_by_its_own_id() {
    let item = Item::new(Id::for_test(42), ItemKind::Key);
    assert_eq!(KeyLook::of(&item), KeyLook::of_lock(Id::for_test(42)));
}

#[test]
fn different_locks_give_a_spread_of_looks() {
    let looks = looks();
    let distinct: HashSet<_> = looks.iter().collect();
    assert!(distinct.len() > 290, "only {} looks", distinct.len());
    let bows: HashSet<_> = looks.iter().map(|look| look.bow).collect();
    let metals: HashSet<_> = looks.iter().map(|look| look.metal).collect();
    let collars: HashSet<_> = looks.iter().map(|look| look.collar).collect();
    assert_eq!((bows.len(), metals.len(), collars.len()), (7, 4, 4));
    assert!(looks.iter().any(|look| look.double));
    assert!(looks.iter().any(|look| look.pipe) && looks.iter().any(|look| !look.pipe));
}

#[test]
fn every_bit_has_two_to_five_wards() {
    for look in looks() {
        let wards = look.cuts.iter().filter(|&&cut| cut > 0).count();
        assert!((2..=5).contains(&wards), "{wards} wards: {look:?}");
    }
}

/// Every corner of every piece placed by `whole`, as (x, z).
fn reach(look: &KeyLook, whole: impl Fn(&[super::piece::Piece]) -> Transform) -> Vec2 {
    let pieces = pieces(look);
    let whole = whole(&pieces);
    let corners = pieces.iter().flat_map(|piece| piece.corners(whole));
    corners.fold(Vec2::ZERO, |most, corner| {
        most.max(Vec2::new(corner.x.abs(), corner.z.abs()))
    })
}

#[test]
fn keys_stay_inside_their_picture() {
    for look in looks() {
        let most = reach(&look, fitted);
        assert!(most.max_element() <= FIT / 2.0 + 1e-4, "{most}: {look:?}");
        // Filling it one way or the other.
        assert!(most.max_element() >= FIT / 2.0 - 1e-3, "{most}: {look:?}");
    }
}

#[test]
fn the_keyrings_keys_stay_inside_its_picture() {
    for (look, at) in crate::item_art::keyring::keys() {
        let long = crate::item_art::keyring::KEY_LONG;
        let most = reach(&look, |pieces| hung_at(pieces, at, long));
        assert!(most.x <= 1.25 && most.y <= 0.62, "{most}: {look:?}");
    }
}
