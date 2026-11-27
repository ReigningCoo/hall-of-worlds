# shared: value

What every item is worth (trading plan PR 1, `docs/context/trading.md`): one deterministic rule both sides work out from the item alone, in whole numbers. Not prices: what shops pay and ask (50% and 150%, standing, the Trading skill) comes later, on top of this.

## Files

- `crates/shared/src/value.rs`: the module list and re-exports; `FINE_PER_COPPER` (1000), `LABOUR_PER_MINUTE` (500 fine, half a copper), `BASE_MINUTES` (60), `quality_times`, `copper_shown`. Tests in `value/tests.rs`.
- `value/item.rs`: `each`, `total`; a made item's materials and labour, its own grams (a bottle's glass, a cask's, vessel's or lantern's without what is poured in), what is poured in it, its main material (armour by `bonus::armour_material`, a blade's blade metal, a ring's or necklace's metal, a gem's, footwear's upper, underclothes' and clothes' cloth, a cask's wood) or its category's usual one (weapons steel, armour and bags leather, clothes linen, jewellery silver, other iron, valuables and materials wood; food none, at `FOOD_PER_KG`).
- `value/raw.rs`: raw goods by grams and abundance, no labour: herbs (a flat rate, their quality being their rarity), fruit, vegetables, pulses, grains and fungi by their table's `Abundance` (built-in tables; an unknown key at a plain rate), eggs, flour, logs and kindling (their wood's price), ash.
- `value/price.rs`: `per_kg(material)`: metals each their own (copper 6 to gold 12,000 copper a kg), gems by tier (20 to 2,000 copper a gram), the rest a category's plainest × 1, 3, 10, 40 by tier.
- `value/liquid.rs`: `per_litre(liquid)`: waters, oils, juice, cordial, drinks, vinegar, alcohol, milk, honey, soy sauce; a potion by its number of effects.
- `value/coins.rs`: `Coins { gold, silver, copper }`, `Coins::from_copper`.

## Public API

- `each(item)`: one of the item (a stack's top), in fine units (thousandths of a copper), with what is in it, set in it and poured in it. `total(item)`: a whole stack.
- `copper_shown(fine)`: rounded **up** to whole copper. Only what is shown or paid is rounded (a stack's total, one "each", later a trade's total); values are kept fine.
- `quality_times(quality)`: Poor 1, Common 2, Uncommon 4, Rare 8, Epic 16, Legendary 32, Artifact 64.
- `Coins::from_copper(copper)`: the fewest coins, gold first, for showing.
- `per_kg(material)`, `per_litre(liquid)`: the price tables, for crafting and shops later.

## Depends on

- `shared::item`, `item_form`, `material`, `bonus` (`armour_material`), `potion`, `pour` (`BOTTLE_ML`), `fruit`, `vegetable`, `fungus`.

## Used by

- client `item_popup` (the value at a popup's bottom right).

## Notes

- The rule (owner, 2026-11-27): value = (materials + labour) × quality. Materials are the item's own weight × its material's price a kg; labour is the time to make it at Common with no bonuses (`BASE_MINUTES` × its kind's `make_percent`), paid in full however skilled the maker. Raw goods are priced by weight and abundance instead.
- Anchored so a Common apple (200 g, Uncommon fruit) is 1 copper, a Poor one ½. Every other figure is Claude's first guess, to balance.
- No quality counts as Common. Coins are worth what they say, never multiplied. Quest items are worth nothing (no shop takes them).
- Containers add their contents' own values (each at its own quality). Liquids add at Common, except a bottle's potion, which takes the bottle's quality (a bottle's quality is its potion's).
- Set pieces (`shared::item_set`) are valued as any item of their kind and quality; nothing about sets changes value yet, and nothing stops them being traded.
- A server's own herbs are valued as any herb (herbs go by weight and quality), so a client without its mods agrees.
- Items made before forms existed weigh their kind's default, and are valued by it.
