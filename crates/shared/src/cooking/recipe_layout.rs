//! A dish's recipe laid out in the grid (`recipe_build`): its tools in one
//! row, or, for a dish that joins rows, all but its last two in the top row,
//! the second-last alone in the second, both joined by the last; foods in
//! each row's first step (starches on top); water where a Cauldron or Sieve
//! first needs it and for the dish's wetness where it stays.

use super::{DishDef, Recipe, RecipeInput, RecipeStep, Tool, Wanted, Wetness};

/// A food (or liquid food) waiting for its place: its grams, whether it
/// counts as liquid in a dish (milk) and whether it is a starch.
pub(super) struct Placed {
    input: RecipeInput,
    grams: u32,
    wet: bool,
    starch: bool,
}

impl Placed {
    pub fn food(foods: Vec<Wanted>, count: u32, grams: u32, starch: bool) -> Self {
        let input = RecipeInput::Food { foods, count };
        Self {
            input,
            grams,
            wet: false,
            starch,
        }
    }

    pub fn liquid(foods: Vec<Wanted>, ml: u32, starch: bool) -> Self {
        let wet = foods.first().is_some_and(is_milk);
        let input = RecipeInput::Liquid { foods, ml };
        Self {
            input,
            grams: ml,
            wet,
            starch,
        }
    }
}

/// The share of a dish that is liquid, in percent, aimed at for a wetness:
/// the middle of its band.
fn liquid_percent(wetness: Wetness) -> u32 {
    match wetness {
        Wetness::Dry => 0,
        Wetness::Moist => 15,
        Wetness::Wet => 37,
        Wetness::Soupy => 65,
    }
}

/// Millilitres rounded to 10, at least 10.
fn ml(amount: u32) -> u32 {
    (amount.div_ceil(10) * 10).max(10)
}

/// `def`'s recipe holding these foods.
pub(super) fn lay_out(def: &DishDef, placed: Vec<Placed>) -> Recipe {
    // Milk through a Sieve loses its whey: it neither wets nor weighs.
    let strained = def.tools.contains(&Tool::Sieve);
    let wet: u32 = placed.iter().filter(|p| p.wet).map(|p| p.grams).sum();
    let food: u32 = placed.iter().map(|p| p.grams).sum::<u32>() - if strained { wet } else { 0 };
    let wet = if strained { 0 } else { wet };
    let percent = liquid_percent(def.liquid);
    let water = match percent {
        0 => 0,
        _ => (percent * food).saturating_sub(100 * wet) / (100 - percent),
    };
    let mut steps = grid(&def.tools, def.joins > 0);
    let (mut top, mut other): (Vec<Placed>, Vec<Placed>) = match is_join(&steps) {
        true => placed.into_iter().partition(|p| p.starch),
        false => (placed, Vec::new()),
    };
    let second = steps.iter().position(|s| s.row == 1);
    if second.is_some() && other.is_empty() && top.len() > 1 {
        other.push(top.pop().expect("more than one"));
    }
    if second.is_some() && top.is_empty() && other.len() > 1 {
        top.push(other.remove(0));
    }
    steps[0].inputs = top.into_iter().map(|p| p.input).collect();
    if let Some(at) = second {
        steps[at].inputs = other.into_iter().map(|p| p.input).collect();
    }
    pour_water(&mut steps, food, water);
    // A row's first step must hold something.
    for step in &mut steps {
        if step.inputs.is_empty() && step.column == 0 {
            step.inputs.push(RecipeInput::Water { ml: 50 });
        }
    }
    Recipe {
        rows: if second.is_some() { 2 } else { 1 },
        sources: Vec::new(),
        steps,
    }
    .normalised()
}

/// Whether the last step joins rows.
fn is_join(steps: &[RecipeStep]) -> bool {
    steps.last().is_some_and(|s| s.height > 1)
}

/// Whether a recipe's food is milk.
fn is_milk(wanted: &Wanted) -> bool {
    match wanted {
        Wanted::Any(source) | Wanted::One(source, _) => *source == super::Source::Milk,
    }
}

/// The empty steps for these tools: one row, or two joined by the last.
fn grid(tools: &[Tool], join: bool) -> Vec<RecipeStep> {
    let step = |tool, column: usize, row, height| RecipeStep {
        tool,
        column: column as u8,
        row,
        height,
        inputs: Vec::new(),
    };
    let n = tools.len();
    if !join || n < 2 {
        return (tools.iter().enumerate())
            .map(|(at, &tool)| step(tool, at, 0, 1))
            .collect();
    }
    let mut steps = Vec::new();
    if n == 2 {
        steps.push(step(tools[0], 0, 0, 1));
        steps.push(step(tools[0], 0, 1, 1));
        steps.push(step(tools[1], 1, 0, 2));
        return steps;
    }
    for (at, &tool) in tools[..n - 2].iter().enumerate() {
        steps.push(step(tool, at, 0, 1));
    }
    steps.push(step(tools[n - 2], 0, 1, 1));
    steps.push(step(tools[n - 1], n - 2, 0, 2));
    steps
}

/// The step taking step `at`'s output: the next in its row, else a join
/// covering its row.
fn next(steps: &[RecipeStep], at: usize) -> Option<usize> {
    let here = &steps[at];
    (steps.iter().enumerate())
        .filter(|(_, s)| s.column > here.column && s.row <= here.row && here.row < s.row + s.height)
        .min_by_key(|(_, s)| s.column)
        .map(|(i, _)| i)
}

/// Whether water put in step `at` is still there at the end: every step
/// after it is a Cauldron.
fn kept(steps: &[RecipeStep], at: usize) -> bool {
    let mut at = at;
    while let Some(after) = next(steps, at) {
        if steps[after].tool != Tool::Cauldron {
            return false;
        }
        at = after;
    }
    true
}

/// Whether step `at`, or one before it, holds water or milk.
fn has_liquid(steps: &[RecipeStep], at: usize) -> bool {
    let own = steps[at].inputs.iter().any(|input| match input {
        RecipeInput::Water { .. } => true,
        RecipeInput::Liquid { foods, .. } => foods.first().is_some_and(is_milk),
        RecipeInput::Food { .. } => false,
    });
    own || (0..steps.len())
        .any(|before| next(steps, before) == Some(at) && has_liquid(steps, before))
}

/// Water for every Cauldron and Sieve that needs it, and `water` for the
/// dish's wetness where it stays (`food` grams of food in all).
fn pour_water(steps: &mut [RecipeStep], food: u32, water: u32) {
    let mut wetted = false;
    let mut order: Vec<usize> = (0..steps.len()).collect();
    order.sort_by_key(|&at| (steps[at].column, steps[at].row));
    for at in order {
        if !steps[at].tool.needs_liquid() || has_liquid(steps, at) {
            continue;
        }
        let amount = match kept(steps, at) {
            // A dry dish keeps under 5% of it.
            true if !wetted => {
                wetted = true;
                if water == 0 { food * 3 / 100 } else { water }
            }
            true => continue,
            false => food / 2,
        };
        steps[at].inputs.push(RecipeInput::Water { ml: ml(amount) });
    }
    if !wetted && water > 0 {
        let out = (0..steps.len())
            .find(|&at| next(steps, at).is_none())
            .unwrap_or(0);
        steps[out].inputs.push(RecipeInput::Water { ml: ml(water) });
    }
}
