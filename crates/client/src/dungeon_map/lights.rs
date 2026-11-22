//! Real point lights over each light source, for tokens.

use bevy::prelude::*;

use super::{CELL_SIZE, DungeonScene, TOKEN_LIGHT_HEIGHT, TOKEN_LIGHT_LUMENS, TokenLight};
use crate::common::{DrawnGround, HallBend, LightSource, SmoothPosition};

/// Keeps a real point light over each `LightSource`, so tokens near it are
/// lit like the tiles are; none for a source giving no light (radius 0).
pub(super) fn sync_lights(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    sources: Query<(Entity, &LightSource, &SmoothPosition)>,
    (heights, bend): (Res<DrawnGround>, Res<HallBend>),
    mut lights: Query<(Entity, &TokenLight, &mut PointLight, &mut Transform)>,
) {
    for (entity, token_light, mut point, mut transform) in &mut lights {
        let found = sources.get(token_light.owner).ok();
        let lit = found.filter(|(_, source, _)| source.radius > 0);
        let Some((_, source, position)) = lit else {
            commands.entity(entity).despawn();
            continue;
        };
        point.color = source.colour;
        point.range = (source.radius as f32 + 1.0) * CELL_SIZE;
        let (x, y) = (position.x as f32, position.y as f32);
        let ground = heights.drawn_between(x, y) * CELL_SIZE;
        let over = Vec3::new(x * CELL_SIZE, ground + TOKEN_LIGHT_HEIGHT, y * CELL_SIZE);
        transform.translation = bend.shift(over, CELL_SIZE);
    }
    for (owner, source, _) in &sources {
        if source.radius == 0 || lights.iter().any(|(_, light, _, _)| light.owner == owner) {
            continue;
        }
        commands.entity(*scene).with_child((
            TokenLight { owner },
            PointLight {
                color: source.colour,
                intensity: TOKEN_LIGHT_LUMENS,
                range: (source.radius as f32 + 1.0) * CELL_SIZE,
                ..default()
            },
            // Placed on the next run.
            Transform::default(),
        ));
    }
}
