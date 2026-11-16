# client: debug_info

Debug Info, for bug reports: the bug button at the top right (just left of the system menu's gear) opens a moving panel, for everyone while playing. "Describe the problem" (a `textarea`, 1000 characters at most) at the top; below it the debug string (`shared::debug_report`: `HOW1.` and the report squeezed) in a read-only `textarea`, with a copy button. Below, only for those with `DebugTool::LoadDebugString`: a box to paste someone's string, "Also set time" (usable only with World Time's permission, off by default) and **Load**, which reads it and sends one `DevGoToPlace` for the server to check and do; once there (`DevPlace(Arrived)`), the player faces the report's way and the camera takes its turn, tilt and zoom. The reporter's words then show read-only under "Reported problem" (`${item:…}` left as written), until the next Load or the panel closes. It goes to the generated place, not another server's saved state.

## Files

- `crates/client/src/debug_info/mod.rs`: `DebugInfoPlugin` (`Startup`: `spawn_button`; `Update`, in order: `note_sent`, `note_told`, `bind_button`, `show_button`, `notice_closed`, `open_panel`, `bind_panel`, `read_events`, `refresh`, `read_load_events`, `hear_answers`, `show_load`). Tests in `tests.rs`.
- `debug_info/button.rs`: `BUTTON` (`hud/debug_info`, top 8 right 48, 34 across, on `top_strip()`, `GlobalZIndex(15)`, `icon:delapouite/ladybug`), `BugButton`, `spawn_button`, `bind_button` (tip; tints the white icon `widgets::INK`, as a button shows its picture untinted), `show_button` (hidden while connecting).
- `debug_info/markup.rs`: `PANEL` (`debug_info/...`: `panel`, `problem`, `string`, `copy_holder`; Load's `load` (hidden), `paste`, `set_time`, `load_button`, `load_status`, `reported_box` (hidden), `reported`), `MOST_PROBLEM`.
- `debug_info/load_state.rs`: `Load` (what is pasted, the checkbox, the reported problem, the status, the camera waiting for arrival, `Shown`: what the panel last showed).
- `debug_info/load.rs`: `read_load_events` (paste box, checkbox, Load), `start_load` (sends the ask, notes the words and camera; or the reason as the status), `ask_for` (the string into a `DevGoToPlace`: refused without the permission, for a string `decode` refuses, or one written outside a world; the time only if asked and allowed), `hear_answers` (`DevPlace`: `Facing` and `OrbitCamera` set on arriving, a camera that isn't one left alone; words for refusals).
- `debug_info/load_show.rs`: `show_load` (Load's part shown to those allowed, the checkbox disabled without World Time, the reported problem, the status; on the panel closing the words and status go).
- `debug_info/state.rs`: `Panel` (document, root, what is typed and when, the description in the string, the report it was made from, the string), `TYPING_PAUSE` (0.6 s, as Connect's address check).
- `debug_info/panel.rs`: `open_panel` (the button's `UiEvent`; brings it to the front if open; puts back what was typed before), `bind_panel` (root; the `widgets::spawn_copy_button` into `copy_holder`, id `debug_info/copy`), `notice_closed` (close button, or leaving the game).
- `debug_info/refresh.rs`: `read_events` (typing kept with its time; the copy button takes the typing at once, remakes and copies), `refresh` (while open: typing in after the pause; remade only when the report, time aside, differs, so the box isn't rewritten every frame).
- `debug_info/gather.rs`: `Sources` (a `SystemParam` of everything read) and `report(problem)`: build, character (id, kind from `inventory::Carried`, gender from `character::CharacterSheet`, difficulty), place by view and `LevelKind` (world map point; wilderness spot, heading and poi cutout; site with its world map point and chunk, kind, ruin, level of levels, spot and heading; Hall origin, spot, heading), the world's time in whole minutes and its rate, `view3d::OrbitCamera` (turn, tilt, distance), God Mode / Truesight / Freeze Mobs, the debug tool's site if in one, and `planet` (`hall_planet`: the world's seed from its first gate's Hall place, checked against its terrain seed).
- `debug_info/known.rs`: `Known` (the character's id from `Welcome`, the wilderness `InPoi` kind, a site asked of Generate POI, Dev Area or Load (`ToServer`) becoming `dev` on the next `EnterLevel` into a site).
- `debug_info/platform.rs`: `build` (version, protocol, native or web, the OS or the browser's user agent), `utc_now` (natively `SystemTime`, in a browser `Date.now`).
- `debug_info/debug_api.rs` (feature, native): `client.debug_string` (`open`, `string`, `report`: as the panel would make it now, with what is typed), `client.load_debug_string` (`text`, `set_time`: opens the panel and loads, answering the report read).

## Public API

- `DebugInfoPlugin`.

## Depends on

- `common` (`CurrentWorld`, `CurrentDungeon`, `WorldReturn`, `MapPosition`, `SpotPosition`, `Facing`, `ServerClock`, `PlayDifficulty`, `GodMode`, `Truesight`, `FreezeMobs`, `GameView`, `Session`, `DebugTools`), `character` (`CharacterSheet`), `inventory` (`Carried`), `player` (`Player`), `view3d` (`OrbitCamera`, `ViewCamera`; Load sets the camera), `connection` (`ToServer`, `FromServer`), `dom` (`Dom`, `UiEvent`, `document`, `top_strip`), `widgets` (`spawn_copy_button`, `copy_to_clipboard`, `ButtonPressed`, `Pressable`, `RaisePanel`, `ButtonTooltip`, `UiId`, `INK`), `shared::debug_report`.

## Used by

- Nothing.

## Notes

- Never anything of the account in the report: no email address, server address or account name (`CLAUDE.md`).
- A site's map seed is the server's own (`SeenPoi` has none): the report gives the world's terrain seed and the chunk, from which the server finds the world (one it has open, or the Hall planet `planet`) and its poi. A world only another server has open can't be gone to.
- A pasted string is anyone's: the server checks the place, and the camera is only set to finite values.
- The description is kept when the panel closes, until the game ends.
