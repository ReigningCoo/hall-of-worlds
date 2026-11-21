//! `Shape`: what was done to an output, for naming it: its tools in order,
//! its joins and how wet it came out.

use super::mix::Foods;
use super::{History, Ingredients, Portion, Source, Tool, Wetness};

/// An output's tool steps and wetness.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape {
    /// Every step's tool, in order, its sources' first.
    pub tools: Vec<Tool>,
    pub joins: u32,
    pub wetness: Wetness,
}

impl Shape {
    /// The shape of an output with this history and portions, `water` the
    /// millilitres of water still in it (`plan`: water taken on from a
    /// Cauldron to any other tool is drained). Milk counts as liquid unless
    /// it went through a Sieve, which drains its whey (recipes plan PR 1:
    /// curds and cheeses come out dry).
    pub fn of(
        history: &History,
        portions: &[Portion],
        water: u32,
        ingredients: &Ingredients,
    ) -> Self {
        let foods = Foods::of(portions, ingredients);
        let strained = history.tools.contains(&Tool::Sieve);
        let milk: u32 = (foods.0.iter())
            .filter(|(def, _)| def.source == Source::Milk)
            .map(|(_, grams)| grams)
            .sum();
        Self {
            tools: history.tools.clone(),
            joins: history.joins(),
            wetness: match strained {
                true => Wetness::of(water, foods.grams() - milk + water),
                false => Wetness::of(milk + water, foods.grams() + water),
            },
        }
    }

    /// Its last step's tool.
    pub fn last(&self) -> Option<Tool> {
        self.tools.last().copied()
    }

    /// How far its tools are from `tools`, counting each tool's uses whatever
    /// the order: the steps one has and the other lacks.
    pub fn distance(&self, tools: &[Tool]) -> u32 {
        Tool::ALL
            .iter()
            .map(|&tool| {
                let ours = self.tools.iter().filter(|&&t| t == tool).count();
                let theirs = tools.iter().filter(|&&t| t == tool).count();
                ours.abs_diff(theirs) as u32
            })
            .sum()
    }
}
