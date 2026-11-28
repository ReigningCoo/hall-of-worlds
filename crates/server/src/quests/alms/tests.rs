use shared::id::Id;
use shared::item::ItemKind;

use super::*;
use crate::quests::standing::standing;
use shared::quest::Standing;

const TOWN: ChunkPos = ChunkPos { x: 1, y: 2 };
const DAY: i64 = 20_000;

#[test]
fn alms_raise_standing_by_the_amount_up_to_a_daily_cap() {
    let mut book = QuestBook::default();
    // Less than a point's worth adds nothing yet, but is kept for the day.
    assert_eq!(give(&mut book, TOWN, DAY, COPPER_PER_POINT - 1), 0);
    assert_eq!(give(&mut book, TOWN, DAY, 1), 1);
    assert_eq!(book.standing[&TOWN], 1);
    // A gold coin's worth reaches no further than the day's cap.
    assert_eq!(give(&mut book, TOWN, DAY, 10_000), POINTS_A_DAY - 1);
    assert_eq!(give(&mut book, TOWN, DAY, 10_000), 0);
    assert_eq!(book.standing[&TOWN], POINTS_A_DAY);
    // Another town has its own cap, and the next day starts again, forgetting
    // the day before.
    let other = ChunkPos { x: 5, y: 5 };
    assert_eq!(give(&mut book, other, DAY, 10_000), POINTS_A_DAY);
    assert_eq!(give(&mut book, TOWN, DAY + 1, 10_000), POINTS_A_DAY);
    assert_eq!(book.alms.len(), 1, "only today's alms kept");
}

#[test]
fn alms_count_past_known() {
    let mut book = QuestBook::default();
    for day in 0..40 {
        give(&mut book, TOWN, day, 10_000);
    }
    assert_eq!(book.standing[&TOWN], 40 * POINTS_A_DAY);
    assert_eq!(standing(&book, TOWN), Standing::Honoured);
}

#[test]
fn only_coins_are_alms() {
    let mut coins = Item::new(Id::for_test(1), ItemKind::SilverCoin);
    coins.count = 3;
    assert_eq!(copper_in(&coins), 300);
    assert_eq!(copper_in(&Item::new(Id::for_test(2), ItemKind::Dagger)), 0);
}
