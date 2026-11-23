//! `client.spell_design`: the spell design panel's design, after opening the
//! panel and setting its steps, if asked, as clicking and dragging would, so
//! the panel can be tested by id. Save and Clear are pressed with
//! `client.ui.press` (`spell_design/save`, ...). See
//! `docs/context/debug-api.md`.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{AddDebugMethod, BrpResult, Value, json, params};
use shared::spell::{Step, spell_cost};

use super::OpenSpellDesign;
use super::draft::Draft;
use super::side::can_store;
use crate::character::CharacterSheet;

pub(super) fn add(app: &mut App) {
    app.add_debug_method("client.spell_design", spell_design);
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct DesignParams {
    /// Opens the panel if it is closed.
    open: bool,
    /// The steps, as `shared::spell::Step` in JSON.
    steps: Option<Vec<Step>>,
}

fn spell_design(
    In(request): In<Option<Value>>,
    mut draft: ResMut<Draft>,
    sheet: Res<CharacterSheet>,
    mut open: MessageWriter<OpenSpellDesign>,
) -> BrpResult {
    let asked: DesignParams = params(request)?;
    if asked.open && !draft.is_open() {
        open.write(OpenSpellDesign);
    }
    if let Some(steps) = asked.steps {
        draft.set_steps(steps);
    }
    let spell = draft.spell();
    let cost = spell_cost(&spell, Some(&sheet.0.skills)).shown();
    Ok(json!({
        "open": draft.is_open(),
        "steps": spell.steps,
        "name": draft.name,
        "cost": cost,
        "saving": draft.naming,
        "problem": can_store(&draft, &sheet.0).err(),
    }))
}
