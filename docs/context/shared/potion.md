# shared: potion

Potions in bottles (owner, 2026-11-17): one item kind, `ItemKind::Bottle`, empty or holding effects, with a seed for its look. Not drinking or brewing (later), nor the bottles' and liquids' pictures (the client's `item_art`, PRs 2 and 3 of the potion bottles plan). Herbs' rule for potions: `docs/context/research-herbs.md`.

## Files

- `crates/shared/src/potion.rs`: the module list and re-exports. Tests in `potion/tests.rs`.
- `potion/contents.rs`: `Contents`, `Dose`.
- `potion/form.rs`: `BottleForm`, `EMPTY_GRAMS`, `POTION_GRAMS`, `Item::bottle`.
- `potion/roll.rs`: `roll_contents`, `MOST_EFFECTS`.

## Public API

- `Dose { stat, step }`: a `herb::Stat` (nine attributes, Health, Mana) moved up (helpful) or down (harmful) by a whole-number step.
- `Contents`: doses in one normal order, so equal effects compare equal: `new(doses)` adds those of one stat together, drops any at 0 and sorts by `Stat`; read back from saves and messages the same way (`serde(from)`). `EMPTY`, `is_empty`, `doses`.
- `BottleForm { contents, look }` (`ItemForm::Bottle`): `look` is a seed (from the item's id, through `give_form`) the client picks the shape, glass and stopper from. `is_empty`, `name` ("Potion" or "Empty Bottle", which `Item::name` says), `grams` (`EMPTY_GRAMS` 120, `POTION_GRAMS` 220). `Item::bottle()`.
- `roll_contents(rng, effects)`: that many doses (at most `MOST_EFFECTS`, 4) on different stats, the first 4 to 8 steps, the rest 2 to 6, each helpful two times in three: Give Item's potions until brewing makes them.

## Depends on

- `herb` (`Stat`), `item` (`Item`), `item_form` (`ItemForm`), `rng`, `serde`.

## Used by

- `item` (`ItemKind::Bottle`: 1 × 1, counted, with quality, Other; stacks compare its form without its look), `item_form` (`give_form_to` rolls a bottle: empty, or with `RollRequest::effects` effects; `Field::Effects`, `Choices::effects`), client `debug_menu` (Give Item's Effects dropdown) and `dev_tools` (`client.give_item`'s `effects`).

## Notes

- Stacking (owner): by contents and quality only, the look ignored (`as_compared` zeroes it), so shapes may differ in a stack and it shows its bottom one's look; empty bottles stack with each other whatever their look. Rarities don't stack: brewing will turn effect totals into steps by quality.
- A new bottle is empty unless asked for effects: nothing makes potions but Give Item yet. Never magical, blessed or cursed (only worn kinds can be).
- The liquid depends only on the contents (owner): the same effects look the same in any bottle and at any rarity; helpful lighter, harmful darker (PR 3).
- No picture yet: the client draws the kind's placeholder until PR 2.
