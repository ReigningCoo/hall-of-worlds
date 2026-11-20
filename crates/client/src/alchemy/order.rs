//! The process on the panel as an order for the server, how many runs what is
//! carried allows, and what one run makes, by the server's own check
//! (`Inventory::check_alchemy_at`), at the crafting bench the player is at if
//! any (`common::BenchHere`): its tools used first, the work 10% purer.

use std::collections::BTreeMap;

use shared::alchemy::{
    AlchemyOrder, AlchemySkills, Apparatus, Checked, MOST_REPEATS, OrderRefused, OrderStep,
};
use shared::herb::Herbs;
use shared::id::Id;
use shared::item::Inventory;

use super::bench::{Bench, Station};
use crate::common::BenchHere;

impl Station {
    /// The step as asked for.
    fn order_step(&self) -> OrderStep {
        match self.apparatus {
            Apparatus::Mortar => OrderStep::Mortar {
                herbs: self.herbs.clone(),
                solvent: self.oil,
            },
            Apparatus::Retort => OrderStep::Retort {
                liquids: self.liquids.clone(),
                solid: self.solid,
            },
            Apparatus::Alembic => OrderStep::Alembic {
                alcohol: self.alcohol,
            },
            Apparatus::Crucible => OrderStep::Crucible,
            Apparatus::Filter => OrderStep::Filter {
                powder: self.powder,
            },
            Apparatus::Pelican => OrderStep::Pelican,
        }
    }
}

impl Bench {
    /// The process as asked for, run `count` times, at the bench at `bench`
    /// (any cell of it) if the player is at one.
    pub(super) fn order(&self, count: u32, bench: Option<(i64, i64)>) -> AlchemyOrder {
        AlchemyOrder {
            bench,
            source: self.source,
            steps: self.steps.iter().map(Station::order_step).collect(),
            count,
        }
    }

    /// What one run makes from what is carried and on the bench `here`, or
    /// why not.
    pub(super) fn check(
        &self,
        carried: &Inventory,
        herbs: &Herbs,
        skills: AlchemySkills,
        here: &BenchHere,
    ) -> Result<Checked, OrderRefused> {
        let order = self.order(1, here.at());
        carried.check_alchemy_at(&order, herbs, skills, here.tools())
    }

    /// How many runs what is carried is enough for: each stack's count over
    /// what one run uses of it, at most `MOST_REPEATS`, at least 1.
    pub(super) fn most_runs(&self, carried: &Inventory) -> u32 {
        let mut uses: BTreeMap<Id, u32> = BTreeMap::new();
        for id in self.stacks() {
            *uses.entry(id).or_insert(0) += 1;
        }
        let runs = uses.iter().map(|(id, used)| {
            let count = carried.find(*id).map_or(0, |stack| stack.count());
            count / used
        });
        runs.min().unwrap_or(1).clamp(1, MOST_REPEATS)
    }
}
