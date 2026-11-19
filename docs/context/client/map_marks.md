# client: map_marks

Personal map marks (`shared::map_mark`): keeps `common::MapMarks` to what the server says, and the full map's popups for placing a mark and for one of the player's own. Drawing the marks on the maps is `minimap`'s.

## Files

- `crates/client/src/map_marks/mod.rs`: `MapMarksPlugin` (`Update`, chained: `read_server`, `recolour_icons`, `press_popup`, `open_on_click`, `show_popup`); `MarkPopup` re-exported.
- `read.rs`: `read_server`: cleared on `Welcome`, replaced by `MarkNews::All`, then each `Added` / `Removed`.
- `pictures.rs`: `MarkPictures`; `make_pictures` (Startup: `dom::Pictures` named `mark.icon.<key>` and `mark.colour.<key>`, 32 across, from `minimap::mark_pixels` and `swatch_pixels`); `recolour_icons` repaints the icons in place when `Preferences::mark_colour` changes.
- `popup.rs`: the markup (`place_mark`: a row of 10 icon buttons `icon.<key>`, `full`, a row of 8 colour buttons `colour.<key>`; `mark`: `name`, `travel`, `remove`), both `popup depth="0"` so they stand over the full map's layer; `MarkPopup` (`Closed`, `Place { cell, at }`, `Mark { id, at }`); `show_popup` spawns and despawns the documents, places them, names the mark, marks the colour chosen `selected`, and swaps the icons for `full`'s words at `MOST_HAND_MARKS`.
- `press.rs`: `open_on_click` (from `minimap::FullMapClick`: a mark within its reach opens `Mark`; known land with no poi opens `Place` at the middle of the point, or in the wilderness (`FullMapClick::wild`) at the cell clicked; anything else closes), `JustClosed` (a press elsewhere that closed a popup opens no other), `press_popup` (a colour is remembered in `Preferences`, an icon sends `MarkAsk::Add`, Remove `MarkAsk::Remove`, Travel Here `player::TravelTo` the mark's point, or in the wilderness its cell the near way round from the player (`travel_target`); a mark gone or the full map closed closes its popup).
- `words.rs`: `too_many` through `map-mark.ftl`.
- `debug_api.rs` (feature only): `client.marks`, `client.marks.add {x, y, icon?, colour?}` (a world map point), `client.marks.remove {id}`.

## Public API

- `MapMarksPlugin`, `MarkPopup` (resource).

## Depends on

- `common` (`MapMarks`, `mark_name`, `point_at`, `in_cells`, `near_way_round`, `Preferences`, `Wording`, `CurrentWorld`, `CurrentDungeon`, `MapPosition`), `minimap` (`FullMap`, `FullMapClick`, `mark_pixels`, `swatch_pixels`), `dom`, `connection`, `player` (`TravelTo`).

## Notes

- Nothing is predicted: a mark shows once the server says `Added`, and goes on `Removed`. The server refuses quietly (over the limit, past the ice).
- Button labels and tooltips are in the markup, as elsewhere; the mark's name and the limit's words go through `map-mark.ftl`.
