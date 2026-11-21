//! Coming to a bench: a click or tap on it turns to it and opens its panel
//! if it is in reach, or else walks to the cell in front of it and does so
//! on arriving; walking anywhere else calls that off.

use bevy::prelude::*;
use shared::crafting_bench::Craft;
use shared::furniture::{Furniture, Piece};
use shared::spot::Spot;

use super::markup::BENCH;
use super::state::{OpenBench, front_cell, in_reach, nearest_cell};
use super::{AT, BenchHeading, BenchPanel, COLUMNS};
use crate::common::{Benches, CurrentDungeon, FaceTowards, SpotPosition};
use crate::dom::{Dom, document, top_strip};
use crate::dungeon_map::BenchClicked;
use crate::player::{Arrived, Player, TakeStep, TravelTo, Walking};
use crate::widgets::StockGrid;

/// Walking anywhere else, or a step of their own: they aren't going to it.
pub(super) fn change_of_mind(
    (mut steps, mut walking): (MessageReader<TakeStep>, MessageReader<Walking>),
    mut journeys: MessageReader<TravelTo>,
    mut heading: ResMut<BenchHeading>,
) {
    let stepped = steps.read().count() + walking.read().count() > 0;
    let front = heading.0.map(|(_, front)| front);
    let elsewhere = (journeys.read()).any(|journey| Some((journey.x, journey.y)) != front);
    if stepped || elsewhere {
        heading.0 = None;
    }
}

/// A click on a bench: its panel at once in reach, else a walk to it.
#[allow(clippy::too_many_arguments)]
pub(super) fn follow_clicked_bench(
    mut clicks: MessageReader<BenchClicked>,
    (dungeon, benches): (Res<CurrentDungeon>, Res<Benches>),
    player: Single<&SpotPosition, With<Player>>,
    mut heading: ResMut<BenchHeading>,
    mut panel: ResMut<BenchPanel>,
    (mut travel, mut face): (MessageWriter<TravelTo>, MessageWriter<FaceTowards>),
    mut dom: Dom,
    mut commands: Commands,
) {
    let Some(BenchClicked { piece }) = clicks.read().last().copied() else {
        return;
    };
    let at = player.0;
    if in_reach(at, &piece) {
        heading.0 = None;
        face.write(FaceTowards(Spot::middle_of(nearest_cell(at, &piece))));
        open_panel(&mut panel, piece, &benches, &mut dom, &mut commands);
    } else if let Some((x, y)) = front_cell(&dungeon.level, at, &piece) {
        heading.0 = Some((piece, (x, y)));
        travel.write(TravelTo { x, y });
    }
}

/// Arriving in front of the bench walked to turns to it and opens its panel.
pub(super) fn open_on_arrival(
    mut arrivals: MessageReader<Arrived>,
    mut heading: ResMut<BenchHeading>,
    benches: Res<Benches>,
    mut panel: ResMut<BenchPanel>,
    mut face: MessageWriter<FaceTowards>,
    mut dom: Dom,
    mut commands: Commands,
) {
    let Some(&Arrived { x, y }) = arrivals.read().last() else {
        return;
    };
    if let Some((piece, front)) = heading.0.take()
        && front == (x, y)
    {
        let here = Spot::middle_of((x, y));
        face.write(FaceTowards(Spot::middle_of(nearest_cell(here, &piece))));
        open_panel(&mut panel, piece, &benches, &mut dom, &mut commands);
    }
}

/// Opens `piece`'s panel, closing another bench's; raises it if open.
fn open_panel(
    panel: &mut BenchPanel,
    piece: Piece,
    benches: &Benches,
    dom: &mut Dom,
    commands: &mut Commands,
) {
    if let Some(open) = &panel.0 {
        if open.piece.at == piece.at {
            dom.raise("bench/panel");
            return;
        }
        commands.entity(open.panel).try_despawn();
    }
    let Furniture::CraftingBench { craft } = piece.furniture else {
        return;
    };
    let entity = commands.spawn(document(&BENCH)).insert(top_strip()).id();
    dom.set("bench/panel", "title", title(craft));
    dom.start_at("bench/panel", AT);
    let tools = benches.0.get(&piece.at);
    let tools = tools.map(super::server::shown).unwrap_or_default();
    panel.0 = Some(OpenBench {
        piece,
        panel: entity,
        holder: None,
        grid: StockGrid::new(Benches::id_of(piece.at), COLUMNS, &tools),
    });
}

/// A bench's panel's title, by its craft.
pub(super) fn title(craft: Craft) -> &'static str {
    craft.name()
}
