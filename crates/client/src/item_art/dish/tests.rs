use bevy::mesh::VertexAttributeValues;
use shared::food::Rest;
use shared::herb::Rgb;

use super::*;
use crate::item_art::{CAMERA_AT, CAMERA_UP, OUTLINE_PIXELS, VIEW_WIDTH};

const FORMS: [DishForm; 19] = [
    DishForm::Bowl,
    DishForm::Plate,
    DishForm::Jar,
    DishForm::Pudding,
    DishForm::Sauce,
    DishForm::EggDish,
    DishForm::Flatbread,
    DishForm::Noodles,
    DishForm::Dumplings,
    DishForm::Cake,
    DishForm::Pie,
    DishForm::Paste,
    DishForm::Loaf,
    DishForm::Ball,
    DishForm::Block,
    DishForm::Strips,
    DishForm::Sausage,
    DishForm::Pat,
    DishForm::Other,
];

/// Every model there is.
fn models() -> impl Iterator<Item = Model> {
    (FORMS.into_iter().map(Model::Dish)).chain(Rest::ALL.into_iter().map(Model::Rest))
}

/// A picture's pixels a cell, as the tools' tests measure their edge by.
const PIXELS_PER_CELL: f32 = 64.0;

/// How far past its picture a model reaches, across and down, leaving room
/// for its edge: at most 0 each way when it fits.
fn overreach(model: Model) -> (f32, f32) {
    let look = DishLook::sample(model, Rgb(200, 150, 90), Rgb(90, 140, 60));
    let mut meshes = Assets::<Mesh>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    let parts = dish_parts(look, &mut meshes, &mut materials);
    assert!(!parts.is_empty(), "{model:?}");
    let view = Transform::from_translation(CAMERA_AT).looking_at(Vec3::ZERO, CAMERA_UP);
    let (wide, long) = (f32::from(look.cells.0), f32::from(look.cells.1));
    let per_cell = VIEW_WIDTH / wide.max(long);
    let edge = OUTLINE_PIXELS / PIXELS_PER_CELL * per_cell;
    let (mut across, mut down) = (f32::MIN, f32::MIN);
    for part in &parts {
        let mesh = meshes.get(&part.mesh).expect("its mesh");
        let Some(VertexAttributeValues::Float32x3(points)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("{model:?}: a mesh without positions");
        };
        for &point in points {
            let at = part.transform.transform_point(Vec3::from(point));
            across = across.max(at.dot(*view.right()).abs() + edge - wide * per_cell / 2.0);
            down = down.max(at.dot(*view.up()).abs() + edge - long * per_cell / 2.0);
        }
    }
    (across, down)
}

#[test]
fn every_model_stays_inside_its_picture_with_room_for_its_edge() {
    let outside: Vec<String> = models()
        .map(|model| (model, overreach(model)))
        .filter(|(_, (across, down))| *across > 0.001 || *down > 0.001)
        .map(|(model, (across, down))| format!("{model:?}: {across:.3} across, {down:.3} down"))
        .collect();
    assert!(outside.is_empty(), "past their pictures: {outside:?}");
}

#[test]
fn every_model_fills_most_of_its_picture() {
    // None left small in the middle of its picture.
    let small: Vec<String> = models()
        .map(|model| (model, overreach(model)))
        .filter(|(_, (across, down))| across.max(*down) < -0.35)
        .map(|(model, (across, down))| format!("{model:?}: {across:.3} across, {down:.3} down"))
        .collect();
    assert!(small.is_empty(), "too small: {small:?}");
}

#[test]
fn the_sample_cells_match_the_items() {
    for form in FORMS {
        let mut food = shared::food::dev_food(
            shared::item::ItemKind::Dish,
            shared::item::Quality::Common,
            shared::rng::Seed::new(1),
        )
        .expect("a sample dish");
        food.dish.form = form;
        let shape = food.shape();
        let sample = DishLook::sample(Model::Dish(form), Rgb(0, 0, 0), Rgb(0, 0, 0));
        assert_eq!(sample.cells, (shape.width, shape.height), "{form:?}");
    }
}
