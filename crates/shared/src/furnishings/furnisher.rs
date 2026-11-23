//! What furnishing one level keeps track of: its rooms, which cells may be
//! furnished, where cellar casks lie, the never-cut-off check and ruin's
//! losses.

use std::collections::BTreeSet;

use super::Floor;
use super::wall::{WallSpot, wall_spots};
use crate::cellar::CellarCask;
use crate::dungeon::RoomId;
use crate::furniture::place::reach::Reach;
use crate::furniture::place::rooms::{Room, rooms_of, usable_cells};
use crate::furniture::{Furniture, Material, Piece, Stone, Wood};
use crate::rng::Seed;
use crate::site::SiteLevel;

/// In 100ths: how much less likely each step of ruin makes a piece to be
/// left (as furniture's).
const RUIN_LOSS_PERCENT: u64 = 15;
/// Purpose numbers for the hashes.
const LOST: i64 = -1;
const WOOD: i64 = -2;
const STONE: i64 = -3;

pub(super) struct Furnisher<'a> {
    pub(super) site: &'a mut SiteLevel,
    pub(super) floor: Floor,
    seed: Seed,
    pub(super) kept_percent: u64,
    pub(super) rooms: Vec<Room>,
    usable: BTreeSet<(i64, i64)>,
    casks: BTreeSet<(i64, i64)>,
    reach: Reach,
}

impl<'a> Furnisher<'a> {
    pub(super) fn new(site: &'a mut SiteLevel, floor: Floor, casks: &[CellarCask]) -> Self {
        let level = &site.level;
        let rooms = rooms_of(level);
        let stairs: Vec<(i64, i64)> = site.stairs.iter().map(|s| (s.x, s.y)).collect();
        let arrivals: Vec<(i64, i64)> = site.entrance.into_iter().chain(stairs.clone()).collect();
        let usable = usable_cells(level, &rooms, &stairs, &arrivals);
        let reach = Reach::new(level, arrivals);
        Self {
            seed: floor.seed.derive("furnishings"),
            kept_percent: 100u64.saturating_sub(RUIN_LOSS_PERCENT * u64::from(floor.ruin)),
            casks: casks.iter().map(|cask| cask.at).collect(),
            site,
            floor,
            rooms,
            usable,
            reach,
        }
    }

    /// A hash of `what` on this level.
    pub(super) fn roll(&self, what: &[i64]) -> u64 {
        self.seed
            .at(&[&[i64::from(self.floor.index)], what].concat())
    }

    /// Whether `cell` may be furnished: usable, with no piece and no cask.
    pub(super) fn is_free(&self, cell: (i64, i64)) -> bool {
        self.usable.contains(&cell)
            && !self.casks.contains(&cell)
            && self.site.level.furniture_at(cell.0, cell.1).is_none()
    }

    /// The room `id`'s floor cells.
    pub(super) fn cells_of(&self, id: RoomId) -> Vec<(i64, i64)> {
        (self.rooms.iter())
            .find(|room| room.id == id)
            .map_or_else(Vec::new, |room| room.cells.clone())
    }

    /// Where a piece `(along, depth)` could stand against a wall with its
    /// back cells on `anchors`, each cell `free`, by hashes of `purpose`.
    pub(super) fn spots(
        &self,
        anchors: &[(i64, i64)],
        size: (u8, u8),
        free: impl Fn((i64, i64)) -> bool,
        purpose: i64,
    ) -> Vec<WallSpot> {
        let hash = |what: &[i64]| self.roll(&[&[purpose], what].concat());
        wall_spots(&self.site.level, anchors, size, free, hash)
    }

    /// A wood for the piece rolled by `what`.
    pub(super) fn wood(&self, what: &[i64]) -> Material {
        let roll = self.roll(&[&[WOOD], what].concat());
        Material::Wood(Seed::new(roll).rng().pick_weighted(&Wood::WEIGHTS))
    }

    /// A stone for the piece rolled by `what`.
    pub(super) fn stone(&self, what: &[i64]) -> Material {
        let roll = self.roll(&[&[STONE], what].concat());
        Material::Stone(Seed::new(roll).rng().pick_weighted(&Stone::WEIGHTS))
    }

    /// Whether ruin has left the thing hashed by `what`.
    pub(super) fn is_left(&self, what: &[i64]) -> bool {
        self.roll(&[&[LOST], what].concat()) % 100 < self.kept_percent
    }

    /// Places `piece` if it cuts nothing off (its cells are the caller's to
    /// check). A ruin has lost some pieces: those count as placed but take
    /// nothing. Whether it counts as placed.
    pub(super) fn place(&mut self, piece: Piece) -> bool {
        if !self.is_left(&[piece.at.0, piece.at.1]) {
            return true;
        }
        let cells: Vec<(i64, i64)> = piece.cells().collect();
        if !self.reach.take(&cells) {
            return false;
        }
        self.site.level.place_furniture(piece);
        true
    }

    /// Places the first spot's piece of `furniture` that cuts nothing off.
    pub(super) fn place_first(
        &mut self,
        spots: Vec<WallSpot>,
        furniture: Furniture,
        material: Material,
    ) -> Option<Piece> {
        let piece = spots
            .into_iter()
            .map(|spot| spot.piece(furniture, material))
            .find(|&piece| self.place(piece))?;
        Some(piece)
    }
}
