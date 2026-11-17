//! Every fruit's picture, whole and as a slice, in every variant: something is
//! drawn, inside the picture with room for its black edge.

use bevy::mesh::VertexAttributeValues;
use bevy::prelude::{Assets, Mesh, StandardMaterial, Vec3};
use shared::fruit::{FruitForm, Fruits};

use super::*;

/// How far from the middle a model may reach in a square picture: it shows
/// 2.5 across, less the black edge and a little margin.
const REACH: f32 = 1.12;

fn look(key: &str, serving: bool, variant: u8) -> FruitItemLook {
    let def = Fruits::built_in().get(key).expect("a fruit");
    let servings = u32::from(def.servings.max(1));
    FruitItemLook::of(&FruitForm {
        fruit: key.to_owned(),
        serving,
        look: u64::from(variant),
        grams: if serving {
            def.grams.div_ceil(servings)
        } else {
            def.grams
        },
    })
}

/// How far the parts reach along x and z, against the picture's half sizes.
fn reach(look: &FruitItemLook) -> Option<(f32, f32)> {
    let mut meshes = Assets::<Mesh>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    let parts = fruit_parts(look, &mut meshes, &mut materials);
    if parts.is_empty() {
        return None;
    }
    let (mut x, mut z) = (0.0f32, 0.0f32);
    for part in &parts {
        let mesh = meshes.get(&part.mesh).expect("its mesh");
        let Some(VertexAttributeValues::Float32x3(points)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("{}: a mesh without points", look.fruit);
        };
        for &point in points {
            let at = part.transform.transform_point(Vec3::from(point));
            x = x.max(at.x.abs());
            z = z.max(at.z.abs());
        }
    }
    let room = room(look.cells) / 0.98;
    Some((x / room.x, z / room.y))
}

#[test]
fn every_fruit_is_drawn_inside_its_picture() {
    let mut outside = Vec::new();
    for def in Fruits::built_in().items() {
        for serving in [false, true] {
            let mut most = (0.0f32, 0.0f32);
            for variant in 0..FRUIT_VARIANTS {
                let look = look(&def.key, serving, variant);
                let (x, z) = reach(&look).unwrap_or_else(|| panic!("{} draws nothing", def.key));
                most = (most.0.max(x), most.1.max(z));
            }
            if most.0 > REACH || most.1 > REACH {
                outside.push(format!("{} (serving {serving}) reaches {most:?}", def.key));
            }
        }
    }
    assert!(
        outside.is_empty(),
        "outside their pictures:\n{}",
        outside.join("\n")
    );
}

#[test]
fn a_fruit_the_table_lacks_draws_nothing() {
    let look = FruitItemLook {
        fruit: "no such fruit".to_owned(),
        serving: false,
        variant: 0,
        cells: (1, 1),
    };
    assert_eq!(reach(&look), None);
}

#[test]
fn a_stack_shows_one_of_the_variants_by_its_look_seed() {
    let form = FruitForm {
        fruit: "apple".to_owned(),
        serving: false,
        look: 37,
        grams: 150,
    };
    assert_eq!(FruitItemLook::of(&form).variant, 37 % FRUIT_VARIANTS);
}
