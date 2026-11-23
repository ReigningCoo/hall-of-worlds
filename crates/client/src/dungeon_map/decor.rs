//! Decor (`shared::decor`, tavern furniture plan PR 5) as models: things
//! hanging on walls' faces, chandeliers over their cells, rugs on the floor,
//! and candles standing on tables (`furniture::Candles`), each built once per
//! look from simple coloured solids (`model.rs`) with a black outline.
//! Remembered like bottles on racks: in sepia once seen, in colour too while
//! in sight; flames only while in sight, in their own colours whatever the
//! light. The light they give is `fixed_lights`'.

use std::collections::HashMap;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::decor::Decor;
use shared::furniture::{Furniture, Piece};

use super::furniture::{TABLE_TOP, lift_on};
use super::herbs::HerbMaterials;
use super::sight_lines::SurfaceMaterial;
use super::trees::TreeMaterials;
use super::{CELL_SIZE, DungeonScene, Shown};
use crate::common::{CurrentDungeon, DrawnGround};

// Split into files (`cargo xtask lint` keeps files small).
mod boards;
mod candles;
mod clutter;
mod colours;
mod games;
mod inn_room;
mod look;
mod model;
mod pegs;
mod rugs;
mod trophies;

pub(super) use inn_room::{CHEST_HEIGHT, WARDROBE_HEIGHT};
use look::{DecorLook, placed, turn_to};
pub(super) use model::DecorMeshes;

/// Decor's models built so far, by look.
#[derive(Resource, Default)]
pub(super) struct DecorModels(HashMap<DecorLook, DecorMeshes>);

/// One piece as drawn, to tell when it must be drawn again.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Drawn {
    lit: bool,
    look: DecorLook,
    /// Where it is, in tenths of a unit.
    at: IVec3,
}

/// What a drawn piece is known by: decor by its place, candles by their
/// cell, a board, barrel or sacks by the cell of the furniture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Key {
    Decor(shared::decor::DecorSlot),
    Candles((i64, i64)),
    Furniture((i64, i64)),
}

/// The decor drawn now.
#[derive(Resource, Default)]
pub(super) struct DrawnDecor {
    drawn: HashMap<Key, (Entity, Drawn)>,
    /// What was seen last time.
    looks: Option<u64>,
}

/// One piece to draw: its key, look, cells and where it stands.
struct Wanted {
    key: Key,
    look: DecorLook,
    cells: Vec<(i64, i64)>,
    place: Transform,
}

