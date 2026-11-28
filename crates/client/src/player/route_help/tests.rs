//! Who is in the player's way, by how the server says they feel.

use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use shared::clearance::Crowding;
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::sheet::Feeling;

use super::{Others, crowd, enemy_cells};
use crate::common::{GodMode, MapPosition, Other, SpotPosition};

/// A wolf told with this feeling (as a mob going after the player is told
/// `Unfriendly`): its crowding and whether its cell counts as an enemy's.
fn in_the_way(feeling: Feeling) -> (Crowding, bool) {
    let mut world = World::new();
    world.spawn((
        Other {
            id: Id::for_test(1),
            kind: CreatureKind::Wolf,
            feeling,
            player: false,
            never_trades: false,
        },
        MapPosition { x: 3, y: 4 },
        SpotPosition(shared::spot::Spot::middle_of((3, 4))),
    ));
    world
        .run_system_once(|others: Others| {
            let crowding = crowd(&others, GodMode(false))[0].crowding;
            (crowding, enemy_cells(&others, GodMode(false)) == [(3, 4)])
        })
        .unwrap()
}

#[test]
fn a_mob_told_unfriendly_blocks_steps_and_the_rest_are_squeezed_past() {
    assert_eq!(in_the_way(Feeling::Unfriendly), (Crowding::Block, true));
    assert_eq!(in_the_way(Feeling::Neutral), (Crowding::Squeeze, false));
}
