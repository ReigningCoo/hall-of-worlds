//! Opening the popups from clicks on the full map, and their buttons.

use bevy::prelude::*;
use shared::map_mark::{MapMark, MarkColour, MarkIcon, middle_of_point, point_of};
use shared::protocol::{ClientMessage, MarkAsk};
use shared::wilderness::SCALE;

use super::popup::MarkPopup;
use crate::common::{
    CurrentDungeon, CurrentWorld, MapMarks, MapPosition, Preferences, in_cells, near_way_round,
    point_at,
};
use crate::connection::ToServer;
use crate::dom::{UiEvent, UiValue};
use crate::minimap::{FullMap, FullMapClick};
use crate::player::{Player, TravelTo};

/// A popup closed itself (a press elsewhere, Esc): the click that pressed
/// elsewhere opens no other.
#[derive(Resource, Default)]
pub(super) struct JustClosed(pub(super) bool);

/// Opens a popup from a click on the full map: on a mark, about it; on known
/// land with no poi, to place one there (on the world map the middle of its
/// point, in the wilderness the cell clicked).
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
        let near = if click.wild {
            marks.near_cell(click.point, click.reach, width)
        } else {
            marks.near(click.point, click.reach, width)
        };
        *popup = if let Some(mark) = near {
            MarkPopup::Mark {
                id: mark.id,
                at: click.at,
            }
        } else if click.known && !click.on_poi {
            MarkPopup::Place {
                // `point_at` finds the cell under it on the wilderness's grid too.
                cell: if click.wild {
                    point_at(click.point)
                } else {
                    middle_of_point(point_at(click.point))
                },
                at: click.at,
            }
        } else {
            MarkPopup::Closed
        };
    }
}

/// Where the player is, for Travel Here in the wilderness.
type PlayerAt<'w, 's> = Option<Single<'w, 's, &'static MapPosition, With<Player>>>;

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
    (dungeon, world, player): (Res<CurrentDungeon>, Option<Res<CurrentWorld>>, PlayerAt),
) {
    // In the wilderness, Travel Here walks to the mark's cell, the near way
    // round from the player.
    let wild = (dungeon.is_wilderness()).then(|| {
        let width = world.as_ref().map_or(0, |world| world.surface.width());
        (player.as_ref().map_or(0, |player| player.x), width)
    });
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
                    let (x, y) = travel_target(mark, wild);
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

/// Where Travel Here goes for a mark: its world map point, or in the
/// wilderness (`wild`: the player's cell `x` and the planet's width in points)
/// its cell, the near way round from the player.
pub(super) fn travel_target(mark: &MapMark, wild: Option<(i64, i64)>) -> (i64, i64) {
    let Some((from, width)) = wild else {
        return point_of(mark.at);
    };
    let (x, y) = point_at(in_cells(mark));
    let across = near_way_round((x - from) as f64, width * SCALE) as i64;
    (from + across, y)
}
