use super::*;
use crate::bonus::{Amount, GEAR_STATS, SKILL_LEVEL, Stat};
use crate::creature::Location;
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};

fn piece_of(set: &ItemSet, piece: usize, n: u128) -> Item {
    let mut item = Item::new(Id::for_test(n), set.pieces[piece]);
    item.quality = Some(set.quality);
    item.set = Some(SetPiece {
        set: set.number,
        piece: piece as u8,
    });
    item
}

#[test]
fn a_set_is_always_the_same() {
    assert_eq!(ItemSet::of(7), ItemSet::of(7));
    assert_ne!(ItemSet::of(7), ItemSet::of(8));
    assert_eq!(ItemSet::of(POOL + 7), ItemSet::of(7));
}

#[test]
fn sets_have_2_to_7_pieces_of_one_layer_or_one_place() {
    let mut sizes = std::collections::BTreeSet::new();
    let (mut by_layer, mut by_place, mut armed) = (0, 0, 0);
    for number in 0..POOL {
        let set = ItemSet::of(number);
        let count = set.pieces.len();
        assert!((MIN_PIECES..=MAX_PIECES).contains(&count), "{set:?}");
        sizes.insert(count);
        let wear = |kind: &ItemKind| kind.def().wear.expect("pieces are worn or held");
        let (held, worn): (Vec<ItemKind>, Vec<ItemKind>) =
            (set.pieces.iter()).partition(|kind| wear(kind).locations.contains(&Location::Held));
        assert!(held.len() <= 2, "{set:?}");
        assert!(held.len() < 2 || held[0] != held[1], "{set:?}");
        armed += usize::from(!held.is_empty());
        let one_layer = worn
            .iter()
            .all(|kind| wear(kind).layer == wear(&worn[0]).layer)
            && apart(&worn);
        let shared = (worn
            .first()
            .map_or(&[][..], |kind| wear(kind).locations)
            .iter())
        .any(|place| worn.iter().all(|kind| wear(kind).locations.contains(place)));
        let layers: std::collections::BTreeSet<_> =
            worn.iter().map(|kind| wear(kind).layer).collect();
        let one_place = shared && layers.len() == worn.len();
        assert!(worn.len() < 2 || one_layer || one_place, "{set:?}");
        by_layer += usize::from(worn.len() >= 2 && one_layer);
        by_place += usize::from(worn.len() >= 2 && one_place && !one_layer);
        assert!(set.quality != Quality::Poor && set.quality != Quality::Artifact);
    }
    assert_eq!(
        sizes.len(),
        MAX_PIECES - MIN_PIECES + 1,
        "every size turns up"
    );
    assert!(by_layer > 200 && by_place > 200, "{by_layer} {by_place}");
    assert!(armed > 200 && armed < 800, "{armed}");
}

/// Whether worn pieces are at different places, but for a second ring.
fn apart(worn: &[ItemKind]) -> bool {
    let mut places: Vec<Location> = Vec::new();
    let mut rings = 0;
    for kind in worn {
        if *kind == ItemKind::Ring {
            rings += 1;
            if rings == 2 {
                continue;
            }
        }
        for &place in kind.def().wear.unwrap().locations {
            if places.contains(&place) {
                return false;
            }
            places.push(place);
        }
    }
    rings <= 2
}

#[test]
fn every_kind_a_piece_may_be_is_in_some_set() {
    let kinds: Vec<ItemKind> = (0..POOL).flat_map(|n| ItemSet::of(n).pieces).collect();
    for kind in kinds::worn_kinds().iter().chain(kinds::weapons()) {
        assert!(kinds.contains(kind), "{kind:?}");
    }
    for kind in [
        ItemKind::Shirt,
        ItemKind::Socks,
        ItemKind::Cloak,
        ItemKind::Sword,
    ] {
        assert!(kinds.contains(&kind), "{kind:?}");
    }
    for kind in [
        ItemKind::FullArmour,
        ItemKind::Barding,
        ItemKind::Lantern,
        ItemKind::Backpack,
    ] {
        assert!(!kinds.contains(&kind), "{kind:?}");
    }
}

#[test]
fn a_full_set_gives_a_bonus_a_piece_each_different_and_stronger() {
    for number in 0..POOL {
        let set = ItemSet::of(number);
        let count = set.pieces.len();
        assert_eq!(set.bonuses.len(), count, "{set:?}");
        assert_eq!(set.bonuses_at(1).count(), 0);
        for worn in 2..=count {
            assert!(set.bonuses_at(worn).count() >= worn - 1, "{set:?}");
        }
        let stats: Vec<Stat> = set.bonuses.iter().map(|each| each.bonus.stat).collect();
        for (at, stat) in stats.iter().enumerate() {
            assert!(!stats[..at].contains(stat), "{set:?}");
        }
        for each in &set.bonuses {
            let usual = match each.bonus.stat {
                Stat::Skill(_) => Amount::Flat(SKILL_LEVEL),
                stat => GEAR_STATS.iter().find(|g| g.stat == stat).unwrap().amount,
            };
            let doubled = match usual {
                Amount::Flat(milli) => Amount::Flat(milli * SET_BONUS_FACTOR),
                Amount::Percent(milli) => Amount::Percent(milli * SET_BONUS_FACTOR),
            };
            assert_eq!(each.bonus.amount, doubled);
        }
    }
}

