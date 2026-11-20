# shared: fungus

Mushrooms, fungi and lichens (owner, 2026-11-19): what each is, the table of them (built in), fungi as counted items (one, a handful, or a large piece cut into servings when eaten), and what eating one raw does. Modelled on `shared::vegetable`, with its own kind and table (`decisions.md`, 2026-11-19). The plan, in 9 PRs, and what each builds: `docs/context/fungi.md` (PR 1 built: the table, items, eating raw). Research and the owner's decisions: `docs/context/research-fungi.md`.

## Files

- `crates/shared/src/fungus.rs`: the module list and re-exports. Tests in `fungus/tests.rs`.
- `fungus/fungi.ron`: the 111 rows (99 real, 12 invented), generated once from the research's `final_data.py` and `items.py` (the script stayed outside the repo); edit by hand from now on. Read by `include_str!`.
- `fungus/def.rs`: `FungusDef`, `Unit`, `check`.
- `fungus/traits.rs`: `Group`, `Where`, `Niche`.
- `fungus/poison.rs`: `Poison` (`when_eaten`, `check`), `Severity`, `Onset`, `Trigger`.
- `fungus/look.rs`: `FungusLook`, `Form`, `MOST_COLOURS`.
- `fungus/table.rs`: `Fungi`.
- `fungus/tag.rs`: `FungusTag`.
- `fungus/form.rs`: `FungusForm`, `new_item`, `Item::fungus`.
- `fungus/eating.rs`: `eating`, `raw_ticks`, `effect_ticks`, `healing`, `poison_ticks`.
- `fungus/cut.rs`: `cut`.
- `fungus/dev.rs`: `dev_fungus`.

## Public API

- `FungusDef { key, name, latin, invented, group, grows (where in RON), niches, biomes, temperature, abundance, unit, servings, grams, kcal, raw, food, herb, poison, growth, regrow, glow, lookalikes, look }`: `key` its id for good; `group` a `Group` (`Mushroom`, `Bracket`, `Jelly`, `Truffle`, `Sclerotium`, `Cup`, `Crust`, `Lichen`); `grows` a `Where` (`Deep` anywhere underground, `Dim` underground near daylight, `Dug` buried, `Surface` above ground only); `niches` what it grows on or by (`Niche`: wood, dung, soil, remains, water, lava, trees, roots, rock, insects, termites); `biomes` needed above ground, none for invented cave kinds; `temperature` herbs' `herb::Temperature` (a cave is at its land's); `abundance` vegetables' `Abundance` (`Common` to `VeryRare`); `unit` `One`, `Handful` (100 g) or `Piece` (cut into `servings`, about 200 g each); `kcal` and `grams` for one item (`kcal` 0 for what isn't food); `raw` vegetables' `Raw`; `food` 0 to 2 vegetables' `Effect`s (raw, cooked or both); `herb` four `herb::Effect`s or none (Alchemy, PR 8); `poison` an optional `Poison`; `growth` and `regrow` herbs'; `glow` its light's colour (PR 5); `lookalikes` keys of fungi it is mistaken for (PR 7), checked to be in the table by `Fungi::read`; `look` a `FungusLook`.
- `Poison { severity, onset, false_recovery, effects, raw_only, trigger }`: `Severity` `Mild` to `Deadly`; `Onset` `Soon`, `Delayed`, `Late` (game time, used by PR 6); 1 to 4 `herb::Effect`s; `Trigger` `Eaten` (the default), `Old`, `Repeated`, `WithAlcohol`, `Touched`, `Breathed`; `when_eaten` (only `Eaten` poisons on eating).
- `FungusLook { form, colours, smallest, largest }`: `Form` (cap and stem, funnel, bracket, ball, coral, cup, jelly, honeycomb, club, truffle, crust, lichen tufts, beard, plates), 1 to `MOST_COLOURS` (3) colours (its body, underneath, its stem), sizes across in mm; for pictures and models (PRs 2 and 4).
- `Fungi`: `built_in()` (`&'static`), `read(ron_text)` (each checked, lookalikes in the same text), `new(defs)`, `get(key)`, `by_tag(tag)`, `iter`, `len`.
- `FungusTag(u64)`, `FungusTag::of(key)`: a key hashed, so `bonus::Source` can name a fungus and stay `Copy`.
- `FungusForm { fungus, serving, look, grams }` (`ItemForm::Fungus`): `new(def, seed)`, `serving(def, seed)`, `is_whole(def)` (a piece of several servings), `name()` ("Giant puffball", "Giant puffball Slice", "Fungus" for a key the table lacks), `shape()` (as fruit's). `new_item(def, count, id, made_at)`; `Item::fungus()`.
- `eating(def, now) -> Vec<Effect>`: one serving eaten raw: healing (`Source::EatenFungus(tag)`, as vegetables': `RAW_SHARE` of a serving's kcal ÷ 4 s, only for `Commonly` or `Sometimes`, ended by moving or being hurt), raw and both food effects (`Source::FungusEffects(tag)`, for `effect_ticks`), and a poison whose trigger is `Eaten` (`Source::FungusPoison(tag)`, at once for now, for `poison_ticks(severity)`: 30, 60, 90 or 120 s; nothing ends either early). Herb effects are not eaten.
- `cut(whole, new_id) -> Option<(eaten, Option<rest>)>`: as `vegetable::cut`.
- `dev_fungus(key, seed)`: Give Item's fungus, by key or rolled from the seed, the same on both sides; also `item_form::give_form`'s.

## Depends on

- `herb` (`Effect`, `Stat`, `Growth`, `Regrow`, `Temperature`, `Rgb`, `EFFECTS`), `vegetable` (`Abundance`, `Raw`, `Effect`, `RAW_SHARE`), `fruit` (`RESTORE`, `bonus_of`), `climate` (`Biome`), `bonus` (`Effect`, `Source`), `item` (`Item`, `ItemKind::Fungus`, `Shape`), `item_form` (`ItemForm::Fungus`), `rng`, `id`, `ron`, `serde`.

## Used by

- `item` (`ItemKind::Fungus`, Food, counted, no quality; stacks compare a fungus's form without its look), `item_form` (`give_form` rolls one; name, shape, grams), `bonus` (`Source::EatenFungus`, `FungusEffects`, `FungusPoison`; `is_food` and `may_eat` take fungi), server `connections/eat_fungus.rs` (`eating`, `cut`), `eat_fruit.rs` (`servings_fit`), `strength_room.rs`, `give_items.rs` (`dev_fungus`), client `effect_bar` (names by `by_tag`, "Poisoned: Death cap", `lorc/mushroom` and `lorc/spotted-mushroom`), `item_popup` (`fungus.rs`: raw healing, food effects, servings, never the poison), `debug_menu` (Give Item's dropdown, "Any fungus", and preview), `dev_tools` (`client.give_item`'s `fungus`).

## Notes

- Built in only, no mods yet: both sides read the same table, so nothing is sent. Saves and messages name a fungus by key; effects by `FungusTag`.
- No quality, never magical, blessed or cursed. A stack keys on fungus and serving; each keeps its own look seed.
- For now every poison eaten raw starts at once and is shown on the effect bar like any other; delayed and hidden onsets, deadly drains, stumbles and the ink cap with alcohol come in PR 6, harm from handling and breathing in PR 9. Item popups leave poisons out until knowing fungi comes (PRs 6 and 7), so they never say a fungus is safe or not.
- Nothing grows them yet (PR 3): they come only from Give Item.
