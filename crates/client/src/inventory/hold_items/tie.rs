//! Letting go on a bag slot, or of something the bag slots keep where it is
//! (`bag_slots`): tying a bag to the belt, or saying why not.

use shared::energy::handling_cost;
use shared::item::Item;
use shared::protocol::ClientMessage;

use super::super::bag_slots::{explain_move, lift_refused};
use super::super::hold::Target;
use super::super::{Carried, HeldBy, Holding};
use super::Outbox;
use crate::common::SpentEnergy;
use crate::connection::ToServer;

/// Ties what is held to the bag slot it was let go on, or says why it can't be,
/// and the same for a tied bag or its belt that can't leave where they are.
/// Whether that was all letting go does.
pub(super) fn let_go_on_slot(
    held: HeldBy,
    aim: Target,
    holding: &Holding,
    carried: &mut Carried,
    out: &mut Outbox,
) -> bool {
    let (note, _, time, _) = &mut out.wear;
    let now = time.elapsed_secs_f64();
    let id = held.item.id;
    if holding.stock.is_none()
        && let Some(why) = lift_refused(&carried.0, id, aim)
    {
        // It stays: the rest of letting go finds it can't, and snaps it back.
        note.say_words(explain_move(why), now);
        return false;
    }
    let Target::BagSlot { slot, allowed } = aim else {
        return false;
    };
    if !allowed {
        let why = carried.0.check_tie(id, slot).err();
        if holding.stock.is_none()
            && let Some(why) = why
        {
            note.say_words(explain_move(why), now);
        }
        return true;
    }
    let grams = carried.0.find(id).map_or(0, Item::total_weight);
    if carried.0.tie_bag(id, slot).is_ok() {
        out.to_server
            .write(ToServer(ClientMessage::TieBag { item: id, slot }));
        out.spent.write(SpentEnergy(handling_cost(grams)));
    }
    true
}
