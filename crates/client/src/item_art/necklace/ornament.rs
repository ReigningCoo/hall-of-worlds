//! What hangs from the strand, in its metal: a pendant's setting on a bail, an
//! amulet's medallion with its gems, stations set along the front, or a charm; and
//! where its gems are (`GemAt`), drawn as rings' in a picture and flat when worn.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::item::Quality;
use shared::item_form::{GemForm, Medallion, Mount, Ornament};

use super::charm::charm;
use super::path::Path;
use super::shapes::{Place, band, ellipse, slab, tube};
use super::{NecklaceLook, Wear};
use crate::item_art::armour::{Model, Stuff, face};
use crate::item_art::body::Triangle;
use crate::item_art::gem::PAVILION_DEPTH;
use crate::item_art::look::SettingLook;

/// How wide a setting of the necklace's own size is, and each size up.
const SETTING: f32 = 8.0;
const SIZE_STEP: f32 = 1.2;
/// How much bigger ornaments are drawn: in a picture, so they aren't lost beside
/// the loop, and worn, to show on a token.
const PICTURE_GROW: f32 = 2.0;
const WORN_GROW: f32 = 1.5;
/// A medallion's half sizes.
const MEDALLION: f32 = 15.0;
/// How thick plates are as made, and worn; and how far a worn one is off the skin.
const PLATE: f32 = 1.6;
const WORN_PLATE: f32 = 0.6;
const WORN_GAP: f32 = 0.5;
/// How high a worn gem stands.
const WORN_GEM: f32 = 0.55;
/// The bail's ring: how far round and how thick.
pub(super) const BAIL: f32 = 2.4;

/// A gem in place: where its girdle's middle is, the way its table faces and how wide.
pub(in crate::item_art) struct GemAt {
    pub gem: GemForm,
    pub quality: Option<Quality>,
    pub at: Vec3,
    pub out: Vec3,
    pub width: f32,
    /// Worn: a low eight-sided stone lying over the skin.
    pub stone: Vec<Triangle>,
}

impl GemAt {
    pub fn turn(&mut self, turn: Quat) {
        self.at = turn * self.at;
        self.out = turn * self.out;
    }
}

/// A low eight-sided stone on the plane at `at`, `half` across, its rim at `h`.
fn stone(place: &Place, at: Vec2, half: f32, h: f32) -> Vec<Triangle> {
    let top = place.at(at, h + WORN_GEM);
    let out = place.out(at);
    let rim: Vec<Vec3> = (0..8)
        .map(|step| {
            let angle = step as f32 / 8.0 * TAU;
            place.at(at + Vec2::new(angle.cos(), angle.sin()) * half, h)
        })
        .collect();
    let mut tris = Vec::new();
    for (index, &a) in rim.iter().enumerate() {
        face(&mut tris, top, a, rim[(index + 1) % 8], out);
    }
    tris
}

/// Heights off the skin of a plate, and of what stands on it.
pub(super) struct Heights {
    pub plate: (f32, f32),
    /// A raised rim or bezel, as high again as this.
    pub rise: f32,
}

impl Heights {
    fn of(wear: Wear) -> Self {
        match wear {
            Wear::Alone => Self {
                plate: (0.0, PLATE),
                rise: 1.2,
            },
            Wear::Worn => Self {
                plate: (WORN_GAP, WORN_GAP + WORN_PLATE),
                rise: 0.3,
            },
        }
    }
}

/// The ornament, and the gems it holds.
pub(super) fn hang(model: &mut Model, look: &NecklaceLook, path: &Path) -> Vec<GemAt> {
    let mut gems = Vec::new();
    let heights = Heights::of(path.wear);
    let scale = match path.wear {
        Wear::Alone => PICTURE_GROW,
        Wear::Worn => WORN_GROW,
    };
    let (point, out) = path.front();
    // Hung from the strand's underside.
    let from = point - out * path.off - Vec3::Y * path.half_wide;
    let place = Place::Hung {
        skin: &path.skin,
        from,
        scale,
        // Worn, it lies on the skin all the way, under whatever is over it.
        free: path.wear == Wear::Alone,
    };
    match look.ornament {
        Ornament::None => {}
        Ornament::Pendant => {
            bail(model, &place, &heights);
            if let Some(setting) = look.settings.first() {
                let width = SETTING * SIZE_STEP.powi(setting.bigger as i32);
                let centre = Vec2::new(0.0, BAIL * 2.0 + width * 0.6);
                set(model, &mut gems, &place, &heights, setting, centre, width);
            }
        }
        Ornament::Amulet(medallion) => {
            bail(model, &place, &heights);
            amulet(model, &mut gems, &place, &heights, look, medallion);
        }
        Ornament::Charm(shape) => {
            bail(model, &place, &heights);
            charm(model, &place, &heights, shape, Vec2::Y * BAIL * 2.0);
        }
        Ornament::Stations => stations(model, &mut gems, look, path, &heights),
    }
    gems
}

