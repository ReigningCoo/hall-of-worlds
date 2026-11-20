//! Glowing fungus patches as fixed lights (fungi plan PR 5, owner's decision
//! 12): glowing patches joined to each other, a one-cell gap ignored, make a
//! group; each group reaches √(its cells) cells from every cell in it, all in
//! the same colour, its kinds' colours blended by how many cells each has.
//! Only full-grown patches glow, so picked ones (back to an earlier stage) and
//! growing ones give none.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use shared::dungeon::Level;
use shared::fungus::Fungi;
use shared::herb::STAGES;

use super::FixedLight;

/// How bright a glowing patch is at its own cell, as a share of full light:
/// a soft glow, less than a lantern.
const FUNGUS_GLOW: f32 = 0.7;

/// How far apart two glowing cells may be, in cells each way, and still be
/// joined: a one-cell gap between them is ignored.
const JOIN: i64 = 2;

/// The lights the level's glowing, full-grown fungus patches give at game
/// minute `now`.
pub(super) fn fungus_lights(level: &Level, now: u64) -> Vec<FixedLight> {
    let fungi = Fungi::built_in();
    let glowing: HashMap<(i64, i64), Vec3> = (level.fungus_patches())
        .filter_map(|(cell, patch)| {
            let def = fungi.get(&patch.fungus)?;
            let glow = def.glow?;
            (patch.stage(def, now) == STAGES).then(|| (cell, linear(glow)))
        })
        .collect();
    groups(&glowing)
        .into_iter()
        .flat_map(|group| {
            let count = group.len();
            let colour = group.iter().map(|cell| glowing[cell]).sum::<Vec3>() / count as f32;
            let radius = ((count as f32).sqrt().round() as u32).max(1);
            group.into_iter().map(move |cell| FixedLight {
                cell,
                radius,
                colour: colour * FUNGUS_GLOW,
            })
        })
        .collect()
}

/// A glow colour as linear RGB.
fn linear(colour: shared::herb::Rgb) -> Vec3 {
    let linear = LinearRgba::from(Color::srgb_u8(colour.0, colour.1, colour.2));
    Vec3::new(linear.red, linear.green, linear.blue)
}

/// The glowing cells in groups joined to each other (within `JOIN` cells each
/// way), each group sorted, in order of their first cell.
pub(super) fn groups<T>(glowing: &HashMap<(i64, i64), T>) -> Vec<Vec<(i64, i64)>> {
    let mut cells: Vec<(i64, i64)> = glowing.keys().copied().collect();
    cells.sort_unstable();
    let mut placed = HashSet::new();
    let mut groups = Vec::new();
    for &start in &cells {
        if !placed.insert(start) {
            continue;
        }
        let mut group = vec![start];
        let mut next = 0;
        while let Some(&(x, y)) = group.get(next) {
            next += 1;
            for dy in -JOIN..=JOIN {
                for dx in -JOIN..=JOIN {
                    let near = (x + dx, y + dy);
                    if glowing.contains_key(&near) && placed.insert(near) {
                        group.push(near);
                    }
                }
            }
        }
        group.sort_unstable();
        groups.push(group);
    }
    groups
}
