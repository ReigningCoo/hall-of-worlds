//! Load, for staff allowed it (`DebugTool::LoadDebugString`): someone's debug
//! string pasted, read here and sent as one `DevGoToPlace` for the server to
//! check and do; once there, the player faces the report's way and the camera
//! is set as it was. Their words show read-only below.

use bevy::prelude::*;
use shared::debug_report::{DebugReport, decode};
use shared::protocol::{ClientMessage, DevGoToPlace, PlaceAnswer, PlaceRefusal, ServerMessage};
use shared::role::DebugTool;

use super::load_state::Load;
use crate::common::{DebugTools, Facing};
use crate::connection::{FromServer, ToServer};
use crate::dom::{UiEvent, UiValue};
use crate::player::Player;
use crate::view3d::{OrbitCamera, ViewCamera};

/// The paste box, the checkbox and the Load button.
pub(super) fn read_load_events(
    mut events: MessageReader<UiEvent>,
    mut load: ResMut<Load>,
    tools: Res<DebugTools>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        match (event.id.parts(), &event.value) {
            (("debug_info", "paste"), UiValue::Text(text)) => load.pasted.clone_from(text),
            (("debug_info", "set_time"), UiValue::Flag(on)) => load.set_time = *on,
            (("debug_info", "load_button"), UiValue::Pressed) => {
                let (text, set_time) = (load.pasted.clone(), load.set_time);
                // How it went is shown in the panel.
                let _ = start_load(&text, set_time, *tools, &mut load, &mut to_server);
            }
            _ => {}
        }
    }
}

/// Reads `text` and asks the server to go there, noting what to show; the
/// report read, or why not.
pub(super) fn start_load(
    text: &str,
    set_time: bool,
    tools: DebugTools,
    load: &mut Load,
    to_server: &mut MessageWriter<ToServer>,
) -> Result<DebugReport, String> {
    match ask_for(text, set_time, tools) {
        Ok((ask, report)) => {
            to_server.write(ToServer(ClientMessage::DevGoToPlace(ask)));
            load.reported = Some(report.problem.clone());
            load.camera = report.camera;
            load.status = "Going there...".to_owned();
            Ok(report)
        }
        Err(why) => {
            load.status.clone_from(&why);
            Err(why)
        }
    }
}

/// What the server is asked for a string, with the report it holds: the time
/// only if asked and allowed.
pub(super) fn ask_for(
    text: &str,
    set_time: bool,
    tools: DebugTools,
) -> Result<(DevGoToPlace, DebugReport), String> {
    if !tools.may(DebugTool::LoadDebugString) {
        return Err("You may not load debug strings.".to_owned());
    }
    let report = decode(text).map_err(|why| why.to_string())?;
    let Some(place) = report.place else {
        return Err("That debug string was written outside any world.".to_owned());
    };
    let ask = DevGoToPlace {
        place,
        planet: report.planet,
        dev: report.dev,
        clock: report
            .clock
            .filter(|_| set_time && tools.may(DebugTool::WorldTime)),
    };
    Ok((ask, report))
}

/// The server's answer: there, facing the report's way with its view, or not.
pub(super) fn hear_answers(
    mut told: MessageReader<FromServer>,
    mut load: ResMut<Load>,
    mut player: Query<&mut Facing, With<Player>>,
    mut cameras: Query<&mut OrbitCamera, With<ViewCamera>>,
) {
    for FromServer(message) in told.read() {
        let ServerMessage::DevPlace(answer) = message else {
            continue;
        };
        load.status = match answer {
            PlaceAnswer::Arrived { heading } => {
                if let Ok(mut facing) = player.single_mut() {
                    facing.0 = *heading;
                }
                if let (Some(view), Ok(mut camera)) = (load.camera.take(), cameras.single_mut()) {
                    set_camera(&mut camera, view);
                }
                "There: what they reported is below.".to_owned()
            }
            PlaceAnswer::Refused(PlaceRefusal::UnknownWorld) => {
                "This server hasn't got that world open.".to_owned()
            }
            PlaceAnswer::Refused(PlaceRefusal::NoSuchPoi) => {
                "That place isn't in this server's world.".to_owned()
            }
        };
    }
}

/// Turns, tilts and zooms the camera as the report had it, if it is a view
/// at all (a pasted string is anyone's).
fn set_camera(camera: &mut OrbitCamera, view: shared::debug_report::Camera) {
    let finite = view.turn.is_finite() && view.tilt.is_finite() && view.zoom.is_finite();
    if !finite || view.zoom <= 0.0 {
        return;
    }
    camera.yaw = view.turn.rem_euclid(std::f32::consts::TAU);
    camera.pitch = view.tilt.clamp(0.0, std::f32::consts::FRAC_PI_2);
    camera.distance = view.zoom;
}
