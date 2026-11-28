use super::*;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::quest::Standing;
use crate::site::BuildingKind;
use crate::value::total;

fn coins(kind: ItemKind, count: u32, id: u128) -> Item {
    Item {
        count,
        ..Item::new(Id::for_test(id), kind)
    }
}

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    for item in items {
        inventory
            .give(item, StackLimits::default(), &mut stand_in())
            .expect("room for it");
    }
    inventory
}

fn stand_in() -> impl FnMut() -> Id {
    let mut next = 1000;
    move || {
        next += 1;
        Id::for_test(next)
    }
}

#[test]
fn rates_move_towards_the_value_with_standing_and_skill() {
    assert_eq!(
        Rates::default(),
        Rates {
            buys: 50,
            sells: 150
        }
    );
    let full = Rates::for_shop(Standing::Champion, 100);
    assert_eq!(
        full,
        Rates {
            buys: 100,
            sells: 100
        }
    );
    let half = Rates::for_shop(Standing::Stranger, 50);
    assert_eq!(
        half,
        Rates {
            buys: 62,
            sells: 138
        }
    );
    // Past 100% the skill takes off no more.
    assert_eq!(skill_percent(400), MOST_SKILL);
    assert!(standing_percent(Standing::Known) < standing_percent(Standing::Trusted));
}

#[test]
fn shops_pay_half_again_less_for_what_they_dont_sell() {
    let rates = Rates::default();
    assert_eq!(shop_pays(1000, rates, true), 500);
    assert_eq!(shop_pays(1000, rates, false), 250);
    assert_eq!(shop_asks(1000, rates), 1500);
    assert!(normally_sold(BuildingKind::Blacksmith, ItemKind::Sword));
    assert!(!normally_sold(BuildingKind::Blacksmith, ItemKind::Fruit));
}

#[test]
fn the_player_pays_rounded_up_and_is_paid_rounded_down() {
    assert_eq!(due(1500, 0), 2);
    assert_eq!(due(0, 1999), -1);
    assert_eq!(due(0, 999), 0);
    assert_eq!(due(2000, 2000), 0);
}

#[test]
fn tips_and_haggles_by_the_fixed_rule() {
    assert_eq!(judge(100, None, 0), Ok((100, Payment::Exact)));
    let small = Payment::Tip {
        copper: 5,
        generous: false,
    };
    assert_eq!(judge(100, Some(105), 0), Ok((105, small)));
    let big = Payment::Tip {
        copper: 6,
        generous: true,
    };
    assert_eq!(judge(100, Some(106), 0), Ok((106, big)));
    // 5% short at full skill, less with less, nothing with none.
    assert_eq!(haggle_room(100, 100), 5);
    assert_eq!(haggle_room(100, 40), 2);
    assert_eq!(judge(100, Some(99), 0), Err(Refusal::TooShort));
    let haggled = Payment::Haggled { copper: 5 };
    assert_eq!(judge(100, Some(95), 100), Ok((95, haggled)));
    // Asking the shop for more works the same way.
    let asked = Payment::Haggled { copper: 2 };
    assert_eq!(judge(-100, Some(-102), 50), Ok((-102, asked)));
}

#[test]
fn coins_are_paid_highest_first_with_change() {
    let mut inventory = carrying(vec![
        coins(ItemKind::GoldCoin, 1, 1),
        coins(ItemKind::CopperCoin, 60, 2),
    ]);
    assert_eq!(inventory.coins_carried(), 10_060);
    let limits = StackLimits::default();
    inventory
        .pay_coins(50, limits, None, &mut stand_in())
        .unwrap();
    assert_eq!(inventory.coins_carried(), 10_010);
    // The change joined the coppers, which turned back up into gold.
    assert_eq!(inventory.count_of(ItemKind::GoldCoin), 1);
    assert_eq!(inventory.count_of(ItemKind::CopperCoin), 10);
    let refused = inventory.pay_coins(20_000, limits, None, &mut stand_in());
    assert_eq!(refused, Err(crate::item::PayRefused::NotEnough));
}

#[test]
fn a_trade_is_all_at_once() {
    let sword = Item::new(Id::for_test(10), ItemKind::Sword);
    let stone = Item::new(Id::for_test(11), ItemKind::Stone);
    let carried = carrying(vec![coins(ItemKind::GoldCoin, 1, 1), stone.clone()]);
    let stock = vec![sword.clone()];
    let mut trade = ShopTrade::new(Id::for_test(99));
    trade.selling.push(stone.id);
    trade.buy(sword.id, None, &stock).unwrap();
    let terms = (Rates::default(), BuildingKind::Blacksmith);
    let totals = trade.totals(&stock, &carried, terms.0, terms.1).unwrap();
    assert_eq!(totals.asks, total(&sword) * 3 / 2);
    assert_eq!(totals.pays, total(&stone) / 4);
    let done = complete(
        &trade,
        &stock,
        &carried,
        terms,
        0,
        StackLimits::default(),
        None,
        &mut stand_in(),
    )
    .unwrap();
    // Bought where it fits; the stone sold.
    assert!(done.inventory.find(sword.id).is_some());
    assert!(done.inventory.find(stone.id).is_none());
    assert_eq!(
        done.stock.iter().map(|item| item.id).collect::<Vec<_>>(),
        [stone.id]
    );
    assert_eq!(done.inventory.coins_carried(), 10_000 - totals.due as u64);
    // The same again: the sword is gone, so nothing happens.
    let again = complete(
        &trade,
        &done.stock,
        &done.inventory,
        terms,
        0,
        StackLimits::default(),
        None,
        &mut stand_in(),
    );
    assert_eq!(again.err(), Some(Refusal::NotStocked));
}

#[test]
fn an_empty_trade_is_nothing_but_a_gift_is_a_tip() {
    let carried = carrying(vec![coins(ItemKind::CopperCoin, 5, 1)]);
    let mut trade = ShopTrade::new(Id::for_test(99));
    let terms = (Rates::default(), BuildingKind::Blacksmith);
    let limits = StackLimits::default();
    let empty = complete(
        &trade,
        &[],
        &carried,
        terms,
        0,
        limits,
        None,
        &mut stand_in(),
    );
    assert_eq!(empty.err(), Some(Refusal::Nothing));
    trade.paying = Some(3);
    let gift = complete(
        &trade,
        &[],
        &carried,
        terms,
        0,
        limits,
        None,
        &mut stand_in(),
    );
    let gift = gift.unwrap();
    assert!(matches!(gift.payment, Payment::Tip { copper: 3, .. }));
    assert_eq!(gift.inventory.coins_carried(), 2);
}

#[test]
fn coins_and_quest_items_are_not_for_sale() {
    let carried = carrying(vec![coins(ItemKind::CopperCoin, 5, 1)]);
    let mut trade = ShopTrade::new(Id::for_test(99));
    trade.selling.push(Id::for_test(1));
    let refused = trade.totals(&[], &carried, Rates::default(), BuildingKind::Blacksmith);
    assert_eq!(refused, Err(Refusal::NotForSale));
}

#[test]
fn reach_is_five_cells_from_the_door() {
    let door = (10, 10);
    assert!(shop_in_reach(crate::spot::Spot::middle_of((15, 10)), door));
    assert!(!shop_in_reach(crate::spot::Spot::middle_of((16, 10)), door));
}
