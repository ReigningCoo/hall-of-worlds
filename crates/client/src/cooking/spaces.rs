//! The drop spaces: where on the panel something can be put (each row's
//! source, a step's foods and liquids, the water faced, the culture), what
//! each takes, how many are shown, and putting something in or taking it
//! out. Dropping a stack already in a step again adds one more of it (or
//! 100 ml more of a liquid); a click or tap on a filled space empties it,
//! and on the water faced adds 100 ml (going back to none past the most).

use bevy::prelude::*;
use shared::cooking::Ingredients;
use shared::id::Id;
use shared::item::{Inventory, Item, ItemKind};
use shared::potion::Liquid;
use shared::pour::{BOTTLE_ML, Held};

use super::bench::{Bench, Station};
use super::kitchen::KitchenNow;

/// The most food spaces a step shows.
pub(super) const MOST_FOODS: usize = 6;
/// The most liquid spaces a step shows.
pub(super) const MOST_LIQUIDS: usize = 2;
/// Millilitres added a time to a liquid or the water faced.
pub(super) const ML_STEP: u32 = BOTTLE_ML;
/// The most millilitres of water faced a step takes before going back to none.
pub(super) const MOST_FACED: u32 = 1000;

/// One drop space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Place {
    /// The row's intermediate to start from.
    Source(u8),
    /// The step's `n`th food.
    Food(usize, usize),
    /// The step's `n`th liquid.
    Liquid(usize, usize),
    /// The step's water from the water faced.
    Faced(usize),
    /// The culture for an output that ferments.
    Culture,
    /// The step's `n`th ghost: what a recipe loaded by Make still misses
    /// there (`ghosts`).
    Ghost(usize, usize),
}

/// The spaces a step shows: those filled and one more while there is room,
/// the water faced while there is water to face (or some is in), and its
/// `ghosts`.
pub(super) fn places(
    step: usize,
    station: &Station,
    water_faced: bool,
    ghosts: usize,
) -> Vec<Place> {
    let foods = (station.foods.len() + 1).min(MOST_FOODS);
    let liquids = (station.liquids.len() + 1).min(MOST_LIQUIDS);
    let mut places: Vec<Place> = (0..foods).map(|n| Place::Food(step, n)).collect();
    places.extend((0..liquids).map(|n| Place::Liquid(step, n)));
    if water_faced || station.faced > 0 {
        places.push(Place::Faced(step));
    }
    places.extend((0..ghosts).map(|n| Place::Ghost(step, n)));
    places
}

/// What is in a space: a stack and how many of it (items, or millilitres).
pub(super) fn in_place(bench: &Bench, place: Place) -> Option<(Id, u32)> {
    match place {
        Place::Source(row) => bench
            .sources
            .get(usize::from(row))
            .copied()?
            .map(|id| (id, 1)),
        Place::Food(step, n) => bench.steps.get(step)?.foods.get(n).copied(),
        Place::Liquid(step, n) => bench.steps.get(step)?.liquids.get(n).copied(),
        Place::Faced(_) | Place::Ghost(..) => None,
        Place::Culture => bench.culture.map(|id| (id, 1)),
    }
}

/// The carried stack `id`, if carried and not worn.
fn carried_stack(carried: &Inventory, id: Id) -> Option<&Item> {
    (!carried.is_worn(id)).then(|| carried.find(id)).flatten()
}

/// The water, milk or honey `item` holds, if any.
fn cooking_liquid(item: &Item) -> Option<Liquid> {
    let liquid = Held::of(item)?.liquid?;
    let ok =
        matches!(liquid, Liquid::Water(_)) || Ingredients::built_in().of_liquid(&liquid).is_some();
    ok.then_some(liquid)
}

/// Whether `id` may go in this space: an intermediate, carried or in the
/// hearth's storage, as a source; carried raw food with culinary data; a
/// bottle, vessel or cask of water, milk or honey; a carried Yeast item as
/// the culture.
pub(super) fn takes(carried: &Inventory, kitchen: &KitchenNow, place: Place, id: Id) -> bool {
    if let Place::Source(_) = place {
        let item = carried_stack(carried, id)
            .map(Item::top)
            .or(kitchen.stored(id));
        return item.is_some_and(|item| item.kind == ItemKind::Intermediate);
    }
    let Some(stack) = carried_stack(carried, id) else {
        return false;
    };
    let top = stack.top();
    match place {
        // A ghost asks the bench (`Bench::ghost_takes`).
        Place::Source(_) | Place::Faced(_) | Place::Ghost(..) => false,
        Place::Food(..) => Ingredients::built_in().of_item(top).is_some(),
        Place::Liquid(..) => cooking_liquid(top).is_some(),
        Place::Culture => top.yeast().is_some(),
    }
}

