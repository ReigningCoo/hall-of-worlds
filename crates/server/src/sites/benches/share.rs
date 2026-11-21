//! Telling players of benches' tools: a bench's when its cells have been
//! sent on the visit (so unseen benches stay unknown), and again when they
//! change, or when a hearth goes out or is lit. Food in a Range's storage is
//! settled to now as it is told (`settle_storage`).

use shared::furniture::Furniture;
use shared::protocol::{BenchNews, SeenBench, ServerMessage, level_chunk};

use super::{BenchKey, bench_tools, settle_storage};
use crate::ServerWorld;
use crate::connections::Client;
use crate::sites::campfires::fire_now;

/// Every tick: sends each player the tools of every bench on their level
/// whose cell they have been sent and haven't had its tools for, and of those
/// that changed this tick.
pub(crate) fn share_benches(clients: &mut [Client], world: &mut ServerWorld) {
    let changes: Vec<BenchKey> = std::mem::take(&mut world.bench_changes);
    let now = fire_now(world);
    for client in clients.iter_mut() {
        let celsius = world.habitat_at(client.world_position()).celsius;
        let Some(visit) = &mut client.visit else {
            continue;
        };
        let sent = |visit: &super::Visit, (x, y): (i64, i64)| {
            let (chunk, bit) = level_chunk(x, y);
            (visit.sent.get(&chunk)).is_some_and(|mask| mask & (1 << bit) != 0)
        };
        // The client lets go of cells the wilderness's window moves off.
        let kept: Vec<_> = (visit.benches_sent.iter())
            .map(|(&at, &lit)| (at, lit))
            .filter(|&(at, _)| sent(visit, at))
            .collect();
        visit.benches_sent = kept.into_iter().collect();
        let mut seen = Vec::new();
        let benches = visit.level.level.furniture();
        for piece in benches.filter(|p| matches!(p.furniture, Furniture::CraftingBench { .. })) {
            if !sent(visit, piece.at) {
                continue;
            }
            let Some(bench) = visit.bench_at(piece.at.0, piece.at.1) else {
                continue;
            };
            let told = visit.benches_sent.get(&piece.at).copied();
            let lit =
                bench.craft.is_hearth() && bench_tools(world, bench.key, bench.craft).is_lit(now);
            if told != Some(lit) || changes.contains(&bench.key) {
                seen.push((piece.at, bench));
            }
        }
        if seen.is_empty() {
            continue;
        }
        let mut told = Vec::with_capacity(seen.len());
        for (at, bench) in seen {
            let tools = settle_storage(world, bench.key, bench.craft, celsius);
            let lit = tools.is_lit(now);
            visit.benches_sent.insert(at, lit);
            told.push(SeenBench { at, tools, lit });
        }
        // Gone: noticed when its messages are next read.
        let _ = client
            .link
            .send(&ServerMessage::Bench(BenchNews::Tools(told)));
    }
}
