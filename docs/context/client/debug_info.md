# client: debug_info

Debug Info, for bug reports: the bug button at the top right (just left of the system menu's gear) opens a moving panel, for everyone while playing. "Describe the problem" (a `textarea`, 1000 characters at most) at the top; below it the debug string (`shared::debug_report`: `HOW1.` and the report squeezed) in a read-only `textarea`, with a copy button. Staff reading a string back and going there is PR 3 of the plan (a permission, a `DevGoToPlace` intent the server checks), not here.

## Files

- `crates/client/src/debug_info/mod.rs`: `DebugInfoPlugin` (`Startup`: `spawn_button`; `Update`, in order: `note_sent`, `note_told`, `bind_button`, `show_button`, `notice_closed`, `open_panel`, `bind_panel`, `read_events`, `refresh`). Tests in `tests.rs`.
- `debug_info/button.rs`: `BUTTON` (`hud/debug_info`, top 8 right 48, 34 across, on `top_strip()`, `GlobalZIndex(15)`, `icon:delapouite/ladybug`), `BugButton`, `spawn_button`, `bind_button` (tip; tints the white icon `widgets::INK`, as a button shows its picture untinted), `show_button` (hidden while connecting).
- `debug_info/markup.rs`: `PANEL` (`debug_info/...`: `panel`, `problem`, `string`, `copy_holder`), `MOST_PROBLEM`.
- `debug_info/state.rs`: `Panel` (document, root, what is typed and when, the description in the string, the report it was made from, the string), `TYPING_PAUSE` (0.6 s, as Connect's address check).
- `debug_info/panel.rs`: `open_panel` (the button's `UiEvent`; brings it to the front if open; puts back what was typed before), `bind_panel` (root; the `widgets::spawn_copy_button` into `copy_holder`, id `debug_info/copy`), `notice_closed` (close button, or leaving the game).
- `debug_info/refresh.rs`: `read_events` (typing kept with its time; the copy button takes the typing at once, remakes and copies), `refresh` (while open: typing in after the pause; remade only when the report, time aside, differs, so the box isn't rewritten every frame).
- `debug_info/gather.rs`: `Sources` (a `SystemParam` of everything read) and `report(problem)`: build, character (id, kind from `inventory::Carried`, gender from `character::CharacterSheet`, difficulty), place by view and `LevelKind` (world map point; wilderness spot, heading and poi cutout; site with its world map point and chunk, kind, ruin, level of levels, spot and heading; Hall origin, spot, heading), the world's time in whole minutes and its rate, `view3d::OrbitCamera` (turn, tilt, distance), God Mode / Truesight / Freeze Mobs, the debug tool's site if in one.
- `debug_info/known.rs`: `Known` (the character's id from `Welcome`, the wilderness `InPoi` kind, a site asked of Generate POI or Dev Area (`ToServer`) becoming `dev` on the next `EnterLevel` into a site).
- `debug_info/platform.rs`: `build` (version, protocol, native or web, the OS or the browser's user agent), `utc_now` (natively `SystemTime`, in a browser `Date.now`).
- `debug_info/debug_api.rs` (feature, native): `client.debug_string` (`open`, `string`, `report`: as the panel would make it now, with what is typed).

## Public API

- `DebugInfoPlugin`.

## Depends on

- `common` (`CurrentWorld`, `CurrentDungeon`, `WorldReturn`, `MapPosition`, `SpotPosition`, `Facing`, `ServerClock`, `PlayDifficulty`, `GodMode`, `Truesight`, `FreezeMobs`, `GameView`, `Session`), `character` (`CharacterSheet`), `inventory` (`Carried`), `player` (`Player`), `view3d` (`OrbitCamera`, `ViewCamera`), `connection` (`ToServer`, `FromServer`), `dom` (`Dom`, `UiEvent`, `document`, `top_strip`), `widgets` (`spawn_copy_button`, `copy_to_clipboard`, `ButtonPressed`, `Pressable`, `RaisePanel`, `ButtonTooltip`, `UiId`, `INK`), `shared::debug_report`.

## Used by

- Nothing.

## Notes

- Never anything of the account in the report: no email address, server address or account name (`CLAUDE.md`).
- A site's map seed is the server's own (`SeenPoi` has none): the report gives the world seed and the chunk, from which the server can find it.
- The description is kept when the panel closes, until the game ends.
