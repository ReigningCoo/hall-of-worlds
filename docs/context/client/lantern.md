# client: lantern

The lantern on screen (lantern task PR 6; rules in `shared::lantern`, the server's side in `server.md`): the player's own light while one burns, its oil counted down between the server's word, and the fuel panel. Not its model (`item_art`'s `tool/lantern.rs`), its fuel bar (`widgets`' `item/fuel.rs`), others' light (`entities`) or the double click that opens the panel (`inventory`).

## Files

- `crates/client/src/lantern/mod.rs`: `LanternPlugin`, `AT` (where the panel opens: right of the Character panel). Tests in `tests.rs`.
- `burn.rs`: `Activity` (when the player last sent something other than a ping or an item look-up, by the real clock), `note_activity`, `burn_locally` (while a lantern held or tied has oil and the player did something in the last 5 minutes, as the server counts playtime: `Inventory::burn_lanterns` on `inventory::Carried` once 36 s, a millilitre, has passed).
- `light.rs`: `follow_lantern` (the player's `common::LightSource` from `Sheet::light_reach`, the lantern's `Inventory::light_reach` or a spell's Light, none under Darkness (magic plan PR 13): 10 cells in `common::LANTERN_COLOUR` while lit, else 0: no light).
- `markup.rs`: `LANTERN` (`lantern/panel`, the slots `lantern/lamp` and `lantern/fill`, the words `lantern/oil`, `lantern/left`, `lantern/fill_words`).
- `panel.rs`: `LanternPanel` (one open, for one lantern), `LampSlot`, `FillSlot`, `open_panel` (on `common::OpenLantern`), `bind_panel`, `close_panel` (its button, or the lantern no longer carried), `close_on_escape`, `show_panel` (again whenever `Carried` changes: the lantern, a faint bottle of oil in the drop space).
- `fill.rs`: `drop_oil` (`PreUpdate`, after `dom::HoldObjects`: green over the drop space when `shared::lantern::fill` would take what is held, red if not; on `dom::Dropped` there, `ClientMessage::FillLantern` with the held stack's id; the bottle goes back where it was, as on the Alchemy panel, until `Inventory` comes).
- `words.rs`: the panel's words (oil, "Burns for 1 h 12 min more…", the drop space's).

## Public API

- `LanternPlugin`. Others reach it through `common::OpenLantern`.

## Depends on

- `common` (`OpenLantern`, `LightSource`, `LANTERN_COLOUR`, `PointerSystems`), `inventory` (`Carried`), `player` (`Player`, `PlayerSystems`), `connection` (`ToServer`), `dom` (`Dom`, `SlotShown`, `SlotThing`, `Dropped`, `HeldObject`, `HoldObjects`, `DomInput`, `UiEvent`, `document`, `top_strip`), `widgets` (`UiId`), `shared::lantern`, `shared::potion`.

## Used by

- `inventory` writes `common::OpenLantern` (a double click, or two taps, on a carried lantern anywhere: grid, hand or belt).

## Notes

- The local count only moves the fount a millilitre at a time (once each 36 s), so `Carried` changes rarely; the server sends the fount again every 10 ml and when a lantern goes out, which puts any drift right.
- The panel takes one bottle a drop (the top of a stack); a fount holds two (owner). What doesn't fit is spilt, as the drop space's words say.
- Lanterns aren't drawn in creatures' hands or on belts: no held thing is on the models yet (blades neither).
