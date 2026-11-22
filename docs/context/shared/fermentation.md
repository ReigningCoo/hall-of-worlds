# shared: fermentation

The fermentation rules (drinks plan PR 5, `docs/context/drinks.md`, owner's decisions 16 to 18): pure functions in whole numbers, no items, clocks or interface of their own. Ferment and vinegar (PR 6) apply them: what goes into a cask, what it becomes, how strong, sweet and good, how long it takes, how it ages, and how long it keeps once opened. Strengths are in tenths of a percent of alcohol (`58` is 5.8%); times in real seconds, a game day being a real hour (`GAME_DAY_SECS`).

## Files

- `crates/shared/src/fermentation.rs`: the module list and re-exports. Tests in `fermentation/tests.rs`.
- `fermentation/base.rs`: `Base`, `DrinkKind`, drink names.
- `fermentation/drink.rs`: `Drink` (the liquid), `Base::vinegar_name`.
- `fermentation/roll.rs`: `rolled_drink`, `rolled_vinegar`.
- `fermentation/mash.rs`: `Mash`, `MashSource`, `FRUIT_SUGAR`, `GRAIN_SUGAR`.
- `fermentation/strength.rs`: `tolerance`, `best_tolerance`, `ferment`, `Fermented`, `Sweetness`, `KCAL_PER_PERCENT`, `LEAST_STRENGTH`, `WILD_TOLERANCE`.
- `fermentation/time.rs`: `ferment_secs`, `base_secs`, `GAME_DAY_SECS`, `PITCH_ML`.
- `fermentation/quality.rs`: `drink_quality`, `mean_quality` (and the step helpers ageing uses).
- `fermentation/ageing.rs`: `aged`, `ageing_step_secs`, `ageing_cap`, and glass's `glass_aged`, `glass_step_secs`, `glass_cap`.
- `fermentation/spoil.rs`: `best_before_secs`, `vinegar_share`, `notice_due`, `wild_cider`, `KEEPS_FROM`, `JUICE_TURNS_SECS`.

## Public API

- `Base::{Fruit(key), Grain(key)}`: what a drink is made from. `DrinkKind` (Ale, Cider, Perry, Wine, FruitWine; `of`, `Base::kind`): grain is ale, apple cider, pear perry, grape wine, any other fruit a fruit wine. `Base::name`: "Ale" (barley), "Wheat Ale", "Cider", "Perry", "Wine", "Plum Wine".
- `Drink { base, strength, sweetness, quality, vinegar }` (`potion::Liquid::Drink`, drinks PR 6): `new(base, fermented, quality)`, `name`, `strength_words` ("5.8%"), `with_vinegar(ml, vinegar_ml)` (the share of vinegar in thousandths). `Base::vinegar_name`: "Cider Vinegar", "Malt Vinegar" (grain), "Wine Vinegar" (other fruit), for `Liquid::Vinegar(base)`. `Sweetness::words` ("dry", "off-dry", "medium", "sweet"). `rolled_drink(rng)` (a rolled juice fruit's drink with a Common yeast), `rolled_vinegar(rng)`: Give Item's.
- `Mash { base, source, kcal, ml }` and `kcal_per_litre()`: what ferments. `Mash::juice(def, ml)` (the juice's `juice::kcal_per_litre`), `cordial(def, ml)` (taken as half water), `fruit(def, grams, water_ml)` (`FRUIT_SUGAR` of the fruit's kcal, in the water plus the juice it gives; any fruit safe raw), `grain(def, sheaves, water_ml)` (`GRAIN_SUGAR` of the grain's kcal; grain only). None without volume. `MashSource` (Juice, Cordial, FruitInWater, Grain) bears on quality.
- `ferment(mash, tolerance) -> Option<Fermented { kind, strength, left_kcal, sweetness }>`: strength kcal a litre × 10 ÷ `KCAL_PER_PERCENT` (68), capped by the tolerance; the sugar left is `Sweetness` (Dry under 40 kcal a litre, OffDry under 100, Medium under 180, Sweet); none under `LEAST_STRENGTH` (1%). `tolerance(quality)`: Poor 7%, Common 9%, +2% a step to Legendary 17%, Artifact 20%. `best_tolerance(yeasts)`: the best yeast's, `WILD_TOLERANCE` (6%) with none.
- `ferment_secs(kind, size, ml, yeasts)`: `base_secs(kind)` (ale 24 game days, cider and perry 36, wines 90, in a Pin with one Common yeast) × size (+10% a cask size past the Pin) × the yeasts' mean quality (Poor 125%, Common 100%, −10% a step to Artifact 50%) × amount (one yeast to `PITCH_ML`, 20 L, is 100%; next to none 200%; four times or more 40%). No yeast is timed as one Poor.
- `drink_quality(yeasts, source, wood)`: the yeasts' `mean_quality` (ranks averaged, rounded down), one step down for cordial or fruit mashed whole and for a pine cask; Poor with no yeast; never above Legendary.
- `aged(made, kind, wood, secs)`: oak a step up each `ageing_step_secs(kind)` (ale 7 game days, cider and perry 14, wines 30) up to `ageing_cap` (ale, cider and perry 1, fruit wines 2, wine 3); cherry 1 step at half the pace; pine a step down at the same pace, 2 at most; chestnut and acacia nothing. Kept from Poor to Legendary. In glass (tavern furniture plan PR 2: sealed bottles, `ferment`'s glass clock): `glass_aged(bottled, kind, secs)`, a step up each `glass_step_secs` (twice oak's: wine 60 game days) up to `glass_cap` (oak's less one: wine 2, fruit wines 1, ale, cider and perry none); no wood's flavour.
- `best_before_secs(strength, vinegar)`: an opened drink's keeping time, 6 game days plus a quarter of its strength in percent squared (5%: 12 days, 12%: 42, 18%: 87), none from `KEEPS_FROM` (20%); `vinegar` (thousandths, `vinegar_share(vinegar_ml, total_ml)`) takes 4 thousandths off for each, down to a tenth. `notice_due(best_before, opened_for)`: in its last tenth.
- `wild_cider(def, ml)`: what opened juice ferments into after `JUICE_TURNS_SECS` (2 game days): wild yeast, up to 6%, always Poor (apple juice: a 5.8% cider).

## Depends on

- `fruit` (`FruitDef`, `Fruits`, `FruitSet`), `vegetable` (`VegetableDef`, `Vegetables`, `VegetableSet`), `juice` (`kcal_per_litre`, `ml_from`), `cask` (`CaskSize`), `item` (`Quality`), `material` (the woods), `serde`.

## Used by

- `ferment` (Ferment's plan, casks' clocks), `potion` (`Liquid::Drink`, `Liquid::Vinegar`, Give Item's `rolled_drink`, `rolled_vinegar`), `pour` (vinegar into a drink), client `item_popup`, `item_art` (a drink's colour).

## Notes

- First guesses for the owner to tune (Claude's): everything numeric above, and these choices: the best yeast's tolerance counts, but the mean quality sets quality and pace; cordial counted as half juice (how much water went in isn't kept); fruit mashed whole ferments 60% of its kcal (as juice gets), grain 50% (about 190 g of grain a litre for a 5% ale, 7 sheaves); fruit whole or cordial costs a quality step, a pine cask a step while fermenting and up to 2 more ageing; brewing never makes Artifacts; grain ales other than barley's are "<Grain> Ale"; opened juice waits 2 game days before turning.
- Pure rules: where the clocks run (sealed casks age, opened ones spoil, the yeast used) is `shared::ferment`'s.
