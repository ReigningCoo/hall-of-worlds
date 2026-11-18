# shared: produce

Fruit, nuts, vegetables, pulses and grains suiting a place, for what town shops sell and what mobs carry (owner, 2026-11-18: both gave only apples). Choosing only: the tables and their rules are `shared::fruit` and `shared::vegetable`. Only the server calls it, as item ids and places' seeds are its own.

## Files

- `crates/shared/src/produce.rs`: everything below. Tests in `produce/tests.rs` (every biome at its hottest and coldest has both, both ways; foraged fruit grows wild there; grown produce suits the place; not only apples; deserts and tundra share no fruit; the same id gives the same).

## Public API

- `Source::{Grown, Foraged}`: shops sell what is grown near by; mobs carry what they picked up.
- `grown_fruits(fruits, habitat)`: fruit items whose biomes and warmth suit the place, cultivated ones too, else `fruit::orchard_fruits` (by warmth alone).
- `fruit_for(fruits, habitat, source, seed)`: grown ones by `orchard_weight`; foraged ones from `fruit::wild_fruits` by `wild_weight`, else grown ones.
- `vegetable_for(vegetables, habitat, source, seed)`: grown ones from `vegetable::field_crops` (the farms' crops there); foraged ones from what grows wild in the biome (`wild_crops` and `wild_only_vegetables`), else those crops; both by `choose_wild`'s weights (the commoner more often).
- `flour_grain_for(vegetables, habitat, seed)`: the grain a miller there mills: one of the farms' crops there (`field_crops`) that is a grain, by `choose_wild`'s weights, else any grain.
- `suit_to_place(item, habitat, source)`: a Fruit, Vegetable or Flour item made one of those, chosen from its id's seed (its look too), its count kept; anything else untouched.

## Depends on

- `fruit`, `vegetable`, `flour` (`FlourForm`), `mob` (`Habitat`), `item`, `item_form` (`ItemForm::Fruit`, `Vegetable`), `rng`; tests use `herb::climate_tests::extremes`.

## Used by

- server `sites/trade.rs` (new stock, `Grown`, at the town's poi), `entities/gear.rs` (`dressed`, `Foraged`, on the land each caller gives: the site's poi, the wilderness block, the quester's world map point for an escortee).

## Notes

- Which kinds are sold and carried is `shop::sells` (herbalists: fruit and vegetables; potion shops: fruit; millers: flour) and `mob`'s `gear_of` (beggars, kobolds, goblins). Grains and dry pulses are sold too, though they heal nothing raw.
- Not part of any generator's version: a shop's stock and a mob's gear are made once and saved, so changing the rules changes only what is made after.
