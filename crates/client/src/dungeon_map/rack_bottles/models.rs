//! Bottles' models for wine and spirit racks, simple for the map: a body, a
//! shoulder, a neck and a cork or cap, from cylinders, upright about their
//! middle, with a black outline grown out from the bottle's axis (so the
//! cylinders' seams leave no cracks) and a sepia copy for memory. A wine
//! bottle is dark green glass, a flask leather with a pewter cap, a small
//! bottle amber with a drink in it or pale with pure alcohol. Built once per
//! look. Sizes are first guesses.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::item::Item;
use shared::item_form::ItemForm;
use shared::potion::Liquid;
use shared::vessel::VesselKind;

use super::super::CELL_SIZE;
use super::super::furniture::TALLEST_BOTTLE;
use crate::plants::Shell;

/// What a bottle on a rack looks like.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum RackBottleLook {
    WineBottle,
    Flask,
    /// A small bottle: of a drink (amber), or of pure alcohol (pale).
    Small {
        drink: bool,
    },
}

impl RackBottleLook {
    /// How `item` looks on a rack, if it is a bottle one takes.
    pub(super) fn of(item: &Item) -> Option<Self> {
        match &item.form {
            Some(ItemForm::Vessel(vessel)) => match vessel.vessel {
                VesselKind::WineBottle => Some(Self::WineBottle),
                VesselKind::Flask => Some(Self::Flask),
                _ => None,
            },
            Some(ItemForm::Bottle(bottle)) => Some(Self::Small {
                drink: matches!(bottle.liquid, Some(Liquid::Drink(_))),
            }),
            _ => None,
        }
    }

    /// Its height, and its body's radius, in world units.
    pub(super) fn size(self) -> (f32, f32) {
        match self {
            Self::WineBottle => (TALLEST_BOTTLE, CELL_SIZE * 0.045),
            Self::Flask => (CELL_SIZE * 0.17, CELL_SIZE * 0.06),
            Self::Small { .. } => (CELL_SIZE * 0.13, CELL_SIZE * 0.04),
        }
    }

    /// The colours of its body and its cork or cap (linear).
    fn colours(self) -> (Vec3, Vec3) {
        let cork = Vec3::new(0.45, 0.28, 0.14);
        match self {
            Self::WineBottle => (Vec3::new(0.05, 0.3, 0.1), cork),
            Self::Flask => (Vec3::new(0.25, 0.12, 0.05), Vec3::new(0.55, 0.56, 0.58)),
            Self::Small { drink: true } => (Vec3::new(0.6, 0.3, 0.05), cork),
            Self::Small { drink: false } => (Vec3::new(0.75, 0.82, 0.85), cork),
        }
    }
}

/// A bottle's meshes: its colours, its outline and its sepia copy.
#[derive(Clone, Debug)]
pub(super) struct BottleMeshes {
    pub(super) fill: Handle<Mesh>,
    pub(super) outline: Handle<Mesh>,
    pub(super) memory: Handle<Mesh>,
}

/// Bottles' models built so far, by look.
#[derive(Resource, Default)]
pub(in crate::dungeon_map) struct BottleModels(HashMap<RackBottleLook, BottleMeshes>);

impl BottleModels {
    /// The model of a bottle of `look`, built if it hasn't been.
    pub(super) fn get(&mut self, look: RackBottleLook, meshes: &mut Assets<Mesh>) -> BottleMeshes {
        let built = self.0.entry(look).or_insert_with(|| {
            let shell = bottle_shell(look);
            let mut grown = shell.clone();
            grown.normals = outward(&shell.positions);
            BottleMeshes {
                fill: meshes.add(shell.fill()),
                outline: meshes.add(grown.outline(1.0)),
                memory: meshes.add(shell.memory()),
            }
        });
        built.clone()
    }
}

/// A bottle of `look` as one shell, upright, its middle at the origin.
fn bottle_shell(look: RackBottleLook) -> Shell {
    let (height, radius) = look.size();
    let (body, cork) = look.colours();
    let flask = look == RackBottleLook::Flask;
    // Shares of its height: body, shoulder, neck, cork; and of its radius.
    let (body_h, shoulder_h, neck_h) = if flask {
        (0.7, 0.1, 0.1)
    } else {
        (0.6, 0.12, 0.2)
    };
    let cork_h = 1.0 - body_h - shoulder_h - neck_h;
    let neck_r = if flask { 0.45 } else { 0.35 };
    let mut shell = Shell::default();
    let mut add = |r: f32, from: f32, share: f32, colour: Vec3| {
        let mesh = Mesh::from(Cylinder::new(r, share * height));
        let y = (from + share / 2.0 - 0.5) * height;
        shell.add(&mesh, Transform::from_xyz(0.0, y, 0.0), colour);
    };
    add(radius, 0.0, body_h, body);
    add(radius * (1.0 + neck_r) / 2.0, body_h, shoulder_h, body);
    add(radius * neck_r, body_h + shoulder_h, neck_h, body);
    add(radius * neck_r * 1.1, 1.0 - cork_h, cork_h, cork);
    shell
}

/// Normals pointing out from the bottle's middle, squashed to its shape, for
/// its outline.
fn outward(positions: &[Vec3]) -> Vec<Vec3> {
    let (radius, half) = (positions.iter()).fold((0.0f32, 0.0f32), |(r, h), p| {
        (r.max(p.x.hypot(p.z)), h.max(p.y.abs()))
    });
    let (radius, half) = (radius.max(1e-6), half.max(1e-6));
    (positions.iter())
        .map(|p| Vec3::new(p.x / radius, p.y / half, p.z / radius).normalize_or(Vec3::Y))
        .collect()
}
