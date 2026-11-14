use shared::creature::CreatureKind;
use shared::item::{Inventory, Item, ItemKind, MadeAt, Quality};
use shared::sheet::Attributes;
use shared::world::WorldSeed;

use super::*;

/// A new `kind` of item for a giant.
fn make(kind: ItemKind) -> Item {
    let giant = Inventory {
        creature: CreatureKind::Giant,
        ..Inventory::default()
    };
    let made_at = MadeAt {
        world: WorldSeed::new(1).id(),
        minute: 0,
    };
    crate::connections::made_for(&giant, kind, None, false, Quality::Common, made_at)
}

#[test]
fn armour_that_works_is_kept_the_first_time() {
    let mut made = 0;
    let strong = Attributes::all(1000);
    let armour = suited(&strong, || {
        made += 1;
        make(ItemKind::FullArmour)
    });
    assert_eq!(made, 1);
    assert!(works_for(&armour, &strong));
}

#[test]
fn clothes_ask_nothing_so_are_kept_the_first_time() {
    let mut made = 0;
    suited(&Attributes::all(1), || {
        made += 1;
        make(ItemKind::Trousers)
    });
    assert_eq!(made, 1);
}

#[test]
fn armour_nothing_suits_is_tried_again_and_the_lightest_kept() {
    let feeble = Attributes::all(1);
    let mut weights = Vec::new();
    let armour = suited(&feeble, || {
        let piece = make(ItemKind::FullArmour);
        weights.push(piece.total_weight());
        piece
    });
    assert_eq!(weights.len(), TRIES as usize);
    assert_eq!(Some(&armour.total_weight()), weights.iter().min());
}
