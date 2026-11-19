//! Aiming flour (`Target::Feed`): at a yeast stack, fed with it, or at a
//! bottle of fresh water, beginning a starter, by `shared::yeast`'s rules.

use shared::item::{Inventory, Item, ItemKind};
use shared::protocol::YeastAsk;

use super::Target;

/// What letting go of flour `item` on `under` would ask, or why not: `None`
/// if it isn't flour on yeast or on a bottle.
pub(in crate::inventory) fn feeding(
    inventory: &Inventory,
    item: &Item,
    under: &Item,
) -> Option<Result<YeastAsk, &'static str>> {
    if item.top().kind != ItemKind::Flour {
        return None;
    }
    let flour = item.id;
    match under.top().kind {
        ItemKind::Yeast => {
            let supplies = inventory.feed_supplies(flour);
            let fed = shared::yeast::plan_feeding(under, supplies);
            Some(
                fed.map(|_| YeastAsk::Feed {
                    yeast: under.id,
                    flour,
                })
                .map_err(|why| why.words()),
            )
        }
        ItemKind::Bottle => {
            let start = shared::yeast::can_start(item.top(), under.top());
            Some(
                start
                    .map(|_| YeastAsk::Start {
                        water: under.id,
                        flour,
                    })
                    .map_err(|why| why.words()),
            )
        }
        _ => None,
    }
}

/// `Target::Feed` for flour held over yeast or a bottle.
pub(super) fn feed_aim(inventory: &Inventory, item: &Item, under: &Item) -> Option<Target> {
    let asked = feeding(inventory, item, under)?;
    Some(Target::Feed {
        onto: under.id,
        allowed: asked.is_ok(),
    })
}
