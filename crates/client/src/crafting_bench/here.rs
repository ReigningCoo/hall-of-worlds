//! The bench the player is at now (`common::BenchHere`): within reach and
//! facing it (`crafting_bench::bench_faced`), as the server checks before
//! Alchemy at a bench.

use bevy::prelude::*;
use shared::crafting_bench::bench_faced;

use crate::common::{BenchHere, Benches, CurrentDungeon, Facing, GameView, SpotPosition};
use crate::player::Player;

/// Keeps `BenchHere` as the player stands and faces; nothing off a level.
pub(super) fn keep_bench_here(
    view: Res<State<GameView>>,
    (dungeon, benches): (Res<CurrentDungeon>, Res<Benches>),
    player: Single<(&SpotPosition, &Facing), With<Player>>,
    mut here: ResMut<BenchHere>,
) {
    let (at, facing) = *player;
    let piece = (*view.get() == GameView::Dungeon)
        .then(|| bench_faced(&dungeon.level, at.0, facing.0))
        .flatten();
    let now = BenchHere(piece.map(|piece| (piece, benches.tools(piece.at).cloned())));
    here.set_if_neq(now);
}
