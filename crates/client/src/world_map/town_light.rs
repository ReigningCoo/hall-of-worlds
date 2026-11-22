//! A warm light over every town on the world map, reaching further the larger
//! the town, so towns glow at night and are easy to find. It is on all the
//! time: the sun drowns it out by day.

use bevy::prelude::*;
use shared::poi::PoiKind;

use super::poi_model::BASE_RADIUS;

/// How high above its token's centre a town's light hangs, in grid points:
/// well above the houses, so they don't glare white by day, yet low enough
/// to light little past its reach.
const HEIGHT: f32 = 10.0;

/// How bright a town's light is, before its size. Tuned at night against the
/// ambient fill, as the World Gates' light is (`gate_view`).
const LUMENS: f32 = 20_000_000.0;

/// Lamplight: a warm orange-yellow.
const COLOUR: Color = Color::srgb(1.0, 0.72, 0.38);

/// How wide a town's light reaches, as a share of its base's width (owner,
/// 2026-11-22: a hamlet about half again as wide as its base, a city three
/// times; a village and a town in between, for now).
pub(super) fn reach_by_base(kind: PoiKind) -> Option<f32> {
    match kind {
        PoiKind::Hamlet => Some(1.5),
        PoiKind::Village => Some(2.0),
        PoiKind::Town => Some(2.5),
        PoiKind::City => Some(3.0),
        _ => None,
    }
}

/// Puts a town's light on its token's view, which stands half a token's
/// thickness above the ground; nothing for any other kind of poi. Hidden with
/// the view, so only towns in the window light anything, which keeps the
/// lights few enough for WebGL 2.
pub(super) fn attach(commands: &mut Commands, view: Entity, kind: PoiKind) {
    let Some(share) = reach_by_base(kind) else {
        return;
    };
    // A point light's range is a distance through the air, so it reaches
    // down past its height to the ground `share` of the base's radius away.
    let range = (BASE_RADIUS * share).hypot(HEIGHT);
    let light = commands
        .spawn((
            PointLight {
                color: COLOUR,
                // Brighter the further it reaches, so a city's glow is as
                // strong at its edge as a hamlet's.
                intensity: LUMENS * share * share,
                range,
                radius: 0.0,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_xyz(0.0, HEIGHT, 0.0),
        ))
        .id();
    commands.entity(view).add_child(light);
}

#[cfg(test)]
mod tests;
