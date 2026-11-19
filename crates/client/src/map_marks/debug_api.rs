//! `client.marks`: the character's marks on this planet, as the client knows
//! them, and the popup open. `client.marks.add { "x", "y", "icon", "colour" }`:
//! asks the server for a mark at a world map point (the middle of it), as the
//! popup's icon does; `client.marks.remove { "id" }`, as Remove does. See
//! `docs/context/debug-api.md`.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{AddDebugMethod, BrpResult, Value, json, params, refused};
use shared::id::Id;
use shared::map_mark::{MarkColour, MarkIcon, middle_of_point};
use shared::protocol::{ClientMessage, MarkAsk};

use super::popup::MarkPopup;
use crate::common::MapMarks;
use crate::connection::ToServer;

pub(super) fn add(app: &mut App) {
    app.add_debug_method("client.marks", marks)
        .add_debug_method("client.marks.add", add_mark)
        .add_debug_method("client.marks.remove", remove_mark);
}

fn marks(In(_): In<Option<Value>>, marks: Res<MapMarks>, popup: Res<MarkPopup>) -> BrpResult {
    let list: Vec<Value> = (marks.0.iter())
        .map(|mark| {
            json!({
                "id": mark.id.0.to_string(),
                "at": [mark.at.0, mark.at.1],
                "point": shared::map_mark::point_of(mark.at),
                "icon": mark.icon.key(),
                "colour": mark.colour.key(),
                "auto": mark.auto,
            })
        })
        .collect();
    Ok(json!({ "marks": list, "popup": format!("{:?}", *popup) }))
}

#[derive(Deserialize)]
struct AddParams {
    x: i64,
    y: i64,
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    colour: Option<String>,
}

fn add_mark(In(request): In<Option<Value>>, mut to_server: MessageWriter<ToServer>) -> BrpResult {
    let AddParams { x, y, icon, colour } = params(request)?;
    let icon = match icon {
        Some(key) => MarkIcon::from_key(&key).ok_or_else(|| refused("no such icon"))?,
        None => MarkIcon::Flag,
    };
    let colour = match colour {
        Some(key) => MarkColour::from_key(&key).ok_or_else(|| refused("no such colour"))?,
        None => MarkColour::default(),
    };
    let at = middle_of_point((x, y));
    to_server.write(ToServer(ClientMessage::Mark(MarkAsk::Add {
        at,
        icon,
        colour,
    })));
    Ok(json!({ "asked": [at.0, at.1] }))
}

#[derive(Deserialize)]
struct RemoveParams {
    id: String,
}

fn remove_mark(
    In(request): In<Option<Value>>,
    mut to_server: MessageWriter<ToServer>,
) -> BrpResult {
    let RemoveParams { id } = params(request)?;
    let id = Id::parse(&id).ok_or_else(|| refused("not an id"))?;
    to_server.write(ToServer(ClientMessage::Mark(MarkAsk::Remove(id))));
    Ok(json!({ "asked": true }))
}
