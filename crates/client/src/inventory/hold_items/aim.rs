//! What letting go of what is held, where it is, would do.

use bevy::prelude::*;
use shared::bonus::{may_use_on_self, usable_on_self};
use shared::item::StackLimits;

use super::super::bag_slots::tie_aim;
use super::super::hold::{Held, Target};
use super::super::wear_note::try_wearing;
use super::super::{Carried, Holding};
use super::{Under, worth_wearing};
use crate::common::{GROUND, GameView, HeldGift, HeldOverGifts, HeldOverText};
use crate::dom::HeldOver;
use crate::widgets::CELL;

/// What letting go of `held` at `at`, over this, would do. Says too whose
/// grid of stock it is over, and what text it would be written into.
pub(super) fn aim_at(
    held: Held,
    (at, over): (Option<Vec2>, HeldOver),
    holding: &Holding,
    carried: &Carried,
    under: &Under,
    view: &State<GameView>,
    (over_gifts, over_text): (&mut ResMut<HeldOverGifts>, &mut ResMut<HeldOverText>),
) -> Target {
    let Under {
        grids,
        marks,
        parents,
        stacks,
        god,
        sheet,
        trading,
    } = under;
    let inventory = &carried.0;
    let from_stock = holding.stock.as_ref().map(|(with, _)| *with);
    // One of our grids, and the cell of it the pointer is in.
    let cell = over.cell.and_then(|(grid, x, y)| {
        let owner = grids.get(grid).ok()?;
        Some((owner.holder, x, y))
    });
    // Only what is in front counts: the bin destroys things.
    // The thing in front, or the nearest thing it is in, that is somewhere to
    // let go: markup nests what it shows (a row's words are in the row).
    let front = over.front.and_then(|front| {
        let up = std::iter::once(front).chain(parents.iter_ancestors(front));
        up.filter_map(|entity| marks.get(entity).ok()).find(
            |(bin, drop, body, gifts, text, slot)| {
                *bin || *drop || *body || gifts.is_some() || text.is_some() || slot.is_some()
            },
        )
    });
    let on_slot = front.and_then(|marks| marks.5).map(|slot| slot.0);
    let (on_bin, on_drop, on_body, on_gifts, on_text) = front
        .map(|(is_bin, is_drop, is_body, gifts, text, _)| {
            (
                is_bin,
                is_drop,
                is_body,
                gifts.map(|gifts| gifts.0),
                text.map(|text| text.0),
            )
        })
        .unwrap_or_default();
    let on_row = on_gifts;
    // Something of theirs can only go back to them (a shop's stock to the
    // Buying box too, and back).
    let on_gifts = on_gifts
        .filter(|&to| from_stock.is_none_or(|from| super::trade::may_go_to(trading, from, to)));
    over_gifts.0 = on_gifts.zip(at).map(|(to, at)| HeldGift {
        to,
        item: held.id,
        rotation: held.rotation,
        corner: at - (Vec2::new(held.grip.0 as f32, held.grip.1 as f32) + 0.5) * CELL,
    });
    // Written into the box: the item itself stays where it was.
    over_text.set_if_neq(HeldOverText(on_text));
    if on_text.is_some() {
        return Target::Nowhere;
    }
    if let Some(slot) = on_slot {
        // Only a whole bag that is carried can be tied.
        let whole = holding.stock.is_none() && holding.group.is_none();
        if !whole || holding.counted.is_some() {
            return Target::BagSlot {
                slot,
                allowed: false,
            };
        }
        return tie_aim(&carried.0, held, slot);
    }
    if let Some((with, item)) = &holding.stock {
        let from_ground = *with == GROUND;
        if on_drop {
            // On the ground already, or someone else's.
            return Target::Drop { allowed: false };
        }
        if from_ground && on_bin {
            return Target::Bin;
        }
        if from_ground && on_body && item.recipe().is_some() {
            // One scroll is read into the recipe book (`read`).
            let allowed = super::read::readable(&sheet.0.book, item);
            return Target::Body { allowed };
        }
        if from_ground && on_body && usable_on_self(item.top()) {
            // Its top one is eaten (`eat`), unless refused to the player's kind.
            let allowed = may_use_on_self(item.top(), carried.0.creature);
            return Target::Body { allowed };
        }
        if from_ground && on_body {
            // Only the top of a stack goes on.
            let tried = try_wearing(&carried.0, (held.id, Some(item)), god.0, stacks.0);
            return Target::Body {
                allowed: worth_wearing(tried),
            };
        }
        // A shop's goes only into the Buying box, never the inventory.
        let shops = super::trade::is_shops(trading, *with);
        if on_bin || on_body || on_row.is_some() || shops {
            return Target::Nowhere;
        }
        // Only what is on the ground can go on a stack, not someone's stock.
        let limits = if from_ground {
            stacks.0
        } else {
            StackLimits::NONE
        };
        let target = super::super::hold::target_for_new(inventory, item, held, cell, limits);
        return target;
    }
    if on_drop {
        // Only sites have ground.
        return Target::Drop {
            allowed: *view.get() == GameView::Dungeon,
        };
    }
    if on_bin {
        // In it already: nothing happens.
        return match carried.0.in_bin(held.id) {
            true => Target::Nowhere,
            false => Target::Bin,
        };
    }
    if let Some(someone) = on_row {
        return Target::Someone(someone);
    }
    if on_body {
        // On it already: nothing happens, whatever it is over.
        if carried.0.is_worn(held.id) {
            return Target::Nowhere;
        }
        if let Some((_, scroll)) = super::read::held_scroll(&carried.0, holding, held.id) {
            // One scroll is read into the recipe book (`read`).
            let allowed = super::read::readable(&sheet.0.book, &scroll);
            return Target::Body { allowed };
        }
        if let Some(stack) = super::eat::carried_food(&carried.0, holding, held.id) {
            // Its top one is eaten (`eat`), unless refused to the player's kind.
            let top = carried.0.find(stack).map(|item| item.top());
            let allowed = top.is_some_and(|top| may_use_on_self(top, carried.0.creature));
            return Target::Body { allowed };
        }
        // What is in its way comes off (owner, 2026-11-07).
        let tried = try_wearing(&carried.0, (held.id, None), god.0, stacks.0);
        return Target::Body {
            allowed: worth_wearing(tried),
        };
    }
    super::super::hold::target(inventory, held, cell, stacks.0)
}
