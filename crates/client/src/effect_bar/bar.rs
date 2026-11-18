//! The bar itself: a row centred at the top of the window (markup on
//! `dom::top_strip()`), its canvas filled with an icon for each timed effect,
//! drawn again whenever which effects show changes; and each icon's shade.

use bevy::prelude::*;
use shared::bonus::Source;

use super::looks::{colour, gone, icon_path, timed};
use crate::character::CharacterSheet;
use crate::common::ServerTick;
use crate::dom::{Solid, Template, document, top_strip};
use crate::html;
use crate::widgets::{ABILITY, ITEM_EDGE, UiId, spawn_outlined_icon};

/// The form the bar's ids are in.
pub(super) const FORM: &str = "effect_bar";

/// The bar: a row as wide as its icons, in the middle of the top. Only the
/// icons are ever in front of the map (the form has no height).
pub(super) static EFFECT_BAR: Template = html!(
    "effect_bar",
    r#"
<form id="effect_bar" align="center">
  <div id="row" margin-top="12">
    <canvas id="icons"/>
  </div>
</form>
"#
);

const GAP: f32 = 4.0;
const EDGE: f32 = 2.0;
const INSET: f32 = 4.0;
/// How dark the shade is, as the action bar's.
const SHADE: Color = Color::srgba(0.0, 0.0, 0.0, 0.6);

/// On the canvas the icons are drawn in.
#[derive(Component)]
pub(super) struct BarCanvas;

/// On each icon's frame: the effect it is, by where it came from (a
/// character has at most one from each).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct EffectIcon(pub Source);

/// On each icon's shade.
#[derive(Component)]
pub(super) struct EffectShade(Source);

pub(super) fn spawn_bar(mut commands: Commands) {
    commands.spawn(document(&EFFECT_BAR)).insert(top_strip());
}

/// Marks the canvas once it is built.
pub(super) fn bind_canvas(mut commands: Commands, new: Query<(Entity, &UiId), Added<UiId>>) {
    for (entity, id) in &new {
        if id.parts() == (FORM, "icons") {
            commands.entity(entity).insert(BarCanvas);
        }
    }
}

/// What the bar shows: each effect by its source and times.
type Shown = Vec<(Source, u64, Option<u64>)>;

/// Draws the icons again whenever which effects show changes: one goes at
/// its end, or at once when the server's sheet no longer has it.
pub(super) fn draw_icons(
    (sheet, clock, time): (Res<CharacterSheet>, Res<ServerTick>, Res<Time>),
    canvas: Query<Entity, With<BarCanvas>>,
    added: Query<(), Added<BarCanvas>>,
    assets: Res<AssetServer>,
    mut drawn: Local<Shown>,
    mut commands: Commands,
) {
    let Ok(canvas) = canvas.single() else {
        return;
    };
    let effects = &sheet.0.effects.0;
    let now = clock.now(time.elapsed_secs_f64());
    let showing: Vec<_> = now.map_or(Vec::new(), |now| timed(effects, now).collect());
    let shown: Shown = showing.iter().map(|e| (e.source, e.start, e.end)).collect();
    if shown == *drawn && added.is_empty() {
        return;
    }
    *drawn = shown;
    let row = Node {
        column_gap: px(GAP),
        ..default()
    };
    let mut canvas = commands.entity(canvas);
    canvas.despawn_related::<Children>();
    canvas.with_children(|inside| {
        inside.spawn(row).with_children(|row| {
            for effect in showing {
                let icon = assets.load(icon_path(effect.source));
                spawn_icon(row, effect.source, icon, colour(effect));
            }
        });
    });
}

/// One icon, as big as an action bar slot, with its shade.
fn spawn_icon(row: &mut ChildSpawnerCommands, source: Source, icon: Handle<Image>, behind: Color) {
    let frame = Node {
        width: px(ABILITY),
        height: px(ABILITY),
        border: UiRect::all(px(EDGE)),
        padding: UiRect::all(px(INSET)),
        border_radius: BorderRadius::all(px(4.0)),
        ..default()
    };
    let colours = (BackgroundColor(behind), BorderColor::from(ITEM_EDGE));
    row.spawn((frame, colours, Solid, EffectIcon(source)))
        .with_children(|frame| {
            spawn_outlined_icon(frame, icon, Color::WHITE);
            // Over the whole frame, growing down from the top.
            let shade = Node {
                position_type: PositionType::Absolute,
                left: px(-EDGE),
                right: px(-EDGE),
                top: px(-EDGE),
                height: percent(0),
                border_radius: BorderRadius::all(px(4.0)),
                ..default()
            };
            frame.spawn((shade, BackgroundColor(SHADE), EffectShade(source)));
        });
}

/// Sizes each shade to how much of its effect's time has gone.
pub(super) fn show_shades(
    sheet: Res<CharacterSheet>,
    clock: Res<ServerTick>,
    time: Res<Time>,
    mut shades: Query<(&mut Node, &EffectShade)>,
) {
    let Some(now) = clock.now(time.elapsed_secs_f64()) else {
        return;
    };
    for (mut node, EffectShade(source)) in &mut shades {
        let effect = sheet.0.effects.0.iter().find(|e| e.source == *source);
        let gone = effect.map_or(1.0, |effect| gone(effect, now));
        // Over the edge too, which the frame's height doesn't count.
        let height = Val::Px((ABILITY) * gone);
        if node.height != height {
            node.height = height;
        }
    }
}
