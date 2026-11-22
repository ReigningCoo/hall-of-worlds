//! Letting go of what is held: taking it from someone, wearing, dropping, giving
//! (or putting it on a crafting bench), binning or moving it, as aimed. The server is told and decides; what it is
//! sure to agree with is shown at once.

use bevy::prelude::*;
use shared::bonus::{may_use_on_self, usable_on_self};
use shared::energy::handling_cost;
use shared::item::Item;
use shared::protocol::{BenchAsk, ClientMessage};

use super::super::hold::{Target, stack_onto};
use super::super::quest_drop::holds_quest_item;
use super::super::wear_note::{refusal, try_wearing};
use super::super::{Carried, HeldBy, Holding};
use super::Outbox;
use crate::common::{
    Benches, Dressing, GROUND, GaveItem, HeldOverGifts, HeldStock, SpentEnergy, TookItem,
};
use crate::connection::ToServer;

/// Lets go of it, as aimed.
pub(super) fn let_go_of(
    held: HeldBy,
    aim: Target,
    holding: &mut ResMut<Holding>,
    carried: &mut ResMut<Carried>,
    (over_gifts, held_stock): (&HeldOverGifts, &mut ResMut<HeldStock>),
    out: &mut Outbox,
) {
    if super::tie::let_go_on_slot(held, aim, holding, carried, out)
        || super::pour::pour_on(held, aim, holding, carried, held_stock, out)
        || super::feed::feed_on(held, aim, holding, carried, held_stock, out)
        || super::read::read_on(held, aim, holding, carried, held_stock, out)
    {
        return;
    }
    let Outbox {
        to_server,
        gave,
        took,
        spent,
        dressing,
        stacks,
        wear: (note, god, time, wording),
        quest_drop,
        ..
    } = out;
    if aim == (Target::Body { allowed: false }) {
        let new = holding.stock.as_ref().map(|(_, item)| item);
        let now = time.elapsed_secs_f64();
        let item = new.or_else(|| carried.0.find(held.item.id));
        let eater = carried.0.creature;
        if let Some(why) = item.and_then(|item| refusal(item, eater, &wording.0)) {
            // Food refused to the player's kind: the server would refuse it too.
            note.say_words(why, now);
        } else if let Err(why) = try_wearing(&carried.0, (held.item.id, new), god.0, stacks.0) {
            note.say(why, &carried.0, now);
        }
    }
    if let Some((with, item)) = holding.stock.take() {
        held_stock.set_if_neq(HeldStock(None));
        if let Some(gift) = over_gifts.0 {
            // Moved about in their grid: nothing to ask anyone.
            gave.write(GaveItem(gift));
        } else if let Some(bench) = Benches::at_of(with) {
            // Off a crafting bench: only into what is carried, where the
            // server puts it (`crafting_bench`).
            let id = held.item.id;
            if let Some((to, x, y, rotation)) =
                super::super::hold::drop_move(&carried.0, held.item, aim)
                && carried.0.add(item, to, x, y, rotation).is_ok()
            {
                let take = BenchAsk::Take { bench, item: id };
                to_server.write(ToServer(ClientMessage::Bench(take)));
                took.write(TookItem { with, item: id });
            }
        } else if aim == Target::Bin {
            // Only from the ground: see `aim`.
            let id = held.item.id;
            carried.0.to_bin_new(item);
            to_server.write(ToServer(ClientMessage::BinFromGround { item: id }));
            took.write(TookItem { with, item: id });
        } else if aim == (Target::Body { allowed: true })
            && usable_on_self(item.top())
            && may_use_on_self(item.top(), carried.0.creature)
        {
            super::eat::eat_from_ground(held.item.id, item, with, out);
        } else if aim == (Target::Body { allowed: true }) {
            let id = held.item.id;
            to_server.write(ToServer(ClientMessage::WearFromGround { item: id }));
            // Only the top of a stack goes on, and the rest stays on the ground.
            let mut item = item;
            // One off a counted stack is new: a stand-in until the server says.
            let top = item.take_top(&mut || holding.stand_in());
            let all_of_it = top.is_none();
            let wearing = top.unwrap_or(item);
            let grams = wearing.total_weight();
            // Otherwise only the server can say. What is in its way comes off.
            if let Ok(swapped) = carried.0.wear_new_swapping(wearing, god.0, stacks.0) {
                if all_of_it {
                    took.write(TookItem { with, item: id });
                }
                spent.write(SpentEnergy(swapped.cost + handling_cost(grams)));
                dressing.write(Dressing(swapped.cost));
            }
        } else if let Some(onto) = stack_onto(aim).filter(|_| with == GROUND) {
            let id = held.item.id;
            let grams = item.total_weight();
            to_server.write(ToServer(ClientMessage::StackFromGround { item: id, onto }));
            // What is left stays on the ground, as the server will say.
            let stand_in = &mut || holding.stand_in();
            if let Ok(rest) = carried.0.add_to_stack(item, onto, stacks.0, stand_in) {
                let left = rest.as_ref().map_or(0, Item::total_weight);
                if rest.is_none() {
                    took.write(TookItem { with, item: id });
                }
                spent.write(SpentEnergy(handling_cost(grams - left)));
            }
        } else if let Some((to, x, y, rotation)) =
            super::super::hold::drop_move(&carried.0, held.item, aim)
        {
            let grams = item.total_weight();
            if carried.0.add(item, to, x, y, rotation).is_ok() {
                let item = held.item.id;
                to_server.write(ToServer(if with == GROUND {
                    ClientMessage::TakeFromGround {
                        item,
                        to,
                        x,
                        y,
                        rotation,
                    }
                } else {
                    ClientMessage::TakeItem {
                        with,
                        item,
                        to,
                        x,
                        y,
                        rotation,
                    }
                }));
                took.write(TookItem { with, item });
                spent.write(SpentEnergy(handling_cost(grams)));
            }
        }
        return;
    }
    // A quest item is never given, and dropping one destroys it: asked first,
    // all of it (owner).
    let dropping = matches!(
        aim,
        Target::Drop { allowed: true } | Target::Someone(GROUND)
    );
    let giving = matches!(aim, Target::Someone(to) if to != GROUND);
    if (dropping || giving) && holds_quest_item(carried, held.item.id) {
        holding.group.take();
        holding.counted.take();
        if dropping {
            quest_drop.asking = Some(held.item.id);
        }
        return;
    }
    // Some of a mixed stack: whatever is asked is for those items together. Part
    // of a counted one: the server makes it, naming it in place of the stand-in.
    let food = super::eat::carried_food(&carried.0, holding, held.item.id);
    let group = holding.group.take();
    let counted = holding.counted.take();
    if let Some(stack) = food.filter(|_| aim == (Target::Body { allowed: true })) {
        super::eat::eat_carried(stack, &mut carried.0, &mut || holding.stand_in(), out);
        return;
    }
    let mut send = |ToServer(then): ToServer| {
        to_server.write(ToServer(match (&group, counted) {
            (Some((stack, items)), _) => ClientMessage::FromStack {
                stack: *stack,
                items: items.clone(),
                then: Box::new(then),
            },
            (None, Some((stack, count))) => ClientMessage::PartOfStack {
                stack,
                count,
                then: Box::new(then),
            },
            (None, None) => then,
        }));
    };
    if aim == (Target::Body { allowed: true }) {
        let item = held.item.id;
        // Only the top of a stack goes on.
        send(ToServer(ClientMessage::WearItem { item }));
        // Otherwise only the server can say. What is in its way comes off.
        if let Ok(swapped) = carried.0.wear_swapping(item, god.0, stacks.0) {
            spent.write(SpentEnergy(swapped.cost));
            dressing.write(Dressing(swapped.cost));
        }
        return;
    }
    if aim == (Target::Drop { allowed: true }) {
        let item = held.item.id;
        let cost = carried.0.put_down_cost(item);
        let taking_off = carried.0.taking_off_cost(item);
        if carried.0.hand_over(item).is_some() {
            send(ToServer(ClientMessage::DropItem { item }));
            spent.write(SpentEnergy(cost));
            dressing.write(Dressing(taking_off));
        }
        return;
    }
    if let Target::Someone(to) = aim {
        let item = held.item.id;
        let cost = carried.0.put_down_cost(item);
        let taking_off = carried.0.taking_off_cost(item);
        if carried.0.hand_over(item).is_some() {
            dressing.write(Dressing(taking_off));
            send(ToServer(match Benches::at_of(to) {
                _ if to == GROUND => ClientMessage::DropItem { item },
                // Onto a crafting bench's panel: the server places, swaps or
                // refuses it.
                Some(bench) => ClientMessage::Bench(BenchAsk::Put { bench, item }),
                None => ClientMessage::GiveItem { item, to },
            }));
            spent.write(SpentEnergy(cost));
            if let Some(gift) = over_gifts.0 {
                gave.write(GaveItem(gift));
            }
        }
        return;
    }
    if aim == Target::Bin {
        let item = held.item.id;
        let taking_off = carried.0.taking_off_cost(item);
        if carried.0.to_bin(item) {
            send(ToServer(ClientMessage::BinItem { item }));
            spent.write(SpentEnergy(taking_off));
            dressing.write(Dressing(taking_off));
        }
        return;
    }
    if let Some(onto) = stack_onto(aim) {
        let item = held.item.id;
        let to = carried.0.placed(onto).map(|(holder, _)| holder);
        let cost = to.map_or(0, |to| carried.0.move_cost(item, to));
        let taking_off = carried.0.taking_off_cost(item);
        let stand_in = &mut || holding.stand_in();
        if carried.0.stack_onto(item, onto, stacks.0, stand_in).is_ok() {
            spent.write(SpentEnergy(cost));
            dressing.write(Dressing(taking_off));
            send(ToServer(ClientMessage::StackItem { item, onto }));
        }
        return;
    }
    if let Some((to, x, y, rotation)) = super::super::hold::drop_move(&carried.0, held.item, aim) {
        // Shown at once; `Inventory` follows, putting it right if the server disagrees.
        let item = held.item.id;
        let cost = carried.0.move_cost(item, to);
        let taking_off = carried.0.taking_off_cost(item);
        if carried.0.move_item(item, to, x, y, rotation).is_ok() {
            spent.write(SpentEnergy(cost));
            dressing.write(Dressing(taking_off));
            send(ToServer(ClientMessage::MoveItem {
                item,
                to,
                x,
                y,
                rotation,
            }));
        }
    }
}
