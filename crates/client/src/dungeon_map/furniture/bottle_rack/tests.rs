use shared::bottle_rack::{BottleRack, BottleRackKind};
use shared::door::Side;
use shared::furniture::{Furniture, Material, Piece, Wood};

use super::*;

fn piece(rack: BottleRack, facing: Side) -> Piece {
    let furniture = Furniture::BottleRack { rack };
    let size = shared::furniture::footprint(furniture, (rack.length, 1), facing);
    Piece {
        at: (4, 6),
        size,
        facing,
        material: Material::Wood(Wood::Oak),
        furniture,
    }
}

#[test]
fn every_bottle_is_over_its_rack_apart_from_the_others_and_under_its_top() {
    let half = TALLEST / 2.0;
    for rack in BottleRack::all() {
        for facing in [Side::North, Side::East, Side::South, Side::West] {
            let piece = piece(rack, facing);
            assert!(piece.is_well_made(), "{rack:?}");
            let spots: Vec<Vec3> = (0..rack.capacity())
                .map(|index| bottle_spot(&piece, index, half).unwrap().0)
                .collect();
            let top = bottle_rack_height(&piece).unwrap();
            for (i, a) in spots.iter().enumerate() {
                let cell = (
                    (a.x / CELL_SIZE + 0.5).floor() as i64,
                    (a.z / CELL_SIZE + 0.5).floor() as i64,
                );
                assert!(piece.covers(cell), "{rack:?} {facing:?} {a}");
                assert!(a.y > 0.0 && a.y < top, "{rack:?} {a}");
                if rack.kind == BottleRackKind::Spirit {
                    assert!(a.y - half > TABLE_TOP, "on the table");
                }
                for b in &spots[i + 1..] {
                    assert!(a.distance(*b) > CELL_SIZE * 0.09, "{rack:?} overlapping");
                }
            }
        }
    }
}

#[test]
fn a_spirit_racks_back_shelf_is_highest_and_by_its_wall() {
    let rack = BottleRack {
        kind: BottleRackKind::Spirit,
        length: 1,
    };
    // Facing south: its back (the wall side) is north, lower `z`.
    let piece = piece(rack, Side::South);
    let front = bottle_spot(&piece, 0, 0.0).unwrap().0;
    let back = bottle_spot(&piece, rack.capacity() - 1, 0.0).unwrap().0;
    assert!(back.y > front.y);
    assert!(back.z < front.z, "{back} {front}");
}
