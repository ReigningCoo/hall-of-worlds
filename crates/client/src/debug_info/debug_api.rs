//! `client.debug_string`: the debug string as the panel would make it now,
//! with the description typed (in the panel or not), and the report it holds.
//! `client.load_debug_string`: Load, as staff pasting a string would.
//! The description is set like any box's, by `client.ui.set` on
//! `debug_info/problem`; the panel opens by `client.ui.press` on
//! `hud/debug_info`. See `docs/context/debug-api.md`.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{AddDebugMethod, BrpResult, Value, json, params, refused};
use shared::debug_report::{DebugReport, encode};

use super::gather::Sources;
use super::load::start_load;
use super::load_state::Load;
use super::panel::open;
use super::platform::utc_now;
use super::refresh::string_now;
use super::state::Panel;
use crate::common::DebugTools;
use crate::connection::ToServer;
use crate::dom::Dom;
use crate::widgets::RaisePanel;

pub(super) fn add(app: &mut App) {
    app.add_debug_method("client.debug_string", debug_string)
        .add_debug_method("client.load_debug_string", load_debug_string);
}

fn debug_string(
    In(_): In<Option<Value>>,
    mut panel: ResMut<Panel>,
    sources: Sources,
    mut dom: Dom,
) -> BrpResult {
    let string = if panel.is_open() {
        string_now(&mut panel, &sources, &mut dom)
    } else {
        encode(&DebugReport {
            utc: utc_now(),
            ..sources.report(&panel.typed)
        })
    };
    let report = shared::debug_report::decode(&string).ok();
    Ok(json!({
        "open": panel.is_open(),
        "string": string,
        "report": report,
    }))
}

#[derive(Deserialize)]
struct LoadParams {
    text: String,
    #[serde(default)]
    set_time: bool,
}

/// `client.load_debug_string { "text": "HOW1.…", "set_time": false }`: opens
/// Debug Info and loads the string, as pasting it and pressing Load does.
/// Answers with the report read (the server's answer shows in the panel), or
/// refuses with why it can't be read or isn't allowed.
fn load_debug_string(
    In(request): In<Option<Value>>,
    mut commands: Commands,
    mut panel: ResMut<Panel>,
    mut load: ResMut<Load>,
    tools: Res<DebugTools>,
    (mut to_server, mut raise): (MessageWriter<ToServer>, MessageWriter<RaisePanel>),
    mut dom: Dom,
) -> BrpResult {
    let LoadParams { text, set_time } = params(request)?;
    open(&mut commands, &mut panel, &mut raise, &mut dom);
    load.pasted.clone_from(&text);
    load.set_time = set_time;
    // Shown afresh, with what is pasted.
    load.shown = None;
    let report = start_load(&text, set_time, *tools, &mut load, &mut to_server).map_err(refused)?;
    Ok(json!({ "report": report }))
}
