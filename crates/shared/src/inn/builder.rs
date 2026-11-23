//! Carving a floor of the inn out of solid wall: rooms, doors and windows.

use super::{DEPTH, InnRoom, RoomKind, WIDTH};
use crate::door::Swing;
use crate::dungeon::{Cell, Level, RoomId};
use crate::site::Rect;

/// A floor being made, and its rooms so far.
pub(super) struct Built {
    pub(super) level: Level,
    pub(super) rooms: Vec<InnRoom>,
}

impl Built {
    /// All wall, no rooms.
    pub(super) fn new() -> Self {
        Self {
            level: Level::new(0, 0, WIDTH, DEPTH),
            rooms: Vec::new(),
        }
    }

    /// A room of `kind` over `rects` (floor, marked with its id, the next
    /// one from 1); returns its index in `rooms`.
    pub(super) fn room(&mut self, kind: RoomKind, rects: Vec<Rect>) -> usize {
        let id = RoomId(self.rooms.len() as u16 + 1);
        for rect in &rects {
            for y in rect.y..rect.y + rect.height {
                for x in rect.x..rect.x + rect.width {
                    self.level.set(x, y, Cell::Floor);
                    self.level.set_room(x, y, Some(id));
                }
            }
        }
        self.rooms.push(InnRoom {
            id,
            kind,
            rects,
            door: None,
        });
        self.rooms.len() - 1
    }

    /// A closed door at `at` (locked if `locked`), the door of room `of` if
    /// given.
    pub(super) fn door(&mut self, (x, y): (i64, i64), of: Option<usize>, locked: bool) {
        self.level.set(x, y, Cell::Door);
        self.level.set_room(x, y, None);
        self.level.set_swing(x, y, Swing::Closed);
        self.level.set_locked(x, y, locked);
        if let Some(index) = of {
            self.rooms[index].door = Some((x, y));
        }
    }

    /// A window in a wall at `at` (inside walls only: the inn has none out).
    pub(super) fn window(&mut self, (x, y): (i64, i64)) {
        self.level.set(x, y, Cell::Window);
        self.level.set_room(x, y, None);
    }
}

/// A rectangle from its corners, both included.
pub(super) const fn span(x0: i64, y0: i64, x1: i64, y1: i64) -> Rect {
    Rect {
        x: x0,
        y: y0,
        width: x1 - x0 + 1,
        height: y1 - y0 + 1,
    }
}
