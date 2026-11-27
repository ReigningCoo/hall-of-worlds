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
fn sets_have_2_to_7_pieces_worn_in_different_places() {
    let mut sizes = std::collections::BTreeSet::new();
    for number in 0..POOL {
        let set = ItemSet::of(number);
        let count = set.pieces.len();
        assert!((MIN_PIECES..=MAX_PIECES).contains(&count), "{set:?}");
        sizes.insert(count);
        let mut places: Vec<Location> = Vec::new();
        let mut rings = 0;
        for kind in &set.pieces {
            let wear = kind.def().wear.expect("pieces are worn or held");
            if *kind == ItemKind::Ring {
                rings += 1;
                if rings == 2 {
                    continue;
                }
            }
            for &place in wear.locations {
                assert!(!places.contains(&place), "{set:?}");
                places.push(place);
            }
        }
        assert!(rings <= 2, "{set:?}");
        assert!(set.quality != Quality::Poor && set.quality != Quality::Artifact);
    }
    assert_eq!(
        sizes.len(),
        MAX_PIECES - MIN_PIECES + 1,
        "every size turns up"
    );
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
    for (n, kind) in crate::quest::PRIZE_KINDS.iter().enumerate() {
        let found = SetPiece::fitting(*kind, Some(Quality::Rare), Id::for_test(n as u128));
        let found = found.expect("every prize kind is in some set");
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
