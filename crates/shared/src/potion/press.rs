//! Press Oil (brewing plan, decisions 3 and 23; PR 5): nuts, seeds or
//! vegetables pressed into an empty bottle. What can be pressed and into
//! which oil, how much a bottle takes, how long it takes, and what the
//! player has enough of. The server decides; the client lists the choices
//! with the same rules. Moromi is pressed with it too, into bottles of soy
//! sauce (cooking plan PR 12: `press_moromi.rs`).

use super::{Liquid, Oil};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::fruit::{FruitSet, Fruits};
use crate::herb::{Herbs, cube_root, skill_percent, skill_value};
use crate::id::Id;
use crate::item::{Inventory, Item};
use crate::item_form::ItemForm;
use crate::skill::Skills;
use crate::vegetable::{Part, VegetableSet, Vegetables};

/// How long the first bottle takes untrained, in seconds.
const FIRST_SECONDS: u32 = 15;

/// Alchemy practice for one bottle pressed (as much as Collect Water gives
/// Water).
pub const PRESS_PRACTICE: u32 = 8;

/// What is being pressed: one fruit, vegetable or herb, by key. Every stack of
/// it counts, wherever it is carried.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Pressing {
    Fruit(String),
    Vegetable(String),
    Herb(String),
    /// Any moromi carried, one whole at a time, into soy sauce.
    Moromi,
}

impl Pressing {
    /// What pressing this item would be, if it is something that can be
    /// pressed (`oil`).
    pub fn of(item: &Item, herbs: &Herbs) -> Option<Self> {
        if crate::food::soy_sauce_bottles(item) > 0 {
            return Some(Self::Moromi);
        }
        let pressing = match item.form.as_ref()? {
            ItemForm::Fruit(form) => Self::Fruit(form.fruit.clone()),
            ItemForm::Vegetable(form) => Self::Vegetable(form.vegetable.clone()),
            ItemForm::Herb(form) => Self::Herb(form.herb.clone()),
            _ => return None,
        };
        pressing.oil(herbs).map(|_| pressing)
    }

    /// The oil it gives (decision 3, Alchemy plan): nuts Nut Oil; vegetables
    /// whose seeds are eaten (beans, peanuts) and herbs whose part used is
    /// their seeds Seed Oil; grain Grain Oil; any other vegetable Neutral Oil.
    /// Fruit other than nuts and other herbs give none.
    pub fn oil(&self, herbs: &Herbs) -> Option<Oil> {
        match self {
            Self::Fruit(key) => {
                let def = Fruits::built_in().get(key)?;
                (def.set == FruitSet::Nut).then_some(Oil::Nut)
            }
            Self::Vegetable(key) => {
                let def = Vegetables::built_in().get(key)?;
                Some(if def.set == VegetableSet::Grain {
                    Oil::Grain
                } else if def.part == Part::Seeds {
                    Oil::Seed
                } else {
                    Oil::Neutral
                })
            }
            Self::Herb(key) => {
                let def = herbs.get(key)?;
                (def.part == crate::herb::Part::Seeds).then_some(Oil::Seed)
            }
            Self::Moromi => None,
        }
    }

    /// What it fills bottles with: its oil, or moromi's soy sauce.
    pub fn liquid(&self, herbs: &Herbs) -> Option<Liquid> {
        match self {
            Self::Moromi => Some(Liquid::SoySauce),
            _ => self.oil(herbs).map(Liquid::Oil),
        }
    }

    /// Whether `item` is this.
    pub fn is(&self, item: &Item) -> bool {
        match (self, item.form.as_ref()) {
            (Self::Fruit(key), Some(ItemForm::Fruit(form))) => form.fruit == *key,
            (Self::Vegetable(key), Some(ItemForm::Vegetable(form))) => form.vegetable == *key,
            (Self::Herb(key), Some(ItemForm::Herb(form))) => form.herb == *key,
            (Self::Moromi, _) => crate::food::soy_sauce_bottles(item) > 0,
            _ => false,
        }
    }
}

