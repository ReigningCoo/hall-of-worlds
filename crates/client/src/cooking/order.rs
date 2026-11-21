//! The process on the panel as an order for the server (`CookOrder`), what
//! one run makes by the server's own check (`Inventory::check_cooking`) at
//! the kitchen the player is at, and how many runs what is carried allows.

use shared::cooking::{
    Checked, CookOrder, CookingSkills, Dishes, Ingredients, MOST_REPEATS, OrderInput, OrderRefused,
    OrderStep,
};
use shared::item::{Inventory, ItemKind};
use shared::pour::{BOTTLE_ML, Held};

use super::bench::{Bench, ROWS, Station};
use super::kitchen::KitchenNow;

impl Station {
    /// The step as asked for.
    fn order_step(&self) -> OrderStep {
        let foods = (self.foods.iter()).map(|&(stack, count)| OrderInput::Food { stack, count });
        let liquids = (self.liquids.iter()).map(|&(stack, ml)| OrderInput::Liquid { stack, ml });
        let faced = (self.faced > 0).then_some(OrderInput::WaterFaced { ml: self.faced });
        OrderStep {
            tool: self.tool,
            column: self.column,
            row: self.row,
            height: self.height,
            inputs: foods.chain(liquids).chain(faced).collect(),
        }
    }
}

impl Bench {
    /// The steps in the order they are run and shown filling: by column,
    /// then row.
    pub(super) fn in_order(&self) -> Vec<usize> {
        let mut order: Vec<usize> = (0..self.steps.len()).collect();
        order.sort_by_key(|&at| (self.steps[at].column, self.steps[at].row));
        order
    }

    /// The process as asked for, run `count` times, at `kitchen`.
    pub(super) fn order(&self, count: u32, kitchen: &KitchenNow) -> CookOrder {
        CookOrder {
            bench: kitchen.bench,
            rows: ROWS as u8,
            sources: self.sources.to_vec(),
            steps: (self.in_order().into_iter())
                .map(|at| self.steps[at].order_step())
                .collect(),
            culture: self.culture,
            count,
        }
    }

    /// What one run makes from what is carried at `kitchen`, or why not.
    pub(super) fn check(
        &self,
        carried: &Inventory,
        kitchen: &KitchenNow,
        skills: CookingSkills,
    ) -> Result<Checked, OrderRefused> {
        let order = self.order(1, kitchen);
        let (ingredients, dishes) = (Ingredients::built_in(), Dishes::built_in());
        carried.check_cooking(&order, kitchen.kitchen(), ingredients, dishes, skills)
    }

    /// How many runs what is carried is enough for: each stack's count (or
    /// what a vessel or cask holds) over what one run uses of it, at most
    /// `MOST_REPEATS`, at least 1. Sources in a hearth's storage allow one.
    pub(super) fn most_runs(&self, carried: &Inventory) -> u32 {
        let runs = self.uses().into_iter().map(|(id, count, ml)| {
            let Some(stack) = carried.find(id) else {
                return 1;
            };
            let top = stack.top();
            match Held::of(top) {
                Some(held) if ml > 0 && top.kind != ItemKind::Bottle => held.ml / ml.max(1),
                _ => stack.count() / (count + ml / BOTTLE_ML).max(1),
            }
        });
        runs.min().unwrap_or(1).clamp(1, MOST_REPEATS)
    }
}
