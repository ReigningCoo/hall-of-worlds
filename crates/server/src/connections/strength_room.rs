//! A player's own grid sized by Strength (`shared::item::cells_for_strength`,
//! owner, 2026-11-17). A change the player makes to what they wear or the effects
//! on them is refused if fewer cells would cut what lies there
//! (`MoveRefused::StrengthInUse`); a loss nobody chose (an effect running out)
//! is taken in every tick, moving what lies on cells cut to free room and
//! dropping what has none, but only where there is ground to drop it on.

use bevy::prelude::*;
use shared::bonus::Effect;
use shared::fruit::Fruits;
use shared::fungus::Fungi;
use shared::herb::{HERBALISM, eating_herb, skill_value};
use shared::id::Id;
use shared::item::{Inventory, Item, MoveRefused, Quality, StackLimits, cells_for_strength};
use shared::sheet::{Sheet, Situation};
use shared::vegetable::Vegetables;

use super::Client;
use crate::ServerWorld;
use crate::ids::new_id;

impl Client {
    /// Whether the player may make this change, by their Strength after it:
    /// `after` is what they would carry and wear, `sheet` what they would be.
    pub(crate) fn strength_allows(
        &self,
        after: &Inventory,
        sheet: &Sheet,
        now: u64,
    ) -> Result<(), MoveRefused> {
        if self.god_mode {
            return Ok(());
        }
        let strength = sheet.attributes_with(&after.worn, now).strength;
        after.check_own_cells(cells_for_strength(strength))
    }

    /// The same for a change to what they carry and wear, with their sheet as it
    /// is: said in the log if refused, and whether it is allowed.
    pub(crate) fn change_allowed(&self, change: impl FnOnce(&mut Inventory), now: u64) -> bool {
        let allowed = self.change_refused(change, now);
        if let Err(why) = allowed {
            warn!(
                "{} refused a change to what it wears: {why:?}",
                self.in_log()
            );
        }
        allowed.is_ok()
    }

    /// Why the player may not make this change to what they carry and wear, by
    /// their Strength after it, if they may not.
    fn change_refused(
        &self,
        change: impl FnOnce(&mut Inventory),
        now: u64,
    ) -> Result<(), MoveRefused> {
        let mut after = self.inventory.clone();
        change(&mut after);
        if after.worn == self.inventory.worn {
            return Ok(());
        }
        self.strength_allows(&after, &self.sheet, now)
    }
}

/// Whether the player may eat `food`, the top one of `item` (carried or lying in
/// reach), by their Strength after its effects: carried, it leaves their grid too.
pub(super) fn eating_allowed(
    client: &Client,
    world: &ServerWorld,
    (item, food): (Id, &Item),
    now: u64,
) -> bool {
    let mut sheet = client.sheet.clone();
    let situation = Situation {
        worn: &client.bonused.wearing,
        playtime: client.activity.played,
        refill: client.playing_on().refill_ticks(),
    };
    match food.herb() {
        _ if food.is_potion() => {
            let bottle = food.bottle().map(|it| it.contents().clone());
            let quality = food.quality.unwrap_or(Quality::Common);
            let drunk = shared::brewing::drunk(&bottle.unwrap_or_default(), quality, now);
            for effect in drunk.effects {
                sheet.add_effect(effect, situation, now);
            }
        }
        Some(form) => {
            let herbalism = skill_value(&sheet.skills, HERBALISM);
            let quality = food.quality.unwrap_or(Quality::Common);
            let effects =
                (world.herbs.get(&form.herb)).map(|def| eating_herb(def, quality, herbalism, now));
            for effect in effects.into_iter().flatten() {
                sheet.add_effect(effect, situation, now);
            }
        }
        None => match eaten_effects(food, now) {
            Some(effects) => {
                for effect in effects {
                    sheet.add_effect(effect, situation, now);
                }
            }
            None => {
                sheet.eat(food.kind, client.inventory.creature, situation, now);
            }
        },
    }
    let mut after = client.inventory.clone();
    after.take_one(item, &mut new_id);
    let allowed = client.strength_allows(&after, &sheet, now);
    if allowed.is_err() {
        warn!(
            "{} refused eating {:?}: its Strength is needed",
            client.in_log(),
            food.id
        );
    }
    allowed.is_ok()
}

const NO_EATER: shared::fungus::Eater = shared::fungus::Eater {
    most_health: 0,
    easy: false,
};

/// What eating a fruit, vegetable or fungus does (`fruit::eating`,
/// `vegetable::eating`, `fungus::eating`), or `None` for anything else.
fn eaten_effects(food: &Item, now: u64) -> Option<Vec<Effect>> {
    if let Some(form) = food.fruit() {
        return Fruits::built_in()
            .get(&form.fruit)
            .map(|def| shared::fruit::eating(def, now));
    }
    if let Some(form) = food.fungus() {
        return Fungi::built_in()
            .get(&form.fungus)
            // Only attributes matter here, not how hard a deadly poison drains.
            .map(|def| shared::fungus::eating(def, now, NO_EATER));
    }
    let form = food.vegetable()?;
    let def = Vegetables::built_in().get(&form.vegetable)?;
    Some(shared::vegetable::eating(def, now))
}

/// On joining, its attributes worked out: sizes its own grid by Strength if
/// nothing would drop (old saves only grow); otherwise `fit_to_strength` will.
pub(super) fn size_on_joining(client: &mut Client, limits: StackLimits) {
    let cells = cells_for_strength(client.bonused.attributes.strength);
    let mut after = client.inventory.clone();
    if after.start_sizing(cells, limits, &mut new_id).is_empty() {
        client.inventory = after;
    }
}

/// Every tick: sizes the player's own grid by their Strength now. What lies on
/// cells cut moves to free room, or drops under them; with no ground here (the
/// world map, the Hall) it waits until there is. Whether the client is still there.
pub(super) fn fit_to_strength(client: &mut Client, world: &mut ServerWorld) -> bool {
    let cells = cells_for_strength(client.bonused.attributes.strength);
    let carried = &client.inventory;
    if carried.sized_by_strength() && carried.own_cells() == cells && carried.size.0 == 8 {
        return true;
    }
    let mut after = client.inventory.clone();
    let dropped = after.start_sizing(cells, world.stacks, &mut new_id);
    if after == client.inventory {
        return true;
    }
    if !dropped.is_empty() && client.visit.is_none() {
        return true;
    }
    client.inventory = after;
    for item in dropped {
        info!(
            "{} dropped {:?}, losing room with Strength",
            client.in_log(),
            item.id
        );
        crate::ground::put_down(client, world, item);
        client.ground_told.again = true;
    }
    client.unsaved = true;
    client.update_speed(world.action_clock.ticks());
    client.send_inventory()
}
