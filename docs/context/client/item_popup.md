# client: item_popup

An item's popup, wherever items are drawn (grids, clothes, bin, ground, shops, `object type="item"`, items in writing, Give Item's preview): its name in its quality's colour, a description for kinds with one (`about-<kind>` in `item.ftl`), anything about where it is (worn notes), what a key remembers (where it was found, the door it turned in), a line per bonus from `shared::bonus` worded through `shared::lang` (`item.ftl`), what it asks of its wearer (`shared::requirement`: unmet ones and "Working at N%" as a warning), and its weight. Not what the pointer is on (the drawers say, through `widgets::PointedItem`) nor how a popup behaves (`dom`'s `popup hover`).

## Files

- `crates/client/src/item_popup/mod.rs`: `ItemPopupPlugin`, the module list. Tests (the markup is good).
- `item_popup/open.rs`: `ItemPopups` (open ones, the item rested on), `REST` (0.15 s), `OFFSET`, `FORM` (`item_popup#<item id>/…`), `forget_closed` (despawns on `Closed`), `follow_pointed` (keeps the one about the pointed item open through `dom::KeptOpen`, or opens one after `REST` in place of any not locked; none while something is held).
- `item_popup/about.rs`: `about` (the description, if `item.ftl` has `about-<kind in kebab case>`: Dwarven Bread's so far), `about_id`. Tests.
- `item_popup/show.rs`: `ITEM_POPUP` (`popup hover` with `name`, `about`, `notes`, `bonuses`, `needs`, `unmet` (`p.warning`), `keys`, `planet` (an `img`), `weight`; `KeyWords`), `fill` (place and words by id), `bind_names` (the name's text in its colour, by the top of a stack).
- `item_popup/words.rs`: `bonus_lines` (worn bonuses by `item_bonuses`, food by `food_effect`: how long and what ends it; "Poisons other kinds: −1 Health a second for 60 s" for food that poisons (`Others::Poisoned`)), worded by `common`'s `bonus_line` and `effect_lines`, which `effect_bar` shares. Tests.
- `item_popup/keys.rs`: `key_lines` (what a key remembers, through `item.ftl`'s `key-` messages: "Found in Ashford (1204, −388)", "Has unlocked a door in Ashford (1204, −388)", "…, on another planet", "in the Hall of Worlds"; and the runes of the planet it was found on when not `common::CurrentWorld`'s, shown under them as a picture, `common::rune_picture`, named in `dom::Pictures` once per code). Tests.
- `item_popup/needs.rs`: `Viewer { attributes, kind }`, `need_lines` ("Needs 110 Strength" and food's kinds, "Dwarves only", "Orcs and goblins only" (`common::kinds_words`), met and unmet against the player's attributes with effects, `Sheet::attributes_without_worn`, and their body's kind (`inventory::Carried`); "Working at 49%" when short; all met before the server has said the attributes). Tests.
- `item_popup/place.rs`: `keep_on_screen` (moves one past the window's right or bottom edge back in, once laid out).

## Public API

- `ItemPopupPlugin` only.

## Depends on

- `dom` (`popup hover`, `Pictures`, `KeptOpen`, `Popup`, `Dom`, `DocumentOf`, `HeldObject`, `BuildDocuments`), `widgets` (`PointedItem`, `ItemsPointed`, `weight_words`, `UiId`), `common` (`Wording`, `CurrentWorld`, `rune_picture`, quality colours, `kinds_words`, the bonus wording: `bonus_line`, `effect_lines`, `stat_words`, `written`), `inventory` (`Carried`: the viewer's kind), `character` (`CharacterSheet`, `WithBonuses`), `shared::bonus`, `shared::requirement`, `shared::lang`.

## Used by

- Nothing: it reads `widgets::PointedItem`, which `widgets` (`ItemName`, items in writing) and `inventory` (its cells) fill.

## Notes

- Only what the client's copy knows goes in: `Item::name()` and the bonuses worked out from the copy's kind, quality, form and weight.
- A stack is named and its bonuses given by its top; its weight is the whole stack's (`Item::total_weight`).
- Ids are by the item's id, so one item has at most one popup; pointing at an item with a locked popup only keeps that.
- Order: `follow_pointed` runs in `PostUpdate` after `widgets::ItemsPointed` and before `dom::BuildDocuments`, so a new popup is built and filled the frame it opens.
