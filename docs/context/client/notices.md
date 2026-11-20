# client: notices

Short notices that go by themselves (a snackbar), at the top middle of the window, under the bar of timed effects (`effect_bar`). For now "Found: <kind>" when the server says the player has found a poi in the wilderness, "Locked" when a door the player tried is locked (`DoorRefused` with `locked`), and "Picked 3 Roseroot" after a harvest (`PickNews::Picked`, named from `common::HerbTable`; `PickedFruit`, from `shared::fruit::Fruits`; `PickedVegetable`, from `shared::vegetable::Vegetables`; `PickedFungus`, from `shared::fungus::Fungi`), and what a run of Alchemy made, "Made 2 Rare potions", "Made Rare Paste", "The process failed" or "The mix came to nothing", with ": 3 more to go" while runs are left (`Landed::Alchemy`), and each Press Juice pressing, "Pressed 690 ml of juice" (`Landed::Juiced`), a cask set fermenting, "Fermenting: ready in 36 days" (`Landed::Fermented`), and "<cask> turns to vinegar soon" (`ServerMessage::VinegarSoon`, once, in an opened drink's last tenth). Not for anything that needs an answer (that is a dialog).

## Files

- `crates/client/src/notices/mod.rs`: `NoticesPlugin`; `NOTICES` (the notice as markup: `notices/row`, a row across the window 72 down, below `effect_bar`, `inert` and hidden, holding a parchment `panel` with `notices/words`), `spawn_notice` (Startup: the document, on `dom::top_strip()`), `show_notices` (Update, through `Dom`: `ServerMessage::PoiFound` or a locked `DoorRefused` sets the words and shows the row for `SHOWN_FOR` (4 s), the newest in place of any showing), `found` (what the notice says, the kind in words), `picked` (a harvest's words).

## Public API

- `NoticesPlugin`.

## Depends on

- `common` (`words`, `HerbTable`), `connection` (`FromServer`), `dom` (`Template`, `Dom`, `document`), `shared::protocol`.

## Used by

- Nothing.

## Notes

- The notice takes no clicks (its row is `inert`, and the document's own node isn't an element), so it never gets in the way of the map under it.
- A document spawned on its own (`document`) is laid out by the node it is given: here `dom::top_strip()`, as wide as the window and stacked, so the row placed in it is as wide as the window. Left as a row, the form in it has no width and the notice sits off the left edge.
- The server sends `PoiFound` once per poi, the first time the character finds it, and never for towns, which everyone knows of.
