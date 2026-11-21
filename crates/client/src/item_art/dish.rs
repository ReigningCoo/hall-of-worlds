//! Placeholder pictures for cooked food (cooking plan PR 6; PR 11 makes the
//! real models by form): a Dish as a pale bowl or plate with a mound of its
//! tint, anything else served (a loaf, a cake, ...) as a tinted lump; an
//! intermediate as a tinted lump on its own.

use bevy::prelude::*;
use shared::cooking::{DishForm, Ingredients};
use shared::food::tint;
use shared::herb::Rgb;
use shared::item::Item;

use super::Part;

/// What a placeholder shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Holder {
    Bowl,
    Plate,
    /// No dish under it: an intermediate, or food served as it is.
    None,
}

/// How a cooked food looks for now: what holds it and its tint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DishLook {
    pub holder: Holder,
    pub tint: Rgb,
    /// Its cells, as `FoodForm::shape` has them.
    pub cells: (u8, u8),
}

impl DishLook {
    /// A Dish's or an intermediate's look, or `None` for anything else.
    pub fn of(item: &Item) -> Option<Self> {
        let food = item.food()?;
        let [red, green, blue] = tint(&food.portions, &food.history, Ingredients::built_in());
        let holder = match (food.rest.is_some(), food.dish.form) {
            (true, _) => Holder::None,
            (false, DishForm::Bowl | DishForm::Pudding | DishForm::Sauce) => Holder::Bowl,
            (false, DishForm::Plate | DishForm::EggDish | DishForm::Dumplings) => Holder::Plate,
            (false, DishForm::Noodles) => Holder::Plate,
            _ => Holder::None,
        };
        let shape = food.shape();
        Some(Self {
            holder,
            tint: Rgb(red, green, blue),
            cells: (shape.width, shape.height),
        })
    }
}

/// The food's parts (`Look::Dish`), against the picture's `VIEW_WIDTH` of
/// 2.5; the camera looks down from above.
pub(super) fn dish_parts(
    look: DishLook,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let mut colour = |Rgb(red, green, blue): Rgb, roughness| {
        materials.add(StandardMaterial {
            base_color: Color::srgb_u8(red, green, blue),
            perceptual_roughness: roughness,
            reflectance: 0.2,
            ..default()
        })
    };
    let food = colour(look.tint, 0.8);
    let crockery = colour(Rgb(236, 232, 222), 0.4);
    let sphere = meshes.add(Sphere::new(1.0));
    let (wide, long) = (f32::from(look.cells.0), f32::from(look.cells.1));
    match look.holder {
        Holder::Bowl | Holder::Plate => {
            let bowl = look.holder == Holder::Bowl;
            let (height, mound) = if bowl { (0.45, 0.25) } else { (0.08, 0.2) };
            vec![
                Part {
                    mesh: meshes.add(Cylinder::new(1.0, height)),
                    material: crockery,
                    transform: Transform::from_xyz(0.0, -height / 2.0, 0.0),
                },
                Part {
                    mesh: sphere,
                    material: food,
                    transform: Transform::from_xyz(0.0, 0.0, 0.0)
                        .with_scale(Vec3::new(0.8, mound, 0.8)),
                },
            ]
        }
        Holder::None => vec![Part {
            mesh: sphere,
            material: food,
            transform: Transform::from_scale(Vec3::new(0.5 * wide, 0.35, 0.5 * long)),
        }],
    }
}
