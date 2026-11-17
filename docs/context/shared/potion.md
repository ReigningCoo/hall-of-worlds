# shared: potion

Bottles (owner, 2026-11-17): one item kind, `ItemKind::Bottle`, empty or holding a liquid (a water, an oil, or a potion's effects), with a seed for its look. Not drinking or brewing yet (plan: `docs/context/brewing.md`); pictures are the client's `item_art`. Herbs' rule for potions: `docs/context/research-herbs.md`.

## Files

- `crates/shared/src/potion.rs`: the module list and re-exports. Tests in `potion/tests.rs` and `potion/liquid_tests.rs`.
- `potion/contents.rs`: `Contents`, `Dose`.
- `potion/liquid.rs`: `Liquid`, `LiquidKind`, `Water`, `Oil`.
- `potion/form.rs`: `BottleForm` (and how it is read from saves and messages), `EMPTY_GRAMS`, `POTION_GRAMS`, `Item::bottle`.
- `potion/roll.rs`: `roll_contents`, `bottle_to`, `MOST_EFFECTS`.

## Public API

- `Dose { stat, step }`: a `herb::Stat` (nine attributes, Health, Mana) moved up (helpful) or down (harmful) by a whole-number step.
- `Contents`: doses in one normal order, so equal effects compare equal: `new(doses)` adds those of one stat together, drops any at 0 and sorts by `Stat`; read back from saves and messages the same way (`serde(from)`). `EMPTY`, `is_empty`, `doses`.
- `Water` (`Plain`, `Cave`, `Moon`, `Salt`; `ALL`, `name`: "Water", "Cave Water", ...), `Oil` (`Vegetable`, `Nut`, `Seed`; `ALL`, `name`: "Nut Oil", ...). What each does to a brew comes with brewing (`brewing.md`, decisions 2 and 3).
- `Liquid`: `Water(Water)`, `Oil(Oil)` or `Potion(Contents)`; `kind`, `contents` (a potion's), `name`.
- `LiquidKind`: the same without a potion's contents (`Potion`), for asking for one: `all()` (waters, oils, Potion), `name`.
- `BottleForm { liquid: Option<Liquid>, look }` (`ItemForm::Bottle`): `new(liquid, look)` (a potion with no doses becomes empty), `is_empty`, `is_potion`, `contents` (a potion's, else empty), `name` ("Empty Bottle", "Potion", "Bottle of Moon Water", which `Item::name` says), `grams` (`EMPTY_GRAMS` 120, any full one `POTION_GRAMS` 220). `look` is a seed (from the item's id, through `give_form`) the client picks the shape, glass and stopper from. `Item::bottle()`.
- `roll_contents(rng, effects)`: that many doses (at most `MOST_EFFECTS`, 4) on different stats, the first 4 to 8 steps, the rest 2 to 6, each helpful two times in three: Give Item's potions until brewing makes them.
- `bottle_to(rng, request, look)`: the bottle a `RollRequest` asks for (`liquid`, `effects`): with no liquid, a potion of `effects` (empty for 0 or none); a potion 1 to 4 effects (rolled if not given, refused at 0); a water or oil refuses effects above 0. Refusals are `Field::Effects`.

## Depends on

- `herb` (`Stat`), `item` (`Item`), `item_form` (`ItemForm`, `RollRequest`, `Refused`), `rng`, `serde`.

## Used by

- `item` (`ItemKind::Bottle`: 1 × 1, counted, with quality, Other; stacks compare its form without its look), `item_form` (`give_form_to` rolls a bottle through `bottle_to`; `Field::Liquid`, `Field::Effects`, `Choices::liquids`, `Choices::effects`), `shop` (`bottled`, `stock_request`: herbalists' and potion shops' waters and oils), client `item_art` (bottle and liquid pictures), `debug_menu` (Give Item's Liquid and Effects dropdowns, the Bottles pictures page) and `dev_tools` (`client.give_item`'s `liquid` and `effects`).

## Notes

- Stacking (owner): by liquid, potions also by quality (waters, oils and empty bottles ignore it), the look ignored (`as_compared` zeroes it), so shapes may differ in a stack and it shows its bottom one's look.
- Saves from before liquids (2026-11-17) have only `contents`: they load as a potion, or an empty bottle when it had none. A bottle is read one way from human-readable formats (RON saves, JSON) and strictly from postcard messages, which carry both fields exactly (`form.rs`).
- A new bottle is empty unless asked for a liquid or effects; shops' bottles hold a liquid picked from the item's seed. Never magical, blessed or cursed (only worn kinds can be).
- The liquid's look depends only on the liquid (owner): the same effects look the same in any bottle and at any rarity; helpful lighter, harmful darker; waters and oils have their own colours.
