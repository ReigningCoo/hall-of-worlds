# shared: potion

Bottles (owner, 2026-11-17): one item kind, `ItemKind::Bottle`, empty or holding a liquid (a water, an oil, or a potion's effects), with a seed for its look. Filled with oil by Press Oil (`press.rs`); a water's bottle takes the potion Brew makes (`shared::brewing::brewed_into`). Drunk through `shared::brewing::drunk` (plan: `docs/context/brewing.md`); pictures are the client's `item_art`. Herbs' rule for potions: `docs/context/research-herbs.md`.

## Files

- `crates/shared/src/potion.rs`: the module list and re-exports. Tests in `potion/tests.rs`, `potion/liquid_tests.rs` and `potion/key_tests.rs`.
- `potion/key.rs`: `PotionKey`.
- `potion/carried.rs`: `Item::is_potion`, `Item::potion_key`, `Inventory::count_of_potion`, `Inventory::potion_to_use`.
- `potion/contents.rs`: `Contents`, `Dose`.
- `potion/liquid.rs`: `Liquid`, `LiquidKind`, `Water`, `Oil`.
- `potion/form.rs`: `BottleForm` (and how it is read from saves and messages), `EMPTY_GRAMS`, `POTION_GRAMS`, `Item::bottle`.
- `potion/roll.rs`: `roll_contents`, `bottle_to`, `MOST_EFFECTS`.
- `potion/press.rs`: Press Oil's rules (brewing PR 5): `Pressing`, `PressChoice`, `grams_per_bottle`, `press_seconds`, `press_ticks`, `pressed`, `PRESS_PRACTICE`, `Inventory::grams_of`, `stack_to_press`, `can_press`, `press_choices`. Tests in `potion/press_tests.rs`.
- `potion/collect.rs`: Collect Water's rules (brewing decision 5): `Source`, `water_from`, `water_faced`, `fill_seconds`, `filled`, `Inventory::empty_bottle`. Tests in `potion/collect_tests.rs`.

## Public API

- `Dose { stat, step }`: a `herb::Stat` (nine attributes, Health, Mana) moved up (helpful) or down (harmful) by a whole-number step.
- `Contents`: doses in one normal order, so equal effects compare equal: `new(doses)` adds those of one stat together, drops any at 0 and sorts by `Stat`; read back from saves and messages the same way (`serde(from)`). `EMPTY`, `is_empty`, `doses`.
- `Water` (`Plain`, `Cave`, `Moon`, `Salt`; `ALL`, `name`: "Water", "Cave Water", ...), `Oil` (`Vegetable`, `Nut`, `Seed`; `ALL`, `name`: "Nut Oil", ...). What each does to a brew is `shared::brewing`'s (`watered`, `oil_suits`).
- `Liquid`: `Water(Water)`, `Oil(Oil)` or `Potion(Contents)`; `kind`, `contents` (a potion's), `name`.
- `PotionKey` (brewing decision 22): a potion's doses as a step per stat in a fixed array, so the action bar's slots stay `Copy`; `of(contents)`, `contents()`, `is_empty`; saved and sent as its `Contents`. `Item::is_potion` (a bottle holding a potion), `Item::potion_key`; `Inventory::count_of_potion(key)` and `potion_to_use(key)` (as `stack_to_use`: the smallest stack in the own grid, else in a bag; never a quest item).
- `LiquidKind`: the same without a potion's contents (`Potion`), for asking for one: `all()` (waters, oils, Potion), `name`.
- `BottleForm { liquid: Option<Liquid>, look }` (`ItemForm::Bottle`): `new(liquid, look)` (a potion with no doses becomes empty), `is_empty`, `is_potion`, `contents` (a potion's, else empty), `name` ("Empty Bottle", "Potion", "Bottle of Moon Water", which `Item::name` says), `grams` (`EMPTY_GRAMS` 120, any full one `POTION_GRAMS` 220). `look` is a seed (from the item's id, through `give_form`) the client picks the shape, glass and stopper from. `Item::bottle()`.
- `roll_contents(rng, effects)`: that many doses (at most `MOST_EFFECTS`, 4) on different stats, the first 4 to 8 steps, the rest 2 to 6, each helpful two times in three: Give Item's potions until brewing makes them.
- `bottle_to(rng, request, look)`: the bottle a `RollRequest` asks for (`liquid`, `effects`): with no liquid, a potion of `effects` (empty for 0 or none); a potion 1 to 4 effects (rolled if not given, refused at 0); a water or oil refuses effects above 0. Refusals are `Field::Effects`.

- Collect Water: `Source { underground, sea, moonlit }` and `water_from` (Cave Water underground, else Salt Water from the sea, else Moon Water outside with no sun up and a moon, else Water: the server works out the source); `holds_water` (`Cell::Water`, `Shallows`), `water_faced(level, at, heading)` (the nearest water cell whose middle is within `COLLECT_REACH` (`shop::DOOR_REACH`) and within `FACING_HALF` (45°) of the heading, or underfoot); `fill_seconds(skills)` / `fill_ticks` (10 − ∛Water% whole seconds, 10 to 6; `WATER_SKILL` `gathering/water`), `COLLECT_PRACTICE` (8 a bottle), `filled(item, water)` (the bottle holding it, its look kept); `Inventory::empty_bottle()` (the stack filled next, smallest first, own grid before bags; also what `Ability::has_tool` and `Learned::learn_from` ask).

- Press Oil (brewing PR 5): `Pressing::{Fruit, Vegetable, Herb}(key)` (`of(item, herbs)`, `oil(herbs)`, `is(item)`): nuts (`FruitSet::Nut`) give Nut Oil, vegetables whose part is Seeds (beans, peanuts) and herbs whose part is Seeds give Seed Oil, other vegetables (grains too) Vegetable Oil, anything else none. `grams_per_bottle` (first guesses, about ten handfuls: Nut 200, Seed 300, Vegetable 1000), weighed by `own_weight` (a whole fruit counts whole; the last one taken may give more than needed). `press_seconds` 15 − ∛Alchemy% (15 to 11), `PRESS_PRACTICE` 8 Alchemy a bottle. `Inventory::can_press(pressing, herbs)` (a bottle's worth and an empty bottle), `stack_to_press` (smallest, own grid first), `press_choices(herbs)` (one `PressChoice { pressing, oil, stack, count, bottles }` per thing with a bottle's worth, bottles capped by the empty ones), `pressed(item, oil)`.

## Depends on

- `herb` (`Stat`, `cube_root`, `skill_value`, `Herbs`, `Part`), `fruit` (`Fruits`, `FruitSet`), `vegetable` (`Vegetables`, `Part`), `ability` (`ALCHEMY`), `dungeon` (`Level`, `Cell`), `facing` (`Heading`), `spot`, `shop` (`DOOR_REACH`), `energy`, `skill`, `item` (`Item`, `Inventory::stacks_where`), `item_form` (`ItemForm`, `RollRequest`, `Refused`), `rng`, `serde`.

## Used by

- `ability` (Collect Water's empty bottle), server `connections/collect_water.rs`, `connections/press_oil.rs`, client `abilities` (`press_oil.rs`: `press_choices`), client `abilities` (`water_faced` before asking), `item` (`ItemKind::Bottle`: 1 × 1, counted, with quality, Other; stacks compare its form without its look), `item_form` (`give_form_to` rolls a bottle through `bottle_to`; `Field::Liquid`, `Field::Effects`, `Choices::liquids`, `Choices::effects`), `shop` (`bottled`, `stock_request`: herbalists' and potion shops' waters and oils), client `item_art` (bottle and liquid pictures), `debug_menu` (Give Item's Liquid and Effects dropdowns, the Bottles pictures page) and `dev_tools` (`client.give_item`'s `liquid` and `effects`).

## Notes

- Stacking (owner): by liquid, potions also by quality (waters, oils and empty bottles ignore it), the look ignored (`as_compared` zeroes it), so shapes may differ in a stack and it shows its bottom one's look.
- Saves from before liquids (2026-11-17) have only `contents`: they load as a potion, or an empty bottle when it had none. A bottle is read one way from human-readable formats (RON saves, JSON) and strictly from postcard messages, which carry both fields exactly (`form.rs`).
- A new bottle is empty unless asked for a liquid or effects; shops' bottles hold a liquid picked from the item's seed. Never magical, blessed or cursed (only worn kinds can be).
- The liquid's look depends only on the liquid (owner): the same effects look the same in any bottle and at any rarity; helpful lighter, harmful darker; waters and oils have their own colours.
