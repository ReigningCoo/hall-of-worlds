//! The drop spaces: where on the panel a carried stack can be put (the
//! source, a Mortar's herbs and oil or pure alcohol, a Retort's liquids, an
//! Alembic's alcohol, a Filter's Powder), what each takes, how many are shown, and putting a
//! stack in or taking it out.

use bevy::prelude::*;
use shared::alchemy::{Apparatus, RETORT_UNITS};
use shared::id::Id;
use shared::item::Inventory;
use shared::potion::Liquid;

use super::bench::{Bench, Station};

/// The most herb spaces a Mortar shows: more kinds than Alchemy allows still
/// go in, halving the chance for each.
pub(super) const MOST_HERBS: usize = 8;

/// One drop space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Place {
    /// A Paste, Ointment or Powder to start from, before the first step.
    Source,
    /// The step's `n`th herb.
    Herb(usize, usize),
    /// The step's oil or pure alcohol (a Mortar's solvent).
    Oil(usize),
    /// The step's drink or pure alcohol fed in (an Alembic's).
    Alcohol(usize),
    /// The step's `n`th 100 ml of liquid.
    Liquid(usize, usize),
    /// The step's Powder (a Filter's).
    Powder(usize),
}

/// The drop spaces a step shows: those filled, one more while there is room,
/// a Mortar's oil, an Alembic's alcohol and a Filter's Powder; a Crucible
/// takes nothing.
pub(super) fn places(step: usize, station: &Station) -> Vec<Place> {
    match station.apparatus {
        Apparatus::Mortar => {
            let herbs = (station.herbs.len() + 1).min(MOST_HERBS);
            let mut places: Vec<Place> = (0..herbs).map(|n| Place::Herb(step, n)).collect();
            places.push(Place::Oil(step));
            places
        }
        Apparatus::Retort => {
            let liquids = (station.liquids.len() + 1).min(RETORT_UNITS);
            (0..liquids).map(|n| Place::Liquid(step, n)).collect()
        }
        Apparatus::Alembic => vec![Place::Alcohol(step)],
        Apparatus::Filter => vec![Place::Powder(step)],
        Apparatus::Crucible | Apparatus::Pelican => Vec::new(),
    }
}

/// What is in a drop space.
pub(super) fn in_place(bench: &Bench, place: Place) -> Option<Id> {
    match place {
        Place::Source => bench.source,
        Place::Herb(step, n) => bench.steps.get(step)?.herbs.get(n).copied(),
        Place::Oil(step) => bench.steps.get(step)?.oil,
        Place::Alcohol(step) => bench.steps.get(step)?.alcohol,
        Place::Liquid(step, n) => bench.steps.get(step)?.liquids.get(n).copied(),
        Place::Powder(step) => bench.steps.get(step)?.powder,
    }
}

/// Whether the carried stack `id` may go in this space: a Paste, Ointment or
/// Powder as the source, a Powder in a Filter, a herb (not one already in that Mortar elsewhere), a bottle
/// of oil or pure alcohol, a bottle of water, oil, a drink or pure alcohol, a
/// bottle of a drink or pure alcohol.
pub(super) fn takes(bench: &Bench, carried: &Inventory, place: Place, id: Id) -> bool {
    if carried.is_worn(id) {
        return false;
    }
    let Some(stack) = carried.find(id) else {
        return false;
    };
    let liquid = stack.top().bottle().and_then(|b| b.liquid.as_ref());
    match place {
        Place::Source => stack.top().mixture().is_some(),
        Place::Herb(step, n) => {
            let elsewhere = (bench.steps.get(step)).is_some_and(|s| {
                s.herbs
                    .iter()
                    .enumerate()
                    .any(|(at, h)| *h == id && at != n)
            });
            !elsewhere && carried.herb_stacks().iter().any(|herb| herb.stack == id)
        }
        Place::Oil(_) => matches!(liquid, Some(Liquid::Oil(_) | Liquid::Alcohol)),
        Place::Liquid(..) => matches!(
            liquid,
            Some(Liquid::Water(_) | Liquid::Oil(_) | Liquid::Drink(_) | Liquid::Alcohol)
        ),
        Place::Alcohol(_) => liquid.is_some_and(|it| it.alcohol_strength() > 0),
        Place::Powder(_) => stack.top().powder().is_some(),
    }
}

/// Puts `id` in a space (or empties it, with `None`): a new herb or liquid
/// goes after the others.
pub(super) fn put(bench: &mut Bench, place: Place, id: Option<Id>) {
    bench.made = None;
    let fill = |list: &mut Vec<Id>, n: usize| match id {
        Some(id) if n < list.len() => list[n] = id,
        Some(id) => list.push(id),
        None if n < list.len() => {
            list.remove(n);
        }
        None => {}
    };
    match place {
        Place::Source => bench.source = id,
        Place::Herb(step, n) => {
            if let Some(station) = bench.steps.get_mut(step) {
                fill(&mut station.herbs, n);
            }
        }
        Place::Oil(step) => {
            if let Some(station) = bench.steps.get_mut(step) {
                station.oil = id;
            }
        }
        Place::Alcohol(step) => {
            if let Some(station) = bench.steps.get_mut(step) {
                station.alcohol = id;
            }
        }
        Place::Powder(step) => {
            if let Some(station) = bench.steps.get_mut(step) {
                station.powder = id;
            }
        }
        Place::Liquid(step, n) => {
            if let Some(station) = bench.steps.get_mut(step) {
                fill(&mut station.liquids, n);
            }
        }
    }
}

/// The faint pictures in empty spaces, saying what each is for.
#[derive(Resource)]
pub(super) struct SpaceIcons {
    source: Handle<Image>,
    herb: Handle<Image>,
    oil: Handle<Image>,
    liquid: Handle<Image>,
    alcohol: Handle<Image>,
    powder: Handle<Image>,
}

impl FromWorld for SpaceIcons {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            source: assets.load("icons/lorc/bubbling-bowl.png"),
            herb: assets.load("icons/delapouite/herbs-bundle.png"),
            oil: assets.load("icons/delapouite/olive.png"),
            liquid: assets.load("icons/delapouite/water-flask.png"),
            alcohol: assets.load("icons/delapouite/beer-bottle.png"),
            powder: assets.load("icons/lorc/powder.png"),
        }
    }
}

impl SpaceIcons {
    pub(super) fn of(&self, place: Place) -> Handle<Image> {
        match place {
            Place::Source => self.source.clone(),
            Place::Herb(..) => self.herb.clone(),
            Place::Oil(_) => self.oil.clone(),
            Place::Liquid(..) => self.liquid.clone(),
            Place::Alcohol(_) => self.alcohol.clone(),
            Place::Powder(_) => self.powder.clone(),
        }
    }
}

/// What a space is for, as its tip.
pub(super) fn tip(place: Place) -> &'static str {
    match place {
        Place::Source => "A Paste, Ointment or Powder to start from, instead of a Mortar",
        Place::Herb(..) => "A herb, one of each kind",
        Place::Oil(_) => "A bottle of oil, pure alcohol for a tincture, or none for a Paste",
        Place::Liquid(..) => "100 ml of water, oil, a drink or pure alcohol",
        Place::Alcohol(_) => {
            "A drink or pure alcohol, reducing what is distilled: the stronger, the more"
        }
        Place::Powder(_) => "A Powder: stats in both add together, the rest are taken out",
    }
}
