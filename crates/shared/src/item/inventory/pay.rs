//! Paying and being paid in coins (trading plan PR 3): coins are taken
//! highest first, the change given back as the fewest coins, and coins come
//! in wherever they fit, whatever they weigh (owner).

use crate::id::Id;
use crate::item::{COINS, Inventory, MadeAt, StackLimits, coin_items};

/// Why coins couldn't be paid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PayRefused {
    /// Not that many carried.
    NotEnough,
    /// No room for the change.
    NoRoom,
}

impl Inventory {
    /// What the coins carried are worth in copper, in every bag; never in the
    /// bin.
    pub fn coins_carried(&self) -> u64 {
        let coins = self.stacks_where(|top| top.kind.coin_value().is_some());
        coins
            .map(|(stack, _)| u64::from(stack.count()) * stack.kind.coin_value().unwrap_or(0))
            .sum()
    }

    /// Pays `copper`: gold first, then silver, then copper, the change given
    /// back. Refused, half done, if that many aren't carried or there is no
    /// room for the change: so tried on a copy first.
    pub fn pay_coins(
        &mut self,
        copper: u64,
        limits: StackLimits,
        made_at: Option<MadeAt>,
        new_id: &mut impl FnMut() -> Id,
    ) -> Result<(), PayRefused> {
        if self.coins_carried() < copper {
            return Err(PayRefused::NotEnough);
        }
        let mut left = copper;
        for kind in COINS.into_iter().rev() {
            let value = kind.coin_value().expect("every one is a coin");
            let stacks = self.stacks_where(|top| top.kind == kind);
            let stacks: Vec<(Id, u32)> = stacks.map(|(s, _)| (s.id, s.count())).collect();
            for (stack, count) in stacks {
                if left == 0 {
                    break;
                }
                let needed = left.div_ceil(value).min(u64::from(count)) as u32;
                match needed == count {
                    true => drop(self.take(stack)),
                    false => {
                        let found = self.find_mut(stack).expect("found above");
                        found.count -= needed;
                    }
                }
                let taken = u64::from(needed) * value;
                if taken >= left {
                    let change = taken - left;
                    left = 0;
                    self.receive_coins(change, limits, made_at, new_id)
                        .map_err(|_| PayRefused::NoRoom)?;
                } else {
                    left -= taken;
                }
            }
        }
        Ok(())
    }

    /// Takes in coins worth `copper` as the fewest coins, wherever they fit.
    /// Refused, half done, if one has no room.
    pub fn receive_coins(
        &mut self,
        copper: u64,
        limits: StackLimits,
        made_at: Option<MadeAt>,
        new_id: &mut impl FnMut() -> Id,
    ) -> Result<(), PayRefused> {
        for coins in coin_items(copper, limits.most, made_at, new_id) {
            (self.give(coins, limits, new_id)).map_err(|_| PayRefused::NoRoom)?;
        }
        Ok(())
    }
}
