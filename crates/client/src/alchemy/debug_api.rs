//! `client.alchemy`: the Alchemy panel's process, after opening the panel,
//! setting its steps, putting a carried stack in a drop space (or emptying
//! one) and setting the count, if asked; as dragging would, so the panel can
//! be tested by id. Start and the count's buttons are pressed with
//! `client.ui.press` (`alchemy/start`, ...). See `docs/context/debug-api.md`.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{AddDebugMethod, BrpResult, Value, json, params, refused};
use shared::alchemy::Apparatus;
use shared::id::Id;

use super::OpenAlchemy;
use super::bench::{Bench, Station};
use super::spaces::{Place, put, takes};
use crate::inventory::Carried;

pub(super) fn add(app: &mut App) {
    app.add_debug_method("client.alchemy", alchemy);
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct AlchemyParams {
    /// Opens the panel if it is closed.
    open: bool,
    /// The steps, emptied, by apparatus names (`"Mortar"`, `"Retort"`).
    steps: Option<Vec<Apparatus>>,
    /// A stack put in a space by its widget id's name: `source`,
    /// `herb.<step × 100 + n>`, `oil.<step>`, `liquid.<step × 100 + n>`;
    /// no `stack` empties it.
    put: Option<PutParams>,
    count: Option<u32>,
}

#[derive(Deserialize)]
struct PutParams {
    space: String,
    stack: Option<Id>,
}

fn alchemy(
    In(request): In<Option<Value>>,
    mut bench: ResMut<Bench>,
    carried: Res<Carried>,
    mut open: MessageWriter<OpenAlchemy>,
) -> BrpResult {
    let asked: AlchemyParams = params(request)?;
    if asked.open && !bench.is_open() {
        open.write(OpenAlchemy);
    }
    if let Some(steps) = asked.steps {
        bench.steps = steps.into_iter().map(Station::new).collect();
        bench.made = None;
    }
    if let Some(PutParams { space, stack }) = asked.put {
        let place = place_named(&space).ok_or_else(|| refused(format!("No space {space}")))?;
        if let Some(id) = stack
            && !takes(&bench, &carried.0, place, id)
        {
            return Err(refused(format!("{space} doesn't take {id}")));
        }
        put(&mut bench, place, stack);
    }
    if let Some(count) = asked.count {
        bench.count = count.max(1);
    }
    let steps: Vec<Value> = (bench.steps.iter())
        .map(|station| {
            json!({
                "apparatus": station.apparatus,
                "herbs": station.herbs,
                "oil": station.oil,
                "liquids": station.liquids,
            })
        })
        .collect();
    Ok(json!({
        "open": bench.is_open(),
        "source": bench.source,
        "steps": steps,
        "count": bench.count,
        "most_runs": bench.most_runs(&carried.0),
        "running": bench.running.as_ref().map(|running| running.started),
        "made": bench.made.as_ref().map(|item| format!("{:?} {:?}", item.quality, item.kind)),
    }))
}

/// A space by its widget id's name.
fn place_named(name: &str) -> Option<Place> {
    if name == "source" {
        return Some(Place::Source);
    }
    let (kind, at) = name.split_once('.')?;
    let at: usize = at.parse().ok()?;
    match kind {
        "herb" => Some(Place::Herb(at / 100, at % 100)),
        "oil" => Some(Place::Oil(at)),
        "liquid" => Some(Place::Liquid(at / 100, at % 100)),
        _ => None,
    }
}
