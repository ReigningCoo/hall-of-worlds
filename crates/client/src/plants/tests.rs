use bevy::prelude::*;
use shared::herb::{Herbs, STAGES};
use shared::rng::Seed;

use super::layout::patch_layout;
use super::plan::{METRE, Plan, plants_in_patch, shown_metres};
use super::shapes::build;

/// Every herb has a model at every stage, in every variant, with an outline
/// round each piece, and none reaches out of reason.
#[test]
fn every_herb_builds_at_every_stage() {
    for def in Herbs::built_in().iter() {
        for stage in 1..=STAGES {
            for variant in 0..4 {
                let mut rng = Seed::new(variant).rng();
                let count = plants_in_patch(&def.plant);
                let plan = Plan::new(&def.plant, stage, count, &mut rng);
                let pieces = build(def.plant.shape, &plan, &mut rng);
                assert!(!pieces.fill.is_empty(), "{} at stage {stage}", def.key);
                assert_eq!(pieces.fill.len(), pieces.outline.len(), "{}", def.key);
                for position in &pieces.fill.positions {
                    let at = Vec3::from_array(*position);
                    assert!(at.y < 3.0 * METRE, "{} grows too tall: {at}", def.key);
                    assert!(at.y > -0.5 * METRE, "{} goes underground: {at}", def.key);
                    let out = Vec2::new(at.x, at.z).length();
                    assert!(out < 1.5 * METRE, "{} spreads too far: {at}", def.key);
                }
            }
        }
    }
}

/// Plants grow: a later stage's model is no smaller than an earlier one's.
#[test]
fn later_stages_are_taller() {
    for def in Herbs::built_in().iter() {
        let tallest = |stage| {
            let mut rng = Seed::new(1).rng();
            let plan = Plan::new(&def.plant, stage, 1, &mut rng);
            let pieces = build(def.plant.shape, &plan, &mut rng);
            pieces
                .fill
                .positions
                .iter()
                .map(|p| p[1])
                .fold(0.0, f32::max)
        };
        assert!(tallest(1) < tallest(3), "{}", def.key);
    }
}

/// Small herbs are drawn bigger than life, tall ones smaller.
#[test]
fn sizes_are_exaggerated() {
    assert!(shown_metres(10) > 0.3);
    assert!(shown_metres(400) < 4.0);
    assert!(shown_metres(50) < shown_metres(100));
}

/// A patch's plants all stand inside its cell, and the same seed lays them
/// out the same.
#[test]
fn patches_stay_in_their_cell() {
    let cell = crate::dungeon_map::CELL_SIZE;
    for def in Herbs::built_in().iter() {
        for seed in 0..20 {
            let layout = patch_layout(&def.plant, seed);
            assert_eq!(layout, patch_layout(&def.plant, seed));
            assert_eq!(layout.len(), plants_in_patch(&def.plant));
            for spot in layout {
                assert!(spot.offset.abs().max_element() < cell / 2.0);
            }
        }
    }
}
