//! A castle's floors above the ground: the keep's, and those of its towers
//! tall enough, each on its own with its own stairs (there is no wall walk).

use super::super::super::SiteLevel;
use super::{Castle, next_room};
use crate::dungeon::{Cell, Level};

impl Castle {
    /// The keep's floor `index` (1 or more) with the towers' floors at that
    /// height, on a level reaching over them all. Round them is `Air`, as
    /// round any building's upper floor (`fill::open_round`), so other floors
    /// are seen past them (`floor_sight`).
    pub(super) fn raise(&self, index: u32, floor: SiteLevel) -> SiteLevel {
        let towers: Vec<_> = (self.towers.iter())
            .filter(|tower| tower.floors > index)
            .collect();
        if towers.is_empty() {
            return floor;
        }
        let (min_x, min_y, width, height) = floor.level.bounds();
        let rects = towers.iter().map(|tower| tower.spot.rect());
        let x0 = rects.clone().map(|r| r.x).fold(min_x, i64::min);
        let y0 = rects.clone().map(|r| r.y).fold(min_y, i64::min);
        let x1 = (rects.clone().map(|r| r.x + r.width)).fold(min_x + width, i64::max);
        let y1 = (rects.map(|r| r.y + r.height)).fold(min_y + height, i64::max);
        let mut level = Level::new(x0, y0, x1 - x0, y1 - y0);
        for y in y0..y1 {
            for x in x0..x1 {
                if floor.level.contains(x, y) {
                    level.set(x, y, floor.level.cell(x, y));
                    level.set_room(x, y, floor.level.room(x, y));
                } else {
                    level.set(x, y, Cell::Air);
                }
            }
        }
        let mut stairs = floor.stairs.clone();
        for tower in towers {
            let room = next_room(&level);
            stairs.extend(tower.build(&mut level, index, room));
        }
        SiteLevel {
            level,
            stairs,
            ..floor
        }
    }
}
