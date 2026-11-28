//! What the server says of trades with players and NPCs (trading plan PR 6):
//! a trade opened or changed (`Between`, after the other's things in
//! `TradeStock`), waiting for the one asked, being asked, the receive box and
//! the Trading setting; and how each side then shows: the own side and
//! theirs where each put things, darkened with a solid edge once locked in.

use shared::item::Placed;
use shared::protocol::TradeNews;
use shared::trade::{Between, Side};

use super::state::TradeWindow;
use crate::common::{TradeSetting, Trading};
use crate::inventory::Carried;

/// One piece of news about trades with characters: `true` if it was one.
pub(super) fn hear_between(
    news: &TradeNews,
    window: &mut TradeWindow,
    trading: &mut Trading,
    setting: &mut TradeSetting,
) -> bool {
    match news {
        TradeNews::Between(between) => {
            if trading.with != Some(between.with) || trading.terms.is_some() {
                // Opened: by this side, or by the other (Allow, or allowed).
                *trading = Trading {
                    with: Some(between.with),
                    ..Trading::default()
                };
                window.said = None;
            }
            // A change under way: what was said before has gone, but not a
            // refusal, which comes just before the state as it was.
            if !std::mem::take(&mut window.just_refused) {
                window.said = None;
            }
            trading.asking = false;
            trading.selling = between.mine.offers.iter().map(|offer| offer.item).collect();
            trading.between = Some(between.clone());
        }
        TradeNews::Asking { with } => {
            *trading = Trading {
                with: Some(*with),
                asking: true,
                ..Trading::default()
            };
            window.forget_trade();
            window.said = None;
        }
        TradeNews::Asked { by, name } => window.asked = Some((*by, name.clone())),
        TradeNews::Received(received) => {
            window.received.clone_from(&received.items);
            window.receive.heard(&received.items);
        }
        TradeNews::Setting(now) => setting.0 = *now,
        _ => return false,
    }
    window.dirty = true;
    true
}

/// The things on a side where its character put them: `items` are theirs
/// (what is carried, or what the other's `TradeStock` said).
pub(super) fn placed(side: &Side, items: &[shared::item::Item]) -> Vec<Placed> {
    (side.offers.iter())
        .filter_map(|offer| {
            let item = items.iter().find(|item| item.id == offer.item)?;
            Some(Placed {
                item: item.clone(),
                x: offer.x,
                y: offer.y,
                rotation: offer.rotation,
            })
        })
        .collect()
}

/// The own side as the server holds it, drawn where things were put and
/// marked locked in; the shop's grid isn't used.
pub(super) fn follow_between(window: &mut TradeWindow, between: &Between, carried: &Carried) {
    window.theirs = None;
    let carried: Vec<_> = (between.mine.offers.iter())
        .filter_map(|offer| carried.0.find(offer.item).cloned())
        .collect();
    window.own.placed(placed(&between.mine, &carried));
    if window.own.locked != between.mine.locked {
        window.own.locked = between.mine.locked;
        window.own.changed = true;
    }
}
