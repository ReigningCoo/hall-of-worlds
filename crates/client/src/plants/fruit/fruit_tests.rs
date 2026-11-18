use super::*;
use shared::fruit::Fruits;

/// Every fruit's plant has a model at every stage, with flowers or fruit
/// added at the stages that have them.
#[test]
fn every_fruit_plant_is_built_at_every_stage() {
    let stages = [
        FruitStage::Young,
        FruitStage::Leafy,
        FruitStage::Flowering,
        FruitStage::Fruiting { ripe: false },
        FruitStage::Fruiting { ripe: true },
    ];
    for def in Fruits::built_in().items() {
        let leafy = build(def, FruitStage::Leafy).fill.len();
        assert!(leafy > 0, "{}", def.key);
        for stage in stages {
            let pieces = build(def, stage);
            assert!(!pieces.outline.is_empty(), "{} {stage:?}", def.key);
            if stage.number() >= 3 {
                assert!(pieces.fill.len() > leafy, "{} {stage:?}", def.key);
            }
        }
    }
}
