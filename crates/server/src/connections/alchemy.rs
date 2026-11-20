//! A player running an Alchemy process (`Ability::Alchemy`, Alchemy plan PR 3):
//! the client sends an `AlchemyOrder` (`ClientMessage::Alchemy`), kept here
//! while each run winds up for the steps' times (`alchemy::run_ticks`). When a
//! run lands the order is checked again against what is carried
//! (`Inventory::check_alchemy`), the ingredients are used up, the chance and
//! luck are rolled from a fresh seed (`alchemy::outcome`), and the product is
//! given: a Paste, Ointment or Powder, or the liquids' bottles (a tincture's pure
//! alcohol's first) holding the potion or pure alcohol (one per 100 ml left),
//! the rest empty, and any Salt an Alembic left. The sub-skills used and
//! Herbalism are practised either way; a success teaches one herb effect (`alchemy::lesson`).
//! A fungus ground that harms by touch or breath harms the alchemist as picking
//! it would (`fungus_harm::handle_fungus`).
//! Then the next run begins, until `count` runs are done or the ingredients run
//! out. Moving or being harmed calls it off, repeats and all
//! (`stop_collecting`). `can_use` has already checked an apparatus is carried.

use bevy::prelude::*;
use shared::ability::Ability;
use shared::alchemy::{
    AlchemyOrder, AlchemySkills, CERTAIN, HERB_PRACTICE, Known, OrderStep, Outcome, Product,
    STEP_PRACTICE, lesson, new_mixture, new_salt, outcome, run_ticks,
};
use shared::brewing::brewed_into;
use shared::herb::HERBALISM;
use shared::item::{Item, ItemKind, Quality};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Liquid};
use shared::protocol::{Alchemised, Landed, ServerMessage};
use shared::rng::Seed;

use super::Client;
use super::action::Action;
use super::alchemy_bench::order_bench;
use super::fungus_harm::{fungi_in_order, handle_fungus};
use super::give::{give, practise};
use crate::ServerWorld;
use crate::combat::{Act, Pending};
use crate::ids::new_id;

/// `ClientMessage::Alchemy`: the order kept for when it begins, and Alchemy
/// asked for like any ability, one at a time. Always stays.
pub(super) fn ask_alchemy(client: &mut Client, order: AlchemyOrder) -> bool {
    client.alchemy = Some(order);
    if !(client.waiting.iter()).any(|a| matches!(a, Action::UseAbility { .. })) {
        client.waiting.push_back(Action::UseAbility {
            ability: Ability::Alchemy,
            target: None,
        });
    }
    true
}

/// Alchemy asked for with the order kept (`alchemy`): a run begins if what is
/// carried allows it. Whether the client is still there, and what it cost.
pub(super) fn start_alchemy(client: &mut Client, world: &mut ServerWorld) -> (bool, u32) {
    let skills = AlchemySkills::of(&client.sheet.skills);
    let Some(order) = client.alchemy.as_ref() else {
        return (true, 0);
    };
    let Ok(bench) = order_bench(client, world, order) else {
        debug!("Not running alchemy: not at the bench asked for");
        client.alchemy = None;
        return (true, 0);
    };
    let checked = (client.inventory).check_alchemy_at(order, &world.herbs, skills, bench.as_ref());
    if let Err(why) = checked {
        debug!("Not running alchemy: {why:?}");
        client.alchemy = None;
        return (true, 0);
    }
    let steps: Vec<_> = order.steps.iter().map(OrderStep::apparatus).collect();
    let tick = world.action_clock.ticks();
    let lands_in = run_ticks(&steps, skills);
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::Alchemy,
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::Alchemy,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    // As long as the wind-up, so the player acts again once it lands.
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// A run's wind-up is over: what it makes from what is still carried, or
/// nothing if that changed; the next run asked for if any are left. `false`
/// if the client has gone.
pub(super) fn land_alchemy(client: &mut Client, world: &mut ServerWorld) -> bool {
    let tick = world.action_clock.ticks();
    let Some(mut order) = client.alchemy.take() else {
        return true;
    };
    let skills = AlchemySkills::of(&client.sheet.skills);
    let bench = order_bench(client, world, &order);
    let checked = bench.and_then(|bench| {
        let herbs = &world.herbs;
        (client
            .inventory
            .check_alchemy_at(&order, herbs, skills, bench.as_ref()))
        .map_err(|_| ())
    });
    let Ok(checked) = checked else {
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    let mut salt = 0;
    for fungus in fungi_in_order(client, &order) {
        handle_fungus(client, fungus, tick);
    }
    let bottles = use_up(client, world, &order);
    let mut rng = Seed::new(new_id().seed().value()).rng();
    let chance_roll = rng.range_u64(0, u64::from(CERTAIN)) as u32;
    let luck_roll = rng.range_u64(0, u64::from(CERTAIN)) as u32;
    let planned = &checked.planned;
    let made = match outcome(planned, chance_roll, luck_roll) {
        Outcome::Failed => {
            bottles
                .into_iter()
                .for_each(|b| give(client, world, brewed_into(b, None)));
            Alchemised::Failed
        }
        Outcome::Made { product, quality } => {
            let sheet = &mut client.sheet;
            let known = Known {
                herbs: &sheet.herbs,
                fungi: &sheet.fungi,
            };
            if let Some(lesson) = lesson(planned, known) {
                lesson.learn(&mut sheet.herbs, &mut sheet.fungi);
            }
            salt = planned.salt;
            if salt > 0 {
                give(client, world, new_salt(salt, new_id()));
            }
            give_product(client, world, product, quality, bottles)
        }
    };
    for apparatus in &planned.practised {
        practise(client, apparatus.sub_skill(), STEP_PRACTICE, tick);
    }
    let handled = HERB_PRACTICE * checked.herbs.len() as u32;
    practise(client, HERBALISM, handled, tick);
    order.count -= 1;
    let left = order.count;
    if left > 0 {
        ask_alchemy(client, order);
    }
    client.unsaved = true;
    client.update_speed(tick);
    client.send_inventory()
        && client.send_sheet(tick)
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Alchemy { made, salt, left },
        })
}

