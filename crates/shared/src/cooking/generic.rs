//! Generic dishes: what an output is called when no real dish is close
//! (`classify`), by its last tool, its wetness and what it is mostly made
//! of, with its biggest foods' names ("Mutton and Turnip Stew").

use serde::{Deserialize, Serialize};

use super::mix::{Foods, RoleMix};
use super::{DishForm, Function, Role, Shape, Source, Tool, Wetness};

/// A kind of generic dish.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Generic {
    Soup,
    Stew,
    Porridge,
    Boiled,
    Fried,
    Omelette,
    Cake,
    Pie,
    Bread,
    Roast,
    Baked,
    Dough,
    Salad,
    Mixed,
    Noodles,
    Chopped,
    Curd,
    Strained,
    Paste,
}

/// Dough is at least this share of a dish's food by weight, in percent: a
/// food that makes dough (flour, grain).
pub const DOUGH_PERCENT: u32 = 30;

impl Generic {
    /// The kind of an output with this shape holding these foods.
    pub(crate) fn of(shape: &Shape, foods: &Foods, mix: &RoleMix) -> Self {
        let dough = (foods.0.iter())
            .filter(|(def, _)| def.functions.contains(&Function::MakesDough))
            .map(|(_, grams)| grams)
            .sum::<u32>();
        let dough = foods.share(dough) >= DOUGH_PERCENT;
        let biggest = (foods.0.iter()).max_by_key(|(_, grams)| *grams);
        let main = biggest.and_then(|(def, _)| def.main);
        let source = biggest.map(|(def, _)| def.source);
        let grain = (foods.0.iter())
            .filter(|(def, _)| def.source != Source::Milk)
            .max_by_key(|(_, grams)| *grams)
            .is_some_and(|(def, _)| def.source == Source::Vegetable && is_grain(&def.key));
        let cooked = shape.tools.iter().any(|tool| tool.needs_heat());
        let used = |tool: Tool| shape.tools.contains(&tool);
        match shape.last() {
            Some(Tool::Cauldron) if grain => Self::Porridge,
            Some(Tool::Cauldron) => match shape.wetness {
                Wetness::Soupy => Self::Soup,
                Wetness::Wet if main == Some(Role::Starch) => Self::Porridge,
                Wetness::Wet => Self::Stew,
                _ if main == Some(Role::Starch) => Self::Porridge,
                _ => Self::Boiled,
            },
            Some(Tool::Pan) if source == Some(Source::Egg) && used(Tool::Bowl) => Self::Omelette,
            Some(Tool::Pan) => Self::Fried,
            Some(Tool::Oven) if dough && mix.has(Role::Sweet) => Self::Cake,
            Some(Tool::Oven) if dough && (mix.has(Role::Protein) || mix.has(Role::Vegetable)) => {
                Self::Pie
            }
            Some(Tool::Oven) if dough => Self::Bread,
            Some(Tool::Oven) if main == Some(Role::Protein) => Self::Roast,
            Some(Tool::Oven) => Self::Baked,
            Some(Tool::Bowl) if dough && !cooked => Self::Dough,
            Some(Tool::Bowl) if !cooked && matches!(main, Some(Role::Vegetable | Role::Sweet)) => {
                Self::Salad
            }
            Some(Tool::Bowl) => Self::Mixed,
            Some(Tool::Knife) if dough && used(Tool::Bowl) => Self::Noodles,
            Some(Tool::Knife) => Self::Chopped,
            Some(Tool::Sieve) if source == Some(Source::Milk) => Self::Curd,
            Some(Tool::Sieve) if used(Tool::Cauldron) => Self::Boiled,
            Some(Tool::Sieve) => Self::Strained,
            Some(Tool::Mortar) | None => Self::Paste,
        }
    }

    /// The word it adds to its foods' names, and whether before them.
    pub const fn word(self) -> (&'static str, bool) {
        match self {
            Self::Soup => ("Soup", false),
            Self::Stew => ("Stew", false),
            Self::Porridge => ("Porridge", false),
            Self::Boiled => ("Boiled", true),
            Self::Fried => ("Fried", true),
            Self::Omelette => ("Omelette", false),
            Self::Cake => ("Cake", false),
            Self::Pie => ("Pie", false),
            Self::Bread => ("Bread", false),
            Self::Roast => ("Roast", true),
            Self::Baked => ("Baked", true),
            Self::Dough => ("Dough", false),
            Self::Salad => ("Salad", false),
            Self::Mixed => ("Mixed", true),
            Self::Noodles => ("Noodles", false),
            Self::Chopped => ("Chopped", true),
            Self::Curd => ("Curd", false),
            Self::Strained => ("Strained", true),
            Self::Paste => ("Paste", false),
        }
    }

    /// What it is served as.
    pub const fn form(self) -> DishForm {
        match self {
            Self::Soup | Self::Stew | Self::Porridge | Self::Salad | Self::Mixed => DishForm::Bowl,
            Self::Boiled | Self::Fried | Self::Roast | Self::Baked | Self::Chopped => {
                DishForm::Plate
            }
            Self::Omelette => DishForm::EggDish,
            Self::Cake => DishForm::Cake,
            Self::Pie => DishForm::Pie,
            Self::Bread => DishForm::Loaf,
            Self::Dough => DishForm::Ball,
            Self::Noodles => DishForm::Noodles,
            Self::Curd => DishForm::Block,
            Self::Strained => DishForm::Jar,
            Self::Paste => DishForm::Paste,
        }
    }

    /// The roles it wants, for balance.
    pub const fn wants(self) -> &'static [Role] {
        match self {
            Self::Soup => &[Role::Vegetable],
            Self::Stew => &[Role::Protein, Role::Vegetable],
            Self::Porridge | Self::Bread | Self::Dough | Self::Noodles => &[Role::Starch],
            Self::Fried => &[Role::Fat],
            Self::Omelette | Self::Roast | Self::Curd => &[Role::Protein],
            Self::Cake => &[Role::Starch, Role::Sweet],
            Self::Pie => &[Role::Starch, Role::Protein],
            Self::Salad => &[Role::Vegetable],
            Self::Boiled | Self::Baked | Self::Mixed | Self::Chopped => &[],
            Self::Strained | Self::Paste => &[],
        }
    }

    /// Its name with these foods' names ("Turnip and Mutton Stew", "Fried
    /// Egg"); just its word with none.
    pub fn name(self, foods: &[String]) -> String {
        let (word, before) = self.word();
        let foods = foods.join(" and ");
        match (foods.is_empty(), before) {
            (true, _) => word.to_owned(),
            (false, true) => format!("{word} {foods}"),
            (false, false) => format!("{foods} {word}"),
        }
    }
}

/// Whether the vegetable with this key is a grain.
fn is_grain(key: &str) -> bool {
    let def = crate::vegetable::Vegetables::built_in().get(key);
    def.is_some_and(|def| def.set == crate::vegetable::VegetableSet::Grain)
}
