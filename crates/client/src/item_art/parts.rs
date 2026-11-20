//! The parts of a look's model, from whichever builder makes that kind of thing.

use bevy::prelude::*;

use super::{Look, Part};

pub(super) fn parts_of(
    look: Look,
    herbs: &shared::herb::Herbs,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    match look {
        Look::Gem { gem, quality } => super::gem::gem_parts(&gem, quality, meshes, materials)
            .into_iter()
            .map(|(mesh, material)| Part {
                mesh,
                material,
                transform: Transform::default(),
            })
            .collect(),
        Look::Ring { ring, quality } => super::ring::ring_parts(&ring, quality, meshes, materials),
        Look::Necklace { necklace, quality } => {
            super::necklace::necklace_parts(&necklace, quality, meshes, materials)
        }
        Look::Blade { blade, quality } => {
            super::blade::blade_parts(&blade, quality, meshes, materials)
        }
        Look::Body {
            layout,
            height,
            colours,
            hair,
        } => super::body::body_parts(&layout, height, colours, hair, meshes, materials),
        Look::Character {
            kind,
            worn,
            colours,
            hair,
            ..
        } => super::character::character_parts(kind, &worn, colours, hair, None, meshes, materials),
        Look::Portrait(character) => match *character {
            Look::Character {
                kind,
                worn,
                colours,
                hair,
                height,
            } => {
                let standing = Some(super::character::portrait_scale(kind, height));
                super::character::character_parts(
                    kind, &worn, colours, hair, standing, meshes, materials,
                )
            }
            _ => Vec::new(),
        },
        Look::Morph(morph) => super::body::morph_parts(&morph, meshes, materials),
        Look::Footwear { footwear, quality } => {
            super::footwear::footwear_parts(&footwear, quality, meshes, materials)
        }
        Look::Parcel => super::parcel::parcel_parts(meshes, materials),
        Look::Keepsake => super::parcel::keepsake_parts(meshes, materials),
        Look::Bag => super::bag::bag_parts(meshes, materials),
        Look::Pouch => super::pouch::pouch_parts(meshes, materials),
        Look::Key(key) => super::key::key_parts(&key, meshes, materials),
        Look::Keyring => super::keyring::keyring_parts(meshes, materials),
        Look::HerbPouch => super::herb_pouch::herb_pouch_parts(meshes, materials),
        Look::Food(kind) => super::food::food_parts(kind, meshes, materials),
        Look::Tool(kind) => super::tool::tool_parts(kind, meshes, materials),
        Look::Flour(flour) => super::flour::flour_parts(flour, meshes, materials),
        Look::Yeast => super::yeast::yeast_parts(meshes, materials),
        Look::Solid(shared::item::ItemKind::Sulphur) => {
            super::solids::sulphur_parts(meshes, materials)
        }
        Look::Solid(shared::item::ItemKind::Mercury) => {
            super::solids::mercury_parts(meshes, materials)
        }
        Look::Solid(_) => super::salt::salt_parts(meshes, materials),
        Look::Cask(cask) => super::cask::cask_parts(cask, meshes, materials),
        Look::Log(wood) => super::log::log_parts(wood, meshes, materials),
        Look::Kindling(wood) => super::log::kindling_parts(wood, meshes, materials),
        Look::Vessel(vessel) => super::vessel::vessel_parts(vessel, meshes, materials),
        Look::Bottle { bottle, liquid } => {
            super::bottle::bottle_parts(&bottle, liquid.as_ref(), meshes, materials)
        }
        Look::Ointment(salve) => super::ointment::ointment_parts(salve.as_ref(), meshes, materials),
        Look::Paste(paste) => super::paste::paste_parts(paste.as_ref(), meshes, materials),
        Look::Powder(powder) => super::powder::powder_parts(powder.as_ref(), meshes, materials),
        Look::Coin(metal) => super::coin::coin_parts(metal, meshes, materials),
        Look::Fruit(fruit) => super::fruit::fruit_parts(&fruit, meshes, materials),
        Look::Vegetable(vegetable) => {
            super::vegetable::vegetable_parts(&vegetable, meshes, materials)
        }
        Look::Fungus(fungus) => super::fungus::fungus_parts(&fungus, meshes, materials),
        Look::Herb(herb) => super::herb::herb_parts(&herb, herbs, meshes, materials),
        Look::Under { under, quality } => {
            super::under::under_parts(&under, quality, meshes, materials)
        }
        Look::Clothing { clothing, quality } => {
            super::clothing::clothing_parts(&clothing, quality, meshes, materials)
        }
        look => super::armour::armour_parts(look, meshes, materials),
    }
}
