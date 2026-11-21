//! The kinds of intermediate (owner, 2026-11-21): what a row stopped early
//! gives when it is one, and how each rests. The long processes (cooking
//! plan PR 12) are some of these: cooked soybeans growing koji, koji salted
//! into moromi (pressed into soy sauce) or miso, fresh cheese salted into a
//! cheese, the last three ageing better for weeks of game days.

use serde::{Deserialize, Serialize};

use super::Curve;
use crate::cooking::{
    DishName, Function, Generic, IngredientDef, Ingredients, Output, Portion, Role, Source, Tool,
    is_seasoning,
};
use crate::yeast::Strain;

const HOUR: u32 = 3600;

/// Warm milk sets into yoghurt after this long with a matching culture, at
/// the usual pace (longer wild: `culture::set_percent`).
pub const YOGHURT_SETS: u32 = 8 * HOUR;

/// Fresh noodles are dried after this long.
pub const NOODLES_DRY: u32 = 12 * HOUR;

/// Cooked soybeans grow koji after this long with a koji culture (2 days, as
/// real koji; longer with another strain or wild).
pub const KOJI_GROWS: u32 = 48 * HOUR;

/// A game day in seconds (`fermentation::GAME_DAY_SECS`): ageing is
/// counted in them, as casks' is.
const GAME_DAY: u32 = crate::fermentation::GAME_DAY_SECS as u32;

/// The least salt, in percent of the foods' weight, that makes koji moromi
/// or miso and fresh cheese a cheese to age.
pub const SALTED_PERCENT: u32 = 5;

/// A kind of intermediate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rest {
    Dough,
    FreshNoodles,
    DriedNoodles,
    Roux,
    Stock,
    Butter,
    FreshCheese,
    WarmMilk,
    Yoghurt,
    // The long processes (cooking plan PR 12); new kinds go at the end, as
    // messages carry one by its place.
    CookedSoybeans,
    Koji,
    Moromi,
    Miso,
    Cheese,
}

impl Rest {
    pub const ALL: [Self; 14] = [
        Self::Dough,
        Self::FreshNoodles,
        Self::DriedNoodles,
        Self::Roux,
        Self::Stock,
        Self::Butter,
        Self::FreshCheese,
        Self::WarmMilk,
        Self::Yoghurt,
        Self::CookedSoybeans,
        Self::Koji,
        Self::Moromi,
        Self::Miso,
        Self::Cheese,
    ];

    /// What it is called (dough and noodles add their biggest food:
    /// `FoodForm::name`).
    pub const fn name(self) -> &'static str {
        match self {
            Self::Dough => "Dough",
            Self::FreshNoodles => "Fresh Noodles",
            Self::DriedNoodles => "Dried Noodles",
            Self::Roux => "Roux",
            Self::Stock => "Stock",
            Self::Butter => "Butter",
            Self::FreshCheese => "Fresh Cheese",
            Self::WarmMilk => "Warm Milk",
            Self::Yoghurt => "Yoghurt",
            Self::CookedSoybeans => "Cooked Soybeans",
            Self::Koji => "Soybean Koji",
            Self::Moromi => "Moromi",
            Self::Miso => "Miso",
            Self::Cheese => "Cheese",
        }
    }

    /// How it changes as it rests.
    pub const fn curve(self) -> Curve {
        match self {
            Self::Dough => Curve::DOUGH,
            Self::FreshNoodles => Curve::Turns {
                after: NOODLES_DRY,
                into: Self::DriedNoodles,
            },
            Self::WarmMilk => Curve::Turns {
                after: YOGHURT_SETS,
                into: Self::Yoghurt,
            },
            Self::CookedSoybeans => Curve::Turns {
                after: KOJI_GROWS,
                into: Self::Koji,
            },
            // A step better a month of game days, as in a cask, three at most.
            Self::Moromi => Curve::Ages {
                every: 30 * GAME_DAY,
                most: 3,
            },
            Self::Miso => Curve::Ages {
                every: 20 * GAME_DAY,
                most: 3,
            },
            Self::Cheese => Curve::Ages {
                every: 15 * GAME_DAY,
                most: 3,
            },
            _ => Curve::Stable,
        }
    }

    /// The culture that suits its ferment, if it ferments.
    pub const fn culture(self) -> Option<Strain> {
        match self {
            Self::WarmMilk => Some(Strain::Yoghurt),
            Self::CookedSoybeans => Some(Strain::Koji),
            _ => None,
        }
    }

    /// The intermediate an output is, if it is one: a long process's
    /// (`long_process`), else by the real dish it is named after, else by its
    /// generic kind, its tools and its foods.
    pub fn of(output: &Output, ing: &Ingredients) -> Option<Self> {
        if let Some(rest) = super::long::long_process(output, ing) {
            return Some(rest);
        }
        let (dish, portions, tools) = (&output.dish, &output.portions, &output.history.tools[..]);
        let foods: Vec<_> = (portions.iter())
            .filter_map(|portion| match portion {
                Portion::Food { source, key, .. } => ing.get(*source, key),
                Portion::Water { .. } => None,
            })
            .collect();
        let heated = tools.iter().any(|tool| tool.needs_heat());
        let all_milk = !foods.is_empty() && foods.iter().all(|f| f.source == Source::Milk);
        if all_milk && heated && tools.last().is_some_and(|t| t.needs_heat()) {
            return Some(Self::WarmMilk);
        }
        // Churned: milk worked in a Bowl alone.
        if all_milk && !tools.is_empty() && tools.iter().all(|&t| t == Tool::Bowl) {
            return Some(Self::Butter);
        }
        let dough = |f: &&IngredientDef| (f.functions).contains(&Function::MakesDough);
        let fat = |f: &&IngredientDef| f.main == Some(Role::Fat);
        let roux = tools == [Tool::Pan] || tools == [Tool::Bowl, Tool::Pan];
        let seasoning = |f: &&IngredientDef| f.main.is_some_and(is_seasoning);
        let only = foods.iter().all(|f| dough(f) || fat(f) || seasoning(f));
        if roux && only && foods.iter().any(dough) && foods.iter().any(fat) {
            return Some(Self::Roux);
        }
        match &dish.name {
            DishName::Real(key) => match key.as_str() {
                "butter" | "ghee" => Some(Self::Butter),
                "yoghurt" => Some(Self::Yoghurt),
                "paneer" | "ricotta" | "cottage_cheese" | "quark" | "labneh" => {
                    Some(Self::FreshCheese)
                }
                "fresh_egg_pasta_dough" => Some(Self::Dough),
                "bone_broth" => Some(Self::Stock),
                _ => None,
            },
            DishName::Generic { kind, .. } => match kind {
                Generic::Dough => Some(Self::Dough),
                Generic::Noodles if !heated => Some(Self::FreshNoodles),
                Generic::Curd => Some(Self::FreshCheese),
                Generic::Boiled | Generic::Strained
                    if tools.last() == Some(&Tool::Sieve) && tools.contains(&Tool::Cauldron) =>
                {
                    Some(Self::Stock)
                }
                _ => None,
            },
        }
    }
}
