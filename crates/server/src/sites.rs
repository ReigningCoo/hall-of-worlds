//! Players in sites: entering them, moving about, taking stairs and leaving, and sending each
//! player the cells of their level as they come to see them (or are about to). What a player has
//! seen is kept with their character; each site keeps its generator version, when its levels
//! were last visited and players' changes (`storage::SiteSave`).

use bevy::prelude::*;
use shared::creature::Size;
use shared::dungeon::Sight;
use shared::movement::{Gait, LevelMove, move_on_level};
use shared::poi::Poi;
use shared::protocol::SeenStairs;
use shared::site::{Site, SiteLevel};
use shared::spot::Spot;

use crate::storage::{CellMasks, SiteId, VisitSave};

// Split into files (`cargo xtask lint` keeps files small).
mod benches;
pub(crate) mod campfires;
mod cellar;
mod cells;
mod dev_area;
mod dev_furniture;
mod dev_place;
mod doors;
mod enter;
mod furnishings;
mod generator;
mod inn;
mod keepers;
mod key_place;
mod leave;
mod levels;
mod moves;
mod other_floors;
mod place;
mod place_name;
mod plants;
mod quest_dots;
mod quest_place;
mod reach;
mod remembered;
mod shop_here;
mod sight;
mod trade;
mod trees;
mod up_down;
mod wild;

pub(crate) use benches::{BenchKey, HeldHere, bench_tools, create_bench, keep_bench, keep_held};
pub(crate) use benches::{fireside, settle_held, settle_storage, share_benches};
use benches::{put_made_benches, stock_benches};
pub(crate) use campfires::{FireTarget, fire_target, light_or_feed, sync_campfires};
use cellar::{stock_cellar, stock_set_in_cellar};
use cells::{
    add_masks, all_cells, beside, cells_in, keep_within, level_chunks, nearby_cells, send_cells,
};
pub(super) use dev_area::generate_area;
pub(crate) use dev_furniture::{FurnitureChange, place_decor, place_furniture, share_furniture};
pub(crate) use dev_place::{dev_wilderness, land_near};
pub(crate) use doors::{
    DoorChange, Standing, door_act, door_to_kick, kick_door, mob_opened, share_doors,
};
use doors::{DoorSite, VisitDoors, door_looks, ensure_locks, remember_doors, set_doors};
use enter::enter_site;
#[cfg(test)]
pub(crate) use enter::test_enter_poi;
pub(super) use enter::{dev_enter, enter_dungeon, enter_poi, generate, resume};
pub(crate) use generator::{seen_poi, site_generator};
pub(crate) use inn::{
    InnMove, InnState, Litter, clear_litter, close_empty_inns, move_inn, out_of_inn,
};
pub(crate) use inn::{close_own_room, hidden_in_own_room, keep_own_room, refuses_harm};
pub(crate) use inn::{dev_to_inn, enter_from_hall, follow_inn, in_inn, inn_poi, leave_inn};
use keepers::{NewKeepers, keepers_in_window, keepers_of};
pub(super) use keepers::{give_to_keeper, keeper_building, keeper_stock, take_from_keeper};
pub(crate) use key_place::key_place;
use leave::leave_for_poi;
pub(super) use leave::{forget_site, killed, leave, leave_dungeon, put_out, use_exit};
use levels::{NewVisit, arrive_again, enter_level};
pub(super) use moves::{moved, refused, set_cell, teleport};
use other_floors::{OtherFloor, floors_of, see_other_floors};
use place::{Arrival, Mirror, Place, SetIn};
pub(crate) use place_name::place_name_of;
pub(crate) use plants::{
    PlantChange, keep_harvest, picked_of, plant_key, put_plants, set_picked, share_plants,
    site_plants,
};
pub(crate) use quest_place::{QuestPlace, beside_giver, town_buildings};
use sight::memory;
pub(crate) use sight::update_sight;
pub(super) use sight::{forget_level, reveal_level, truesight_changed};
pub(super) use trade::open_trade;
pub(crate) use trees::{
    TreeChange, TreeNews, grow_trees, keep_tree, saved_tree, share_trees, tree_here, tree_stage,
};
pub(super) use up_down::{go_down, go_up};
pub(crate) use wild::wild_chunk;
pub(super) use wild::{area_level, explore, follow, leave_by_key};

/// How far in a town others can be made out, in cells.
const TOWN_SIGHT: u32 = 40;

/// A player's stay on a site level, or in the wilderness.
pub(crate) struct Visit {
    place: Place,
    id: SiteId,
    index: u32,
    level: SiteLevel,
    /// Its stairs as players see them.
    stairs: Vec<SeenStairs>,
    /// Its doors, as they are now (also set on `level`).
    doors: VisitDoors,
    /// Where the player is, in units.
    position: Spot,
    /// The cells the client has been sent on this visit.
    sent: CellMasks,
    /// The cells already looked out from for what to send, and what was seen
    /// from them, so each step only looks from the cells that are new.
    looked_from: Sight,
    loaded: Sight,
    /// What can be seen now, by the game's rule: who else is in sight. Worked out
    /// from `view_from`, and only again once the player has moved.
    view: Sight,
    view_from: Option<(i64, i64)>,
    /// Its cells that are another map's too.
    mirrors: Vec<Mirror>,
    /// In the wilderness, which live area's mobs are about the player
    /// (`entities::WildAreas`); 0 until one is given.
    wild_area: u32,
    /// Cells Observation adds to sight's reach (`facing::Field::further`).
    further: u32,
    /// A building's other floors, seen from this one (`other_floors`).
    floors: Vec<OtherFloor>,
    /// Benches told of, by cell, and whether a hearth was told lit (`benches`).
    benches_sent: std::collections::BTreeMap<(i64, i64), bool>,
    /// In The Way Inn, where what is in the player's own rooms is kept
    /// (`SiteId::InnRoom`): their copy of each, whoever else is in it.
    own_room: Option<SiteId>,
}