/// How many grams pressed fill one bottle (first guesses, about ten handfuls
/// each): nuts are richest in oil, then seeds, and grain and other vegetables
/// give least.
pub fn grams_per_bottle(oil: Oil) -> u32 {
    match oil {
        Oil::Nut => 200,
        Oil::Seed => 300,
        Oil::Grain | Oil::Neutral => 1000,
    }
}

/// How long one bottle takes, in whole seconds: 15 less the cube root of the
/// Alchemy skill's percent, so 15 down to 11.
pub fn press_seconds(skills: &Skills) -> u32 {
    let alchemy = skill_value(skills, crate::ability::ALCHEMY);
    FIRST_SECONDS - cube_root(skill_percent(alchemy))
}

/// The same in action ticks.
pub fn press_ticks(skills: &Skills) -> u64 {
    u64::from(press_seconds(skills)) * ACTION_TICKS_PER_SECOND as u64
}

/// The bottle `item` holding `oil`, keeping its look.
pub fn pressed(item: Item, oil: Oil) -> Item {
    super::pressed_into(item, Liquid::Oil(oil))
}

/// One thing the player could press now, for choosing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PressChoice {
    pub pressing: Pressing,
    /// An oil, or soy sauce.
    pub liquid: Liquid,
    /// One of its stacks, to show and to ask with.
    pub stack: Id,
    /// How many of it are carried.
    pub count: u32,
    /// How many bottles it would fill, with the empty bottles carried.
    pub bottles: u32,
}

impl Inventory {
    /// How many grams of `pressing` are carried.
    pub fn grams_of(&self, pressing: &Pressing) -> u32 {
        self.stacks_where(|item| pressing.is(item))
            .map(|(stack, _)| stack.total_weight())
            .sum()
    }

    /// The stack of `pressing` pressed next: as `empty_bottle`, the smallest
    /// in the character's own grid, else the smallest in a bag.
    pub fn stack_to_press(&self, pressing: &Pressing) -> Option<Id> {
        let smallest = |own: bool| {
            self.stacks_where(|item| pressing.is(item))
                .filter(|&(_, in_own)| in_own == own)
                .min_by_key(|(stack, _)| stack.count())
                .map(|(stack, _)| stack.id)
        };
        smallest(true).or_else(|| smallest(false))
    }

    /// The oil a bottle of `pressing` would be, if there is enough of it for
    /// one and an empty bottle to fill.
    pub fn can_press(&self, pressing: &Pressing, herbs: &Herbs) -> Option<Oil> {
        let oil = pressing.oil(herbs)?;
        let enough = self.grams_of(pressing) >= grams_per_bottle(oil);
        (enough && self.empty_bottle().is_some()).then_some(oil)
    }

    /// Everything that could be pressed now, a bottle's worth at least, in the
    /// order first found (own grid first). Empty if no empty bottle is carried.
    pub fn press_choices(&self, herbs: &Herbs) -> Vec<PressChoice> {
        let empty = self.empty_bottle_count();
        let mut choices: Vec<PressChoice> = Vec::new();
        for (stack, _) in self.stacks_where(|_| true) {
            let Some(pressing) = Pressing::of(stack.top(), herbs) else {
                continue;
            };
            if choices.iter().any(|choice| choice.pressing == pressing) {
                continue;
            }
            let Some(liquid) = pressing.liquid(herbs) else {
                continue;
            };
            let count = self
                .stacks_where(|item| pressing.is(item))
                .map(|(stack, _)| stack.count())
                .sum();
            let bottles = match pressing.oil(herbs) {
                Some(oil) => self.grams_of(&pressing) / grams_per_bottle(oil),
                None => self.soy_sauce_carried(),
            };
            let bottles = bottles.min(empty);
            if bottles > 0 {
                choices.push(PressChoice {
                    pressing,
                    liquid,
                    stack: stack.id,
                    count,
                    bottles,
                });
            }
        }
        choices
    }
}
