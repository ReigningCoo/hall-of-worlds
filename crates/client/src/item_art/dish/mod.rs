//! Cooked food's models (cooking plan PR 11): a Dish by what it is served
//! as, an intermediate by its kind, in its foods' tint with a second colour
//! for pieces, toppings and fillings. Black outlines come from the
//! pictures' edge, as every item's do.

mod baked;
mod kit;
mod look;
mod pieces;
mod rest;
mod served;
mod shaped;

use bevy::prelude::*;
use shared::cooking::DishForm;

use super::Part;
use kit::Kit;
pub use look::{DishLook, Model};

/// How far each model is tipped towards the camera, so its sides show:
/// tall ones more.
fn tip(model: Model) -> f32 {
    match model {
        Model::Dish(DishForm::Jar | DishForm::Pudding | DishForm::Cake) => -0.45,
        Model::Dish(
            DishForm::Loaf | DishForm::Flatbread | DishForm::Strips | DishForm::Sausage,
        ) => 0.0,
        _ => -0.3,
    }
}

/// The food's parts (`Look::Dish`), against the picture's `VIEW_WIDTH` of
/// 2.5; the camera looks down from above.
pub(super) fn dish_parts(
    look: DishLook,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let mut kit = Kit::new(&look, meshes, materials);
    match look.model {
        Model::Rest(kind) => rest::rest(&mut kit, kind),
        Model::Dish(form) => match form {
            DishForm::Bowl => served::bowl(&mut kit),
            DishForm::Plate => served::plate(&mut kit),
            DishForm::Sauce => served::sauce(&mut kit),
            DishForm::Pudding => served::pudding(&mut kit),
            DishForm::Jar => served::jar(&mut kit),
            DishForm::Paste => served::paste(&mut kit),
            DishForm::Loaf => baked::loaf(&mut kit),
            DishForm::Flatbread => baked::flatbread(&mut kit),
            DishForm::Pie => baked::pie(&mut kit),
            DishForm::Cake => baked::cake(&mut kit),
            DishForm::Noodles => shaped::noodles(&mut kit),
            DishForm::Dumplings => shaped::dumplings(&mut kit),
            DishForm::EggDish => shaped::egg_dish(&mut kit),
            DishForm::Ball => pieces::balls(&mut kit),
            DishForm::Block => pieces::block(&mut kit),
            DishForm::Pat => pieces::pat(&mut kit),
            DishForm::Sausage => pieces::sausages(&mut kit),
            DishForm::Strips => pieces::strips(&mut kit),
            DishForm::Other => pieces::other(&mut kit),
        },
    }
    let size = match look.model {
        Model::Dish(DishForm::Jar) => 1.3,
        _ => 1.0,
    };
    kit.finish(tip(look.model), size)
}

#[cfg(test)]
mod tests;
