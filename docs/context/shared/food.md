# shared: food

Food items (cooking plan PR 6, `docs/context/cooking.md`): what cooking makes. A Dish (`ItemKind::Dish`) is real food in servings, eaten anywhere for Well Fed; an intermediate (`ItemKind::Intermediate`) is a row stopped early (dough, noodles, roux, stock, butter, fresh cheese, warm milk, yoghurt) that starts a later row from a source space, resting on a clock run by warmth. Cultures are strains on the Yeast item (`yeast::Strain`). The rules of cooking themselves are `shared::cooking`; nothing cooks yet (the server's Cook ability is plan PR 9), so these come from Give Item.

## Files

- `crates/shared/src/food.rs`: the module list and re-exports. Tests in `food/tests.rs`.
- `food/form.rs`: `FoodForm`, `Item::food`, `food_mut`.
- `food/rest.rs`: `Rest` (the kinds of intermediate), `Rest::of`, `YOGHURT_SETS`, `NOODLES_DRY`.
- `food/curve.rs`: `Curve` (Stable, Proves, Turns), `Curve::DOUGH`.
- `food/clock.rs`: `RestClock`, `Rested`, `Item::{rest_food, lay_food_down, settle_lying_food}`, `Inventory::rest_food`.
- `food/warmth.rs`: `warmth_percent`, `CARRIED_CELSIUS`.
- `food/culture.rs`: `Culture`, `set_percent`, `turned_quality`, `culture_from`.
- `food/make.rs`: `food_item`. `food/serving.rs`: `servings`, `SERVING_GRAMS`.
- `food/tint.rs`: `tint`. `food/eat.rs`: `eating`, `food_poisons`. `food/give.rs`: `dev_food`.

## Public API

- `FoodForm { dish, portions, history, rest }` (`ItemForm::Food`, boxed): `dish` (`cooking::Dish`, named when made), `portions` (one serving's for a Dish, the whole for an intermediate), `history` (`cooking::History`), `rest` (`Some` only for an intermediate). `name()` (a Dish by its dish; an intermediate by its kind, dough and noodles with their biggest food: "Wheat Dough"), `grams()` (water a gram a ml), `shape()` (bowls, plates, pies, cakes 2 × 2; loaves, flatbreads, strips, sausages 2 × 1; else and intermediates 1 × 1), `diet(ingredients)`, `carried()` (a `cooking::Carried` for a source space).
- `food_item(output, quality, culture, ingredients, id, made_at)`: an output as an item: `Rest::of` (milk heated alone: Warm Milk; milk worked in Bowls alone: Butter; flour and a fat in a Pan: Roux; real butter and ghee, yoghurt, paneer and other fresh cheeses, fresh egg pasta dough, bone broth; generic Dough, uncooked Noodles, Curd, a Sieve after a Cauldron: Stock), else a Dish counted in `servings` (about `SERVING_GRAMS`, 300 g, each), each serving's portions cut evenly. A culture goes in only where `Rest::culture` wants one.
- `Rest` (`ALL`, `name`, `curve`, `culture`): Dough proves (a step better from 1 h to 4 h, as made to 12 h, then a step worse each 8 h); Fresh Noodles turn into Dried Noodles after 12 h; Warm Milk sets into Yoghurt after 8 h with a yoghurt culture; the rest are stable (no spoilage for now).
- `RestClock { kind, base, rested, lying_since, culture }`: `rested` in hundredths of a second at the usual pace (seconds × `warmth_percent`); `seconds()`, `turns_in()`, `quality()`. Item quality is set from the curve as it rests; on turning, the kind changes, `base` becomes `turned_quality` (ferments only), the clock starts again.
- Clocks as yeast's: `Item::rest_food(secs, now, celsius)` (carried playtime; one picked up first counts the real time it lay), `lay_food_down(now)`, `settle_lying_food(now, celsius)`; `Inventory::rest_food(secs, now)` at `CARRIED_CELSIUS` (20). `Rested { changed, hour_passed }`, `seen()`.
- `warmth_percent(celsius)`: 0 at freezing or below, 5 a degree to 100 at 20 °C, 5 more a degree to 200 at 40 °C and above.
- Cultures: `Culture { strain, quality }`; `set_percent` (matching 100, another strain 150, wild 200), `turned_quality` (wild a step worse, another strain as made, a matching better culture a step better); `culture_from(yoghurt)`: a Yeast item of the yoghurt strain at its quality.
- `eating(form, quality, now, ingredients)`: Well Fed (`cooking::WellFed`), replacing any before. `food_poisons(form, fungi)`: `cooking::poisons` for the server.
- `tint(portions, history, ingredients)`: sRGB bytes, foods' colours by main role (milk and eggs their own) by grams, 80% when a Pan or Oven browned it.
- `dev_food(kind, quality, seed)`: Give Item's sample, planned by the rules: dishes (fried egg, scrambled eggs, beef stew), intermediates (wheat dough, fresh noodles, warm milk, butter, a roux).

## Depends on

- `cooking` (plan, `Dish`, `Portion`, `History`, `Diet`, `WellFed`, `poisons`, `food_name`), `item`, `item_form`, `yeast` (`Strain`, `new_item`, `SECONDS_PER_HOUR`), `fungus`, `bonus`, `rng`, `serde`.

## Used by

- `item` (the kinds: Food, Dish counted with quality, Intermediate one each with quality), `item_form` (`Food`, name, shape, grams, Give Item), `bonus` (a Dish is `edible` and `may_eat`, not `is_food`: not on the action bar yet).
- Server: `connections/eat_dish.rs` (eating), `connections/playtime.rs` (carried clocks), `ground/bags.rs`, `ground/tell.rs` (ground clocks at the land's warmth).
- Client: `item_art/dish.rs` (placeholder pictures by tint), `item_popup/food.rs` (lines).

## Notes

- Claude's choices for the owner to tune: the curves' hours, 300 g servings, the warmth line, cultures' paces and qualities, intermediates not eaten by themselves, the tint's colours, carried food resting at 20 °C and ground food at the land's yearly temperature (`Habitat::celsius` where the player stands; indoors and underground the same for now), a yoghurt culture not fed (a new one is taken from each batch).
- No trade exists yet: Dishes are bought and sold once shops trade (taverns later).
