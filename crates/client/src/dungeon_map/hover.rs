//! Naming the door, stairs, herb, fruit plant, tree, vegetable or border the pointer is over,
//! for the hover box.

use bevy::prelude::*;
use shared::door::Swing;
use shared::dungeon::{Cell, Level};
use shared::sheet::Sheet;
use shared::site::StairsDirection;

use super::Shown;
use super::fruit_picking::fruit_clicked;
use super::ground_hit::Ground;
use super::picking::{door_clicked, pick};
use super::tree_picking::{tree_clicked, tree_name};
use crate::character::CharacterSheet;
use crate::common::{
    CurrentDungeon, DrawnGround, GameView, HerbTable, HoveredOnMap, PointerPosition, ServerClock,
};
use crate::view3d::ViewCamera;

/// What the hatched border round a poi in the wilderness is called.
const BORDER_NAME: &str = "Way back to the world map";

/// Names what the pointer is over on the site's map: a building's door (a
/// shop's by what it sells), the border round a poi, a herb patch, a fruit
/// plant, a vegetable, or stairs. Nothing elsewhere, or off the site.
pub(super) fn name_hovered_place(
    view: Res<State<GameView>>,
    pointer: Res<PointerPosition>,
    camera: Single<(&Camera, &GlobalTransform), With<ViewCamera>>,
    (dungeon, shown): (Res<CurrentDungeon>, Res<Shown>),
    heights: Res<DrawnGround>,
    (table, sheet, clock): (Res<HerbTable>, Res<CharacterSheet>, Res<ServerClock>),
    mut hovered: ResMut<HoveredOnMap>,
) {
    let (camera, camera_transform) = *camera;
    let ground = Ground(Some(&heights));
    let name = pointer
        .0
        .filter(|_| *view.get() == GameView::Dungeon)
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .and_then(|ray| {
            let door = door_clicked(ray, &dungeon.buildings, &shown.solid_faces, ground);
            if let Some(id) = door {
                let building = dungeon.buildings.iter().find(|b| b.id == id)?;
                return Some(building.kind.name().to_owned());
            }
            let now = clock.minutes.max(0.0) as u64;
            let (level, seen) = (&dungeon.level, &dungeon.remembered);
            let plant = fruit_clicked(ray, level, seen, &shown.solid_faces, ground);
            if let Some(name) = plant.and_then(|at| super::fruit::fruit_name(level, now, at)) {
                return Some(name);
            }
            let tree = tree_clicked(ray, level, seen, &shown.solid_faces, ground);
            if let Some(name) = tree.and_then(|at| tree_name(level, at)) {
                return Some(name);
            }
            let (x, y) = pick(
                ray,
                &dungeon.level,
                &dungeon.remembered,
                &shown.solid_faces,
                ground,
            )?;
            if dungeon.borders.contains(&(x, y)) {
                return Some(BORDER_NAME.to_owned());
            }
            if let Some(name) = door_name(&dungeon.level, (x, y)) {
                let lock = dungeon.door_locks.contains(&(x, y));
                return Some(match (lock, dungeon.level.is_locked(x, y)) {
                    (_, true) => format!("{name}, locked"),
                    (true, false) => format!("{name} with a lock"),
                    (false, false) => name.to_owned(),
                });
            }
            if let Some(name) = herb_name(&dungeon.level, &table, &sheet.0, now, (x, y)) {
                return Some(name);
            }
            if let Some(name) = super::fruit::fruit_name(&dungeon.level, now, (x, y)) {
                return Some(name);
            }
            if let Some(name) = super::vegetables::vegetable_name(&dungeon.level, now, (x, y)) {
                return Some(name);
            }
            if let Some(name) = tree_name(&dungeon.level, (x, y)) {
                return Some(name);
            }
            let stairs = dungeon.stairs.iter().find(|s| (s.x, s.y) == (x, y))?;
            Some(stairs_name(stairs.direction).to_owned())
        });
    if hovered.0 != name {
        hovered.0 = name;
    }
}

/// What a door is called, by how it hangs (one whose swing isn't known
/// counts as open, as it is drawn).
fn door_name(level: &Level, (x, y): (i64, i64)) -> Option<&'static str> {
    let secret = match level.cell(x, y) {
        Cell::Door => false,
        Cell::SecretDoor => true,
        _ => return None,
    };
    Some(match (level.swing(x, y), secret) {
        (Some(Swing::Closed), false) => "Closed door",
        (Some(Swing::Closed) | None, true) => "Closed secret door",
        (Some(Swing::Broken(_)), _) => "Broken door",
        (_, false) => "Open door",
        (_, true) => "Open secret door",
    })
}

/// What stairs are called, by the way they go.
fn stairs_name(direction: StairsDirection) -> &'static str {
    match direction {
        StairsDirection::Up => "Stairs up",
        StairsDirection::Down => "Stairs down",
    }
}

/// What a herb patch is called: its herb's name once the character knows it,
/// "Unknown plant" until then, saying when it is ready to pick; nothing while
/// it is dug up.
fn herb_name(
    level: &Level,
    table: &HerbTable,
    sheet: &Sheet,
    now: u64,
    (x, y): (i64, i64),
) -> Option<String> {
    let patch = level.herb_at(x, y)?;
    let def = table.get(&patch.herb)?;
    if patch.stage(def, now) == 0 {
        return None;
    }
    let name = if sheet.herbs.knows_name(&def.key) {
        def.name.as_str()
    } else {
        "Unknown plant"
    };
    Some(if patch.can_pick(def, now) {
        format!("{name}, ready to pick")
    } else {
        name.to_owned()
    })
}
