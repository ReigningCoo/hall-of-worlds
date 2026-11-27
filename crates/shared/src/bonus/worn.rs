//! What everything worn or held gives, added up as it is (without
//! requirements), for rules that only need what gear adds: speed and light
//! (`Sheet::speed`, `Sheet::light_reach`), and the client's Darkvision.

use super::item::item_bonuses;
use super::totals::Totals;
use crate::item::Inventory;
use crate::item_set::set_bonuses;

impl Inventory {
    /// Every bonus its worn and held items give, with their sets', added up.
    pub fn worn_totals(&self) -> Totals {
        let mut bonuses: Vec<_> = self.worn.iter().flat_map(item_bonuses).collect();
        bonuses.extend(set_bonuses(&self.worn));
        Totals::of(&bonuses)
    }
}
