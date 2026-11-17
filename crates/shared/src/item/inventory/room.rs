//! Finding room for something in an `Inventory`, and giving new items.

use crate::id::Id;
use crate::item::{Holder, Inventory, Item, Placed, Rotation, Shape, StackLimits};

impl Inventory {
    /// The first place in this grid where an item of this shape has room,
    /// starting with how it is turned now: rows from the top, then the next
    /// quarter turn. `moving` is lifted out, if it is carried here already.
    pub(super) fn first_room(
        &self,
        holder: Holder,
        shape: Shape,
        turned_from: Rotation,
        moving: Option<Id>,
    ) -> Option<(u8, u8, Rotation)> {
        let (width, height) = self.grid_size(holder)?;
        let mut rotation = turned_from;
        for _ in Rotation::ALL {
            let turned = shape.turned(rotation);
            for y in 0..height {
                for x in 0..width {
                    if self.has_room(holder, turned, x, y, moving) {
                        return Some((x, y, rotation));
                    }
                }
            }
            rotation = rotation.next();
        }
        None
    }

    /// Where this carried item would go if dropped on this grid without saying
    /// where (on a closed bag): the first place it can.
    pub fn first_place_for(
        &self,
        id: Id,
        holder: Holder,
        turned_from: Rotation,
    ) -> Option<(u8, u8, Rotation)> {
        let item = self.find(id)?;
        self.may_hold(holder, item).ok()?;
        self.first_room(holder, item.shape(), turned_from, Some(id))
    }

    /// Where something coming from outside what is carried goes first (owner,
    /// 2026-11-17): the first tied holder that takes it (`ContainerDef::holder`: a
    /// keyring for keys) and has room for all of it, on a stack or in a free place;
    /// else the own grid. Both sides route by this: `give` on the server, quick
    /// moves on the client. What the player drags to a chosen cell stays there.
    pub fn route_for(&self, item: &Item, limits: StackLimits) -> Holder {
        self.holders_for(item.kind)
            .map(|holder| Holder::Inside(holder.id))
            .find(|&holder| {
                self.first_stack_for(item, holder, limits).is_some()
                    || self
                        .first_place_for_new(item, holder, Rotation::None)
                        .is_some()
            })
            .unwrap_or(Holder::Inventory)
    }

    /// Puts a new item where `route_for` says: on the first stack it can all go on,
    /// or else in the first place it fits. Gives it back if there is no room. Coins
    /// turn as for `stack_onto`, any new stack of them with an id from `new_id`.
    #[allow(clippy::result_large_err)] // Handing it back whole is the point, and rare.
    pub fn give(
        &mut self,
        item: Item,
        limits: StackLimits,
        new_id: &mut impl FnMut() -> Id,
    ) -> Result<(), Item> {
        let holder = self.route_for(&item, limits);
        if let Some(onto) = self.first_stack_for(&item, holder, limits) {
            self.add_to_stack(item, onto, limits, new_id)
                .expect("first_stack_for found room for all of it");
            return Ok(());
        }
        match self.first_place_for_new(&item, holder, Rotation::None) {
            Some((x, y, rotation)) => {
                let grid = self.grid_mut(holder).expect("route_for found its grid");
                grid.push(Placed {
                    item,
                    x,
                    y,
                    rotation,
                });
                Ok(())
            }
            None => Err(item),
        }
    }
}
