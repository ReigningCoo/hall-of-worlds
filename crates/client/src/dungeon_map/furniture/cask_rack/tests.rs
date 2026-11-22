use shared::cask::CaskSize;
use shared::cask_rack::CaskRack;
use shared::door::Side;
use shared::furniture::{Furniture, Material, Piece, Wood};

use super::*;

fn piece(rack: CaskRack, facing: Side) -> Piece {
    let size = shared::furniture::footprint(Furniture::CaskRack { rack }, (rack.length, 1), facing);
    Piece {
        at: (4, 6),
        size,
        facing,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::CaskRack { rack },
    }
}

#[test]
fn every_cask_lies_over_its_piece_without_overlapping_another() {
    for rack in CaskRack::all() {
        for facing in [Side::North, Side::East, Side::South, Side::West] {
            let piece = piece(rack, facing);
            assert!(piece.is_well_made(), "{rack:?}");
            let radius = cask_radius(rack.size);
            let spots: Vec<Vec3> = (0..rack.capacity())
                .map(|index| cask_spot(&piece, index).unwrap().0)
                .collect();
            for (i, a) in spots.iter().enumerate() {
                let cell = (
                    (a.x / CELL_SIZE + 0.5).floor() as i64,
                    (a.z / CELL_SIZE + 0.5).floor() as i64,
                );
                assert!(piece.covers(cell), "{rack:?} {facing:?} {a}");
                assert!(a.y - radius >= 0.0, "{rack:?} under the floor");
                for b in &spots[i + 1..] {
                    assert!(
                        a.distance(*b) >= 2.0 * radius * 0.99,
                        "{rack:?} overlapping"
                    );
                }
            }
            assert!(rack_height(&piece).unwrap() > spots.iter().map(|s| s.y).fold(0.0, f32::max));
        }
    }
}

#[test]
fn casks_on_racks_fit_their_places_and_big_ones_their_cells() {
    for size in CaskSize::ALL {
        let length = cask_height(size) * cask_scale(size);
        let across = 2.0 * cask_radius(size);
        match size {
            CaskSize::Tun => assert!(length <= 1.9 * CELL_SIZE && across <= CELL_SIZE),
            _ => assert!(length <= CELL_SIZE && across <= CELL_SIZE, "{size:?}"),
        }
    }
}
