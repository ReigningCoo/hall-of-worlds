use shared::dungeon::{Cell, Level};
use shared::facing::Heading;
use shared::id::{Id, stand_ins};
use shared::item::{Inventory, StackLimits};
use shared::material::Material;
use shared::spot::Spot;
use shared::woodcutting::new_log;

use super::{log_aim, tree_to_fell};

#[test]
fn a_tree_faced_is_felled_and_one_beside_is_turned_to_first() {
    let mut level = Level::test_room();
    level.set(1, 0, Cell::Tree);
    let at = Spot::middle_of((0, 0));
    assert_eq!(tree_to_fell(&level, at, Heading::EAST), Some(None));
    let turned = tree_to_fell(&level, at, Heading::WEST).flatten();
    let (cell, heading) = turned.expect("turned to the tree behind");
    assert_eq!(cell, (1, 0));
    assert!(heading.apart(Heading::EAST) < 100, "{heading:?}");
    let far = Spot::middle_of((-4, 0));
    assert_eq!(
        tree_to_fell(&level, far, Heading::EAST),
        None,
        "out of reach"
    );
}

#[test]
fn a_carried_log_comes_before_one_on_the_ground() {
    let lying = vec![new_log(Material::Oak, Id::for_test(2), None)];
    let mut carried = Inventory {
        creature: shared::creature::CreatureKind::Human,
        size: (40, 40),
        ..Inventory::default()
    };
    assert_eq!(log_aim(&carried, &[]), None, "no log");
    assert_eq!(log_aim(&carried, &lying), Some(Some(Id::for_test(2))));
    let log = new_log(Material::Pine, Id::for_test(1), None);
    carried
        .give(log, StackLimits::NONE, &mut stand_ins())
        .unwrap();
    assert_eq!(log_aim(&carried, &lying), Some(None), "the server's choice");
}
