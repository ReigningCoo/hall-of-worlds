//! A player's own grid sized by Strength through a connected client: on
//! joining, growing and shrinking with an effect, moving or dropping what lies
//! on cells lost, and refusing a change of the player's own.

use shared::bonus::{Bonus, Effect, EndsOn, Source, Stat};
use shared::id::Id;
use shared::item::{Holder, Item, ItemKind, MoveRefused, Rotation, StackLimits};
use shared::protocol::ServerMessage;
use shared::sheet::Attribute;

use super::strength_room::fit_to_strength;
use super::tests::TestClient;

/// Strength for `ticks` action ticks from now, in whole points.
fn strength_for(test: &mut TestClient, points: i32, ticks: u64) {
    let now = test.world.action_clock.ticks();
    let effect = Effect {
        source: Source::RawHerb,
        bonuses: vec![Bonus::flat(
            Stat::Attribute(Attribute::Strength),
            points * 1000,
        )],
        start: now,
        end: Some(now + ticks),
        ends_on: EndsOn::NOTHING,
    };
    let (sheet, situation) = test.client.sheet_and_situation();
    sheet.add_effect(effect, situation, now);
}

/// A tick's own work: attributes worked out again, then the grid fitted to them.
fn tick(test: &mut TestClient) -> Vec<ServerMessage> {
    let now = test.world.action_clock.ticks();
    assert!(test.client.keep_vitals(now));
    assert!(fit_to_strength(&mut test.client, &mut test.world));
    std::iter::from_fn(|| test.end.receive().unwrap()).collect()
}

/// Puts `count` apples, ids from 1, at the end of the own grid, never to stack.
fn apples_at_the_end(test: &mut TestClient, count: u128) {
    test.world.stacks = StackLimits::NONE;
    let inventory = &mut test.client.inventory;
    let cells = inventory.own_cells();
    for n in 1..=count {
        let at = cells - n as u32;
        let apple = Item::new(Id::for_test(n), ItemKind::Apple);
        let (x, y) = ((at % 8) as u8, (at / 8) as u8);
        inventory
            .add(apple, Holder::Inventory, x, y, Rotation::None)
            .unwrap();
    }
}

#[test]
fn a_player_joins_with_32_cells_8_wide() {
    let test = TestClient::new();
    assert!(test.client.inventory.sized_by_strength());
    assert_eq!(test.client.inventory.own_cells(), 32);
    assert_eq!(test.client.inventory.size, (8, 4));
}

#[test]
fn more_strength_gives_more_cells_and_the_client_is_told() {
    let mut test = TestClient::new();
    let strength = test.client.sheet.attributes.strength as i32;
    strength_for(&mut test, 200 - strength, 1000);
    let told = tick(&mut test);
    assert_eq!(test.client.inventory.own_cells(), 50);
    assert_eq!(test.client.inventory.size, (8, 7));
    let sent = told.iter().any(|answer| match answer {
        ServerMessage::Inventory(inventory) => inventory.own_cells() == 50,
        _ => false,
    });
    assert!(sent);
}

#[test]
fn when_it_wears_off_what_lies_on_lost_cells_moves_or_drops() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let strength = test.client.sheet.attributes.strength as i32;
    strength_for(&mut test, 200 - strength, 10);
    tick(&mut test);
    assert_eq!(test.client.inventory.own_cells(), 50);
    // Fill every cell: at 32, eighteen apples have nowhere to go.
    apples_at_the_end(&mut test, 50);
    test.world.action_clock.advance(20);
    tick(&mut test);
    assert_eq!(test.client.inventory.own_cells(), 32);
    assert_eq!(test.client.inventory.items.len(), 32);
    assert!(test.client.ground_told.again, "the rest dropped");
}

#[test]
fn on_the_world_map_nothing_drops_until_there_is_ground() {
    let mut test = TestClient::new();
    let strength = test.client.sheet.attributes.strength as i32;
    strength_for(&mut test, 200 - strength, 10);
    tick(&mut test);
    apples_at_the_end(&mut test, 50);
    test.world.action_clock.advance(20);
    tick(&mut test);
    assert_eq!(test.client.inventory.own_cells(), 50, "waits");
    assert_eq!(test.client.inventory.items.len(), 50);
}

#[test]
fn a_change_of_the_player_s_own_is_refused_while_the_cells_hold_something() {
    let mut test = TestClient::new();
    let strength = test.client.sheet.attributes.strength as i32;
    strength_for(&mut test, 200 - strength, 1000);
    tick(&mut test);
    apples_at_the_end(&mut test, 1);
    let now = test.world.action_clock.ticks();
    // Weaker by what they would do: the effect gone.
    let mut weaker = test.client.sheet.clone();
    weaker.effects.0.clear();
    let after = test.client.inventory.clone();
    let refused = test.client.strength_allows(&after, &weaker, now);
    assert_eq!(refused, Err(MoveRefused::StrengthInUse));
    test.client.inventory.items.clear();
    let after = test.client.inventory.clone();
    assert_eq!(test.client.strength_allows(&after, &weaker, now), Ok(()));
    // And a change to nothing worn is never refused.
    assert!(test.client.change_allowed(|_| {}, now));
}
