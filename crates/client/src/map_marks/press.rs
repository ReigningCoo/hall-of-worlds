//! Opening the popups from clicks on the full map, and their buttons.

use bevy::prelude::*;
use shared::map_mark::{MarkColour, MarkIcon, middle_of_point, point_of};
use shared::protocol::{ClientMessage, MarkAsk};

use super::popup::MarkPopup;
use crate::common::{CurrentWorld, MapMarks, Preferences, point_at};
use crate::connection::ToServer;
use crate::dom::{UiEvent, UiValue};
use crate::minimap::{FullMap, FullMapClick};
use crate::player::TravelTo;

/// A popup closed itself (a press elsewhere, Esc): the click that pressed
/// elsewhere opens no other.
#[derive(Resource, Default)]
pub(super) struct JustClosed(pub(super) bool);

/// Opens a popup from a click on the full map: on a mark, about it; on known
/// land with no poi, to place one there (the middle of its world map point).
pub(super) fn open_on_click(
    mut clicks: MessageReader<FullMapClick>,
    marks: Res<MapMarks>,
    world: Option<Res<CurrentWorld>>,
    mut popup: ResMut<MarkPopup>,
    mut closed: ResMut<JustClosed>,
) {
    for click in clicks.read() {
        if std::mem::take(&mut closed.0) {
            continue;
        }
        let width = world.as_ref().map_or(0, |world| world.surface.width());
        *popup = if let Some(mark) = marks.near(click.point, click.reach, width) {
            MarkPopup::Mark {
                id: mark.id,
                at: click.at,
            }
        } else if click.known && !click.on_poi {
            MarkPopup::Place {
                cell: middle_of_point(point_at(click.point)),
                at: click.at,
            }
        } else {
            MarkPopup::Closed
        };
    }
}

/// The popups' buttons: a colour is chosen (and remembered), an icon places
/// the mark, Travel Here goes to a mark and Remove asks the server to remove
/// it. A mark gone (removed elsewhere), or the full map closing, closes its popup.
#[allow(clippy::too_many_arguments)]
pub(super) fn press_popup(
    mut events: MessageReader<UiEvent>,
    mut popup: ResMut<MarkPopup>,
    marks: Res<MapMarks>,
    full_map: Res<FullMap>,
    mut preferences: ResMut<Preferences>,
    mut closed: ResMut<JustClosed>,
    mut to_server: MessageWriter<ToServer>,
    mut travel: MessageWriter<TravelTo>,
) {
    for event in events.read() {
        let (form, name) = event.id.parts();
        if !matches!(form, "place_mark" | "mark") {
            continue;
        }
        if name == "box" && event.value == UiValue::Closed {
            if *popup != MarkPopup::Closed {
                *popup = MarkPopup::Closed;
                closed.0 = true;
            }
            continue;
        }
        if event.value != UiValue::Pressed {
            continue;
        }
        match (*popup, name.split_once('.')) {
            (MarkPopup::Place { .. }, Some(("colour", key))) => {
                if let Some(colour) = MarkColour::from_key(key)
                    && preferences.mark_colour != colour
                {
                    preferences.mark_colour = colour;
                }
            }
            (MarkPopup::Place { cell, .. }, Some(("icon", key))) => {
                if let Some(icon) = MarkIcon::from_key(key) {
                    let colour = preferences.mark_colour;
                    let ask = MarkAsk::Add {
                        at: cell,
                        icon,
                        colour,
                    };
                    to_server.write(ToServer(ClientMessage::Mark(ask)));
                    *popup = MarkPopup::Closed;
                }
            }
            (MarkPopup::Mark { id, .. }, None) if name == "remove" => {
                to_server.write(ToServer(ClientMessage::Mark(MarkAsk::Remove(id))));
                *popup = MarkPopup::Closed;
            }
            (MarkPopup::Mark { id, .. }, None) if name == "travel" => {
                if let Some(mark) = marks.0.iter().find(|mark| mark.id == id) {
                    let (x, y) = point_of(mark.at);
                    travel.write(TravelTo { x, y });
                }
                *popup = MarkPopup::Closed;
            }
            _ => {}
        }
    }
    let gone = |id| !marks.0.iter().any(|mark| mark.id == id);
    let close = match *popup {
        MarkPopup::Closed => false,
        MarkPopup::Mark { id, .. } if gone(id) => true,
        _ => !full_map.open,
    };
    if close {
        *popup = MarkPopup::Closed;
    }
    if !full_map.open {
        closed.0 = false;
    }
}
