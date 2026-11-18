# shared: juice

Juice and cordial (drinks plan PR 3, `docs/context/drinks.md`): fruit pressed into juice with the oil press (Press Juice), how much juice a fruit gives, cordial poured from juice and water, and what drinking either does. The liquids are `potion::Liquid::{Juice, Cordial}` (by the fruit's key), held by casks, vessels and bottles like any other; fermenting them is later PRs'.

## Files

- `crates/shared/src/juice.rs`: the module list and re-exports. Tests in `juice/tests.rs`.
- `juice/pressing.rs`: `juicable`, `juice_fruits`, `ml_from`, `kcal_per_litre`, `PRESSING_GRAMS`, `JUICE_PRACTICE`, `press_ticks`.
- `juice/carried.rs`: `JuiceChoice`, `Inventory::{juice_target, can_press_juice, juice_choices}`, `with_juice`.
- `juice/drink.rs`: `drink_effect`, `MANA_A_SECOND`, `JUICE_SECONDS`, `CORDIAL_SECONDS`, `Liquid::is_drink`, `Item::is_drinkable_bottle`.
- `juice/mix.rs`: `mix`, `Mixed`.
- `juice/roll.rs`: `juice_fruit`.

## Public API

- `juicable(def)`: a fruit item (not a nut or a herb's fruit), safe raw, at most 120 kcal in 100 g (dates and the like are too dry). `juice_fruits()`.
- `ml_from(def, grams)`: four fifths of the fruit's water, a gram a millilitre; its water is what isn't dry matter, taken as kcal ÷ 4 a gram (an apple's 200 g: 141 ml; a kilogram of apples about 0.7 L, so a Pin takes about 28 kg). `kcal_per_litre(def)`: 60% of the fruit's kcal in its juice (apple 400, about 6% when fermented at ÷ 68).
- Press Juice: each pressing takes `press_ticks` (as a bottle of oil, 15 to 11 s by Alchemy) and up to `PRESSING_GRAMS` (2 kg) of the fruit, one at a time, stopping once its juice fills the target; `JUICE_PRACTICE` Alchemy each. `Inventory::juice_target(fruit)`: the carried cask or vessel (alone in its place) taking `Juice(fruit)`, one already holding it first, own grid first, fullest first. `can_press_juice(fruit)`, `juice_choices()` (`JuiceChoice { fruit, stack, count, ml, room }`, for the client's requester). `with_juice(container, juice, ml)`: the cask or vessel with more of it, opened.
- `drink_effect(liquid, ml, now)`: juice restores `MANA_A_SECOND` (1 Mana a second) for `JUICE_SECONDS` (30) a serving of 250 ml, cordial for `CORDIAL_SECONDS` (20), by the share of a serving drunk (a bottle's 100 ml of juice: 12 s); `Source::Juice(tag)` or `Cordial(tag)`, the latest drink's replacing any earlier, nothing ending it early. Water: none.
- `Liquid::is_drink` (water, juice, cordial: what vessels are drunk from), `Item::is_drinkable_bottle` (a bottle of one: drunk whole and given back empty, `bonus::usable_on_self`).
- `mix(poured, held) -> Option<Mixed>`: juice and water other than salt water make `Cordial` of the juice's fruit, either way round (`WaterIntoJuice`, `JuiceIntoWater`); `pour` keeps it to no more water than juice.
- `juice_fruit(rng)`: Give Item's juice or cordial fruit.

## Depends on

- `fruit` (`FruitDef`, `Fruits`, `FruitSet`, `FruitTag`), `potion` (`Liquid`, `Water`, `Pressing`, `press_ticks`, `PRESS_PRACTICE`), `pour` (`Held`), `vessel` (`SERVING_ML`), `bonus` (`Effect`, `Source`), `item`, `item_form`, `vitals`, `energy`, `skill`, `rng`, `id`.

## Used by

- `pour` (`mix`), `bonus` (`usable_on_self`), `vessel` (`drinkable`), `cask`, `vessel`, `potion` (Give Item's `juice_fruit`), `ability` (`PressJuice`), server `connections/press_juice.rs`, `drink_vessel.rs`, client `abilities/press_juice.rs`, `item_art` (a juice's colour is its fruit's ripe colour, cordial paler), `effect_bar` (names and icon).

## Notes

- Opened juice should ferment wild into a Poor cider (drinks decision 14): left for drinks PR 6, which brings the clocks, best-before and fermenting.
- Juice is never sealed by pressing; a cask pressed into is opened.
- Pressing loses any juice beyond the target's room (the last fruit taken may give more than fits).
