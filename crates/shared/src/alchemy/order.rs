//! A process as asked for (Alchemy plan PR 3): which carried stacks go into
//! each step's drop spaces, and how many times to run it; checked against what
//! is carried with the same rules on both sides (`Inventory::check_alchemy`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{
    AlchemySkills, Apparatus, Planned, Process, Refused, Solid, Solvent, Step, Stored, plan,
};
use crate::brewing::{GroundHerb, Ingredient};
use crate::fungus::Fungi;
use crate::herb::Herbs;
use crate::id::Id;
use crate::item::{Inventory, Item};
use crate::potion::{Contents, Liquid};

/// The most times one order may be run without the player stepping in.
pub const MOST_REPEATS: u32 = 99;

/// A process asked for: perhaps a carried Paste, Ointment or Powder to start from,
/// the steps with the stacks put into each, and how many times to run it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AlchemyOrder {
    pub source: Option<Id>,
    pub steps: Vec<OrderStep>,
    /// 1 to `MOST_REPEATS`: each run its own, with its own rolls.
    pub count: u32,
}

/// One step as asked for: an apparatus and the carried stacks in its drop
/// spaces. One of each herb stack, the solvent's bottle, each liquid's bottle
/// (a stack listed twice gives two), the Alembic's alcohol and the Filter's
/// Powder are used up by each run.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderStep {
    /// Herb stacks, one of each kind, and a bottle of oil (an Ointment) or
    /// pure alcohol (a Tincture), or nothing (dry: a Paste).
    Mortar {
        herbs: Vec<Id>,
        solvent: Option<Id>,
    },
    /// Bottles of liquid, one 100 ml unit each, in order, and a solid (Salt,
    /// Sulphur, Mercury or a vegetable with a transformation), one used a run.
    Retort {
        liquids: Vec<Id>,
        solid: Option<Id>,
    },
    /// A bottle of a drink or pure alcohol fed in, or nothing.
    Alembic {
        alcohol: Option<Id>,
    },
    Crucible,
    /// A carried Powder filtered through, or nothing (refused by the rules).
    Filter {
        powder: Option<Id>,
    },
    Pelican,
}

impl OrderStep {
    /// The apparatus it uses.
    pub fn apparatus(&self) -> Apparatus {
        match self {
            Self::Mortar { .. } => Apparatus::Mortar,
            Self::Retort { .. } => Apparatus::Retort,
            Self::Alembic { .. } => Apparatus::Alembic,
            Self::Crucible => Apparatus::Crucible,
            Self::Filter { .. } => Apparatus::Filter,
            Self::Pelican => Apparatus::Pelican,
        }
    }
}

/// Why an order can't be run with what is carried.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderRefused {
    /// The count is 0 or over `MOST_REPEATS`.
    Count,
    /// This apparatus isn't carried.
    Apparatus(Apparatus),
    /// The source isn't a carried Paste, Ointment or Powder.
    Source,
    /// The Filter's Powder isn't a carried Powder.
    Powder,
    /// A herb stack isn't a carried herb.
    Herb,
    /// A herb the server's table doesn't have.
    UnknownHerb,
    /// The solvent isn't a carried bottle of oil or pure alcohol.
    Solvent,
    /// A liquid isn't a carried, filled bottle.
    Liquid,
    /// A Retort's solid isn't a carried solid it takes (`Solid::of_item`).
    Solid,
    /// A stack holds fewer than one run uses.
    TooFew,
    /// The rules refused the process.
    Rules(Refused),
}

/// An order checked: what one run makes, and the herbs handled (their keys,
/// for Herbalism's practice).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checked {
    pub planned: Planned,
    pub herbs: Vec<String>,
}

