//! Showing Load's part of the panel: only to those allowed it, the checkbox
//! only usable with World Time's permission, the reported problem once loaded,
//! and how it went. Changed only when it differs.

use bevy::prelude::*;
use shared::role::DebugTool;

use super::load_state::{Load, Shown};
use super::state::Panel;
use crate::common::DebugTools;
use crate::dom::Dom;

pub(super) fn show_load(
    panel: Res<Panel>,
    mut load: ResMut<Load>,
    tools: Res<DebugTools>,
    mut dom: Dom,
) {
    if !panel.is_open() {
        // Their words go with the panel.
        if load.shown.take().is_some() {
            load.reported = None;
            load.status.clear();
        }
        return;
    }
    let shown = Shown {
        allowed: tools.may(DebugTool::LoadDebugString),
        may_set_time: tools.may(DebugTool::WorldTime),
        reported: load.reported.clone(),
        status: load.status.clone(),
    };
    if load.shown.as_ref() == Some(&shown) {
        return;
    }
    if load.shown.is_none() {
        // Just opened: as it was left.
        dom.set("debug_info/paste", "value", &load.pasted);
    }
    dom.set_flag("debug_info/load", "hidden", !shown.allowed);
    dom.set_flag("debug_info/set_time", "disabled", !shown.may_set_time);
    dom.set_flag(
        "debug_info/set_time",
        "checked",
        load.set_time && shown.may_set_time,
    );
    dom.set_flag(
        "debug_info/reported_box",
        "hidden",
        shown.reported.is_none(),
    );
    let reported = shown.reported.as_deref().unwrap_or_default();
    dom.set("debug_info/reported", "value", reported);
    dom.set_text("debug_info/load_status", shown.status.clone());
    load.shown = Some(shown);
}
