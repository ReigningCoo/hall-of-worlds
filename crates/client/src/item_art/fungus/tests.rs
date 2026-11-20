//! Every fungus's picture, whole and (when cut) as a slice, in every variant:
//! something is drawn, finite, inside the picture with room for its black
//! edge; and one standing fungus has pieces for every kind.

use bevy::mesh::VertexAttributeValues;
use bevy::prelude::{Assets, Mesh, StandardMaterial, Vec3};
use shared::fungus::{Fungi, FungusForm};
use shared::rng::Seed;

use super::model::fungus_one;
use super::*;

/// How far from the middle a model may reach, against the picture's half size.
const REACH: f32 = 1.0;

fn look(key: &str, serving: bool, variant: u8) -> FungusItemLook {
    let def = Fungi::built_in().get(key).expect("a fungus");
    let form = match serving {
        true => FungusForm::serving(def, Seed::new(0)),
        false => FungusForm::new(def, Seed::new(0)),
    };
    FungusItemLook::of(&FungusForm {
        look: u64::from(variant),
        ..form
    })
}

/// How far the parts reach along x and z, against the picture's half sizes.
fn reach(look: &FungusItemLook) -> Option<(f32, f32)> {
    let mut meshes = Assets::<Mesh>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    let parts = fungus_parts(look, &mut meshes, &mut materials);
    if parts.is_empty() {
        return None;
    }
    let (mut x, mut z) = (0.0f32, 0.0f32);
    for part in &parts {
        let mesh = meshes.get(&part.mesh).expect("its mesh");
        let Some(VertexAttributeValues::Float32x3(points)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("{}: a mesh without points", look.fungus);
        };
        for &point in points {
            let at = part.transform.transform_point(Vec3::from(point));
            assert!(at.is_finite(), "{}: a point at {at}", look.fungus);
            x = x.max(at.x.abs());
            z = z.max(at.z.abs());
        }
    }
    let room = room(look.cells) / 0.98;
    Some((x / room.x, z / room.y))
}

#[test]
fn every_fungus_is_drawn_inside_its_picture() {
    let mut outside = Vec::new();
    for def in Fungi::built_in().iter() {
        let cut = FungusForm::new(def, Seed::new(0)).is_whole(def);
        for serving in [false, true].into_iter().filter(|&s| !s || cut) {
            for variant in 0..FUNGUS_VARIANTS {
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
fn a_fungus_the_table_lacks_draws_nothing() {
    let look = FungusItemLook {
        fungus: "no such fungus".to_owned(),
        serving: false,
        variant: 0,
        cells: (1, 1),
    };
    assert_eq!(reach(&look), None);
}

#[test]
fn every_fungus_stands_with_its_vertex_colours() {
    for def in Fungi::built_in().iter() {
        let pieces = fungus_one(def, 0);
        assert!(!pieces.is_empty(), "{}", def.key);
        for piece in &pieces {
            assert!(
                piece.mesh.attribute(Mesh::ATTRIBUTE_COLOR).is_some(),
                "{}: a piece without colours",
                def.key
            );
        }
    }
}