/// Draws decor and candles again when what is seen, the level or the ground
/// change.
#[allow(clippy::too_many_arguments)]
pub(super) fn update_decor(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, shown, heights): (Res<CurrentDungeon>, Res<Shown>, Res<DrawnGround>),
    (lit, sepia): (Res<HerbMaterials>, Res<TreeMaterials>),
    mut models: ResMut<DecorModels>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut drawn: ResMut<DrawnDecor>,
) {
    let unchanged = !dungeon.is_changed() && !heights.is_changed();
    if drawn.looks == Some(shown.looks) && unchanged {
        return;
    }
    drawn.looks = Some(shown.looks);
    let level = &dungeon.level;
    // The highest ground under some cells, in world units.
    let ground = |cells: &[(i64, i64)]| {
        (cells.iter())
            .map(|&(x, y)| heights.drawn_between(x as f32, y as f32) * CELL_SIZE)
            .fold(f32::MIN, f32::max)
    };
    let mut wanted: Vec<Wanted> = (level.decor())
        .map(|decor| {
            let cells = decor.cells();
            // A wall's own cell has no ground to speak of: the floor in front.
            let under = match decor {
                Decor::Wall { .. } => &cells[1..],
                _ => &cells[..],
            };
            Wanted {
                key: Key::Decor(decor.slot()),
                look: DecorLook::of(decor),
                place: placed(decor, ground(under)),
                cells,
            }
        })
        .collect();
    for piece in level.furniture_on_top() {
        let Furniture::Candles { candles } = piece.furniture else {
            continue;
        };
        let under = level.furniture_at(piece.at.0, piece.at.1);
        let table: Vec<(i64, i64)> = under.map_or(vec![piece.at], |table| table.cells().collect());
        let top = ground(&table) + TABLE_TOP + under.map_or(0.0, lift_on);
        wanted.push(Wanted {
            key: Key::Candles(piece.at),
            look: DecorLook::Candles(candles),
            cells: vec![piece.at],
            place: candles_place(piece, top),
        });
    }
    wanted.extend(level.furniture().filter_map(|piece| {
        let (look, lift) = furniture_look(piece)?;
        let cells: Vec<(i64, i64)> = piece.cells().collect();
        Some(Wanted {
            key: Key::Furniture(piece.at),
            look,
            place: furniture_place(piece, ground(&cells) + lift),
            cells,
        })
    }));
    let mut keep = HashMap::new();
    for want in wanted {
        if !want
            .cells
            .iter()
            .any(|&(x, y)| dungeon.remembered.is_seen(x, y))
        {
            continue;
        }
        let in_sight = (shown.sight.as_ref())
            .is_some_and(|sight| want.cells.iter().any(|&(x, y)| sight.is_seen(x, y)));
        let now = Drawn {
            lit: in_sight,
            look: want.look,
            at: (want.place.translation * 10.0).round().as_ivec3(),
        };
        if let Some((entity, was)) = drawn.drawn.remove(&want.key) {
            if was == now {
                keep.insert(want.key, (entity, was));
                continue;
            }
            commands.entity(entity).despawn();
        }
        let model = (models.0)
            .entry(want.look)
            .or_insert_with(|| want.look.build().build(&mut meshes, want.look.thin()))
            .clone();
        let mut spawned = commands.spawn((want.place, Visibility::default()));
        spawned.with_children(|piece| {
            let mut copy = |mesh: &Handle<Mesh>, material: &Handle<SurfaceMaterial>| {
                piece.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    NotShadowCaster,
                ));
            };
            copy(&model.memory, &sepia.memory);
            copy(&model.outline, &sepia.outline);
            if now.lit {
                copy(&model.fill, &lit.fill);
                copy(&model.outline, &lit.outline);
                if let Some(flames) = &model.flames {
                    copy(flames, &lit.glow);
                }
            }
        });
        let entity = spawned.id();
        commands.entity(*scene).add_child(entity);
        keep.insert(want.key, (entity, now));
    }
    for (_, (entity, _)) in drawn.drawn.drain() {
        commands.entity(entity).despawn();
    }
    drawn.drawn = keep;
}

/// What furniture drawn with decor looks like (a games table's board, a
/// barrel, sacks, a bed, chest or wardrobe), and how high over the ground it
/// stands; `None` for the rest.
fn furniture_look(piece: &Piece) -> Option<(DecorLook, f32)> {
    Some(match piece.furniture {
        Furniture::GamesTable { game } => (DecorLook::Board(game), TABLE_TOP),
        Furniture::Barrel { barrel } => (DecorLook::Barrel(barrel, piece.material), 0.0),
        Furniture::Sacks { sacks } => {
            let seed = shared::rng::Seed::new(0).derive("furniture look");
            let variant = seed.at(&[piece.at.0, piece.at.1]) % 9;
            (DecorLook::Sacks(sacks, variant as u8), 0.0)
        }
        Furniture::Bed { bed } => {
            let seed = shared::rng::Seed::new(0).derive("furniture look");
            let variant = seed.at(&[piece.at.0, piece.at.1]) % 5;
            (DecorLook::Bed(bed, piece.material, variant as u8), 0.0)
        }
        Furniture::Chest { chest } => (DecorLook::Chest(chest, piece.material), 0.0),
        Furniture::Wardrobe { wardrobe } => (DecorLook::Wardrobe(wardrobe, piece.material), 0.0),
        _ => return None,
    })
}

/// Where a piece of furniture drawn with decor stands, `up` over the middle
/// of its cells, turned the way it faces.
fn furniture_place(piece: &Piece, up: f32) -> Transform {
    let (x, y) = piece.at;
    let (width, height) = (f32::from(piece.size.0), f32::from(piece.size.1));
    let middle = Vec2::new(
        x as f32 + (width - 1.0) / 2.0,
        y as f32 + (height - 1.0) / 2.0,
    );
    let at = Vec3::new(middle.x * CELL_SIZE, up, middle.y * CELL_SIZE);
    Transform::from_translation(at).with_rotation(turn_to(piece.facing))
}

/// Where candles stand on their table's top at `top`: a pair side by side
/// across the way they face.
fn candles_place(piece: &Piece, top: f32) -> Transform {
    let (x, y) = piece.at;
    let at = Vec3::new(x as f32 * CELL_SIZE, top, y as f32 * CELL_SIZE);
    Transform::from_translation(at).with_rotation(turn_to(piece.facing))
}
