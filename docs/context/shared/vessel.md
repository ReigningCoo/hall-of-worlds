# shared: vessel

Vessels (drinks plan PR 2, `docs/context/drinks.md`): one item kind, `ItemKind::Vessel`, not counted, whose form says which vessel it is and the one liquid in it by the millilitre (`shared::potion`'s liquids, never a potion). Filled and emptied by `shared::pour`; water drunk from one does nothing, juice and cordial restore Mana (`shared::juice`). Pictures are the client's `item_art` (`vessel.rs`).

## Files

- `crates/shared/src/vessel.rs`: the module list and re-exports. Tests in `vessel/tests.rs`.
- `vessel/kind.rs`: `VesselKind`.
- `vessel/form.rs`: `VesselForm`, `Item::vessel`.
- `vessel/drink.rs`: `SERVING_ML`, `VesselForm::drinkable`, `drunk`, `Item::is_drinkable_vessel`.
- `vessel/roll.rs`: `vessel_to`, `vessel_for_shop`.

## Public API

- `VesselKind` (Tankard 500 ml, Cup 250, Goblet 200, DrinkingHorn 300, WineBottle 750, Jug 1500, Flask 500, Waterskin 2000; `ALL` smallest first): `name`, `most_ml`, `empty_grams` (200 g to 1.1 kg, first guesses), `shape` (1 × 1, the horn and wine bottle 1 × 2, jug and waterskin 2 × 2), `seals` (only the wine bottle).
- `VesselForm { vessel, liquid, ml, sealed }` (`ItemForm::Vessel`): `new` (empty), `holding(liquid, ml)` (clamped, unsealed; none or 0 is empty), `full_of`, `sealed()` (if it seals and holds something), `is_empty`, `room`, `name` ("Tankard", "Tankard of Water"), `grams` (empty plus a gram a millilitre).
- Drinking: `SERVING_ML` (250); `drinkable` (it holds a water, juice or cordial: `Liquid::is_drink`), `drunk()` (the form after one serving or what is left, and the millilitres; nothing for oil), `Item::is_drinkable_vessel` (which `bonus::usable_on_self` and `may_use_on_self` accept, so letting go of one on the clothes drinks).
- `vessel_to(rng, request)`: Give Item's and shops' vessel: which (`RollRequest::vessel`, `Field::Vessel`) rolled unless asked, empty or full of an asked water or oil; a potion is refused (`Field::Liquid`). `vessel_for_shop(shop)`: carpenters Tankard and Cup, leatherworkers Flask, Waterskin and Drinking Horn, jewellers Goblet, herbalists Wine Bottle and Jug (`shop::stock_request` asks for one).

## Depends on

- `potion` (`Liquid`, `LiquidKind`), `item` (`Item`, `Shape`), `item_form` (`ItemForm`, `RollRequest`, `Refused`, `Field`), `site` (`BuildingKind`), `rng`, `serde`.

## Used by

- `item` (`ItemKind::Vessel`, Other; its form's cells and grams), `item_form` (`give_form_to`, `fields`, `choices`), `pour`, `bonus` (drinking), `shop` (`sells`, `stock_request`), server `connections/drink_vessel.rs`, client `item_art` (`Look::Vessel`), `item_popup` (what it holds), `inventory` (drinking shown at once), `debug_menu` (Give Item's vessel dropdown).

## Notes

- Not counted, so never stacks unless two look the same (both empty). Vessels have no material to choose: each kind's look is fixed.
- Only a wine bottle seals, when filled from a cask; any pour or drink from it unseals it.
