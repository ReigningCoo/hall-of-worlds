//! What placing furniture on one level keeps track of.

use std::collections::{BTreeMap, BTreeSet};

use super::super::{BenchStyle, ChairStyle, Material, Piece, TableStyle};
use super::Setting;
use super::palette::Palette;
use super::reach::Reach;
use super::rooms::{Room, rooms_of, usable_cells};
use crate::dungeon::{Level, RoomId};
use crate::rng::Seed;
use crate::site::SiteLevel;

/// Purpose number for whether a ruin has lost a piece.
const LOST: i64 = 0x7275;

/// How a room is furnished: everything in it matches.
#[derive(Clone, Copy, Debug)]
pub(super) struct RoomSet {
    pub(super) material: Material,
    pub(super) table: TableStyle,
    pub(super) chair: ChairStyle,
    pub(super) bench: BenchStyle,
    /// Whether its tables have benches along them rather than chairs.
    pub(super) benches: bool,
}

/// Placing one level's furniture.
pub(super) struct Placer<'a> {
    pub(super) level: &'a Level,
    pub(super) setting: Setting,
    /// This level's seed.
    seed: Seed,
    palette: Palette,
    pub(super) rooms: Vec<Room>,
    usable: BTreeSet<(i64, i64)>,
    reach: Reach,
    taken: BTreeSet<(i64, i64)>,
    pieces: Vec<Piece>,
    sets: BTreeMap<RoomId, RoomSet>,
    /// In 100ths: how many pieces a ruin leaves.
    kept_percent: u64,
}

impl<'a> Placer<'a> {
    pub(super) fn new(
        site: &'a SiteLevel,
        setting: Setting,
        seed: Seed,
        index: u32,
        kept_percent: u64,
    ) -> Self {
        let level = &site.level;
        let stairs: Vec<(i64, i64)> = site.stairs.iter().map(|s| (s.x, s.y)).collect();
        let arrivals: Vec<(i64, i64)> = site.entrance.into_iter().chain(stairs.clone()).collect();
        let rooms = rooms_of(level);
        let usable = usable_cells(level, &rooms, &stairs, &arrivals);
        Self {
            level,
            setting,
            seed: Seed::new(seed.at(&[i64::from(index)])),
            palette: Palette::new(seed, setting.stone_percent),
            rooms,
            usable,
            reach: Reach::new(level, arrivals),
            taken: BTreeSet::new(),
            pieces: Vec::new(),
            sets: BTreeMap::new(),
            kept_percent,
        }
    }

    /// A hash for what `what` names on this level.
    pub(super) fn roll(&self, what: &[i64]) -> u64 {
        self.seed.at(what)
    }

    /// How room `room` is furnished, the same for all its pieces.
    pub(super) fn set_of(&mut self, room: RoomId) -> RoomSet {
        if let Some(&set) = self.sets.get(&room) {
            return set;
        }
        let roll = |what: i64| self.roll(&[i64::from(room.0), 0x7365, what]);
        let material = self.palette.material(roll(0));
        let tables: &[TableStyle] = match material {
            Material::Stone(_) => &TableStyle::STONE,
            Material::Wood(_) => &TableStyle::WOOD,
        };
        let table = tables[(roll(1) % tables.len() as u64) as usize];
        let (chairs, benches) = (table.chairs(), table.benches());
        let set = RoomSet {
            material,
            table,
            chair: chairs[(roll(2) % chairs.len() as u64) as usize],
            bench: benches[(roll(3) % benches.len() as u64) as usize],
            benches: roll(4).is_multiple_of(2),
        };
        self.sets.insert(room, set);
        set
    }

    /// Whether `(x, y)` may be furnished: usable and not yet taken.
    pub(super) fn is_free(&self, cell: (i64, i64)) -> bool {
        self.usable.contains(&cell) && !self.taken.contains(&cell)
    }

    /// Whether nothing is placed within `margin` cells of `piece`'s.
    pub(super) fn clear_round(&self, piece: &Piece, margin: i64) -> bool {
        let (x, y) = piece.at;
        let (width, height) = (i64::from(piece.size.0), i64::from(piece.size.1));
        (y - margin..y + height + margin)
            .all(|cy| (x - margin..x + width + margin).all(|cx| !self.taken.contains(&(cx, cy))))
    }

    /// Places `piece` if its cells are free and it cuts nothing off. A ruin
    /// has lost some pieces: those count as placed but take nothing, so what
    /// is left round them is checked as it will be.
    pub(super) fn try_place(&mut self, piece: Piece) -> bool {
        let cells: Vec<(i64, i64)> = piece.cells().collect();
        if !cells.iter().all(|&cell| self.is_free(cell)) {
            return false;
        }
        let roll = self.roll(&[piece.at.0, piece.at.1, LOST]);
        if roll % 100 >= self.kept_percent {
            return true;
        }
        if !self.reach.take(&cells) {
            return false;
        }
        self.taken.extend(cells);
        self.pieces.push(piece);
        true
    }

    pub(super) fn into_pieces(self) -> Vec<Piece> {
        self.pieces
    }
}
