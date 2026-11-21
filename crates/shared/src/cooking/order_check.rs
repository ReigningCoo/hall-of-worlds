//! An order checked against what is carried and the kitchen
//! (`Inventory::check_cooking`), the same way on both sides: the server
//! again before each run, the client for its preview.

use std::collections::BTreeMap;

use super::{
    CookOrder, CookingSkills, Dishes, Ingredients, Kitchen, OrderInput, OrderRefused, Planned,
    Portion, Process, Source, Step, Tool, plan,
};
use crate::food::Culture;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind};
use crate::potion::Liquid;
use crate::pour::{BOTTLE_ML, Held};

/// Every item kind serving as a cooking tool (`Tool::of_item`).
const TOOL_ITEMS: [ItemKind; 7] = [
    ItemKind::Knife,
    ItemKind::Dagger,
    ItemKind::Pan,
    ItemKind::Cauldron,
    ItemKind::Bowl,
    ItemKind::Sieve,
    ItemKind::MortarAndPestle,
];

/// An order checked: what one run makes, the fungi handled (their keys, for
/// harm by touch or breath), and the culture offered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checked {
    pub planned: Planned,
    pub fungi: Vec<String>,
    pub culture: Option<Culture>,
}

/// What one run takes of a stack: items, or millilitres of a container.
#[derive(Default)]
struct Uses {
    count: u32,
    ml: u32,
}

impl Inventory {
    /// What one run of `order` makes from what is carried at `kitchen`, or
    /// why not. The server checks again before each run, taking what was used.
    pub fn check_cooking(
        &self,
        order: &CookOrder,
        kitchen: Kitchen,
        ingredients: &Ingredients,
        dishes: &Dishes,
        skills: CookingSkills,
    ) -> Result<Checked, OrderRefused> {
        if !(1..=super::MOST_REPEATS).contains(&order.count) {
            return Err(OrderRefused::Count);
        }
        let mut uses: BTreeMap<Id, Uses> = BTreeMap::new();
        let mut sources = Vec::new();
        for source in &order.sources {
            sources.push(match source {
                Some(id) => {
                    let carried = self.cook_stack(*id);
                    let stored = || kitchen.storage.iter().find(|item| item.id == *id);
                    let item = carried.map(Item::top).or_else(stored);
                    let item = item.filter(|item| item.kind == ItemKind::Intermediate);
                    let form = item.and_then(Item::food).ok_or(OrderRefused::Source)?;
                    if carried.is_some() {
                        uses.entry(*id).or_default().count += 1;
                    }
                    Some(form.carried())
                }
                None => None,
            });
        }
        let mut steps = Vec::new();
        let mut fungi = Vec::new();
        for step in &order.steps {
            if !self.has_cooking_tool(step.tool, kitchen) {
                return Err(OrderRefused::Tool(step.tool));
            }
            let mut portions = Vec::new();
            for input in &step.inputs {
                portions.push(self.portion(input, kitchen, ingredients, &mut uses)?);
                if let Some(Portion::Food {
                    source: Source::Fungus,
                    key,
                    ..
                }) = portions.last()
                {
                    fungi.push(key.clone());
                }
            }
            steps.push(Step {
                tool: step.tool,
                column: step.column,
                row: step.row,
                height: step.height,
                portions,
            });
        }
        let culture = match order.culture {
            Some(id) => {
                let item = self.cook_stack(id).ok_or(OrderRefused::Culture)?;
                let yeast = item.yeast().ok_or(OrderRefused::Culture)?;
                Some(Culture {
                    strain: yeast.strain,
                    quality: item.quality.unwrap_or_default(),
                })
            }
            None => None,
        };
        for (id, used) in &uses {
            let stack = self.find(*id).ok_or(OrderRefused::TooFew)?;
            let held = Held::of(stack.top());
            let enough = match held {
                Some(held) if used.ml > 0 && stack.top().kind != ItemKind::Bottle => {
                    held.ml >= used.ml && used.count == 0
                }
                _ => stack.count() >= used.count + used.ml / BOTTLE_ML,
            };
            if !enough {
                return Err(OrderRefused::TooFew);
            }
        }
        let process = Process {
            rows: order.rows,
            sources,
            steps,
            heat: kitchen.heat,
        };
        let planned = plan(&process, ingredients, dishes, skills).map_err(OrderRefused::Rules)?;
        Ok(Checked {
            planned,
            fungi,
            culture,
        })
    }

    /// Whether `tool` is to hand: an Oven at a lit hearth or in a town, any
    /// other on the hearth or in the town, or carried.
    pub fn has_cooking_tool(&self, tool: Tool, kitchen: Kitchen) -> bool {
        if tool == Tool::Oven {
            return kitchen.oven;
        }
        let serves = |kind: &ItemKind| Tool::of_item(*kind) == Some(tool);
        kitchen.tools.iter().any(serves)
            || TOOL_ITEMS
                .iter()
                .filter(|kind| serves(kind))
                .any(|kind| self.count_of(*kind) > 0)
    }

    /// One input as a portion, noting what it takes of its stack.
    fn portion(
        &self,
        input: &OrderInput,
        kitchen: Kitchen,
        ingredients: &Ingredients,
        uses: &mut BTreeMap<Id, Uses>,
    ) -> Result<Portion, OrderRefused> {
        match *input {
            OrderInput::Food { stack, count } => {
                let item = self.cook_stack(stack).ok_or(OrderRefused::Food)?;
                let top = item.top();
                let def = ingredients.of_item(top).ok_or(OrderRefused::Food)?;
                if count == 0 {
                    return Err(OrderRefused::Food);
                }
                uses.entry(stack).or_default().count += count;
                Ok(Portion::Food {
                    source: def.source,
                    key: def.key.clone(),
                    grams: top.own_weight().saturating_mul(count),
                    quality: top.quality,
                })
            }
            OrderInput::Liquid { stack, ml } => {
                let item = self.cook_stack(stack).ok_or(OrderRefused::Liquid)?;
                let top = item.top();
                let held = Held::of(top).ok_or(OrderRefused::Liquid)?;
                let liquid = held.liquid.ok_or(OrderRefused::Liquid)?;
                let bottle = top.kind == ItemKind::Bottle;
                if ml == 0 || (bottle && ml % BOTTLE_ML != 0) {
                    return Err(OrderRefused::Liquid);
                }
                uses.entry(stack).or_default().ml += ml;
                match &liquid {
                    Liquid::Water(_) => Ok(Portion::Water { ml }),
                    _ => {
                        let def = ingredients.of_liquid(&liquid).ok_or(OrderRefused::Liquid)?;
                        Ok(Portion::Food {
                            source: def.source,
                            key: def.key.clone(),
                            grams: ml,
                            quality: top.quality,
                        })
                    }
                }
            }
            OrderInput::WaterFaced { ml } => {
                if !kitchen.water_faced {
                    return Err(OrderRefused::NoWaterFaced);
                }
                Ok(Portion::Water { ml })
            }
        }
    }

    /// The stack `id`, if it is in what is carried and not worn.
    pub(super) fn cook_stack(&self, id: Id) -> Option<&Item> {
        let mut stacks = self.stacks_where(|_| true);
        let (stack, _) = stacks.find(|(stack, _)| stack.id == id)?;
        (!self.is_worn(id)).then_some(stack)
    }
}
