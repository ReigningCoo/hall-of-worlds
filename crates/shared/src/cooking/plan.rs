//! Working a whole process out, the same way on both sides: its chance, and
//! each output's ingredients, history, balance and least quality.

use super::grid::{Input, flow};
use super::{
    CookingSkills, Diet, Dish, Dishes, Heat, History, Ingredients, Portion, Process, Refused,
    Shape, Tool, balance, chance, classify, ingredient_quality, least_quality,
};
use crate::item::Quality;

/// What one output (a step nothing takes on from) holds and what it is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Output {
    /// The step it comes out of.
    pub step: usize,
    /// Everything that went into it, sources' included.
    pub portions: Vec<Portion>,
    /// Its steps (this session's and its sources') and the sub-skills at
    /// them, for an intermediate to carry on.
    pub history: History,
    /// What it is called and served as (`classify`).
    pub dish: Dish,
    pub diet: Diet,
    /// Wanting what its dish wants (`Dish::wants`).
    pub balance: i32,
    /// Its least quality, before the server's luck.
    pub quality: Quality,
}

/// A process worked out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Planned {
    /// In hundredths of a percent, by this session's steps alone.
    pub chance: u32,
    /// By the column, then the row, of the step each comes out of.
    pub outputs: Vec<Output>,
    /// Each step's tool, in the order given: the sub-skills practised.
    pub practised: Vec<Tool>,
    pub heat: Heat,
}

/// Works `process` out, or says why it can't be made.
pub fn plan(
    process: &Process,
    ingredients: &Ingredients,
    dishes: &Dishes,
    skills: CookingSkills,
) -> Result<Planned, Refused> {
    let flow = flow(process)?;
    let known = |portions: &[Portion]| {
        portions.iter().all(|portion| match portion {
            Portion::Food { source, key, .. } => ingredients.get(*source, key).is_some(),
            Portion::Water { .. } => true,
        })
    };
    for (row, source) in process.sources.iter().enumerate() {
        if source.as_ref().is_some_and(|s| !known(&s.portions)) {
            return Err(Refused::UnknownInSource { row: row as u8 });
        }
    }
    for (i, step) in process.steps.iter().enumerate() {
        if !known(&step.portions) {
            return Err(Refused::UnknownIngredient { step: i });
        }
        if step.tool.needs_heat() && process.heat == Heat::Cold {
            return Err(Refused::NoHeat { step: i });
        }
        let (portions, _, _) = gather(process, &flow.inputs, skills, i);
        if step.tool.needs_liquid() && !portions.iter().any(Portion::is_liquid) {
            return Err(Refused::NoLiquid { step: i });
        }
    }
    let halves: u32 = process.steps.iter().map(|step| step.halves()).sum();
    let chance = chance(skills.cooking_percent(), halves);
    let outputs = (flow.ends.iter())
        .map(|&step| {
            let (portions, history, water) = gather(process, &flow.inputs, skills, step);
            let shape = Shape::of(&history, &portions, water, ingredients);
            let dish = classify(&portions, &shape, ingredients, dishes);
            let balance = balance(&portions, ingredients, &dish.wants(dishes));
            let quality = least_quality(
                ingredient_quality(&portions),
                balance,
                &history,
                chance,
                process.heat,
            );
            Output {
                step,
                diet: Diet::of(&portions, ingredients),
                portions,
                history,
                dish,
                balance,
                quality,
            }
        })
        .collect();
    Ok(Planned {
        chance,
        outputs,
        practised: process.steps.iter().map(|step| step.tool).collect(),
        heat: process.heat,
    })
}

/// Everything that went into step `i` (its own portions after what it
/// takes), the history so far (its steps and every one before it, with this
/// cook's sub-skills, and its sources' histories), and the millilitres of
/// water still in it: water taken on to anything but a Cauldron is drained.
fn gather(
    process: &Process,
    inputs: &[Vec<Input>],
    skills: CookingSkills,
    i: usize,
) -> (Vec<Portion>, History, u32) {
    let step = &process.steps[i];
    let mut portions = Vec::new();
    let mut history = History::default();
    let mut taken_water = 0;
    for &input in &inputs[i] {
        match input {
            Input::Step(j) => {
                let (before, past, water) = gather(process, inputs, skills, j);
                taken_water += water;
                portions.extend(before);
                history.merge(&past);
            }
            Input::Source(row) => {
                if let Some(Some(carried)) = process.sources.get(usize::from(row)) {
                    taken_water += water_in(&carried.portions);
                    portions.extend(carried.portions.iter().cloned());
                    history.merge(&carried.history);
                }
            }
        }
    }
    portions.extend(step.portions.iter().cloned());
    history.halves += step.halves();
    history.tools.push(step.tool);
    history.add_root(step.tool, skills.sub_root(step.tool));
    let kept = if step.tool == Tool::Cauldron {
        taken_water
    } else {
        0
    };
    (portions, history, kept + water_in(&step.portions))
}

/// The millilitres of water in these portions.
fn water_in(portions: &[Portion]) -> u32 {
    (portions.iter())
        .map(|portion| match portion {
            Portion::Water { ml } => *ml,
            Portion::Food { .. } => 0,
        })
        .sum()
}
