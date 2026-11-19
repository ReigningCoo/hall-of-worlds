//! A player's drinking (`shared::alcohol`, drinks plan PR 7): units taken in
//! with a drink, the stage's effect kept up to date every tick (offline time
//! counted on joining), speed changed with it, a collapse spending the
//! energy it lasts, and Tolerance practised.

use shared::alcohol::{Stage, TOLERANCE, tolerance_practice, units_in};
use shared::potion::Liquid;

use super::Client;
use super::grind::practise;

impl Client {
    /// Takes in the units in `ml` of `liquid` drunk at `now`, if it holds any.
    pub(super) fn drink_units(&mut self, liquid: &Liquid, ml: u32, now: u64) {
        let milli = units_in(liquid, ml);
        if milli == 0 {
            return;
        }
        let before = self.sheet.alcohol_stage();
        let kind = self.inventory.creature;
        let (sheet, situation) = self.sheet_and_situation();
        sheet.drink_alcohol(milli, kind, situation, now);
        let tipsy = self
            .sheet
            .alcohol_stage()
            .is_some_and(|s| s != Stage::Hangover);
        practise(self, TOLERANCE, tolerance_practice(milli, tipsy), now);
        self.after_stage(before, now);
        self.unsaved = true;
    }

    /// Every tick: swaps the stage's effect if the stage has changed,
    /// telling the client. `false` if it has gone.
    pub(super) fn keep_alcohol(&mut self, now: u64) -> bool {
        let before = self.sheet.alcohol_stage();
        let kind = self.inventory.creature;
        let (sheet, situation) = self.sheet_and_situation();
        if !sheet.keep_alcohol(kind, situation, now) {
            return true;
        }
        self.after_stage(before, now);
        self.unsaved = true;
        self.send_sheet(now)
    }

    /// The stage has just changed from `before`: the new speed, and on
    /// collapsing, no energy until the collapse is over.
    fn after_stage(&mut self, before: Option<Stage>, now: u64) {
        self.update_speed(now);
        self.refresh_attributes(now);
        let collapsed = self.sheet.alcohol_stage() == Some(Stage::Collapsed);
        if collapsed && before != Some(Stage::Collapsed) {
            let ticks = self.sheet.alcohol.collapsed_until.saturating_sub(now);
            let cost = u64::from(self.speed) * ticks;
            self.energy
                .spend(now, self.speed, u32::try_from(cost).unwrap_or(u32::MAX));
        }
    }
}
