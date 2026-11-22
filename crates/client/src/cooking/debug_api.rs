//! `client.cooking`: the Cooking panel's process, after opening the panel,
//! setting its steps, putting a stack in a drop space (or emptying one) and
//! setting the count, if asked; as dragging would, so the panel can be tested
//! by id. Start and the count's buttons are pressed with `client.ui.press`
//! (`cooking/start`, ...). See `docs/context/debug-api.md`.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{AddDebugMethod, BrpResult, Value, json, params, refused};
use shared::cooking::Tool;
use shared::id::Id;

use super::OpenCooking;
use super::bench::{Bench, Station};
use super::kitchen::Kitchens;
use super::spaces::{Place, more_faced, put, takes};
use crate::inventory::Carried;

pub(super) fn add(app: &mut App) {
    app.add_debug_method("client.cooking", cooking);
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct CookingParams {
    /// Opens the panel if it is closed.
    open: bool,
    /// The steps, emptied: `{"tool": "Pan", "column": 0, "row": 0,
    /// "height": 1}`.
    steps: Option<Vec<StepParams>>,
    /// A stack put in a space by its widget id's name: `source.<row>`,
    /// `food.<step × 100 + n>`, `liquid.<step × 100 + n>`, `faced.<step>`
    /// (100 ml more), `culture`; no `stack` empties it. Putting a stack again
    /// adds one more (or 100 ml).
    put: Option<PutParams>,
    count: Option<u32>,
}

#[derive(Deserialize)]
struct StepParams {
    tool: Tool,
    column: u8,
    #[serde(default)]
    row: u8,
    #[serde(default)]
    height: Option<u8>,
}

#[derive(Deserialize)]
struct PutParams {
    space: String,
    stack: Option<Id>,
}

fn cooking(
    In(request): In<Option<Value>>,
    mut bench: ResMut<Bench>,
    (carried, kitchens): (Res<Carried>, Res<Kitchens>),
    mut open: MessageWriter<OpenCooking>,
) -> BrpResult {
    let asked: CookingParams = params(request)?;
    if asked.open && !bench.is_open() {
        open.write(OpenCooking);
    }
    if let Some(steps) = asked.steps {
        bench.steps = (steps.into_iter())
            .map(|step| Station {
                height: step.height.unwrap_or(1).max(1),
                ..Station::new(step.tool, step.column, step.row)
            })
            .collect();
        bench.wanted_sources = Default::default();
        bench.made = None;
    }
    if let Some(PutParams { space, stack }) = asked.put {
        let place = place_named(&space).ok_or_else(|| refused(format!("No space {space}")))?;
        match (place, stack) {
            (Place::Faced(step), Some(_)) => more_faced(&mut bench, step),
            (_, Some(id)) if !takes(&carried.0, &kitchens.0, place, id) => {
                return Err(refused(format!("{space} doesn't take {id}")));
            }
            _ => put(&mut bench, &carried.0, place, stack),
        }
    }
    if let Some(count) = asked.count {
        bench.count = count.max(1);
    }
    let steps: Vec<Value> = (bench.steps.iter())
        .map(|step| {
            json!({
                "tool": step.tool,
                "column": step.column,
                "row": step.row,
                "height": step.height,
                "foods": step.foods,
                "liquids": step.liquids,
                "faced": step.faced,
                "ghosts": (step.ghosts(&carried.0).iter())
                    .map(|ghost| json!({"input": ghost.input, "missing": ghost.missing}))
                    .collect::<Vec<Value>>(),
            })
        })
        .collect();
    Ok(json!({
        "open": bench.is_open(),
        "sources": bench.sources,
        "wanted_sources": bench.wanted_sources.iter().map(|rest| rest.map(|r| r.name())).collect::<Vec<_>>(),
        "missing": bench.missing_any(&carried.0),
        "steps": steps,
        "culture": bench.culture,
        "count": bench.count,
        "most_runs": bench.most_runs(&carried.0),
        "kitchen": format!("{:?} {:?}", kitchens.0.place, kitchens.0.heat),
        "running": bench.running.as_ref().map(|running| running.started),
        "made": bench.made.as_ref().map(|item| format!("{:?} {:?} ×{}", item.quality, item.kind, item.count)),
    }))
}

/// A space by its widget id's name.
fn place_named(name: &str) -> Option<Place> {
    if name == "culture" {
        return Some(Place::Culture);
    }
    let (kind, at) = name.split_once('.')?;
    let at: usize = at.parse().ok()?;
    match kind {
        "source" => Some(Place::Source(u8::try_from(at).ok()?)),
        "food" => Some(Place::Food(at / 100, at % 100)),
        "liquid" => Some(Place::Liquid(at / 100, at % 100)),
        "faced" => Some(Place::Faced(at)),
        _ => None,
    }
}
