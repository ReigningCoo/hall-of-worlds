//! Telling players what lies on furniture with places (benches' tools, cask
//! racks' casks): a piece's when its cells have been sent on the visit (so
//! unseen pieces stay unknown), and again when it changes, or when a hearth
//! goes out or is lit. Food in a Range's storage and casks on a rack are
//! settled to now as they are told (`settle_held`).

use shared::furniture::Holder;
use shared::protocol::{BenchNews, SeenBench, ServerMessage, level_chunk};

use super::{BenchKey, bench_tools, settle_held};
use crate::ServerWorld;
use crate::connections::Client;
use crate::sites::campfires::fire_now;

/// Every tick: sends each player what lies on every piece with places on
/// their level whose cell they have been sent and haven't been told of, and
/// on those that changed this tick.
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
        let pieces = visit.level.level.furniture();
        for piece in pieces.filter(|p| Holder::of(p.furniture).is_some()) {
            if !sent(visit, piece.at) {
                continue;
            }
            let Some(bench) = visit.held_at(piece.at.0, piece.at.1) else {
                continue;
            };
            let told = visit.benches_sent.get(&piece.at).copied();
            let lit = match bench.holder {
                Holder::Bench(craft) => {
                    craft.is_hearth() && bench_tools(world, bench.key, craft).is_lit(now)
                }
                Holder::Casks(_) => false,
            };
            if told != Some(lit) || changes.contains(&bench.key) {
                seen.push((piece.at, bench));
            }
        }
        if seen.is_empty() {
            continue;
        }
        let mut told = Vec::with_capacity(seen.len());
        for (at, bench) in seen {
            let held = settle_held(world, bench.key, bench.holder, celsius);
            let lit = held.bench().is_some_and(|tools| tools.is_lit(now));
            visit.benches_sent.insert(at, lit);
            told.push(SeenBench { at, held, lit });
        }
        // Gone: noticed when its messages are next read.
        let _ = client
            .link
            .send(&ServerMessage::Bench(BenchNews::Tools(told)));
    }
}