/// The most millilitres a stack can give: whole bottles, or what a vessel
/// or cask holds.
pub(super) fn most_ml(carried: &Inventory, id: Id) -> u32 {
    let Some(stack) = carried.find(id) else {
        return 0;
    };
    match stack.top().kind {
        ItemKind::Bottle => stack.count() * BOTTLE_ML,
        _ => Held::of(stack.top()).map_or(0, |held| held.ml),
    }
}

/// Puts `id` in a space, or one more of it where it already is in that
/// step; `None` empties the space.
pub(super) fn put(bench: &mut Bench, carried: &Inventory, place: Place, id: Option<Id>) {
    bench.made = None;
    match place {
        Place::Source(row) => {
            if let Some(source) = bench.sources.get_mut(usize::from(row)) {
                *source = id;
            }
        }
        Place::Culture => bench.culture = id,
        Place::Ghost(..) => {}
        Place::Faced(step) => {
            if let Some(station) = bench.steps.get_mut(step) {
                station.faced = match id {
                    None => 0,
                    Some(_) => station.faced + ML_STEP,
                };
            }
        }
        Place::Food(step, n) => {
            if let Some(station) = bench.steps.get_mut(step) {
                fill(&mut station.foods, n, id, 1, u32::MAX);
            }
        }
        Place::Liquid(step, n) => {
            let most = id.map_or(0, |id| most_ml(carried, id));
            if let Some(station) = bench.steps.get_mut(step) {
                fill(&mut station.liquids, n, id, ML_STEP, most);
            }
        }
    }
}

/// The water faced clicked or tapped: 100 ml more, none past the most.
pub(super) fn more_faced(bench: &mut Bench, step: usize) {
    bench.made = None;
    if let Some(station) = bench.steps.get_mut(step) {
        let more = station.faced + ML_STEP;
        station.faced = if more > MOST_FACED { 0 } else { more };
    }
}

/// One list of a step's spaces: `id` added `step` more where it is already
/// (at most `most`), else put at `n` (replacing what was there) or after the
/// rest; `None` takes the `n`th out.
fn fill(list: &mut Vec<(Id, u32)>, n: usize, id: Option<Id>, step: u32, most: u32) {
    let Some(id) = id else {
        if n < list.len() {
            list.remove(n);
        }
        return;
    };
    if let Some(entry) = list.iter_mut().find(|(it, _)| *it == id) {
        entry.1 = (entry.1 + step).min(most).max(entry.1);
    } else if n < list.len() {
        list[n] = (id, step.min(most));
    } else if most > 0 {
        list.push((id, step.min(most)));
    }
}

/// The faint pictures in empty spaces, saying what each is for.
#[derive(Resource)]
pub(super) struct SpaceIcons {
    source: Handle<Image>,
    food: Handle<Image>,
    liquid: Handle<Image>,
    faced: Handle<Image>,
    culture: Handle<Image>,
    oven: Handle<Image>,
    knife: Handle<Image>,
}

impl FromWorld for SpaceIcons {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            source: assets.load("icons/lorc/bubbling-bowl.png"),
            food: assets.load("icons/delapouite/berries-bowl.png"),
            liquid: assets.load("icons/delapouite/water-flask.png"),
            faced: assets.load("icons/lorc/drop.png"),
            culture: assets.load("icons/delapouite/bread.png"),
            oven: assets.load("icons/delapouite/chicken-oven.png"),
            knife: assets.load("icons/lorc/bowie-knife.png"),
        }
    }
}

impl SpaceIcons {
    /// A tool's icon, for those drawn without an item (`boxes::tool_item`).
    pub(super) fn tool(&self, tool: shared::cooking::Tool) -> Handle<Image> {
        match tool {
            shared::cooking::Tool::Knife => self.knife.clone(),
            _ => self.oven.clone(),
        }
    }

    pub(super) fn of(&self, place: Place) -> Handle<Image> {
        match place {
            Place::Source(_) => self.source.clone(),
            Place::Food(..) | Place::Ghost(..) => self.food.clone(),
            Place::Liquid(..) => self.liquid.clone(),
            Place::Faced(_) => self.faced.clone(),
            Place::Culture => self.culture.clone(),
        }
    }
}

/// What a space is for, as its tip.
pub(super) fn tip(place: Place) -> &'static str {
    match place {
        Place::Source(_) => "Dough, stock, butter or another intermediate to start the row from",
        Place::Food(..) => "A raw food: drop it again for one more",
        Place::Liquid(..) => "Water, milk or honey: 100 ml a drop",
        Place::Faced(_) => "Water from the water faced: click or tap for 100 ml more",
        Place::Culture => "A culture (yeast, yoghurt, koji) for whatever ferments",
        Place::Ghost(..) => "Missing: drop one here, or click or tap to cook without it",
    }
}
