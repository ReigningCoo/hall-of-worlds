//! What everything worn or held gives, added up as it is (without
//! requirements), for rules that only need what gear adds: speed and light
//! (`Sheet::speed`, `Sheet::light_reach`), and the client's Darkvision.

use super::item::item_bonuses;
use super::totals::Totals;
use crate::item::Inventory;

impl Inventory {
    /// Every bonus its worn and held items give, added up.
    pub fn worn_totals(&self) -> Totals {
        let bonuses: Vec<_> = self.worn.iter().flat_map(item_bonuses).collect();
        Totals::of(&bonuses)
    }
}
