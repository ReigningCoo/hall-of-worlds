# shared: lantern

The lantern's rules (owner, 2026-11-21; lantern task PR 4 of 6): the oil kept on the item, burning by playtime while held or tied to the belt, refilling from a bottle of oil, and how far a carrier's light reaches. The server runs the clock and tells viewers (PR 5, `server.md`); the model, light and fuel panel are the client's (PR 6). The kind is `ItemKind::Lantern` (800 g, 1 × 2, scroll shops).

## Files

- `crates/shared/src/lantern.rs`: the module list and re-exports. Tests in `lantern/tests.rs`.
- `lantern/form.rs`: `LanternForm`, `FOUNT_ML`, `SECONDS_PER_ML`, `Item::lantern` (and the private `lantern_mut`).
- `lantern/burn.rs`: `Burnt`, `LanternForm::burn`, `Inventory::burning_places`, `lantern_lit`, `burn_lanterns`.
- `lantern/fill.rs`: `fill`, `Filled`, `FillRefused`.
- `lantern/light.rs`: `LANTERN_LIGHT`, `Inventory::light_reach`.

## Public API

- `LanternForm { ml, burnt }` (`ItemForm::Lantern`): oil left in millilitres (at most `FOUNT_ML`, 200: two bottles) and seconds burnt into the next millilitre (under `SECONDS_PER_ML`, 36: 100 ml an hour). Whole numbers only. `has_oil`, `room`, `seconds_left` (a full fount 7200 s), `grams` (a gram a millilitre, added to the lantern's 800 g by `ItemForm::grams_at`). Any oil burns the same, so which is not kept.
- `Item::lantern()`: its fount if it is a lantern; a lantern without a form (from before) counts as empty, and gets one when first filled or burnt. New lanterns (`give_form_to`, so shops and Give Item) are empty.
- `LanternForm::burn(secs) -> Burnt { used, went_out }`: a millilitre each 36 s of playtime, the part second kept, nothing when dry.
- `Inventory::burning_places()`: lanterns held in a hand (`Location::Held`) or tied to the belt (`bag_slots`), oil or not; `lantern_lit()`: one of them has oil; `burn_lanterns(secs) -> Burnt`: each of them burns its own oil. What the server calls as it counts playtime (paused offline); anywhere else (a grid, a bag, the ground) uses nothing.
- `fill(lantern, bottle) -> Result<Filled { lantern, bottle, ml, spilt }, FillRefused>`: one full bottle of any `Liquid::Oil` poured in, as much as fits (`room`, at most `pour::BOTTLE_ML`); the bottle comes back empty with the same id and look, unsealed, nothing stacked on it; what didn't fit is spilt. Refused: `NotALantern`, `NotOil` (an empty bottle, another liquid, not a bottle), `Full`. Pure: the server takes one bottle off its stack, calls it and puts both back (`Inventory::replace_carried`, `give`).
- `LANTERN_LIGHT` (10), `Inventory::light_reach()`: `LANTERN_LIGHT` while `lantern_lit`, else `dungeon::CARRIED_LIGHT` (6). A lit carrier is a light source for everything that can see (PR 5 sends it to viewers).

## Depends on

- `item` (`Item`, `ItemKind`, `Inventory`, `ItemDef::belt_tool`), `item_form` (`ItemForm`), `creature` (`Location`), `potion` (`BottleForm`, `Liquid`, `Oil`), `pour` (`BOTTLE_ML`), `dungeon` (`CARRIED_LIGHT`), `serde`.

## Used by

- `item_form` (`ItemForm::Lantern`: name, weight, `give_form_to`).
- The server (PR 5, `connections/lantern.rs`): `ClientMessage::FillLantern` through `fill`, `burn_lanterns` by playtime, `light_reach` sent as `mob::EntityLook::light`. The client (PR 6): `widgets`' fuel bar (`seconds_left`), `lantern` (`burn_lanterns` counted down locally, `light_reach`, `fill` for the drop space's tint), `entities` (`EntityLook::light`).

## Notes

- Held: `ItemDef` `HELD` like a one-handed blade, so it takes a hand and a two-handed weapon clashes with it (`WearRefused::Clash`) either way round; a size either way of whom it was made for.
- Tied: `ItemDef::belt_tool` (in `item.md`, bag slots): tied to a bag slot by itself, adding no cells, one lantern on the belt at a time (`MoveRefused::HolderTied`; on its own slot it swaps), always lit there while it has oil. A held lantern must go to the grid before being tied (`tie_bag` refuses worn things, `NotForBelt`).
- Open flames (owner, 2026-11-21): candles, torches and the like (not built yet) can only be held, never tied to a belt: don't give them `belt_tool`.
- Spilling what doesn't fit (rather than refusing a part bottle) is Claude's reading of "as much as fits, the empty bottle returns": bottles are never part full.
- Sight has no darkness rule yet (`dungeon.md`), so the reach only says how far is lit; Darkvision still counts from `CARRIED_LIGHT`.
