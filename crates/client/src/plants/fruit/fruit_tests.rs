use std::collections::HashSet;

use super::*;
use shared::fruit::Fruits;

const STAGES: [FruitStage; 5] = [
    FruitStage::Young,
    FruitStage::Leafy,
    FruitStage::Flowering,
    FruitStage::Fruiting { ripe: false },
    FruitStage::Fruiting { ripe: true },
];

/// Every fruit's plant has a model at every stage in every variant, with
/// flowers or fruit added at the stages that have them.
#[test]
fn every_fruit_plant_is_built_at_every_stage() {
    for def in Fruits::built_in().iter() {
        for variant in 0..VARIANTS {
            let leafy = build(def, FruitStage::Leafy, variant).fill.len();
            assert!(leafy > 0, "{}", def.key);
            for stage in STAGES {
                let pieces = build(def, stage, variant);
                assert!(!pieces.outline.is_empty(), "{} {stage:?}", def.key);
                if stage.number() >= 3 {
                    assert!(pieces.fill.len() > leafy, "{} {stage:?}", def.key);
                }
            }
        }
    }
}

/// A plant keeps its shape from stage to stage: fruiting, it has every
/// corner it has when leafy, and more.
#[test]
fn a_plant_keeps_its_shape() {
    let corners = |pieces: Pieces| {
        let positions = pieces.fill.positions.into_iter();
        positions
            .map(|p| p.map(f32::to_bits))
            .collect::<HashSet<_>>()
    };
    for def in Fruits::built_in().iter() {
        let leafy = corners(build(def, FruitStage::Leafy, 1));
        let ripe = corners(build(def, FruitStage::Fruiting { ripe: true }, 1));
        assert!(leafy.is_subset(&ripe), "{}", def.key);
    }
}

/// Every model stays within the size clicks reach, and young ones are
/// smaller than grown ones.
#[test]
fn models_stay_within_their_size() {
    let mut wrong = Vec::new();
    for def in Fruits::built_in().iter() {
        let (height, reach) = fruit_plant_size(def);
        let size = |stage| {
            let pieces = build(def, stage, 2).fill;
            let top = pieces.positions.iter().map(|p| p[1]).fold(0.0, f32::max);
            let out = (pieces.positions.iter())
                .map(|p| Vec2::new(p[0], p[2]).length())
                .fold(0.0, f32::max);
            (top, out)
        };
        let (young, _) = size(FruitStage::Young);
        let (top, out) = size(FruitStage::Fruiting { ripe: true });
        if young >= top || top > height * 1.1 || out > reach * 1.3 {
            wrong.push(format!(
                "{}: young {young}, top {top} of {height}, out {out} of {reach}",
                def.key
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
