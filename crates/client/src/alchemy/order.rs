//! The process on the panel as an order for the server, how many runs what is
//! carried allows, and what one run makes, by the server's own check
//! (`Inventory::check_alchemy`).

use std::collections::BTreeMap;

use shared::alchemy::{
    AlchemyOrder, AlchemySkills, Apparatus, Checked, MOST_REPEATS, OrderRefused, OrderStep,
};
use shared::herb::Herbs;
use shared::id::Id;
use shared::item::Inventory;

use super::bench::{Bench, Station};

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
            },
            Apparatus::Alembic => OrderStep::Alembic {
                alcohol: self.alcohol,
            },
            Apparatus::Crucible => OrderStep::Crucible,
            Apparatus::Filter => OrderStep::Filter,
            Apparatus::Pelican => OrderStep::Pelican,
        }
    }
}

impl Bench {
    /// The process as asked for, run `count` times.
    pub(super) fn order(&self, count: u32) -> AlchemyOrder {
        AlchemyOrder {
            source: self.source,
            steps: self.steps.iter().map(Station::order_step).collect(),
            count,
        }
    }

    /// What one run makes from what is carried, or why not.
    pub(super) fn check(
        &self,
        carried: &Inventory,
        herbs: &Herbs,
        skills: AlchemySkills,
    ) -> Result<Checked, OrderRefused> {
        carried.check_alchemy(&self.order(1), herbs, skills)
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
