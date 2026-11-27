//! A town's quests: every giver's slots, each a chain of one to three quests
//! on hooks near the town; then needs on smaller towns' feeders, on other
//! givers' first quests and on standing, the town's own feeder, and in a city its
//! artifact line.

use std::collections::{BTreeMap, BTreeSet};

use super::area::{Candidate, candidates};
use super::artifact_line::artifact_line;
use super::feeder_quest::{add_feeder_needs, feeder_quest};
use super::neighbour::add_neighbour_needs;
use super::opening::add_standing_needs;
use super::slot::{Claims, slot_quests};
use super::world::{MOST_RADIUS, pois_within, radius};
use super::{Quest, QuestWorld, givers};
use crate::poi::Poi;
use crate::site::{BuildingKind, BuildingLook};

/// Every normal quest `town` offers, givers in the buildings' order, each
/// slot's chain in order, then its feeder. `buildings` are its site's
/// (`SiteLevel::buildings`' looks). Empty for anything but a town.
pub fn town_quests(world: &QuestWorld, town: &Poi, buildings: &[BuildingLook]) -> Vec<Quest> {
    if !town.kind.is_town() {
        return Vec::new();
    }
    let shops = shops_of(buildings);
    let candidates = town_candidates(world, town, false, &shops);
    let mut claims = Claims {
        taken: BTreeSet::new(),
        uses: BTreeMap::new(),
    };
    let mut quests = Vec::new();
    for giver in givers(town, buildings) {
        for slot in 0..giver.slots {
            quests.extend(slot_quests(
                world,
                town,
                &giver,
                slot,
                &candidates,
                &shops,
                &mut claims,
            ));
        }
    }
    add_feeder_needs(world, town, &mut quests);
    add_neighbour_needs(town, &mut quests);
    add_standing_needs(town, &mut quests);
    quests.extend(feeder_quest(world, town, buildings));
    quests.extend(artifact_line(world, town, buildings));
    quests
}

/// What `town`'s quests (or dailies) may use. `shops`: the giver's town's
/// buildings' kinds (`shops_of`), whose stock no gathering step asks for.
pub(super) fn town_candidates(
    world: &QuestWorld,
    town: &Poi,
    daily: bool,
    shops: &[BuildingKind],
) -> Vec<Candidate> {
    let reach = radius(town.kind, daily) + MOST_RADIUS;
    let towns: Vec<Poi> = (pois_within(world.pois, town, reach).into_iter())
        .filter(|poi| poi.kind.is_town())
        .collect();
    let gathers = |poi: &Poi| !herb_choices(world, poi, shops).is_empty();
    candidates(world.pois, town, &towns, daily, &gathers)
}

/// The kinds of a town's buildings, for what its shops could stock.
pub(super) fn shops_of(buildings: &[BuildingLook]) -> Vec<BuildingKind> {
    buildings.iter().map(|building| building.kind).collect()
}

/// The herbs a gathering step at `poi` may ask for (`herb_ask`).
pub(super) fn herb_choices<'a>(
    world: &QuestWorld<'a>,
    poi: &Poi,
    shops: &[BuildingKind],
) -> Vec<&'a crate::herb::HerbDef> {
    let stocked = |herb: &str| super::herb_ask::shops_stock(shops, herb);
    super::herb_ask::herb_choices(world.herbs, &(world.habitat_of)(poi), &stocked)
}
