# shared: cooking

Cooking (plan: `docs/context/cooking.md`, research: `docs/context/research-cooking.md`). The culinary data of every raw food: its roles in a dish, how it is used and cooked, texture, functions, methods, flavours, strength, max share and diet flags. And a process's rules (plan PR 4): the grid of rows and tool steps (a tall step joining rows), tools and their sub-skills, chance, balance, least quality with an intermediate's history, heat and the Rare cap away from a hearth; pure functions on whole numbers, so the client's preview matches the server. Not yet: naming dishes, Well Fed, diet labels (PR 5), items (PR 6).

## Files

- `crates/shared/src/cooking.rs`: the module list and re-exports. Tests in `cooking/tests.rs`.
- `cooking/ingredients.ron`: the table, a row per raw food (483), generated once from the research and edited by hand from now on; diet flags written only when true.
- `cooking/traits.rs`: `Source`, `Role`, `Usage`, `CookingNeed`, `Texture`, `Function`, `Method`, `Flavour`.
- `cooking/def.rs`: `IngredientDef` and its `check`, `MAX_STRENGTH`.
- `cooking/table.rs`: `Ingredients`, built in.
- `cooking/tool.rs`: `Tool` (Knife, Pan, Cauldron, Bowl, Oven, Sieve, Mortar; `ALL`, `name`, `sub_skill`, `needs_heat`, `needs_liquid`, `of_item`, `index`), `COOKING` and the sub-skills' paths (`CUTTING` ... `POUNDING`).
- `cooking/skills.rs`: `CookingSkills { cooking, sub }` (`of`, `cooking_percent`, `sub_root`).
- `cooking/process.rs`: `Process`, `Step`, `Portion`, `Carried`, `History`, `Heat`, `Refused`, `MOST_ROWS`, `MOST_COLUMNS`.
- `cooking/grid.rs`: crate-private `flow`: the grid checked, each step's inputs, the outputs. Tests in `cooking/grid_tests.rs`.
- `cooking/chance.rs`: `chance`.
- `cooking/balance.rs`: `balance`, `is_seasoning` and its constants.
- `cooking/quality.rs`: `ingredient_quality`, `skill_steps`, `least_quality`, `capped`, `AWAY_CAP`.
- `cooking/plan.rs`: `plan`, `Planned`, `Output`. Tests (with the rest of the rules) in `cooking/rules_tests.rs`.
- `cooking/outcome.rs`: `outcome`, `Outcome`.

## Public API

- `IngredientDef { source, key, usage, roles, main, cooking, texture, functions, methods, flavours, strength, max_share, meat, animal_product, gluten, dairy, nut_allergen }`; `check()` (strength 1 to 5, share at most 100%, the main role among the roles, meat an animal product).
- `Source`: Herb, Fruit, Vegetable, Fungus (by their tables' keys), Meat, Milk, Egg, Honey (by `livestock`'s animal key), Part (by `parts.ron`'s key).
- `Ingredients::built_in()`, `read(text)` (checked, each source and key once), `new`, `get(source, key)`, `of_form(&ItemForm)` (herb, fruit, vegetable, fungus by key, flour as its grain, an egg by its animal, a bottle, vessel or cask by its liquid; `None` otherwise, and for a server's modded herbs), `of_liquid(&Liquid)` (milk and honey by animal), `from_source`, `iter`, `len`.

- `plan(process, ingredients, skills) -> Result<Planned { chance, outputs, practised, heat }, Refused>`. `Process { rows (1 to MOST_ROWS, 2), sources: Vec<Option<Carried>> by row, steps, heat }`; `Step { tool, column (below MOST_COLUMNS, 12), row (its top), height, portions }` (`Step::at` one row; `is_join` when taller; `halves` 2, or 3 for a join). Each step takes, from every row it covers, the latest step before it in that row, else the row's source; a join needs something in every row (`JoinMissing`); an output goes on to at most one step (`Split`); every source used (`UnusedSource`); something in every step (`NothingIn`); no two steps on one place (`Overlap`). Steps nothing takes are the outputs (`Output { step, portions, history, balance, quality }`, by column then row): a process can give several (rows left apart, an intermediate stopped early). Heat steps (Pan, Cauldron, Oven) refused with `Heat::Cold` (`NoHeat`); a Cauldron or Sieve needs water or milk in or before it (`NoLiquid`); foods with no culinary data refused (`UnknownIngredient`, `UnknownInSource`).
- `Portion::Food { source, key, grams, quality }` (none counts as Common) or `Water { ml }` (any source; no role or quality). `Carried { portions, history }`: an intermediate in a source space; `History { halves, roots }` (its steps in halves, a join 3; each tool's sub-skill root once, the lower kept on `merge` / `add_root`).
- `chance(cooking%, halves)`: Alchemy's (100% + Cooking%, halved per step over `alchemy::steps_allowed`), the part over counted in halves and rounded up to whole halvings; this session's steps only.
- `balance(portions, ingredients)`: roles counted (main role full, others half; a role from 10% of the weight, seasonings Herb, Spice and Aromatic from 1%), at most `ROLES_COUNTED` (4), less ⌈points past max share ÷ `EXCESS_STEP` (20)⌉ per food (at least 1 for an inedible one, max share 0), less 1 per medicinal herb; water left out.
- `least_quality(ingredients, balance, history, chance, heat)`: `ingredient_quality` (mean rank by grams, rounded down) moved by balance − `BALANCED_ROLES` (2), plus `skill_steps` (halves × Σroots ÷ (tools × 20), Alchemy's steps × mean ∛ ÷ 10) only when certain; Poor to Legendary, then `capped` at `AWAY_CAP` (Rare) unless `Heat::Hearth`. `outcome(planned, chance_roll, luck_roll) -> Outcome::{Failed, Made { qualities }}`: Alchemy's luck, still capped.
- `Tool::of_item`: knives and daggers are a Knife, the Cauldron and Mortar & pestle themselves; pans, bowls, ovens and sieves have no items until hearths (PR 8).

## Depends on

- `item_form` (`ItemForm` and the herb, fruit, vegetable, fungus, flour and egg forms' keys), `potion` (`Liquid`), `ron`, `serde`.
- `alchemy` (`CERTAIN`, `steps_allowed`, `moved`, `luck_steps`, `succeeds`), `herb` (`skill_value`, `skill_percent`, `cube_root`), `item` (`ItemKind`, `Quality`), `skill` (`Skills`).

## Used by

- `farmyard` (`Usage` weighting which animal's milk or eggs a town sells). The rules of cooking next (plan PRs 4 and 5).

## Notes

- A test checks every herb, fruit, vegetable and fungus has a row, and every row names something in its table (livestock and parts included): adding a food means adding its row.
- `animal_product` is true for meat too (the research had it only for what is not meat): vegetarian is no `meat`, vegan no `animal_product`.
- `max_share` 0 means it should never be in a dish (inedible things).
- Claude's choices in the rules (PR 4, for the owner to tune): the Cooking sub-skills are Cutting, Frying, Boiling, Mixing, Baking, Straining and Pounding (the Mortar's, apart from Alchemy's Grinding); the Rare cap holds for everything worked away from a hearth, cold steps too; history keeps a tool's lower root when two cooks used it, so an intermediate never lifts a dish past its weakest hand; an intermediate's portions go into the dish and its quality is worked out again from them (not added), so its steps count once; balance is a first guess until PR 5 gives each dish form the roles it wants: a single food is far past its max share (an egg alone loses 2).
