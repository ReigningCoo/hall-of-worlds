use shared::id::Id;
use shared::item::Item;
use shared::item_set::{ItemSet, POOL, SetPiece};
use shared::lang::{Lang, Language};

use super::*;

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
fn an_item_of_no_set_has_no_section() {
    let lang = Lang::new(Language::English);
    let item = Item::new(Id::for_test(1), shared::item::ItemKind::Helmet);
    assert!(set_lines(&item, &[], &lang).is_empty());
}

#[test]
fn pieces_and_bonuses_are_grey_until_worn() {
    let lang = Lang::new(Language::English);
    let set = (0..POOL)
        .map(ItemSet::of)
        .find(|set| set.pieces.len() >= 3)
        .unwrap();
    let worn = vec![piece_of(&set, 0, 1), piece_of(&set, 2, 2)];
    let lines = set_lines(&worn[0], &worn, &lang);
    assert_eq!(lines[0].words, set.name);
    assert!(!lines[0].indented && !lines[0].off);
    let pieces = &lines[1..=set.pieces.len()];
    assert!(pieces.iter().all(|line| line.indented));
    let off: Vec<bool> = pieces.iter().map(|line| line.off).collect();
    let mut expected = vec![true; set.pieces.len()];
    (expected[0], expected[2]) = (false, false);
    assert_eq!(off, expected);
    let rest = &lines[1 + set.pieces.len()..];
    assert_eq!(rest[0].words, "2 worn:");
    assert!(!rest[0].indented && !rest[0].off, "two are worn");
    assert!(rest[1].indented && !rest[1].off);
    assert!(rest[1].words.contains(": "), "{}", rest[1].words);
    let three = rest.iter().position(|line| line.words == "3 worn:");
    let three = three.expect("a bonus at three worn");
    assert!(rest[three].off && rest[three + 1].off);
    let bonuses = rest.iter().filter(|line| line.indented).count();
    assert_eq!(bonuses, set.bonuses.len());
}
