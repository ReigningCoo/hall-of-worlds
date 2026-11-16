# shared: floor_sight

What a viewer on one floor of a building sees of its other floors (owner's design, 2026-11-16): the outside in vertical slices, and straight down through a ruin's holes. A game rule both sides work out alike: the server sends and remembers those cells, the client (multi-level buildings PR 2 and 3) draws them. Not sight on the viewer's own floor (`dungeon::Sight`), and not mobs on other floors (later).

## Files

- `crates/shared/src/floor_sight.rs`: `other_floors`, `is_out`, `GROUND_REACH`, and the private steps (`looked_at`: columns, faces and holes seen on the viewer's floor; `show_column`, `show_face`). Tests: looking out of a window shows the ground but not the wall under it, from outside a face and its window but not the room behind, a floor set back shows its face behind the air, holes show straight down, the ground's reach, every floor full height, rooms under air seen as their roof.

## Public API

- `other_floors(storeys, viewer, at, seen) -> Vec<Sight>`: given a building's floors above ground (from the ground up, no basement), the viewer's floor index, their cell and what they see there (`Sight`, as the server works it out), what they see of each floor (their own left empty).
- `is_out(cell)`: outside: `Cell::Air` or `is_outside()` ground.
- `GROUND_REACH` (24 cells, straight line from the viewer): how far the ground floor's outside is seen from other floors. To tune.

## Depends on

- `shared::dungeon` (`Cell`, `Level`, `Sight`).

## Used by

- `server` (`sites/other_floors.rs`).
- client `dungeon_map` (`other_floors/which.rs`: what is seen of the floors below now, for drawing them lit).

## Notes

- Rules: an outside cell seen (ground, or `Air` beside an upper floor) shows its whole column on every floor, with the walls, windows and doors facing into it; a wall face seen from outside (a wall, window or door beside a seen outside cell) shows on every floor full height, and where a floor is set back, the air in front of it and its face behind; a lower floor's wall reaching out under a seen column shows too. No floor cap.
- A face only counts from its outside: the viewer must be at least a cell beyond its line (`beyond`), so the wall under the window they look out of, and the rest of that wall's line, is never seen.
- Rooms inside other floors are never seen, only straight down through `Hole`s (each floor below, until one isn't a hole). But in a seen column, a floor's cell under `Air` on the floor directly above is seen (`show_column`): its roof is, where a building narrows going up, and the client draws a roof there (multi-level buildings PR 3, owner).
- Only buildings' floors above ground (`Site::is_building`, not towns), never caves or a basement: callers pass only those.