/// Takes what one run uses: the source, one of each herb, the solvent (an
/// oil's bottle given back empty, pure alcohol's kept as a tincture's), the
/// liquids' bottles, returned for the product, the Retort's solid, the
/// Alembic's alcohol (its bottle given back empty) and the Filter's Powder.
fn use_up(client: &mut Client, world: &mut ServerWorld, order: &AlchemyOrder) -> Vec<Item> {
    let mut bottles = Vec::new();
    if let Some(source) = order.source {
        client.inventory.take_one(source, &mut new_id);
    }
    for step in &order.steps {
        match step {
            OrderStep::Mortar { herbs, solvent } => {
                for &herb in herbs {
                    client.inventory.take_one(herb, &mut new_id);
                }
                let solvent = solvent.and_then(|id| client.inventory.take_one(id, &mut new_id));
                match solvent {
                    Some(bottle) if holds(&bottle, &Liquid::Alcohol) => bottles.push(bottle),
                    Some(bottle) => give(client, world, brewed_into(bottle, None)),
                    None => {}
                }
            }
            OrderStep::Retort { liquids, solid } => {
                for &liquid in liquids {
                    bottles.extend(client.inventory.take_one(liquid, &mut new_id));
                }
                if let Some(solid) = solid {
                    client.inventory.take_one(*solid, &mut new_id);
                }
            }
            OrderStep::Alembic { alcohol } => {
                let fed = alcohol.and_then(|id| client.inventory.take_one(id, &mut new_id));
                if let Some(bottle) = fed {
                    give(client, world, brewed_into(bottle, None));
                }
            }
            OrderStep::Filter {
                powder: Some(powder),
            } => {
                client.inventory.take_one(*powder, &mut new_id);
            }
            OrderStep::Filter { powder: None } | OrderStep::Crucible | OrderStep::Pelican => {}
        }
    }
    bottles
}

/// Whether `bottle` holds `liquid`.
fn holds(bottle: &Item, liquid: &Liquid) -> bool {
    bottle.bottle().and_then(|b| b.liquid.as_ref()) == Some(liquid)
}

/// `bottle` filled with pure alcohol, which has no quality.
fn alcohol_in(mut bottle: Item) -> Item {
    let look = bottle.bottle().map_or(0, |bottle| bottle.look);
    bottle.quality = None;
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(
        Some(Liquid::Alcohol),
        look,
    )));
    bottle
}

/// Gives what a run made, at its quality: a mixture, or the first bottles
/// holding the potion or pure alcohol (one per 100 ml the process ended
/// with), the rest empty, as they all are for nothing or when it fails.
fn give_product(
    client: &mut Client,
    world: &mut ServerWorld,
    product: Product,
    quality: Option<Quality>,
    bottles: Vec<Item>,
) -> Alchemised {
    let quality = quality.unwrap_or_default();
    let filled = match &product {
        Product::Potion { bottles, .. } | Product::Alcohol { bottles } => *bottles as usize,
        _ => 0,
    };
    let mut count = 0;
    for (at, bottle) in bottles.into_iter().enumerate() {
        let item = match &product {
            Product::Potion { contents, .. } if at < filled => {
                let mut potion = brewed_into(bottle, Some(contents.clone()));
                potion.quality = Some(quality);
                potion
            }
            Product::Alcohol { .. } if at < filled => alcohol_in(bottle),
            _ => brewed_into(bottle, None),
        };
        count += u32::from(at < filled);
        give(client, world, item);
    }
    match product {
        Product::Mixture { form, contents } => {
            let item = new_mixture(form, contents, quality, 1, new_id());
            let kind = item.kind;
            give(client, world, item);
            Alchemised::Made {
                kind,
                count: 1,
                quality,
            }
        }
        Product::Potion { .. } => Alchemised::Made {
            kind: ItemKind::Bottle,
            count,
            quality,
        },
        Product::Alcohol { .. } => Alchemised::Alcohol { count },
        Product::Nothing => Alchemised::Nothing,
    }
}
