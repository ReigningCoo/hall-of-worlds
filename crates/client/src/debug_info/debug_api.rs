//! `client.debug_string`: the debug string as the panel would make it now,
//! with the description typed (in the panel or not), and the report it holds.
//! The description is set like any box's, by `client.ui.set` on
//! `debug_info/problem`; the panel opens by `client.ui.press` on
//! `hud/debug_info`. See `docs/context/debug-api.md`.

use bevy::prelude::*;
use server::debug_api::{AddDebugMethod, BrpResult, Value, json};
use shared::debug_report::{DebugReport, encode};

use super::gather::Sources;
use super::platform::utc_now;
use super::refresh::string_now;
use super::state::Panel;
use crate::dom::Dom;

pub(super) fn add(app: &mut App) {
    app.add_debug_method("client.debug_string", debug_string);
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
