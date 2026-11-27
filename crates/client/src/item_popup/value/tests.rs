use shared::id::Id;
use shared::item::{Item, ItemKind};
use shared::lang::{Lang, Language};

use super::*;

fn coins(kind: ItemKind, count: u32) -> Item {
    Item {
        count,
        ..Item::new(Id::for_test(1), kind)
    }
}

#[test]
fn none_of_a_coin_is_left_out() {
    let lang = Lang::new(Language::English);
    let gold = coins(ItemKind::GoldCoin, 1);
    assert_eq!(value_pieces(&gold, &lang), [Piece::Coin(Metal::Gold, 1)]);
}

#[test]
fn a_stack_says_what_one_is_worth() {
    let lang = Lang::new(Language::English);
    let silver = coins(ItemKind::SilverCoin, 3);
    assert_eq!(
        value_pieces(&silver, &lang),
        [
            Piece::Coin(Metal::Silver, 3),
            Piece::Words("(".to_owned()),
            Piece::Coin(Metal::Silver, 1),
            Piece::Words("each)".to_owned()),
        ]
    );
}

#[test]
fn worth_nothing_shows_nothing() {
    let lang = Lang::new(Language::English);
    let mut parcel = Item::new(Id::for_test(2), ItemKind::Parcel);
    parcel.quest = Some(shared::quest::QuestId {
        town: shared::poi::ChunkPos { x: 0, y: 0 },
        slot: shared::quest::Slot::Feeder,
    });
    assert!(value_pieces(&parcel, &lang).is_empty());
}
