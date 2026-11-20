//! What bags are drawn with: meshes, the weave picture, and sparkles' sizes.

use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, Face, TextureDimension, TextureFormat};
use shared::cask::{CaskLook, CaskSize, WOODS};
use shared::ground::Load;
use std::collections::BTreeMap;

use super::bag::{Model, size_of};
use super::{
    BagAssets, BagMeshes, LOWEST_SPARKLE, SPARKLE, SPARKLE_SCALE, STAR_EDGE, STAR_HOVER_OUTLINE,
    STAR_LONG, STAR_OUTLINE, STAR_SHORT, WEAVE_PIXELS,
};
use crate::view3d::{XRAY_WHITE, XrayMaterial};

/// How big a look's sparkle is drawn.
pub(super) fn sparkle_scale(load: Load) -> f32 {
    SPARKLE_SCALE * size_of(load).scale
}

/// How far up a look's sparkle stands.
pub(super) fn sparkle_up(load: Load) -> f32 {
    let size = size_of(load);
    (size.height * 0.85).max(LOWEST_SPARKLE * size.scale)
}

pub(super) fn to_mesh(model: Model) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, model.positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, model.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, model.uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, model.colours)
    .with_inserted_indices(Indices::U32(model.indices))
}

/// Canvas: threads over and under, near enough white for its colour to come from the
/// mesh, drawn here so there is no file to ship for it.
pub(super) fn weave_picture() -> Image {
    let mut bytes = Vec::with_capacity(WEAVE_PIXELS * WEAVE_PIXELS * 4);
    for y in 0..WEAVE_PIXELS {
        for x in 0..WEAVE_PIXELS {
            let over = (x / 2 + y / 2).is_multiple_of(2);
            let edge = if over { x % 2 == 0 } else { y % 2 == 0 };
            let shade = match (over, edge) {
                (true, false) => 255,
                (false, false) => 228,
                (_, true) => 200,
            };
            bytes.extend([shade, shade, shade, 255]);
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: WEAVE_PIXELS as u32,
            height: WEAVE_PIXELS as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        bytes,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    });
    image
}

/// A look's meshes: lit and remembered, its outlines and its sparkle's.
fn meshes_of(
    load: Load,
    model: Model,
    for_outline: &Model,
    meshes: &mut Assets<Mesh>,
) -> BagMeshes {
    let scale = sparkle_scale(load);
    let star_outline = super::bag::star(STAR_LONG, STAR_SHORT, STAR_OUTLINE / scale);
    let star_hover_outline = super::bag::star(STAR_LONG, STAR_SHORT, STAR_HOVER_OUTLINE / scale);
    BagMeshes {
        remembered: meshes.add(to_mesh(model.sepia_copy())),
        outline: meshes.add(to_mesh(super::bag::outline(for_outline))),
        hover_outline: meshes.add(to_mesh(super::bag::hover_outline(for_outline))),
        lit: meshes.add(to_mesh(model)),
        star_outline: meshes.add(to_mesh(star_outline)),
        star_hover_outline: meshes.add(to_mesh(star_hover_outline)),
    }
}

/// Makes a log pile's meshes the first time one is seen: there are too many
/// woods to make them all at the start.
pub(super) fn ensure_meshes(assets: &mut BagAssets, meshes: &mut Assets<Mesh>, load: Load) {
    let Load::Logs { wood, count } = load else {
        return;
    };
    if assets.bags.contains_key(&(load, 0)) {
        return;
    }
    let (lit, for_outline) = super::logs::log_models(wood, count);
    let made = meshes_of(load, lit, &for_outline, meshes);
    assets.bags.insert((load, 0), made);
}

pub(super) fn make_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut xray: ResMut<Assets<XrayMaterial>>,
) {
    let mut bags = BTreeMap::new();
    let casks = CaskSize::ALL.into_iter().flat_map(|size| {
        WOODS.map(|wood| {
            let (lit, for_outline) = super::cask::cask_models(CaskLook { size, wood });
            (Load::Cask(CaskLook { size, wood }), 0, lit, for_outline)
        })
    });
    let looks = Load::ALL.into_iter().flat_map(|load| {
        (0..super::bag::VARIETIES).map(move |variety| {
            let model = super::bag::bag(load, variety);
            let for_outline = model.clone();
            (load, variety, model, for_outline)
        })
    });
    for (load, variety, model, for_outline) in looks.chain(casks) {
        let made = meshes_of(load, model, &for_outline, &mut meshes);
        bags.insert((load, variety), made);
    }
    let weave = images.add(weave_picture());
    commands.insert_resource(BagAssets {
        bags,
        star: meshes.add(to_mesh(super::bag::star(STAR_LONG, STAR_SHORT, 0.0))),
        lit: materials.add(StandardMaterial {
            base_color_texture: Some(weave.clone()),
            perceptual_roughness: 0.95,
            ..default()
        }),
        remembered: materials.add(StandardMaterial {
            base_color_texture: Some(weave),
            unlit: true,
            ..default()
        }),
        cask_lit: materials.add(StandardMaterial {
            perceptual_roughness: 0.75,
            ..default()
        }),
        log_lit: materials.add(StandardMaterial {
            perceptual_roughness: 0.95,
            reflectance: 0.1,
            ..default()
        }),
        cask_remembered: materials.add(StandardMaterial {
            unlit: true,
            ..default()
        }),
        sparkle: materials.add(StandardMaterial {
            base_color: SPARKLE,
            unlit: true,
            ..default()
        }),
        // Drawn from inside, so only what stands out past the bag shows.
        outline: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            unlit: true,
            cull_mode: Some(Face::Front),
            ..default()
        }),
        star_edge: materials.add(StandardMaterial {
            base_color: STAR_EDGE,
            unlit: true,
            cull_mode: None,
            ..default()
        }),
        xray_fill: xray.add(XrayMaterial::fill(XRAY_WHITE)),
        xray_outline: xray.add(XrayMaterial::outline(XRAY_WHITE)),
    });
}
