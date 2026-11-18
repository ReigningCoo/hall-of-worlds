//! Millers (owner, 2026-11-18): a round mill put into a finished town where
//! its 9 × 9 and a cell of bare ground round it are free on the town's level
//! (inside a wall, within what is built, clear of the ground kept along the
//! wall), with its door towards the nearest road. If nothing fits, the town
//! has none.
//!
//! It never moves anything else: placed from its own seed (`derive("miller")`)
//! once everything else is decided, and added at the end of the buildings, so
//! the others keep their ids (and their quests). Taking 9 × 9 out of a field
//! of bare ground with a ring of it left round leaves every cell as reachable
//! as it was.

use std::collections::VecDeque;

use super::super::grid::{STEPS, roll};
use super::{Block, BuildingKind, Placed, Town, TownBuilding};
use crate::dungeon::{Cell, Level};
use crate::rng::Seed;

/// A mill's side, in cells, and how far its corners are rounded.
const MILL_SIZE: i64 = 9;
const MILL_CORNER: i64 = 4;

/// Where a mill can go: its corner, its door and the way the door faces.
#[derive(Clone, Copy, Debug)]
struct Spot {
    x: i64,
    y: i64,
    door: (i64, i64),
    faces: (i64, i64),
}

impl Town {
    /// Puts in as many millers as the plan asks for and fit, at the end of
    /// `buildings`.
    pub(super) fn add_millers(&mut self, buildings: &mut Vec<TownBuilding>, area: Block) {
        let mut rng = Seed::new(self.seed.derive("miller").at(&[1])).rng();
        for _ in 0..self.millers {
            let spots = self.mill_spots(area);
            if spots.is_empty() {
                return;
            }
            let spot = spots[roll(&mut rng, 0, spots.len() as i64 - 1) as usize];
            let cells = mill_cells(spot.x, spot.y);
            for &(x, y) in &cells {
                self.level.set(x, y, Cell::Wall);
            }
            let placed = Placed {
                cells,
                door: spot.door,
                faces: spot.faces,
                on_road: true,
            };
            let index = buildings.len();
            buildings.push(self.make_building(index, &placed, BuildingKind::Miller));
        }
    }

    /// Every place a mill fits in `area` (its ring may reach a cell past it),
    /// with its door on the side nearest
    /// a road (by walking).
    fn mill_spots(&self, area: Block) -> Vec<Spot> {
        let level = &self.level;
        let bare = Bare::new(level);
        let roads = road_distances(level);
        let mut spots = Vec::new();
        // The mill inside `area`, with a ring of one cell round it.
        for y in area.y0..=area.y1 - MILL_SIZE {
            for x in area.x0..=area.x1 - MILL_SIZE {
                if !bare.all(x - 1, y - 1, MILL_SIZE + 2) {
                    continue;
                }
                let middle = MILL_SIZE / 2;
                let doors = STEPS.map(|(dx, dy)| {
                    let door = (x + middle + dx * (middle), y + middle + dy * (middle));
                    (door, (dx, dy))
                });
                let nearest = doors
                    .iter()
                    .filter_map(|&(door, faces)| {
                        let front = (door.0 + faces.0, door.1 + faces.1);
                        roads.get(front).map(|distance| (distance, door, faces))
                    })
                    .min_by_key(|&(distance, _, _)| distance);
                if let Some((_, door, faces)) = nearest {
                    spots.push(Spot { x, y, door, faces });
                }
            }
        }
        spots
    }
}

/// A mill's cells with its corner at `(x, y)`: a square with its corners
/// rounded, as a building's `Outline::RoundCorners`.
fn mill_cells(x: i64, y: i64) -> Vec<(i64, i64)> {
    let r = MILL_CORNER;
    let mut cells = Vec::new();
    for cy in 0..MILL_SIZE {
        for cx in 0..MILL_SIZE {
            let dx = cx.min(MILL_SIZE - 1 - cx);
            let dy = cy.min(MILL_SIZE - 1 - cy);
            // Within a quarter circle in each corner, in half cells.
            let (ax, ay) = (2 * (r - dx) - 1, 2 * (r - dy) - 1);
            if dx >= r || dy >= r || ax * ax + ay * ay <= 4 * r * r {
                cells.push((x + cx, y + cy));
            }
        }
    }
    cells
}

/// Counts of bare ground (`Cell::Ground`) over the level, summed so any
/// square can be checked at once.
struct Bare {
    x0: i64,
    y0: i64,
    width: i64,
    height: i64,
    /// Bare cells above and left of each grid corner.
    sums: Vec<i64>,
}

impl Bare {
    fn new(level: &Level) -> Self {
        let (x0, y0, width, height) = level.bounds();
        let stride = width + 1;
        let mut sums = vec![0; (stride * (height + 1)) as usize];
        for row in 0..height {
            for col in 0..width {
                let here = i64::from(level.cell(x0 + col, y0 + row) == Cell::Ground);
                let at = |c: i64, r: i64| (r * stride + c) as usize;
                sums[at(col + 1, row + 1)] =
                    here + sums[at(col, row + 1)] + sums[at(col + 1, row)] - sums[at(col, row)];
            }
        }
        Self {
            x0,
            y0,
            width,
            height,
            sums,
        }
    }

    /// Whether the square of `side` cells from `(x, y)` is all bare ground.
    fn all(&self, x: i64, y: i64, side: i64) -> bool {
        let (c0, r0) = (x - self.x0, y - self.y0);
        let (c1, r1) = (c0 + side, r0 + side);
        if c0 < 0 || r0 < 0 || c1 > self.width || r1 > self.height {
            return false;
        }
        let stride = self.width + 1;
        let at = |c: i64, r: i64| self.sums[(r * stride + c) as usize];
        at(c1, r1) - at(c0, r1) - at(c1, r0) + at(c0, r0) == side * side
    }
}

/// Steps from each walkable cell to the nearest road, walking.
struct Distances {
    x0: i64,
    y0: i64,
    width: i64,
    steps: Vec<Option<u32>>,
}

impl Distances {
    fn index(&self, (x, y): (i64, i64)) -> Option<usize> {
        let (col, row) = (x - self.x0, y - self.y0);
        let height = self.steps.len() as i64 / self.width.max(1);
        ((0..self.width).contains(&col) && (0..height).contains(&row))
            .then_some((row * self.width + col) as usize)
    }

    fn get(&self, at: (i64, i64)) -> Option<u32> {
        self.index(at).and_then(|i| self.steps[i])
    }
}

fn road_distances(level: &Level) -> Distances {
    let (x0, y0, width, height) = level.bounds();
    let mut distances = Distances {
        x0,
        y0,
        width,
        steps: vec![None; (width * height) as usize],
    };
    let mut queue = VecDeque::new();
    for y in y0..y0 + height {
        for x in x0..x0 + width {
            if level.cell(x, y).is_road() {
                let i = distances.index((x, y)).expect("on the level");
                distances.steps[i] = Some(0);
                queue.push_back((x, y));
            }
        }
    }
    while let Some((x, y)) = queue.pop_front() {
        let here = distances.get((x, y)).expect("reached");
        for (dx, dy) in STEPS {
            let next = (x + dx, y + dy);
            let Some(i) = distances.index(next) else {
                continue;
            };
            if distances.steps[i].is_none() && level.is_walkable(next.0, next.1) {
                distances.steps[i] = Some(here + 1);
                queue.push_back(next);
            }
        }
    }
    distances
}
