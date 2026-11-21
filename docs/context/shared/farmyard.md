# shared: farmyard

Milk, eggs and honey as items (cooking plan PR 3, `docs/context/cooking.md`): one of each for every kept animal giving it in `shared::livestock`'s produce (11 milks, 7 eggs, honey from honeybees). Milk and honey are liquids (`potion::Liquid::Milk`, `Liquid::Honey`, by animal key), held by the millilitre in bottles, vessels and casks as water is, poured by `shared::pour`; eggs are counted items stacking by animal. Not given by animals yet: livestock production hooks in later (owner, 2026-11-21). Not eaten or cooked yet either: cooking (plan PRs 4 on) finds them through `cooking::Ingredients::of_form`.

## Files

- `crates/shared/src/farmyard.rs`: the module list and re-exports. Tests in `farmyard/tests.rs`.
- `farmyard/animals.rs`: `givers`, `dev_animal`, `rolled_milk`, `rolled_honey`; `LivestockDef::gives`, `egg_grams`.
- `farmyard/egg.rs`: `EggForm`, `new_egg`, `Item::egg`.
- `farmyard/names.rs`: `milk_name`, `egg_name`, `honey_name`.
- `farmyard/place.rs`: `animal_for`.

## Public API

- `givers(product)`: every animal giving a `livestock::Product`, in the table's order. `LivestockDef::gives(product)`, `egg_grams()` (`Produce::grams`, 0 for one laying none).
- `EggForm { animal, grams }` (`ItemForm::Egg`): `of(def)` (`None` for an animal laying none), `name` ("Hen's Egg", "Duck Egg"), `shape` (1 × 1). `new_egg(def, count, id, made_at)`: counted, without quality. `Item::egg()`.
- `milk_name(key)` ("Cow's Milk", "Ewe's Milk", "Mare's Milk", "Yak Milk"), `egg_name`, `honey_name` ("Honey"): English, as liquids' names are; `Liquid::name` uses them.
- `dev_animal(product, key, seed)`: Give Item's animal by key, or rolled from the seed, the same on both sides. `rolled_milk(rng)`, `rolled_honey(rng)`: the liquids Give Item's bottles, vessels and casks hold for `LiquidKind::Milk` and `Honey`.
- `animal_for(product, biome, seed)`: the animal whose product a shop sells: one kept in that biome (`LivestockDef::kept`), weighted by its cooking `Usage` (Staple 8, Common 4, Occasional 2, else 1), else any giving it; `None` for anything but milk, eggs or honey.

## Depends on

- `livestock` (`Livestock`, `LivestockDef`, `Product`), `cooking` (`Ingredients`, `Source`, `Usage`, for weights), `potion` (`Liquid`), `item`, `item_form`, `climate` (`Biome`), `rng`, `id`, `serde`.

## Used by

- `potion` (`Liquid::Milk`, `Honey` and their names; `bottle_to`), `vessel` and `cask` (`vessel_to`, `cask_to`), `juice` (milk is a drink: `is_drink`), `item` (`ItemKind::Egg`: Food, counted, without quality), `item_form` (`give_form_to`, measure), `cooking` (`of_form`: eggs by animal, milk and honey in a bottle, vessel or cask by `of_liquid`), `produce` (`suit_to_place`), `shop` (herbalists' eggs and bottles of milk and honey), server `connections/give_items.rs`, client `item_art` (`egg.rs`, milk and honey in bottles), `debug_menu` (Give Item's egg dropdown), `dev_tools` (`client.give_item`'s `egg`).

## Notes

- Milk is drinkable and does nothing yet, as water; honey can't be drunk. Both weigh a gram a millilitre, as every liquid does.
- No farm shops or general stores exist yet, so herbalists sell eggs and bottles of milk and honey (as they sell yeast until there are bakers), of an animal kept near the town.
- A test checks every giver has a cooking row and every Milk, Egg and Honey row has a giver.
