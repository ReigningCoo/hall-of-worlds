# shared: cask

Casks (drinks plan PR 1, `docs/context/drinks.md`): one item kind, `ItemKind::Cask`, not counted, whose form says its size, wood and the one liquid in it by the litre (`shared::potion`'s liquids). Collect Water pours into one once no empty bottle is left. Pictures and ground models are the client's `item_art` and `ground`. Research: `docs/context/research-drinks.md`.

## Files

- `crates/shared/src/cask.rs`: the module list and re-exports. Tests in `cask/tests.rs`.
- `cask/size.rs`: `CaskSize`.
- `cask/form.rs`: `CaskForm`, `CaskLook`, `WOODS`, `Item::cask`. `cask/form/saved.rs`: reading one back (postcard exactly; RON saves from before millilitres, with `litres` and no `opened`, load sealed).
- `cask/roll.rs`: `cask_to`, `cask_wood_weights`.
- `cask/time.rs`: `CaskTime`.
- `cask/fill.rs`: Collect Water's casks: `CASK_FILL_SECONDS`, `CASK_PRACTICE`, `cask_fill_ticks`, `cask_litres`, `Inventory::cask_for_water`, `Inventory::cask_to_fill`, `fill_cask_with`.

## Public API

- `CaskSize` (Pin, Firkin, Kilderkin, Barrel, Hogshead, Puncheon, Butt, Tun; `ALL` smallest first, `Ord`): `name`, `litres` (20 to 960), `oak_grams` (empty in oak, 8 to 150 kg, rough guesses), `shape` (cells lying in the pack, 2 × 3 to 7 × 8).
- `WOODS`: the five woods, `material::Material`'s Oak, Chestnut, Acacia, Cherry, Pine.
- `CaskTime { fermenting, made, aged, opened_for, warned, lying_since }` (drinks PR 6; `is_fermenting`): a cask's clocks in real seconds, let pass by `shared::ferment` (`CaskForm::pass`). Emptying a cask resets them; `holding` with a liquid keeps them.
- `CaskForm { size, wood, liquid: Option<Liquid>, ml, opened, time }` (`ItemForm::Cask`; by the millilitre since drinks PR 2): `new` (empty, sealed), `most_ml`, `holding(liquid, ml)` (clamped; none or 0 is empty; `opened` kept), `full_of`, `opened()`, `litres()` (whole, rounded down), `is_empty`, `room` (ml), `takes(liquid)` (room, and empty or the same liquid), `poured(liquid, ml)` (the cask after, opened, and how many millilitres went in), `name` ("Oak Firkin", "Oak Firkin of Moon Water"), `empty_grams` (oak's by the wood's `density` against oak's), `grams` (plus a gram a millilitre), `look`. `Item::cask()`. Pouring is `shared::pour` (a tap to draw from one).
- `CaskLook { size, wood }` (`Ord`): what shows from afar; `ground::Load::Cask` holds one.
- `cask_to(rng, request)`: Give Item's and shops' cask: size (`RollRequest::cask_size`, `Field::CaskSize`) and wood (`material`) rolled by weight unless asked, empty, or full of an asked water or oil; a potion is refused (`Field::Liquid`), a material not in `WOODS` too (`Field::Material`). `cask_wood_weights`.
- Collect Water: `CASK_FILL_SECONDS` (5), `cask_fill_ticks`, `cask_litres(skills)` (5 + ∛Water% × 5 / 4: 5 to 10 litres a pour), `CASK_PRACTICE` (as a bottle's); `Inventory::cask_for_water()` (a carried cask with room, empty or holding a water: `Ability::has_tool`), `Inventory::cask_to_fill(liquid)` (the one taking it next: already holding it before empty, fullest first, own grid before bags), `fill_cask_with(inventory, id, liquid, litres)` (how many whole litres went in; opens it).

## Depends on

- `potion` (`Liquid`, `LiquidKind`, `WATER_SKILL`, `COLLECT_PRACTICE`), `material`, `item` (`Item`, `Inventory::stacks_where`, `Inventory::find_mut`, `Shape`), `item_form` (`ItemForm`, `RollRequest`, `Refused`, `Field`, `keep_only`), `herb` (`cube_root`, `skill_percent`, `skill_value`), `energy`, `skill`, `rng`, `serde`.

## Used by

- `cellar` (castles' and towers' casks and what they hold), `item` (`ItemKind::Cask`, Other; its form's cells and grams), `item_form` (`give_form_to`, `fields`, `choices`), `ground` (`Load::of`: the biggest cask in a pile), `ability` (`has_tool`: Collect Water), `shop` (carpenters), server `connections/collect_water.rs`, `pour` (`Held`, opening), client `item_art` (`Look::Cask`, the mesh), `ground` (casks standing), `item_popup` (how much it holds, sealed or opened), `debug_menu` (Give Item's size dropdown).

## Notes

- Not counted, so never stacks. Big casks simply exist as items (owner); how they are moved or used is for later.
- Collect Water fills casks (bottles first, then a cask); pouring between containers is `shared::pour`. A cask is sealed until poured from or into (drinks decision 13); its clocks (`CaskTime`) and best-before once opened are `shared::ferment`'s (PR 6).
