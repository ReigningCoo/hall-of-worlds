//! Fixed lights: lights that stay where they are on a level (glowing fungus
//! patches, lava's rims and candles: sconces, chandeliers, table candles; spots
//! a spell's Light lit), worked out once into a glow grid and kept until the
//! level, the hour or the lit spots change, then added to the light in `update_meshes`
//! as town windows' glow is. Only the floor the player is on.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::dungeon::Level;

use super::mesh::{LightGrid, reach};
use crate::common::{CurrentDungeon, ServerClock};

mod candles;
mod fungi;
mod lava;
mod spell;

pub(super) use spell::{SpellLights, follow_spell_lights};

/// A light that stays where it is on a level.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct FixedLight {
    pub(super) cell: (i64, i64),
    /// How far it reaches, in cells (as `LightGrid::add_light`).
    pub(super) radius: u32,
    /// Its colour and strength, as linear RGB (1 is full light).
    pub(super) colour: Vec3,
}

/// The level's fixed lights as one glow grid, kept until the level or the
/// game hour changes. Changes only when the grid does.
#[derive(Resource, Default)]
pub(super) struct FixedLights {
    /// The light they give, or none on a level without any.
    pub(super) glow: Option<LightGrid>,
    /// The game hour it was worked out for.
    hour: Option<u64>,
}

/// Works the glow grid out again when the level or the game hour changes.
pub(super) fn update_fixed_lights(
    dungeon: Res<CurrentDungeon>,
    clock: Res<ServerClock>,
    spells: Res<SpellLights>,
    mut fixed: ResMut<FixedLights>,
) {
    let now = clock.minutes.max(0.0) as u64;
    let hour = Some(now / 60);
    if fixed.hour == hour && !dungeon.is_changed() && !spells.is_changed() {
        return;
    }
    let mut lights = fungi::fungus_lights(&dungeon.level, now);
    lights.extend(lava::lava_lights(&dungeon.level));
    lights.extend(candles::candle_lights(&dungeon.level));
    lights.extend(spells.lights());
    let glow = (!lights.is_empty()).then(|| glow_grid(&dungeon.level, &lights));
    if fixed.glow.is_none() && glow.is_none() {
        // Nothing to light: no change for `update_meshes` to see.
        fixed.bypass_change_detection().hour = hour;
        return;
    }
    *fixed = FixedLights { glow, hour };
}

/// The strongest light of one colour at each cell it reaches (0 to 1).
type Strongest = HashMap<(i64, i64), f32>;

/// The light `lights` give together: the strongest of each colour at each
/// cell, so many lights of one colour side by side are no brighter than the
/// nearest, and then the colours added up.
pub(super) fn glow_grid(level: &Level, lights: &[FixedLight]) -> LightGrid {
    let mut by_colour: HashMap<[u32; 3], (Vec3, Strongest)> = HashMap::new();
    for light in lights {
        let key = light.colour.to_array().map(f32::to_bits);
        let (_, strongest) = by_colour
            .entry(key)
            .or_insert_with(|| (light.colour, HashMap::new()));
        for (cell, strength) in reach(level, light.cell, light.radius) {
            let kept = strongest.entry(cell).or_insert(0.0);
            *kept = kept.max(strength);
        }
    }
    let mut grid = LightGrid::new(level);
    for (colour, strongest) in by_colour.values() {
        let mut one = LightGrid::new(level);
        for (&(x, y), &strength) in strongest {
            one.set(x, y, *colour * strength);
        }
        grid.add_grid(&one);
    }
    grid
}

#[cfg(test)]
mod lava_tests;
#[cfg(test)]
mod tests;
