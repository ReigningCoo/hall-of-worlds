//! How a map mark lies on the world map (owner): a flat disc just above the
//! ground at its point, like a poi's base, painted just as the mark is on the
//! maps (`minimap::mark_pixels`: a black disc with a thin edge and the icon in
//! the mark's colour) and lit by the suns and moons like the land, so it feels
//! part of the world. It turns only with the camera, so its icon stays upright
//! on screen. Its black disc is its own outline (the models' black outline
//! rule).

use std::collections::HashMap;
use std::f32::consts::FRAC_PI_2;

use bevy::asset::RenderAssetUsages;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use shared::map_mark::{MarkColour, MarkIcon};

use super::marks::MarkToken;
use crate::minimap::mark_pixels;
use crate::view3d::{OrbitCamera, TOKEN_HALF_THICKNESS, TokenView, ViewCamera};

/// How far across a mark's disc is, in grid points: three quarters of a
/// poi's token, as wide as a Small creature's base, whose circle takes its
/// clicks.
const DISC: f32 = 12.0;

/// How far above the ground the disc lies: a little above a poi's base
/// (`pois::BASE_LIFT`), so a mark on one shows over it.
const LIFT: f32 = 0.3;

/// The disc's colour, above white: undoes the sun's dimming of flat ground, as
/// a poi's base's `BASE_WHITE` does, so its colours show as on the maps by day.
const LIT_WHITE: f32 = 1.41;

/// How many pixels across a disc's picture is.
const PIXELS: u32 = 64;

/// The disc on a mark's token view, turned with the camera.
#[derive(Component)]
pub(super) struct MarkDisc;

/// The disc's one mesh and a material per look, each made once.
#[derive(Resource, Default)]
pub(super) struct MarkDiscs {
    mesh: Option<Handle<Mesh>>,
    materials: HashMap<(MarkIcon, MarkColour), Handle<StandardMaterial>>,
}

/// Gives each new mark's token view its disc, as a child so it goes where the
/// view goes and is hidden with it (outside the window).
pub(super) fn add_discs(
    mut commands: Commands,
    mut store: ResMut<MarkDiscs>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    views: Query<(Entity, &TokenView), Added<TokenView>>,
    marks: Query<&MarkToken>,
) {
    for (view, token_view) in &views {
        let Ok(mark) = marks.get(token_view.owner) else {
            continue;
        };
        let mesh = (store.mesh)
            .get_or_insert_with(|| meshes.add(Rectangle::new(DISC, DISC)))
            .clone();
        let material = (store.materials.entry(mark.look))
            .or_insert_with(|| {
                let picture = images.add(picture(mark.look));
                materials.add(StandardMaterial {
                    base_color: Color::linear_rgb(LIT_WHITE, LIT_WHITE, LIT_WHITE),
                    base_color_texture: Some(picture),
                    perceptual_roughness: 0.9,
                    // No shine, or its black looks grey.
                    reflectance: 0.0,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })
            })
            .clone();
        commands.entity(view).with_child((
            MarkDisc,
            Mesh3d(mesh),
            MeshMaterial3d(material),
            NotShadowCaster,
            // The view stands half a token's thickness above the ground.
            Transform::from_xyz(0.0, LIFT - TOKEN_HALF_THICKNESS, 0.0),
        ));
    }
}

/// A mark's picture, as on the maps.
fn picture(look: (MarkIcon, MarkColour)) -> Image {
    Image::new(
        Extent3d {
            width: PIXELS,
            height: PIXELS,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        mark_pixels(look, PIXELS),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

/// Every frame, after tokens are placed: lays each disc flat, face up, turned
/// as far as the camera is (as gates are, `gate_view`), so its icon's top is
/// always away from the viewer.
pub(super) fn turn_with_camera(
    camera: Single<&OrbitCamera, With<ViewCamera>>,
    mut discs: Query<&mut Transform, With<MarkDisc>>,
) {
    // The quad faces +Z with its picture's top along +Y: laid back, its top
    // points north, then turned with the view.
    let rotation = Quat::from_rotation_y(camera.yaw) * Quat::from_rotation_x(-FRAC_PI_2);
    for mut transform in &mut discs {
        if transform.rotation != rotation {
            transform.rotation = rotation;
        }
    }
}