/// The small ring the ornament hangs by.
fn bail(model: &mut Model, place: &Place, heights: &Heights) {
    let ring = ellipse(Vec2::Y * BAIL, Vec2::splat(BAIL * 0.7), 10);
    band(
        model.of(Stuff::Main),
        place,
        &ring,
        |_| BAIL * 0.3,
        heights.plate,
        true,
    );
}

/// A medallion, its rim raised, with one gem in the middle or several round it.
fn amulet(
    model: &mut Model,
    gems: &mut Vec<GemAt>,
    place: &Place,
    heights: &Heights,
    look: &NecklaceLook,
    medallion: Medallion,
) {
    let half = match medallion {
        Medallion::Oval => Vec2::new(MEDALLION * 0.8, MEDALLION * 1.15),
        _ => Vec2::splat(MEDALLION),
    };
    let steps = match medallion {
        Medallion::Octagon => 8,
        _ => 32,
    };
    let centre = Vec2::new(0.0, BAIL * 2.0 + half.y);
    let mut outline = ellipse(centre, half, steps);
    if medallion == Medallion::Octagon {
        // Flat at the top, not a point.
        outline = ellipse(centre, half, 16)
            .into_iter()
            .skip(1)
            .step_by(2)
            .collect();
    }
    slab(model.of(Stuff::Main), place, &outline, heights.plate);
    let rim = ellipse(centre, half * 0.88, steps.max(16));
    let raised = (heights.plate.1, heights.plate.1 + heights.rise);
    band(model.of(Stuff::Shade), place, &rim, |_| 0.8, raised, true);
    let count = look.settings.len();
    for setting in &look.settings {
        let width = SETTING * SIZE_STEP.powi(setting.bigger as i32);
        let at = if count == 1 {
            centre
        } else {
            let angle = setting.angle as f32 / 256.0 * TAU;
            centre + Vec2::new(angle.sin(), -angle.cos()) * half * 0.58
        };
        set(model, gems, place, heights, setting, at, width);
    }
}

/// Small settings along the front of the strand, each at its angle round the neck.
fn stations(
    model: &mut Model,
    gems: &mut Vec<GemAt>,
    look: &NecklaceLook,
    path: &Path,
    heights: &Heights,
) {
    let scale = match path.wear {
        Wear::Alone => 1.0,
        Wear::Worn => WORN_GROW,
    };
    for setting in &look.settings {
        let angle = setting.angle as f32 / 256.0 * TAU;
        let (point, out) = path.at(angle);
        let (ahead_point, _) = path.at(angle + 0.01);
        let u = (ahead_point - point).normalize_or(Vec3::X);
        let out = (out - u * out.dot(u)).normalize_or(Vec3::Z);
        let place = Place::Flat {
            at: point - out * path.off,
            u,
            v: out.cross(u),
            out,
            scale,
        };
        let width = SETTING * SIZE_STEP.powi(setting.bigger as i32);
        set(model, gems, &place, heights, setting, Vec2::ZERO, width);
    }
}

/// A setting at a point of a plate: a bezel's cup or claws, and its gem or the dark
/// of its empty seat.
fn set(
    model: &mut Model,
    gems: &mut Vec<GemAt>,
    place: &Place,
    heights: &Heights,
    setting: &SettingLook,
    at: Vec2,
    width: f32,
) {
    let (low, high) = heights.plate;
    let top = high + heights.rise;
    let worn = heights.rise < 1.0;
    match (setting.mount, worn) {
        (Mount::Bezel, _) | (Mount::Claws, true) => {
            let cup = ellipse(at, Vec2::splat(width * 0.6), 16);
            slab(model.of(Stuff::Main), place, &cup, (low, top));
        }
        (Mount::Claws, false) => {
            let seat = ellipse(at, Vec2::splat(width * 0.35), 10);
            slab(model.of(Stuff::Main), place, &seat, (low, top));
            for corner in 0..4 {
                let round = (corner as f32 + 0.5) / 4.0 * TAU;
                let way = Vec2::new(round.cos(), round.sin());
                let points = [
                    place.at(at + way * width * 0.3, top),
                    place.at(at + way * width * 0.5, top + width * 0.4),
                ];
                tube(model.of(Stuff::Main), &points, width * 0.06, 5);
            }
        }
    }
    let Some(gem) = &setting.gem else {
        let seat = ellipse(at, Vec2::splat(width * 0.4), 12);
        slab(model.of(Stuff::Backing), place, &seat, (top, top + 0.05));
        return;
    };
    let across = width * SIZE_STEP.powi(gem.bigger as i32);
    let width = across * place.scale();
    let (sits, stone) = match worn {
        true => (0.0, stone(place, at, across / 2.0, top)),
        false => (width * PAVILION_DEPTH * 0.6, Vec::new()),
    };
    gems.push(GemAt {
        gem: gem.gem,
        quality: gem.quality,
        at: place.at(at, top + sits),
        out: place.out(at),
        width,
        stone,
    });
}
