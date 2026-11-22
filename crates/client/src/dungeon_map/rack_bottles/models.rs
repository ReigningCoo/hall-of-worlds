//! Bottles' models for wine and spirit racks, and drinking vessels' for bar
//! counters, simple for the map: a body, a shoulder, a neck and a cork or cap
//! (a vessel's own stack: a tankard's lid, a goblet's foot and stem), from
//! cylinders, upright about their
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
    /// A drinking vessel on a bar counter.
    Vessel(VesselKind),
}

impl RackBottleLook {
    /// How `item` looks on a rack, if it is a bottle one takes.
    pub(super) fn of(item: &Item) -> Option<Self> {
        match &item.form {
            Some(ItemForm::Vessel(vessel)) => match vessel.vessel {
                VesselKind::WineBottle => Some(Self::WineBottle),
                VesselKind::Flask => Some(Self::Flask),
                kind => Some(Self::Vessel(kind)),
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
            Self::Vessel(kind) => match kind {
                VesselKind::Jug => (CELL_SIZE * 0.13, CELL_SIZE * 0.055),
                VesselKind::Tankard => (CELL_SIZE * 0.1, CELL_SIZE * 0.04),
                VesselKind::DrinkingHorn => (CELL_SIZE * 0.11, CELL_SIZE * 0.03),
                VesselKind::Goblet => (CELL_SIZE * 0.09, CELL_SIZE * 0.032),
                _ => (CELL_SIZE * 0.065, CELL_SIZE * 0.035),
            },
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
            Self::Vessel(kind) => {
                let pewter = Vec3::new(0.55, 0.56, 0.58);
                let clay = Vec3::new(0.55, 0.3, 0.18);
                match kind {
                    VesselKind::Tankard | VesselKind::Goblet => (pewter, pewter * 0.8),
                    VesselKind::DrinkingHorn => (Vec3::new(0.75, 0.68, 0.5), pewter),
                    _ => (clay, clay * 0.8),
                }
            }
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
    let mut shell = Shell::default();
    let mut from = 0.0;
    for (r, share, top) in segments(look) {
        let mesh = Mesh::from(Cylinder::new(radius * r, share * height));
        let y = (from + share / 2.0 - 0.5) * height;
        let colour = if top { cork } else { body };
        shell.add(&mesh, Transform::from_xyz(0.0, y, 0.0), colour);
        from += share;
    }
    shell
}

/// A look's cylinders from the foot up: each one's radius (a share of the
/// look's), its share of the height, and whether it is the top's colour (a
/// cork, cap, lid or rim).
fn segments(look: RackBottleLook) -> Vec<(f32, f32, bool)> {
    match look {
        RackBottleLook::Flask => vec![
            (1.0, 0.7, false),
            (0.725, 0.1, false),
            (0.45, 0.1, false),
            (0.5, 0.1, true),
        ],
        RackBottleLook::WineBottle | RackBottleLook::Small { .. } => vec![
            (1.0, 0.6, false),
            (0.675, 0.12, false),
            (0.35, 0.2, false),
            (0.385, 0.08, true),
        ],
        RackBottleLook::Vessel(kind) => match kind {
            VesselKind::Tankard => vec![(1.0, 0.85, false), (1.1, 0.15, true)],
            VesselKind::Goblet => vec![
                (0.9, 0.08, true),
                (0.25, 0.42, true),
                (0.7, 0.15, false),
                (1.0, 0.35, false),
            ],
            VesselKind::DrinkingHorn => vec![
                (0.35, 0.3, false),
                (0.6, 0.3, false),
                (0.85, 0.3, false),
                (1.0, 0.1, true),
            ],
            VesselKind::Jug => vec![
                (0.8, 0.15, false),
                (1.0, 0.45, false),
                (0.75, 0.15, false),
                (0.55, 0.15, false),
                (0.65, 0.1, true),
            ],
            _ => vec![(0.85, 0.2, false), (1.0, 0.7, false), (1.05, 0.1, true)],
        },
    }
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
