//! Drawing the eye: a white almond over the player's token with a dark pupil,
//! its height from closed (a line) to open by how noticeable they are, wide and
//! reddened while a mob is Alert to them. Gone while not sneaking or off levels.

use bevy::prelude::*;
use shared::noise::Footsteps;

use super::noticeable::noticeable;
use crate::character::CharacterSheet;
use crate::common::{CurrentDungeon, GameView, MobAlert, Other, Sneaking, SpotPosition};
use crate::inventory::Carried;
use crate::player::Player;
use crate::view3d::{TOKEN_DIAMETER, TokenView, ViewCamera};

/// The eye's width, in interface pixels.
const WIDTH: f32 = 30.0;
/// How high above the token's middle it floats, in world units.
const ABOVE: f32 = TOKEN_DIAMETER * 2.1;
const WHITE: Color = Color::srgb(0.95, 0.95, 0.92);
const ALERT: Color = Color::srgb(1.0, 0.72, 0.68);
const DARK: Color = Color::srgb(0.08, 0.06, 0.05);

/// The eye.
#[derive(Component)]
pub(super) struct Eye;

/// Its pupil.
#[derive(Component)]
pub(super) struct Pupil;

/// How the eye is now: how open (0 to 1) and whether wide for Alert.
fn look(open: f32, alert: bool) -> (f32, f32, Color) {
    let height = if alert {
        WIDTH * 0.8
    } else {
        2.0 + (WIDTH * 0.55 - 2.0) * open
    };
    let pupil = (height - 4.0).clamp(0.0, WIDTH * 0.35);
    (height, pupil, if alert { ALERT } else { WHITE })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn show_eye(
    (sneaking, alert, view): (Res<Sneaking>, Res<MobAlert>, Res<State<GameView>>),
    (dungeon, sheet, carried): (Res<CurrentDungeon>, Res<CharacterSheet>, Res<Carried>),
    ui_scale: Res<UiScale>,
    camera: Single<(&Camera, &GlobalTransform), With<ViewCamera>>,
    player: Single<(Entity, &SpotPosition), With<Player>>,
    tokens: Query<(&TokenView, &GlobalTransform)>,
    others: Query<(&Other, &SpotPosition), Without<Player>>,
    mut eyes: Query<(Entity, &mut Node, &mut BackgroundColor), With<Eye>>,
    mut pupils: Query<&mut Node, (With<Pupil>, Without<Eye>)>,
    mut commands: Commands,
) {
    let (me, at) = *player;
    let (camera, eye) = *camera;
    let token = (tokens.iter()).find(|(view, _)| view.owner == me);
    let place = token.filter(|_| sneaking.0 && *view.get() == GameView::Dungeon);
    let place = place.and_then(|(_, token)| {
        let over = token.translation() + Vec3::Y * ABOVE;
        camera.world_to_viewport(eye, over).ok()
    });
    let Some(place) = place else {
        for (entity, ..) in &eyes {
            commands.entity(entity).despawn();
        }
        return;
    };
    let steps = Footsteps::of(true, &sheet.0.skills, &carried.0.worn);
    let mobs = (others.iter())
        .filter(|(other, _)| !other.player)
        .map(|(other, spot)| (other.kind, spot.0));
    let open = noticeable(&dungeon.level, at.0, steps, mobs) as f32 / 1000.0;
    let (height, pupil, colour) = look(open, alert.0);
    let scale = ui_scale.0.max(0.01);
    let node = Node {
        position_type: PositionType::Absolute,
        left: px(place.x / scale - WIDTH / 2.0),
        top: px(place.y / scale - height / 2.0),
        width: px(WIDTH),
        height: px(height),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(px(2.0)),
        border_radius: BorderRadius::all(percent(50.0)),
        ..default()
    };
    let pupil_node = Node {
        width: px(pupil),
        height: px(pupil),
        border_radius: BorderRadius::all(percent(50.0)),
        ..default()
    };
    if let Ok((_, mut shown, mut background)) = eyes.single_mut() {
        *shown = node;
        background.0 = colour;
        for mut shown in &mut pupils {
            *shown = pupil_node.clone();
        }
        return;
    }
    commands
        .spawn((Eye, node, BackgroundColor(colour), BorderColor::all(DARK)))
        .with_child((Pupil, pupil_node, BackgroundColor(DARK)));
}

#[cfg(test)]
mod tests;
