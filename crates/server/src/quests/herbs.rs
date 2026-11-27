//! Gathering steps' herbs (owner, 2026-11-27; `shared::quest::HerbAsk`): a
//! step's count is how many of its herb the character carries, anywhere in
//! what they carry, however they got them. It is worked out again whenever
//! their inventory changes (`Client::send_inventory` marks it), on taking the
//! quest and before handing it in, so it goes down as well as up: the step
//! holds while enough is carried. Taking the quest teaches the herb's name.

use shared::quest::{Quest, QuestId};

use super::{cache, done_flags, log, tell_markers};
use crate::ServerWorld;
use crate::connections::Client;

/// Counts every gathering step of the character's quests on this planet
/// again, telling the client of any that changed. `false` if it has gone.
pub(super) fn recount_all(client: &mut Client, world: &mut ServerWorld, now: i64) -> bool {
    let ids: Vec<QuestId> = client.character.quests.taken.keys().copied().collect();
    let mut sent = true;
    for id in ids {
        if let Some(quest) = cache::quest(world, id)
            && recount(client, &quest)
        {
            sent &=
                log::tell_changed(client, world, id) && tell_markers(client, world, id.town, now);
        }
    }
    sent
}

/// Counts `quest`'s gathering steps from what is carried, and whether it is
/// ready; whether anything changed.
pub(super) fn recount(client: &mut Client, quest: &Quest) -> bool {
    if quest.steps.iter().all(|step| step.herb.is_none()) {
        return false;
    }
    let Some(progress) = client.character.quests.taken.get_mut(&quest.id) else {
        return false;
    };
    progress.counts.resize(quest.steps.len(), 0);
    let mut changed = false;
    for (i, step) in quest.steps.iter().enumerate() {
        let Some(ask) = &step.herb else {
            continue;
        };
        let carried = ask.carried(&client.inventory).min(step.count.unwrap_or(1));
        changed |= progress.counts[i] != carried;
        progress.counts[i] = carried;
    }
    let ready = quest.ready_to_turn_in(&done_flags(quest, &progress.counts));
    changed |= progress.ready != ready;
    progress.ready = ready;
    if changed {
        client.unsaved = true;
    }
    changed
}

/// A quest just taken: its herbs' names are learned and its gathering steps
/// counted. `false` if the client has gone.
pub(super) fn taken(client: &mut Client, world: &ServerWorld, quest: &Quest) -> bool {
    recount(client, quest);
    let mut learned = false;
    for ask in quest.steps.iter().filter_map(|step| step.herb.as_ref()) {
        learned |= client.sheet.herbs.learn_name(&ask.herb);
    }
    if !learned {
        return true;
    }
    client.unsaved = true;
    client.send_sheet(world.action_clock.ticks())
}
