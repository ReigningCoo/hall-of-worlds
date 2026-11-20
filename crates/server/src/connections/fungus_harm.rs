//! A fungus's harm to the one eating or handling it: who they are for a deadly
//! drain (`Eater`), whether a poison is recognised before it starts
//! (`fungus::recognised`: known, or a Healing or Mycology check), and the harm
//! from handling one with bare hands or an uncovered mouth
//! (`fungus::handling`, fungi plan PR 9), when picked or used in Alchemy.

use shared::alchemy::{AlchemyOrder, OrderStep};
use shared::brewing::Ingredient;
use shared::difficulty::Difficulty;
use shared::fungus::{Eater, Fungi, FungusDef, Guarded, MYCOLOGY, handling, recognised};
use shared::herb::{skill_percent, skill_value};
use shared::magic::heal_skill;
use shared::rng::Seed;

use super::Client;
use crate::ids::new_id;

/// The client as poisoned at action tick `now`.
pub(super) fn eater(client: &Client, now: u64) -> Eater {
    Eater {
        most_health: client.limits(now).health,
        easy: client.playing_on() == Difficulty::Easy,
    }
}

/// Whether `def`'s poison is recognised before it starts, from a fresh roll.
pub(super) fn recognises(client: &Client, def: &FungusDef) -> bool {
    let healing = client.sheet.skills.percent(heal_skill());
    let mycology = skill_percent(skill_value(&client.sheet.skills, MYCOLOGY));
    let roll = Seed::new(new_id().seed().value()).rng().range_u64(0, 100) as u32;
    recognised(&client.sheet.fungi, def, healing.max(mycology), roll)
}

/// Handling `def` at action tick `now`: its poison laid on if touch or breath
/// brings it on and nothing worn keeps it off. Whether it harmed them.
pub(super) fn handle_fungus(client: &mut Client, def: &FungusDef, now: u64) -> bool {
    let guarded = Guarded::of(&client.inventory);
    let harm = handling(def, guarded, now, eater(client, now));
    if harm.is_empty() {
        return false;
    }
    let known = recognises(client, def);
    let (sheet, situation) = client.sheet_and_situation();
    sheet.lay_on(harm, known, situation, now);
    client.unsaved = true;
    client.refresh_attributes(now);
    true
}

/// The fungi ground in `order`'s Mortars from what is carried, once each, for
/// the harm of handling them (`handle_fungus`).
pub(super) fn fungi_in_order(client: &Client, order: &AlchemyOrder) -> Vec<&'static FungusDef> {
    let ground: Vec<_> = (order.steps.iter())
        .flat_map(|step| match step {
            OrderStep::Mortar { herbs, .. } => herbs.as_slice(),
            _ => &[],
        })
        .collect();
    let mut fungi: Vec<&'static FungusDef> = (client.inventory.herb_stacks().into_iter())
        .filter(|stack| stack.ingredient == Ingredient::Fungus && ground.contains(&&stack.stack))
        .filter_map(|stack| Fungi::built_in().get(&stack.herb))
        .collect();
    fungi.dedup_by(|a, b| a.key == b.key);
    fungi
}
