//! The popup's pictures, made here and named for markup (`picture:mark.icon.<key>`,
//! `picture:mark.colour.<key>`): each icon as a whole mark in the colour chosen,
//! painted again when it changes, and a disc of each colour.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use shared::map_mark::{MarkColour, MarkIcon};

use crate::common::Preferences;
use crate::dom::Pictures;
use crate::minimap::{mark_pixels, swatch_pixels};

/// How big the popup's pictures are: a button's picture is 32 across.
const SIZE: u32 = 32;

/// The icons' pictures, by `MarkIcon::ALL`'s order, and the colour they were painted in.
#[derive(Resource, Default)]
pub(super) struct MarkPictures {
    icons: Vec<Handle<Image>>,
    colour: Option<MarkColour>,
}

fn image(pixels: Vec<u8>) -> Image {
    Image::new(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

/// Makes and names every picture (Startup).
pub(super) fn make_pictures(
    mut images: ResMut<Assets<Image>>,
    mut pictures: ResMut<Pictures>,
    mut made: ResMut<MarkPictures>,
) {
    for colour in MarkColour::ALL {
        let handle = images.add(image(swatch_pixels(colour, SIZE)));
        pictures.name(format!("mark.colour.{}", colour.key()), handle);
    }
    made.icons = (MarkIcon::ALL.iter())
        .map(|icon| {
            let handle = images.add(image(vec![0; (SIZE * SIZE * 4) as usize]));
            pictures.name(format!("mark.icon.{}", icon.key()), handle.clone());
            handle
        })
        .collect();
}

/// Paints the icons again in the colour chosen, when it changes.
pub(super) fn recolour_icons(
    preferences: Res<Preferences>,
    mut made: ResMut<MarkPictures>,
    mut images: ResMut<Assets<Image>>,
) {
    let colour = preferences.mark_colour;
    if made.colour == Some(colour) {
        return;
    }
    made.colour = Some(colour);
    for (icon, handle) in MarkIcon::ALL.into_iter().zip(&made.icons) {
        if let Some(mut picture) = images.get_mut(handle) {
            picture.data = Some(mark_pixels((icon, colour), SIZE));
        }
    }
}
