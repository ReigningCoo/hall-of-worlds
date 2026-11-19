# shared: map_mark

Personal map marks (owner's design, 2026-11-19): a character's own places on a planet's maps, an icon in a colour on a black disc. The rules and types both sides use; not their drawing (client `minimap`, `map_marks`) nor their keeping (server `connections/map_marks.rs`, `CharacterWorldSave::marks`).

## Files

- `crates/shared/src/map_mark.rs`: everything below.

## Public API

- `MarkIcon` (10: Flag, House, Cave, Mine, Farm, Orchard, Bag, Logs, Camp, Danger; `ALL`, `key()` / `from_key` in kebab case for language files and debug tools). Serde by name, new ones at the end.
- `MarkColour` (8: Red, Orange, Yellow, Green, Cyan, Blue, Purple, White; `ALL`, `rgb()`, `key()` / `from_key`; Red the default).
- `MapMark { id, at, icon, colour, auto }`: `at` in wilderness cells (`wilderness::SCALE`, 16, a world map point), `x` within the planet's width; `auto` (serde default) for marks the game places (PR 2), which don't count.
- `MOST_HAND_MARKS` (100 per character per planet), `hand_marks`, `may_add_by_hand`.
- `middle_of_point` (a mark placed on the world map goes in the middle of its point), `point_of`, `place(at, points)`: `x` brought round the planet, `None` past the ice caps (`movement::past_the_ice`).

## Depends on

- `id`, `movement` (`past_the_ice`), `wilderness` (`SCALE`).

## Used by

- `protocol` (`MarkAsk`, `MarkNews`), the server (checks and saves), the client (`common::MapMarks`, `minimap`, `map_marks`).

## Notes

- Not in sites or the Hall (owner): marks are planet-wide places on the world map's maps (PR 2 adds the wilderness's).
- The plan: PR 1 by hand (this), PR 2 automatic white marks on piles a player starts on wilderness ground (logs icon for log piles, bag otherwise), PR 3 marks in the 3D world map.
