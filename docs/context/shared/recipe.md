# shared: recipe

Recipes (plan: `docs/context/recipes.md`): recipe scrolls as items, and each character's recipe book of learnt and own recipes. Craft-agnostic (Cooking now, Alchemy later). A cooking recipe itself, and every real dish's, is `shared::cooking`'s `Recipe`.

## Files

- `crates/shared/src/recipe.rs`: the module list and re-exports. Tests in `recipe/tests.rs`.
- `recipe/craft.rs`: `Craft` (Cooking, Alchemy; `name`).
- `recipe/form.rs`: `RecipeForm`, `RECIPE_GRAMS`, `new_item`, `dev_recipe`, `Item::recipe`.
- `recipe/book.rs`: `RecipeBook`, `Entry`, `Own`, `Refused`, `Removed`, `MOST_OWN`.

## Public API

- `ItemKind::Recipe` (Other, 1 × 1, counted, no quality, `RECIPE_GRAMS` 20 g) with `ItemForm::Recipe(RecipeForm { craft, dish })`: stacks by craft and dish. `RecipeForm::cooking(dish)`, `name()` ("Recipe for Naan"), `is_known()` (its dish has a recipe), `shape()`. `new_item(form, count, id, made_at)`. `dev_recipe(dish, seed)`: Give Item's, the dish asked for or one rolled.
- `RecipeBook` (`Sheet::book`, saved and sent with the sheet, empty in older saves), latest first: `read(&scroll)` (learns it, `Refused::{NotAScroll, Unknown, Known}`; the caller uses the scroll up on `Ok`), `learn(form, scroll)`, `knows(form)`, `remember(Own)` / `remember_cooking(recipe, name)` (to the front, the same process replaced, own ones past `MOST_OWN` (100) dropped oldest first), `remove(&entry) -> Option<Removed { scroll }>` (a learnt one read from a scroll gives it back), `iter`, `len`, `is_empty`.
- `Entry::{Learnt { craft, dish, scroll }, Own(Own)}` (`same`: learnt by craft and dish, own by process); `Own::Cooking { recipe: cooking::Recipe, name: cooking::DishName }` (`craft`, `same`).

## Depends on

- `cooking` (`Dishes`, `dish_recipe`, `Recipe`, `DishName`), `item` (`Item`, `ItemKind`, `Shape`, `MadeAt`), `item_form`, `id`, `rng` (`Seed`), `serde`.

## Used by

- `sheet` (`Sheet::book`), `item` (`ItemKind::Recipe`'s row), `item_form` (`ItemForm::Recipe`, Give Item); the server (`connections/recipe_book.rs`: reading and removing; `cook.rs`: own recipes; `quests/prize.rs`: reward scrolls), `shop` (scroll shops, inns and taverns), `protocol` (`RecipeAsk`, `RecipeInserted`), the client (`inventory`'s `read.rs`, `item_art`'s `scroll.rs`, `item_popup`'s `recipe.rs`, `cooking`'s reading panel: `Entry`); PR 5 to come.
