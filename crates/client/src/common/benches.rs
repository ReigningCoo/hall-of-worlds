//! Furniture with places on the player's level (`shared::furniture::Held`:
//! crafting benches, cask racks): what the server last told lies on each,
//! and the crafting bench the player is at now.
//! `crafting_bench` keeps both; `dungeon_map` draws the tools, `inventory`
//! sends what is let go on a bench's panel, and `abilities` and `alchemy`
//! work at the bench the player is at.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use shared::crafting_bench::BenchTools;
use shared::furniture::{Held, Piece};
use shared::id::Id;

/// What tells a bench's stand-in id from others' (`Id::stand_in`): the
/// top 16 bits of its number (the UUID's variant bits agree with them).
const BENCH_MARK: u64 = 0xBE0C << 48;
/// Each of a bench's cell's numbers is kept in 24 bits.
const CELL_BITS: u64 = 0xFF_FFFF;

/// What lies on each piece with places as the server last told it (a bench's
/// tools, a rack's casks), by its piece's north-west cell (`Piece::at`), and
/// the hearths it said are lit (cooking plan PR 8). Out of sight they stay
/// as last told.
#[derive(Resource, Default, Debug, PartialEq)]
pub struct Benches(pub HashMap<(i64, i64), Held>, pub HashSet<(i64, i64)>);

impl Benches {
    /// The tools of the bench at `at`, if told and it is a bench.
    pub fn tools(&self, at: (i64, i64)) -> Option<&BenchTools> {
        self.0.get(&at).and_then(Held::bench)
    }

    /// The bench at `at` as an id, for its panel's grid (`StockCell`,
    /// `TakesGifts`): a stand-in, never a real id, that `at_of` reads back.
    pub fn id_of(at: (i64, i64)) -> Id {
        let (x, y) = (at.0 as u64 & CELL_BITS, at.1 as u64 & CELL_BITS);
        Id::stand_in(BENCH_MARK | x << 24 | y)
    }

    /// The bench whose id `id` is (`id_of`), if it is a bench's.
    pub fn at_of(id: Id) -> Option<(i64, i64)> {
        if !id.is_stand_in() {
            return None;
        }
        let bytes = id.0.as_bytes();
        let n = u64::from_be_bytes(bytes[8..].try_into().ok()?);
        if n & !(CELL_BITS << 24 | CELL_BITS) != BENCH_MARK {
            return None;
        }
        // Back from 24 bits, keeping the sign.
        let cell = |bits: u64| ((bits << 40) as i64) >> 40;
        Some((cell(n >> 24 & CELL_BITS), cell(n & CELL_BITS)))
    }
}

/// The crafting bench the player is at now (`crafting_bench::bench_faced`),
/// with its tools if they have been told; `None` away from any.
#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct BenchHere(pub Option<(Piece, Option<BenchTools>)>);

impl BenchHere {
    /// Its piece's north-west cell, to send with an order.
    pub fn at(&self) -> Option<(i64, i64)> {
        self.0.as_ref().map(|(piece, _)| piece.at)
    }

    /// Its tools, if told.
    pub fn tools(&self) -> Option<&BenchTools> {
        self.0.as_ref().and_then(|(_, tools)| tools.as_ref())
    }
}

#[cfg(test)]
mod tests;