#[test]
fn set_bonuses_count_different_pieces_of_the_right_kind() {
    let set = (0..POOL)
        .map(ItemSet::of)
        .find(|set| set.pieces.len() >= 3)
        .unwrap();
    let one = vec![piece_of(&set, 0, 1)];
    assert!(set_bonuses(&one).is_empty());
    let twice = vec![piece_of(&set, 0, 1), piece_of(&set, 0, 2)];
    assert!(
        set_bonuses(&twice).is_empty(),
        "the same piece twice is one"
    );
    let two = vec![piece_of(&set, 0, 1), piece_of(&set, 1, 2)];
    assert_eq!(set_bonuses(&two).len(), 1);
    let mut wrong = piece_of(&set, 1, 2);
    wrong.kind = if wrong.kind == ItemKind::Ring {
        ItemKind::Necklace
    } else {
        ItemKind::Ring
    };
    assert!(set_bonuses(&[piece_of(&set, 0, 1), wrong]).is_empty());
    let all: Vec<Item> = (0..set.pieces.len())
        .map(|piece| piece_of(&set, piece, piece as u128 + 1))
        .collect();
    assert_eq!(set_bonuses(&all).len(), set.pieces.len());
    assert_eq!(worn_pieces(&all)[&set.number], set.pieces.len());
}

#[test]
fn fitting_finds_a_piece_of_the_kind_and_quality() {
    for (n, kind) in kinds::worn_kinds()
        .iter()
        .chain(kinds::weapons())
        .enumerate()
    {
        let found = SetPiece::fitting(*kind, Some(Quality::Rare), Id::for_test(n as u128));
        let found = found.expect("every kind a piece may be is in some set");
        assert_eq!(found.kind(), Some(*kind));
        assert_eq!(found.quality(), Quality::Rare);
    }
    assert_eq!(
        SetPiece::fitting(ItemKind::Bread, None, Id::for_test(1)),
        None
    );
}

#[test]
fn old_saves_set_flag_becomes_a_piece() {
    let mut item = Item::new(Id::for_test(9), ItemKind::Helmet);
    item.quality = Some(Quality::Epic);
    let saved = ron::to_string(&item).unwrap();
    let old = saved.replace("set:None", "set:true");
    assert_ne!(old, saved, "{saved}");
    let read: Item = ron::from_str(&old).unwrap();
    let piece = read.set.expect("a piece of a set");
    assert_eq!(piece.kind(), Some(ItemKind::Helmet));
    let plain: Item = ron::from_str(&saved.replace("set:None", "set:false")).unwrap();
    assert_eq!(plain.set, None);
    item.set = Some(piece);
    let again: Item = ron::from_str(&ron::to_string(&item).unwrap()).unwrap();
    assert_eq!(again.set, Some(piece));
    let bytes = postcard::to_allocvec(&item).unwrap();
    assert_eq!(postcard::from_bytes::<Item>(&bytes).unwrap(), item);
}

#[test]
fn pieces_worn_names_each_piece_once() {
    let set = (0..POOL)
        .map(ItemSet::of)
        .find(|set| set.pieces.len() >= 3)
        .unwrap();
    let worn = vec![
        piece_of(&set, 2, 1),
        piece_of(&set, 0, 2),
        piece_of(&set, 2, 3),
    ];
    let pieces: Vec<u8> = pieces_worn(&worn, set.number).into_iter().collect();
    assert_eq!(pieces, vec![0, 2]);
    assert!(pieces_worn(&worn, (set.number + 1) % POOL).is_empty());
}

#[test]
fn a_saved_piece_of_another_kind_is_fitted_again() {
    let set = ItemSet::of(3);
    let other = if set.pieces[0] == ItemKind::Helmet {
        ItemKind::Ring
    } else {
        ItemKind::Helmet
    };
    let mut item = piece_of(&set, 0, 7);
    item.kind = other;
    let read: Item = ron::from_str(&ron::to_string(&item).unwrap()).unwrap();
    let piece = read.set.expect("a piece of its own kind");
    assert_eq!(piece.kind(), Some(other));
    let again: Item = ron::from_str(&ron::to_string(&item).unwrap()).unwrap();
    assert_eq!(again.set, Some(piece), "always the same piece");
    let right = piece_of(&set, 0, 7);
    let kept: Item = ron::from_str(&ron::to_string(&right).unwrap()).unwrap();
    assert_eq!(kept.set, right.set, "a piece of its kind is kept");
}
