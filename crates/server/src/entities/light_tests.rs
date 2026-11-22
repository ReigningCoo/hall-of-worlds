//! Whoever sees a player is told how far their light reaches, and told again
//! when a lantern lights or goes out (`EntityLook::light`).

use shared::dungeon::CARRIED_LIGHT;
use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::item_form::ItemForm;
use shared::lantern::{LANTERN_LIGHT, LanternForm, SECONDS_PER_ML};
use shared::protocol::ServerMessage;

use super::tests::Two;

/// The light of everyone told of in these answers (only players here).
fn lights(answers: &[ServerMessage]) -> Vec<u32> {
    (answers.iter())
        .filter_map(|answer| match answer {
            ServerMessage::EntitySeen(look) => Some(look.light),
            _ => None,
        })
        .collect()
}

/// What both players were told in the next tick.
fn both_told(two: &mut Two) -> Vec<u32> {
    let [first, second] = two.tick();
    [lights(&first), lights(&second)].concat()
}

#[test]
fn a_lantern_lighting_and_going_out_is_told_to_whoever_sees_its_carrier() {
    let mut two = Two::new();
    let lantern = Item {
        form: Some(ItemForm::Lantern(LanternForm { ml: 1, burnt: 0 })),
        ..Item::new(Id::for_test(50), ItemKind::Lantern)
    };
    let carrier = &mut two.connections.clients_mut()[0];
    let new_id = &mut shared::id::stand_ins();
    let given = carrier.inventory.give(lantern, StackLimits::NONE, new_id);
    assert!(given.is_ok());
    assert!(carrier.inventory.wear(Id::for_test(50)).is_ok());
    assert_eq!(both_told(&mut two), [LANTERN_LIGHT], "held and lit");
    assert_eq!(both_told(&mut two), [] as [u32; 0], "told once");

    // Its last millilitre burns: only the light changes, and is told.
    let carrier = &mut two.connections.clients_mut()[0];
    let burnt = carrier.inventory.burn_lanterns(SECONDS_PER_ML);
    assert!(burnt.went_out);
    assert_eq!(both_told(&mut two), [CARRIED_LIGHT], "gone out");
}
