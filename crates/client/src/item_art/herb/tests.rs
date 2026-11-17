//! Every herb's picture, in every variant: something is drawn, inside the
//! picture with room for its black edge, and its colours read from the herb.

use bevy::mesh::VertexAttributeValues;
use bevy::prelude::{Assets, Mesh, StandardMaterial, Vec3};
use shared::herb::{Herbs, Part as Used};

use super::colour::said_colour;
use super::*;

/// How far from the middle a model may reach: the picture shows 2.5 across,
/// less the black edge and a little margin.
const REACH: f32 = 1.05;

#[test]
fn every_herb_is_drawn_inside_its_picture() {
    let herbs = Herbs::built_in();
    for def in herbs.iter() {
        for variant in 0..HERB_VARIANTS {
            let mut meshes = Assets::<Mesh>::default();
            let mut materials = Assets::<StandardMaterial>::default();
            let look = HerbLook {
                herb: def.key.clone(),
                variant,
            };
            let parts = herb_parts(&look, &herbs, &mut meshes, &mut materials);
            assert!(!parts.is_empty(), "{} draws nothing", def.key);
            for part in &parts {
                let mesh = meshes.get(&part.mesh).expect("its mesh");
                let Some(VertexAttributeValues::Float32x3(points)) =
                    mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                else {
                    panic!("{}: a mesh without points", def.key);
                };
                for &point in points {
                    let at = part.transform.transform_point(Vec3::from(point));
                    let far = at.x.abs().max(at.z.abs());
                    assert!(
                        far <= REACH,
                        "{} (variant {variant}) reaches {far}",
                        def.key
                    );
                }
            }
        }
    }
}

#[test]
fn a_herb_the_table_lacks_draws_nothing() {
    let mut meshes = Assets::<Mesh>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    let look = HerbLook {
        herb: "no_such_herb".into(),
        variant: 0,
    };
    assert!(herb_parts(&look, &Herbs::built_in(), &mut meshes, &mut materials).is_empty());
}

#[test]
fn descriptions_colour_berries_and_roots() {
    let black = said_colour("Shrub with glossy black berries in autumn.", Used::Berries);
    assert_eq!(black, Some((0.12, 0.08, 0.12)));
    let red = said_colour("A red root, thick and twisted.", Used::Root);
    assert_eq!(red, Some((0.72, 0.1, 0.1)));
    assert_eq!(
        said_colour("Yellow flowers; berries later.", Used::Berries),
        None
    );
    assert_eq!(
        said_colour("Red berries.", Used::Leaves),
        None,
        "only for those parts"
    );
}

#[test]
fn stacks_variants_come_from_the_look_seed() {
    let form = |look| shared::herb::HerbForm {
        herb: "roseroot".into(),
        look,
        grams: 60,
    };
    assert_eq!(HerbLook::of(&form(3)).variant, 0);
    assert_eq!(HerbLook::of(&form(4)).variant, 1);
    assert_eq!(HerbLook::of(&form(5)).herb, "roseroot");
}