impl Inventory {
    /// What one run of `order` makes from what is carried, or why not. The
    /// server checks again before each run, taking what was used.
    pub fn check_alchemy(
        &self,
        order: &AlchemyOrder,
        herbs: &Herbs,
        skills: AlchemySkills,
    ) -> Result<Checked, OrderRefused> {
        if !(1..=MOST_REPEATS).contains(&order.count) {
            return Err(OrderRefused::Count);
        }
        let mut uses = BTreeMap::new();
        let source = match order.source {
            Some(id) => Some(self.alchemy_source(id, &mut uses)?),
            None => None,
        };
        let herb_stacks = self.herb_stacks();
        let mut steps = Vec::new();
        let mut handled = Vec::new();
        for step in &order.steps {
            let apparatus = step.apparatus();
            let kind = apparatus.item();
            if kind.is_some_and(|kind| self.count_of(kind) == 0) {
                return Err(OrderRefused::Apparatus(apparatus));
            }
            steps.push(match step {
                OrderStep::Mortar {
                    herbs: ids,
                    solvent,
                } => {
                    let mut ground = Vec::new();
                    for id in ids {
                        let stack = herb_stacks.iter().find(|stack| stack.stack == *id);
                        let stack = stack.ok_or(OrderRefused::Herb)?;
                        let unknown = OrderRefused::UnknownHerb;
                        ground.push(match stack.ingredient {
                            Ingredient::Herb => {
                                let def = herbs.get(&stack.herb).ok_or(unknown)?;
                                GroundHerb::herb(def, stack.quality)
                            }
                            Ingredient::Fungus => {
                                let def = Fungi::built_in().get(&stack.herb).ok_or(unknown)?;
                                GroundHerb::fungus(def)
                            }
                        });
                        handled.push(stack.herb.clone());
                        *uses.entry(*id).or_insert(0) += 1;
                    }
                    let solvent = match solvent {
                        Some(id) => {
                            *uses.entry(*id).or_insert(0) += 1;
                            match self.alchemy_liquid(*id) {
                                Some(Liquid::Oil(oil)) => Solvent::Oil(oil),
                                Some(Liquid::Alcohol) => Solvent::Alcohol,
                                _ => return Err(OrderRefused::Solvent),
                            }
                        }
                        None => Solvent::Dry,
                    };
                    Step::Mortar {
                        herbs: ground,
                        solvent,
                    }
                }
                OrderStep::Retort {
                    liquids: ids,
                    solid,
                } => {
                    let mut liquids = Vec::new();
                    for id in ids {
                        liquids.push(self.alchemy_liquid(*id).ok_or(OrderRefused::Liquid)?);
                        *uses.entry(*id).or_insert(0) += 1;
                    }
                    let solid = match solid {
                        Some(id) => {
                            *uses.entry(*id).or_insert(0) += 1;
                            Some(self.alchemy_solid(*id).ok_or(OrderRefused::Solid)?)
                        }
                        None => None,
                    };
                    Step::Retort { liquids, solid }
                }
                OrderStep::Alembic { alcohol } => {
                    let fed = match alcohol {
                        Some(id) => {
                            *uses.entry(*id).or_insert(0) += 1;
                            Some(self.alchemy_liquid(*id).ok_or(OrderRefused::Liquid)?)
                        }
                        None => None,
                    };
                    Step::Alembic { fed }
                }
                OrderStep::Crucible => Step::Crucible,
                OrderStep::Filter { powder } => {
                    let powder = match powder {
                        Some(id) => {
                            *uses.entry(*id).or_insert(0) += 1;
                            Some(self.alchemy_powder(*id).ok_or(OrderRefused::Powder)?)
                        }
                        None => None,
                    };
                    Step::Filter { powder }
                }
                OrderStep::Pelican => Step::Pelican,
            });
        }
        let enough =
            |(id, wanted): (&Id, &u32)| self.find(*id).is_some_and(|it| it.count() >= *wanted);
        if !uses.iter().all(enough) {
            return Err(OrderRefused::TooFew);
        }
        let process = Process { source, steps };
        let planned = plan(&process, skills).map_err(OrderRefused::Rules)?;
        Ok(Checked {
            planned,
            herbs: handled,
        })
    }

    /// The carried Paste, Ointment or Powder stack `id`, as a source.
    fn alchemy_source(&self, id: Id, uses: &mut BTreeMap<Id, u32>) -> Result<Stored, OrderRefused> {
        let stack = self.alchemy_stack(id).ok_or(OrderRefused::Source)?;
        let (form, contents) = stack.top().mixture().ok_or(OrderRefused::Source)?;
        uses.insert(id, 1);
        Ok(Stored {
            form,
            contents: contents.clone(),
        })
    }

    /// The doses in the carried Powder stack `id`, if it is one.
    fn alchemy_powder(&self, id: Id) -> Option<Contents> {
        Some(self.alchemy_stack(id)?.top().powder()?.contents.clone())
    }

    /// The solid the carried stack `id` is, if a Retort takes it.
    fn alchemy_solid(&self, id: Id) -> Option<Solid> {
        Solid::of_item(self.alchemy_stack(id)?.top())
    }

    /// The liquid in the carried bottle stack `id`, if it holds one.
    fn alchemy_liquid(&self, id: Id) -> Option<Liquid> {
        let stack = self.alchemy_stack(id)?;
        stack.top().bottle()?.liquid.clone()
    }

    /// The stack `id`, if it is in what is carried and not worn.
    fn alchemy_stack(&self, id: Id) -> Option<&Item> {
        let mut stacks = self.stacks_where(|_| true);
        let (stack, _) = stacks.find(|(stack, _)| stack.id == id)?;
        (!self.is_worn(id)).then_some(stack)
    }
}
