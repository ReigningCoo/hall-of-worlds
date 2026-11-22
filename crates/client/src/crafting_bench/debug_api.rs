//! `client.benches`: the crafting benches on the level as the client knows
//! them (where, which way they face, their tools by kind), the one the player
//! is at and the open panel's; and `click: [x, y]` acts as a click or tap on
//! the bench covering that cell. See `docs/context/debug-api.md`.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{AddDebugMethod, BrpResult, Value, json, params, refused};
use shared::furniture::Holder;

use super::BenchPanel;
use crate::common::{BenchHere, Benches, CurrentDungeon};
use crate::dungeon_map::BenchClicked;

pub(super) fn add(app: &mut App) {
    app.add_debug_method("client.benches", benches);
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct BenchesParams {
    /// A cell of the bench to click.
    click: Option<(i64, i64)>,
}

fn benches(
    In(request): In<Option<Value>>,
    (dungeon, told): (Res<CurrentDungeon>, Res<Benches>),
    (here, panel): (Res<BenchHere>, Res<BenchPanel>),
    mut clicked: MessageWriter<BenchClicked>,
) -> BrpResult {
    let asked: BenchesParams = params(request)?;
    if let Some((x, y)) = asked.click {
        let piece = dungeon.level.furniture_at(x, y).copied();
        let piece = piece.filter(|p| Holder::of(p.furniture).is_some());
        let piece = piece.ok_or_else(|| refused(format!("No bench at {x}, {y}")))?;
        clicked.write(BenchClicked { piece });
    }
    let list: Vec<Value> = (dungeon.level.furniture())
        .filter(|piece| Holder::of(piece.furniture).is_some())
        .map(|piece| {
            let held = told.0.get(&piece.at);
            let kinds: Option<Vec<String>> = held.map(|held| {
                (held.items())
                    .map(|item| format!("{:?}", item.kind))
                    .collect()
            });
            json!({
                "at": piece.at,
                "size": piece.size,
                "facing": format!("{:?}", piece.facing),
                "furniture": format!("{:?}", piece.furniture),
                "tools": kinds,
            })
        })
        .collect();
    Ok(json!({
        "benches": list,
        "here": here.at(),
        "panel": panel.0.as_ref().map(|open| open.piece.at),
    }))
}
