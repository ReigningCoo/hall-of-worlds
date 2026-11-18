//! Vegetable plants in farms' fields and the wilderness: what one is called on
//! hover, by how it has grown by the world's clock (`VegetablePlant::stage`).
//! They are drawn as crops (`field::build_crops`) until they have models of
//! their own; picking them by click or tap is `herbs::pick`'s, as for herbs.

use shared::dungeon::Level;
use shared::herb::STAGES;
use shared::vegetable::Vegetables;

/// What a vegetable plant is called on hover: its name and how it is growing
/// ("Turnip, growing"; "Turnip, ready to pick").
pub(super) fn vegetable_name(level: &Level, now: u64, (x, y): (i64, i64)) -> Option<String> {
    let plant = level.vegetable_at(x, y)?;
    let def = Vegetables::built_in().get(&plant.vegetable)?;
    let how = match plant.stage(def, now) {
        0 => "dug up",
        1 => "sprouting",
        stage if stage < STAGES => "growing",
        _ if plant.can_pick(def, now) => "ready to pick",
        _ => "growing back",
    };
    Some(format!("{}, {how}", def.name))
}
