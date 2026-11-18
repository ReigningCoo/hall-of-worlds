# client: item_popup

An item's popup, wherever items are drawn (grids, clothes, bin, ground, shops, `object type="item"`, items in writing, Give Item's preview): its name in its quality's colour (a herb's as the character knows it; a bottle's is its form's, "Potion" or "Empty Bottle", with no effect lines yet: `shared::potion`; an ointment's lines are what rubbing one on does, `ointment.rs`: `brewing::skin_effect`, attributes with how long and Health or Mana "at once"), a description for kinds with one (`about-<kind>` in `item.ftl`), anything about where it is (worn notes), what a key remembers (where it was found, the door it turned in), a line per bonus from `shared::bonus` worded through `shared::lang` (`item.ftl`), the size a worn thing was made in when not the player's, what it asks of its wearer (`shared::requirement`: unmet ones and "Working at N%" as a warning), its weight, and for staff with any debug tool (`Permission::Debug`) its id with a copy button. Not what the pointer is on (the drawers say, through `widgets::PointedItem`) nor how a popup behaves (`dom`'s `popup hover`).

## Files

- `crates/client/src/item_popup/mod.rs`: `ItemPopupPlugin`, the module list. Tests (the markup is good).
- `item_popup/open.rs`: `ItemPopups` (open ones, the item rested on), `REST` (0.15 s), `OFFSET`, `FORM` (`item_popup#<item id>/…`), `forget_closed` (despawns on `Closed`), `follow_pointed` (keeps the one about the pointed item open through `dom::KeptOpen`, or opens one after `REST` in place of any not locked; none while something is held).
- `item_popup/herb.rs`: `HerbWords`, `herb_words` (a herb's name, or "Unknown herb" until the character knows it (`Sheet::herbs`); its part ("Root") in place of a description; its four effects worded ("Slightly raises Toughness", `item.ftl`'s `herb-effect`) where known, else "?", in place of bonuses), `effect_words`. Tests.
- `item_popup/about.rs`: `about` (the description, if `item.ftl` has `about-<kind in kebab case>`: Dwarven Bread's, the bag's, the cask's, flour's; a cask's adds how many litres it holds, `item-cask-holds`), `about_id`. Tests.
- `item_popup/show.rs`: `ITEM_POPUP` (`popup hover` with `name`, `about`, `notes`, `bonuses`, `size`, `needs`, `unmet` (`p.warning`), `keys`, `planet` (an `img`), `weight`, and `id_row` (`item_id` and `copy_holder`, shown only for staff with a debug tool: `common::AccountRole`'s `debug_tools`); `KeyWords`), `fill` (place and words by id), `bind_names` (the name's text in its colour, by the top of a stack).
- `item_popup/words.rs`: `bonus_lines` (worn bonuses by `item_bonuses`, food by `food_effect`: how long and what ends it; a fruit's healing and effects by `fruit::eating`, and a whole one's "Cut into N servings when eaten" (`fruit_lines`); a vegetable's raw healing and effects by `vegetable::eating`, "Heals nothing raw" or for grain "Heals nothing until milled", and its servings (`vegetable_lines`); "Poisons other kinds: −1 Health a second for 60 s" for food that poisons (`Others::Poisoned`)), worded by `common`'s `bonus_line` and `effect_lines`, which `effect_bar` shares. Tests.
- `item_popup/keys.rs`: `key_lines` (what a key remembers, through `item.ftl`'s `key-` messages: "Found in Ashford (1204, −388)", "Has unlocked a door in Ashford (1204, −388)", "…, on another planet", "in the Hall of Worlds"; and the runes of the planet it was found on when not `common::CurrentWorld`'s, shown under them as a picture, `common::rune_picture`, named in `dom::Pictures` once per code). Tests.
- `item_popup/needs.rs`: `Viewer { attributes, kind }`, `need_lines` ("Needs 110 Strength" and food's kinds, "Dwarves only", "Orcs and goblins only" (`common::kinds_words`), met and unmet against the player's attributes with effects, `Sheet::attributes_without_worn`, and their body's kind (`inventory::Carried`); "Working at 49%" when short; all met before the server has said the attributes). Tests.
- `item_popup/size.rs`: `size_line` ("Size: Large (you: Medium)", `item.ftl`'s `item-size` and `item-size-name`): kinds that come in sizes and are worn (clothes, armour, footwear, rings; not loose blades or gems), against `common::PlayerSize`; nothing when it is theirs. Tests.
- `item_popup/copy.rs`: `CopyId`, `bind_copy` (a `widgets::spawn_copy_button` into `copy_holder`, id `item_popup#<id>/copy`, `Pressable`), `copy_pressed` (`widgets::copy_to_clipboard`, as `debug_info` copies).
- `item_popup/place.rs`: `keep_on_screen` (moves one past the window's right or bottom edge back in, once laid out).

## Public API

- `ItemPopupPlugin` only.

## Depends on

- `dom` (`popup hover`, `Pictures`, `KeptOpen`, `Popup`, `Dom`, `DocumentOf`, `HeldObject`, `BuildDocuments`), `widgets` (`PointedItem`, `ItemsPointed`, `weight_words`, `UiId`, `spawn_copy_button`, `copy_to_clipboard`, `ButtonPressed`, `Pressable`), `common` (`Wording`, `PlayerSize`, `AccountRole`, `HerbTable`, `CurrentWorld`, `rune_picture`, quality colours, `kinds_words`, the bonus wording: `bonus_line`, `effect_lines`, `stat_words`, `written`), `inventory` (`Carried`: the viewer's kind), `character` (`CharacterSheet`, `WithBonuses`), `shared::bonus`, `shared::requirement`, `shared::lang`.

## Used by

- Nothing: it reads `widgets::PointedItem`, which `widgets` (`ItemName`, items in writing) and `inventory` (its cells) fill.

## Notes

- Only what the client's copy knows goes in: `Item::name()` and the bonuses worked out from the copy's kind, quality, form and weight.
- A stack is named and its bonuses given by its top; its weight is the whole stack's (`Item::total_weight`).
- Ids are by the item's id, so one item has at most one popup; pointing at an item with a locked popup only keeps that.
- Order: `follow_pointed` runs in `PostUpdate` after `widgets::ItemsPointed` and before `dom::BuildDocuments`, so a new popup is built and filled the frame it opens.
- The id row's copy button is reached by moving onto the popup, which a `popup hover` stays open for; its press goes no further (`Pressable`).
