//! Every vegetable's picture, whole and (when cut) as a slice, in every
//! variant: something is drawn, inside the picture with room for its black
//! edge.

use bevy::mesh::VertexAttributeValues;
use bevy::prelude::{Assets, Mesh, StandardMaterial, Vec3};
use shared::vegetable::{VegetableForm, Vegetables};

use super::*;

/// How far from the middle a model may reach, against the picture's half size.
const REACH: f32 = 1.0;

fn look(key: &str, serving: bool, variant: u8) -> VegetableItemLook {
    let def = Vegetables::built_in().get(key).expect("a vegetable");
    let servings = u32::from(def.servings.max(1));
    VegetableItemLook::of(&VegetableForm {
        vegetable: key.to_owned(),
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
fn reach(look: &VegetableItemLook) -> Option<(f32, f32)> {
    let mut meshes = Assets::<Mesh>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    let parts = vegetable_parts(look, &mut meshes, &mut materials);
    if parts.is_empty() {
        return None;
    }
    let (mut x, mut z) = (0.0f32, 0.0f32);
    for part in &parts {
        let mesh = meshes.get(&part.mesh).expect("its mesh");
        let Some(VertexAttributeValues::Float32x3(points)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("{}: a mesh without points", look.vegetable);
        };
        for &point in points {
            let at = part.transform.transform_point(Vec3::from(point));
            assert!(at.is_finite(), "{}: a point at {at}", look.vegetable);
            x = x.max(at.x.abs());
            z = z.max(at.z.abs());
        }
    }
    let room = room(look.cells) / 0.98;
    Some((x / room.x, z / room.y))
}

#[test]
fn every_vegetable_is_drawn_inside_its_picture() {
    let mut outside = Vec::new();
    for def in Vegetables::built_in().iter() {
        let cut = def.servings > 1;
        for serving in [false, true].into_iter().filter(|&s| !s || cut) {
            for variant in 0..VEGETABLE_VARIANTS {
                let look = look(&def.key, serving, variant);
                let (x, z) = reach(&look).unwrap_or_else(|| panic!("{} draws nothing", def.key));
                if x > REACH + 1e-3 || z > REACH + 1e-3 {
                    outside.push(format!("{} {serving} {variant}: {x}, {z}", def.key));
                }
            }
        }
    }
    assert!(outside.is_empty(), "outside: {outside:#?}");
}

#[test]
fn a_vegetable_the_table_lacks_draws_nothing() {
    let look = VegetableItemLook {
        vegetable: "no such vegetable".to_owned(),
        serving: false,
        variant: 0,
        cells: (1, 1),
    };
    assert_eq!(reach(&look), None);
}

#[test]
fn most_variants_are_the_usual_colour_and_every_other_colour_shows() {
    for def in Vegetables::built_in().iter() {
        let colours = def.look.colours.len();
        let picked: Vec<usize> = (0..VEGETABLE_VARIANTS)
            .map(|v| Variety::of(v, colours).colour)
            .collect();
        let usual = picked.iter().filter(|&&c| c == 0).count();
        assert!(usual * 2 > picked.len(), "{}", def.key);
        for colour in 0..colours {
            assert!(
                picked.contains(&colour),
                "{} never shows colour {colour}",
                def.key
            );
        }
    }
}
