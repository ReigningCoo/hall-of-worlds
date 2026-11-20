//! A town's yards (`site::pen`): a few small fenced yards on bare ground for
//! hens, pigeons and the odd goat or pig, placed last from their own seed.

use super::super::farm;
use super::super::pen::{PenKind, PenSize, add_pen};
use super::super::{BuildingLook, FieldLook};
use super::Town;
use super::road::Block;
use crate::rng::Seed;

const YARD: PenSize = PenSize {
    kind: PenKind::Yard,
    inside: (3, 3),
    margin: 1,
    dovecote_percent: 40,
};

/// One yard for every this many buildings planned, at least one, at most
/// `MOST_YARDS` (first guesses).
const BUILDINGS_A_YARD: i64 = 20;
const MOST_YARDS: i64 = 3;

impl Town {
    /// Fences the town's yards within `area`, each near a building's door
    /// chosen by a hash but never just in front of one or the way in.
    pub(super) fn add_pens(
        &mut self,
        buildings: &[BuildingLook],
        area: Block,
        entrance: (i64, i64),
    ) -> Vec<FieldLook> {
        if buildings.is_empty() {
            return Vec::new();
        }
        let seed = self.seed.derive("town pens");
        let wanted = (1 + self.target / BUILDINGS_A_YARD).min(MOST_YARDS);
        let mut avoid: Vec<_> = buildings.iter().map(|look| look.door).collect();
        avoid.push(entrance);
        let area = farm::Plot {
            x0: area.x0,
            y0: area.y0,
            x1: area.x1,
            y1: area.y1,
        };
        (0..wanted)
            .filter_map(|index| {
                let own = Seed::new(seed.at(&[index]));
                let near = buildings[(own.at(&[-1]) % buildings.len() as u64) as usize].door;
                add_pen(&mut self.level, area, YARD, near, &avoid, own)
            })
            .collect()
    }
}