impl Visit {
    /// The cell the player is in on the level: what cell rules go by.
    pub(crate) fn position(&self) -> (i64, i64) {
        self.position.cell()
    }

    /// Where exactly the player is on the level, in units.
    pub(crate) fn spot(&self) -> Spot {
        self.position
    }

    /// The ground kept level here: the wilderness's cutouts in its window.
    #[cfg(test)]
    pub(crate) fn flats(&self) -> Vec<shared::movement::FlatGround> {
        self.place.flats()
    }

    /// How a move from here to `to` goes, if someone with this `gait` may take
    /// it (`movement::move_on_level`: a good swimmer swims water instead of
    /// jumping it).
    pub(crate) fn landing(&self, to: Spot, gait: Gait, size: Size) -> Option<LevelMove> {
        move_on_level(&self.level.level, size, gait, self.position, to)
    }

    /// The stairs the player stands on (`Spot::stands_on`), if any.
    pub(crate) fn stairs_underfoot(&self) -> Option<&shared::site::Stairs> {
        let stairs = self.level.stairs.iter();
        stairs
            .into_iter()
            .find(|s| self.position.stands_on((s.x, s.y)))
    }

    /// Which level of the site it is.
    pub(crate) fn index(&self) -> u32 {
        self.index
    }

    /// Whose mobs are about the player: the level's, or in the wilderness their
    /// live area's.
    pub(crate) fn population(&self) -> (SiteId, u32) {
        if self.is_wilderness() {
            (self.id, self.wild_area)
        } else {
            (self.id, self.index)
        }
    }

    /// The wilderness's live area the player is in.
    pub(crate) fn wild_area(&self) -> u32 {
        self.wild_area
    }

    pub(crate) fn set_wild_area(&mut self, area: u32) {
        self.wild_area = area;
    }

    /// Which site it is, for the debug API.
    #[cfg(feature = "debug-api")]
    pub(crate) fn site_id(&self) -> SiteId {
        self.id
    }

    /// The level's cells as the server has them (for reach, and the debug API).
    pub(crate) fn cells(&self) -> &shared::dungeon::Level {
        &self.level.level
    }

    /// Whether something at this cell can be seen from where the player is.
    pub(crate) fn can_see(&mut self, x: i64, y: i64) -> bool {
        if self.view_from != Some(self.position()) {
            let (from_x, from_y) = self.position();
            let level = &self.level.level;
            self.view = Sight::new(level);
            self.view.add_view(level, from_x, from_y, self.reach());
            self.view_from = Some(self.position());
        }
        self.view.is_seen(x, y)
    }

    /// Which site it is.
    pub(crate) fn id(&self) -> SiteId {
        self.id
    }

    /// The poi it is behind, if a site's.
    pub(crate) fn poi(&self) -> Option<&Poi> {
        match &self.place {
            Place::Site { poi, .. } => Some(poi),
            Place::Wilderness { .. } => None,
        }
    }

    /// Its site, if a site's.
    pub(crate) fn site(&self) -> Option<&Site> {
        match &self.place {
            Place::Site { site, .. } => Some(site),
            Place::Wilderness { .. } => None,
        }
    }

    /// Whether it is in the wilderness.
    pub(crate) fn is_wilderness(&self) -> bool {
        matches!(self.place, Place::Wilderness { .. })
    }

    /// Where a cell's column is kept (remembered, changed): where it is round
    /// the planet in the wilderness (see `wild`), else itself.
    pub(crate) fn kept_at(&self, x: i64) -> i64 {
        self.wraps_at().map_or(x, |width| x.rem_euclid(width))
    }

    /// Whether a cell is another map's too, or a poi's cutout in the
    /// wilderness (see `sight::learnt_elsewhere`).
    fn shares(&self, x: i64, y: i64) -> bool {
        let inside = |(min_x, min_y, width, height): (i64, i64, i64, i64)| {
            (min_x..min_x + width).contains(&x) && (min_y..min_y + height).contains(&y)
        };
        self.mirrors.iter().any(|mirror| inside(mirror.bounds))
            || match &self.place {
                Place::Wilderness { cutouts, .. } => cutouts.iter().any(|c| inside(c.bounds)),
                Place::Site { .. } => false,
            }
    }

    /// Where its cells wrap round the planet: the wilderness's width.
    pub(crate) fn wraps_at(&self) -> Option<i64> {
        match self.place {
            Place::Wilderness { width, .. } => Some(width),
            Place::Site { .. } => None,
        }
    }

    /// Whether they came into the site from the wilderness.
    pub(crate) fn came_from_wilderness(&self) -> bool {
        matches!(
            self.place,
            Place::Site {
                from_wilderness: true,
                ..
            }
        )
    }

    /// What is saved with the character.
    pub(crate) fn save(&self) -> VisitSave {
        VisitSave {
            site: self.id,
            level: self.index,
            position: self.position,
            from_wilderness: self.came_from_wilderness(),
        }
    }
}

#[cfg(test)]
mod dev_area_tests;
#[cfg(test)]
mod difficulty_sight_tests;
#[cfg(test)]
mod leave_key_tests;
#[cfg(test)]
mod left_at_tests;
#[cfg(test)]
mod moves_tests;
#[cfg(test)]
mod other_floors_tests;
#[cfg(test)]
mod place_name_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod up_down_tests;
#[cfg(test)]
mod visit_for_tests;
