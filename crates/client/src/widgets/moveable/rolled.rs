//! A moving panel drawn as an unrolled scroll (recipes plan PR 4): a rolled
//! end across its top and its bottom, wider than the parchment, each a
//! shaded cylinder with a wooden knob at either end. The top one moves the
//! panel, as its title bar does.

use bevy::prelude::*;

use super::super::PANEL_EDGE;
use super::super::theme::PARCHMENT;
use super::spawn::PanelPart;
use super::{CONTENT_GAP, PADDING, TOP_PADDING};

/// How thick a rolled end is.
const ROLL: f32 = 22.0;
/// How far it stands out past the parchment's sides.
const OVERHANG: f32 = 10.0;
/// A knob's width, and its height as a share of the roll's.
const KNOB: f32 = 7.0;
const KNOB_SHARE: f32 = 70.0;
/// The knobs' dark wood.
const WOOD: Color = Color::srgb(0.36, 0.22, 0.11);
/// Bands across a rolled end, top to bottom: how much of the parchment's
/// colour each keeps, so it looks round, lit from above.
const BANDS: [f32; 7] = [0.62, 0.78, 0.92, 1.0, 0.9, 0.74, 0.56];

/// Gives a moving panel (as `spawn_moveable_panel` made it) rolled ends.
pub fn roll_ends(commands: &mut Commands, panel: Entity) {
    let top = spawn_roll(commands, panel, true);
    let bottom = spawn_roll(commands, panel, false);
    commands.entity(panel).insert_children(0, &[top]);
    commands.entity(panel).add_child(bottom);
}

/// One rolled end: half of it standing out past the parchment's edge.
fn spawn_roll(commands: &mut Commands, panel: Entity, top: bool) -> Entity {
    let side = -(PADDING + PANEL_EDGE + OVERHANG);
    let outward = -(PANEL_EDGE + ROLL / 2.0);
    let margin = match top {
        true => UiRect {
            left: px(side),
            right: px(side),
            top: px(outward - TOP_PADDING),
            bottom: px(0),
        },
        false => UiRect {
            left: px(side),
            right: px(side),
            top: px(CONTENT_GAP),
            bottom: px(outward - PADDING),
        },
    };
    let part = match top {
        true => PanelPart::TitleBar(panel),
        false => PanelPart::Body(panel),
    };
    let roll = Node {
        height: px(ROLL),
        margin,
        align_items: AlignItems::Center,
        ..default()
    };
    commands
        .spawn((part, roll))
        .with_children(|roll| {
            spawn_knob(roll);
            spawn_cylinder(roll);
            spawn_knob(roll);
        })
        .id()
}

/// The roll itself: bands of the parchment's colour, rounded at its ends.
fn spawn_cylinder(parent: &mut ChildSpawnerCommands) {
    let node = Node {
        flex_grow: 1.0,
        height: percent(100.0),
        flex_direction: FlexDirection::Column,
        overflow: Overflow::clip(),
        border_radius: BorderRadius::all(px(ROLL / 4.0)),
        ..default()
    };
    parent.spawn(node).with_children(|cylinder| {
        let [red, green, blue] = PARCHMENT.map(f32::from);
        for keep in BANDS {
            let shade = |channel: f32| channel * keep / 255.0;
            cylinder.spawn((
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
                BackgroundColor(Color::srgb(shade(red), shade(green), shade(blue))),
            ));
        }
    });
}

/// A wooden knob at one end of the roll.
fn spawn_knob(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: px(KNOB),
            height: percent(KNOB_SHARE),
            border_radius: BorderRadius::all(px(KNOB / 2.0)),
            ..default()
        },
        BackgroundColor(WOOD),
    ));
}
