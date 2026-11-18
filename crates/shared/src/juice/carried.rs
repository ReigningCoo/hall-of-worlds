//! What a player could press into juice now, and where the juice goes: a
//! carried cask or vessel that takes it (empty, or holding the same juice),
//! alone in its place. The server decides; the client lists the choices by
//! the same rules.

use super::pressing::{juicable, ml_from};
use crate::fruit::Fruits;
use crate::id::Id;
use crate::item::{Inventory, Item};
use crate::potion::{Liquid, Pressing};

/// One fruit the player could press now, for choosing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JuiceChoice {
    /// The fruit, by key.
    pub fruit: String,
    /// One of its stacks, to show and to ask with.
    pub stack: Id,
    /// How many of it are carried.
    pub count: u32,
    /// The millilitres of juice all of it would give.
    pub ml: u32,
    /// The room in the cask or vessel it would go into first.
    pub room: u32,
}

/// Whether `item` (a stack's top) is a cask or vessel that takes some of
/// `juice`: room, and empty or holding the same.
fn takes(item: &Item, juice: &Liquid) -> bool {
    let held = crate::pour::Held::of(item);
    let container = item.cask().is_some() || item.vessel().is_some();
    container
        && held.is_some_and(|held| {
            held.room > 0 && held.liquid.as_ref().is_none_or(|liquid| liquid == juice)
        })
}

impl Inventory {
    /// The carried cask or vessel `fruit`'s juice goes into next: one alone
    /// in its place (not stacked), already holding it before an empty one,
    /// the own grid before bags, the fullest first.
    pub fn juice_target(&self, fruit: &str) -> Option<Id> {
        let juice = Liquid::Juice(fruit.to_owned());
        self.stacks_where(|item| takes(item, &juice))
            .filter(|(stack, _)| stack.count() == 1)
            .max_by_key(|(stack, own)| {
                let ml = crate::pour::Held::of(stack).map_or(0, |held| held.ml);
                (ml > 0, *own, ml)
            })
            .map(|(stack, _)| stack.id)
    }

    /// Whether `fruit` can be pressed now: some is carried, it can be
    /// juiced, and something carried takes its juice.
    pub fn can_press_juice(&self, fruit: &str) -> bool {
        let juicy = Fruits::built_in().get(fruit).is_some_and(juicable);
        let pressing = Pressing::Fruit(fruit.to_owned());
        juicy && self.stack_to_press(&pressing).is_some() && self.juice_target(fruit).is_some()
    }

    /// Every fruit that could be pressed now, in the order first found (own
    /// grid first). Empty if nothing carried would take juice.
    pub fn juice_choices(&self) -> Vec<JuiceChoice> {
        let mut choices: Vec<JuiceChoice> = Vec::new();
        for (stack, _) in self.stacks_where(|item| item.fruit().is_some()) {
            let Some(form) = stack.top().fruit() else {
                continue;
            };
            let fruit = form.fruit.clone();
            if choices.iter().any(|choice| choice.fruit == fruit) {
                continue;
            }
            let Some(def) = Fruits::built_in().get(&fruit).filter(|def| juicable(def)) else {
                continue;
            };
            let Some(target) = self.juice_target(&fruit) else {
                continue;
            };
            let pressing = Pressing::Fruit(fruit.clone());
            let stacks = || self.stacks_where(|item| pressing.is(item));
            let count = stacks().map(|(stack, _)| stack.count()).sum();
            let ml = ml_from(def, self.grams_of(&pressing));
            let room = self
                .find(target)
                .and_then(|found| crate::pour::Held::of(found.top()))
                .map_or(0, |held| held.room);
            choices.push(JuiceChoice {
                fruit,
                stack: stack.id,
                count,
                ml,
                room,
            });
        }
        choices
    }
}

/// `container` (a cask or vessel that takes it) with `ml` more of `juice`,
/// opened; anything else unchanged.
pub fn with_juice(container: &Item, juice: Liquid, ml: u32) -> Item {
    use crate::item_form::ItemForm;
    let form = match &container.form {
        Some(ItemForm::Cask(cask)) => ItemForm::Cask(cask.clone().poured(juice, ml).0),
        Some(ItemForm::Vessel(vessel)) => {
            let total = vessel.ml + ml;
            ItemForm::Vessel(vessel.clone().holding(Some(juice), total))
        }
        _ => return container.clone(),
    };
    Item {
        form: Some(form),
        ..container.clone()
    }
}
