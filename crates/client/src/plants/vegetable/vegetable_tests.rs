use std::collections::HashSet;

use super::look::Bears;
use super::*;
use crate::dungeon_map::CELL_SIZE;
use shared::herb::STAGES;
use shared::vegetable::Vegetables;

/// The most triangles a cell's model may have (its outline as many again):
/// fields hold many cells, so they must stay cheap to draw.
const MOST_TRIANGLES: usize = 1600;

fn top(pieces: &Pieces) -> f32 {
    pieces
        .fill
        .positions
        .iter()
        .map(|p| p[1])
        .fold(0.0, f32::max)
}

/// Every vegetable's cell has a model at every stage in every variant, with
/// its outline, and none has too many triangles.
#[test]
fn every_vegetable_is_built_at_every_stage() {
    let mut heavy = Vec::new();
    for def in Vegetables::built_in().iter() {
        for variant in 0..VARIANTS {
            for stage in 1..=STAGES {
                let cell = build_cell(def, stage, variant);
                assert!(!cell.fill.is_empty(), "{} {stage}", def.key);
                assert_eq!(cell.fill.len(), cell.outline.len(), "{}", def.key);
                if cell.fill.len() > MOST_TRIANGLES {
                    heavy.push(format!("{} {stage}: {}", def.key, cell.fill.len()));
                }
            }
        }
    }
    assert!(heavy.is_empty(), "{}", heavy.join("\n"));
}

/// A plant keeps its shape from stage 3 to full grown: only what shows
/// besides its leaves and stems changes.
#[test]
fn a_plant_keeps_its_shape() {
    let corners = |pieces: Pieces| {
        let positions = pieces.fill.positions.into_iter();
        positions
            .map(|p| p.map(f32::to_bits))
            .collect::<HashSet<_>>()
    };
    for def in Vegetables::built_in().iter() {
        let slot = layout::slot_metres(def);
        let grown = corners(build(def, (3, slot), (1, 0), false));
        let full = corners(build(def, (STAGES, slot), (1, 0), false));
        assert_eq!(grown, full, "{}", def.key);
    }
}

/// Full grown, what is picked shows (or flowers, where it is underground),
/// except on plants picked for their leaves, which are the leaves.
#[test]
fn full_grown_shows_what_is_picked() {
    let mut bare = Vec::new();
    for def in Vegetables::built_in().iter() {
        let slot = layout::slot_metres(def);
        let plain = build(def, (STAGES, slot), (2, 0), false).fill.len();
        let shown = build(def, (STAGES, slot), (2, 0), true).fill.len();
        let mut rng = Seed::new(0).rng();
        let look = Look::new(def, STAGES, slot, &mut rng);
        // Picked for its leaves, it is its leaves (a rosette's head aside).
        let stalks = look.part_size.x > look.part_size.y * 2.5;
        let rosette = def.plant == PlantShape::LeafyRosette && !stalks;
        let leaves = look.bears == Bears::Heart && !rosette;
        if shown <= plain && !leaves && def.plant != PlantShape::AgaveRosette {
            bare.push(def.key.clone());
        }
    }
    assert!(bare.is_empty(), "{bare:?}");
}

/// Young plants are smaller than grown ones, and a cell of small plants
/// stays near its cell.
#[test]
fn plants_grow_and_stay_in_their_cell() {
    let mut wrong = Vec::new();
    for def in Vegetables::built_in().iter() {
        let young = build_cell(def, 1, 0);
        let grown = build_cell(def, STAGES, 0);
        if top(&young) >= top(&grown) {
            wrong.push(format!(
                "{}: young {} of {}",
                def.key,
                top(&young),
                top(&grown)
            ));
        }
        if layout::per_side(def) > 1 {
            let out = (grown.fill.positions.iter())
                .map(|p| p[0].abs().max(p[2].abs()))
                .fold(0.0, f32::max);
            if out > CELL_SIZE * 0.85 {
                wrong.push(format!("{}: out {out}", def.key));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
