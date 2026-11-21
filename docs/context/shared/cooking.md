# shared: cooking

Cooking (plan: `docs/context/cooking.md`, research: `docs/context/research-cooking.md`). So far only the culinary data of every raw food: its roles in a dish, how it is used and cooked, texture, functions, methods, flavours, strength, max share and diet flags, for the rules of cooking to come.

## Files

- `crates/shared/src/cooking.rs`: the module list and re-exports. Tests in `cooking/tests.rs`.
- `cooking/ingredients.ron`: the table, a row per raw food (483), generated once from the research and edited by hand from now on; diet flags written only when true.
- `cooking/traits.rs`: `Source`, `Role`, `Usage`, `CookingNeed`, `Texture`, `Function`, `Method`, `Flavour`.
- `cooking/def.rs`: `IngredientDef` and its `check`, `MAX_STRENGTH`.
- `cooking/table.rs`: `Ingredients`, built in.

## Public API

- `IngredientDef { source, key, usage, roles, main, cooking, texture, functions, methods, flavours, strength, max_share, meat, animal_product, gluten, dairy, nut_allergen }`; `check()` (strength 1 to 5, share at most 100%, the main role among the roles, meat an animal product).
- `Source`: Herb, Fruit, Vegetable, Fungus (by their tables' keys), Meat, Milk, Egg, Honey (by `livestock`'s animal key), Part (by `parts.ron`'s key).
- `Ingredients::built_in()`, `read(text)` (checked, each source and key once), `new`, `get(source, key)`, `of_form(&ItemForm)` (herb, fruit, vegetable, fungus by key, flour as its grain, an egg by its animal, a bottle, vessel or cask by its liquid; `None` otherwise, and for a server's modded herbs), `of_liquid(&Liquid)` (milk and honey by animal), `from_source`, `iter`, `len`.

## Depends on

- `item_form` (`ItemForm` and the herb, fruit, vegetable, fungus, flour and egg forms' keys), `potion` (`Liquid`), `ron`, `serde`.

## Used by

- `farmyard` (`Usage` weighting which animal's milk or eggs a town sells). The rules of cooking next (plan PRs 4 and 5).

## Notes

- A test checks every herb, fruit, vegetable and fungus has a row, and every row names something in its table (livestock and parts included): adding a food means adding its row.
- `animal_product` is true for meat too (the research had it only for what is not meat): vegetarian is no `meat`, vegan no `animal_product`.
- `max_share` 0 means it should never be in a dish (inedible things).
