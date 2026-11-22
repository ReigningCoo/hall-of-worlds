# client: cooking

The Cooking panel (cooking plan PR 10, `docs/context/cooking.md`), grown from the Alchemy panel (`client/alchemy.md`): opened by the Cook ability, a moving panel where the player places kitchen tools in a grid of two rows (a step can join both), puts carried food, water, milk and honey into each step's drop spaces (and an intermediate at the start of a row, a culture for what ferments), sees what one run makes at the kitchen they are at, and starts it. Also the recipe reading panel (recipes plan PR 4, `docs/context/recipes.md`): one recipe from the book on an unrolled scroll. Every figure comes from `shared::cooking` (`Inventory::check_cooking`, `plan`), the server's own rules. Native and web.

## Files

- `crates/client/src/cooking/mod.rs`: `CookingPlugin`, `OpenCooking`, the module list.
- `bench.rs`: `Bench` (resource: the panel's document, each row's source, the steps (`Station`: tool, column, top row, height, foods and liquids as `(stack, count or ml)`, water faced ml), the culture, the count, `Running`, what the last run made); `free`, `step_at`, `row_end`, `columns_shown`, `add` (end of the top row), `place`, `move_to`, `remove`, `toggle_join`, `uses`, `keep_there`; `ROWS`. Kept when the panel closes.
- `kitchen.rs`: `KitchenNow` (heat, Oven, tools, storage, water faced, the hearth's cell for the order, `Place`), `KitchenHere` (system parameter: a hearth faced and told (`common::BenchHere`, lit by `common::Benches`), else a town (`town_hearth`), else a campfire faced, else cold, as the server's `connections/cook_kitchen.rs`), `Kitchens` (resource) kept by `keep_kitchen` while the panel is open.
- `spaces.rs`: `Place` (`Source(row)`, `Food(step, n)`, `Liquid(step, n)`, `Faced(step)`, `Culture`), `places` (filled ones and one more, up to `MOST_FOODS` and `MOST_LIQUIDS`; the water faced while water is faced), `in_place`, `takes` (an intermediate carried or in the hearth's storage; raw food with culinary data; a bottle, vessel or cask of water, milk or honey; a Yeast item), `put` (one more, or 100 ml more, where the stack already is), `more_faced`, `most_ml`, `SpaceIcons` (also the Knife's and Oven's icons), `tip`.
- `order.rs`: `Bench::in_order` (by column then row: the order sent and filled), `order` (a `CookOrder` at the kitchen), `check` (one run by `check_cooking`), `most_runs`.
- `markup.rs`: `COOKING` (`cooking/panel`, `words`, the canvases `tools`, `grid` and `culture_space`, `steps`, `kitchen`, `start`, the count's `fewest`, `fewer`, `count`, `more`, `most`, the `object-slot` `product`, `product_name`, `chance`, `about`), `WORDS`.
- `open.rs`: `open_panel` (with `inventory::OpenCharacter`), `bind_panel`, `notice_closed`.
- `boxes.rs`: `spawn_box` (a tool's box, a join's two tall, its picture an item's or an icon: `Picture`, `tool_item`), `spawn_running`, `spawn_picture`, `space_shown`, `spawn_place` (an empty place, gold or red while aimed at), `id`; `BOX`, `GOLD`, `WASH`.
- `grid.rs`: `draw_grid` (the canvases drawn again on any change): `ToolRow`, `GridCanvas`, `CultureCanvas`, `ToolNode`, `StepNode`, `PlaceNode`, `SpaceNode`; `grid_nodes.rs`: `at`, `row_node`, `spaces_node`, `place_id`, `water_shown`; the grid is Bevy UI's CSS grid (a column for the sources, then the places; a join spans both rows).
- `drag.rs`: `press_bench` (`PreUpdate`, before `dom`'s presses and `HoldObjects`: a click or tap on a tool adds it, on a step joins or splits it, on a filled space empties it, on the water faced adds 100 ml; a drag picks up a tool or step as `dom::HeldObject`), `drop_on_bench` (after `HoldObjects`: `BenchAim`, the place or space aimed at and whether it may go there; on `dom::Dropped`, the change; a step let go off the grid is taken out), `BenchDrag`, `BenchAim`, `Dragging`, `BenchSpots`.
- `side.rs`: `answer_panel` (Start sends `ClientMessage::Cook`, Stop `CancelAbility`, the count's buttons), `show_side` (steps allowed, the kitchen, Start's state, the first output's frame, every output's name, the chance, diet labels and Well Fed), `ProductSlot`.
- `words.rs`: `output_item` (`food::food_item` at a quality), `is_intermediate`, `output_name`, `output_lines` (diet labels; Well Fed's minutes, restoring and bonuses through `common::bonus_line`), `output_colour` (`food::tint`), `chance_words`, `kitchen_words`, `refused_words`.
- `run.rs`: `follow_run` (`AbilityStarted` and `AbilityLanded` of Cook: `Landed::Cooked` shows what was made), `show_fill` (wash, and the fill along the steps in the order they run by each step's seconds), `stage_colour`.
- `debug_api.rs` (debug API only): `client.cooking`.
- `reading/` (the reading panel): `mod.rs` (`add`: its systems, after `keep_kitchen`); `open.rs` (`Reading`: the document and the `recipe::Entry` on it; `open_reading` on `ReadRecipe` or `ServerMessage::RecipeInserted`, raising it if open; `bind_reading`; `notice_closed`); `markup.rs` (`READING`: form `recipe`, a `panel moveable rolled` titled with the dish, `cuisine`, `own` (a quill and `own_words` in `hand` ink), the canvas `grid`, `beyond`, `steps` of copies of the template `step`, `makes`); `entry.rs` (`Shown::of`: the recipe, dish name and cuisine of an entry, a real dish's from `cooking::dish_recipe`, an own one's as remembered); `have.rs` (`Have`: each step's tool (`has_cooking_tool` at `Kitchens`), each input (`Inventory::has_recipe_input`, water faced counting), each row's intermediate (`has_intermediate` or the hearth's storage); `input_token`, `foods_named`); `draw.rs` (`RecipeGrid`, `draw_recipe`: the grid read only from `boxes`' `spawn_box` and `space_shown`, each input the kind's stand-in item with its count (liquids in 100 ml), red when not to hand); `written.rs` (`written`: `Line`s from `recipe.ftl` by column then row, `missing_in`, `render`); `show.rs` (`show_reading`: everything again when the entry, what is carried, the sheet or the kitchen change; the beyond-skill note from `steps_beyond` and `chance`); `debug_api.rs` (`client.read_recipe`).

## Public API

- `CookingPlugin`.
- `OpenCooking` (message): opens the panel, or closes it if open (`abilities`, on using Cook).
- `ReadRecipe(recipe::Entry)` (message): opens the reading panel on that recipe, or shows it there if open (the Recipes tab, recipes plan PR 5). Reading a scroll opens it by itself (`RecipeInserted`).

## Depends on

- `inventory` (`Carried`, `OpenCharacter`), `character` (`CharacterSheet`), `player` (`Player`), `common` (`BenchHere`, `Benches`, `CurrentDungeon`, `Facing`, `GameView`, `SpotPosition`, `OwnAbility`, `Session`, `Wording`, `bonus_line`, `UiPointer`, `TouchGesture`, `PointerSystems`), `connection` (`ToServer`, `FromServer`: `RecipeInserted`), `common::Wording` (`recipe.ftl`), `dom` (`Dom`, `UiEvent`, `SlotShown`, `HeldObject`, `Dropped`, `Finder`, `AbilityThing`, `DomInput`, `HoldObjects`), `widgets` (`AbilityLook`, `HoverTip`, `UiId`, `item_picture`, `logical_rect`), `item_art` (`look_of`), `shared::cooking`, `shared::food`, `shared::campfire`, `shared::crafting_bench`, `shared::potion`, `shared::pour`, `shared::recipe` (`Entry`), `shared::item_text` (`KindToken`), `shared::lang`.

## Used by

- `abilities` (`OpenCooking`).

## Notes

- Claude's choices (for the owner to tune) are in `docs/context/cooking.md`'s "The panel". Everything works by touch: taps are clicks, and a finger drags tools and steps; items are lifted by a finger resting on them, as the Inventory does everywhere.
- The kitchen is the client's guess at the server's: a campfire counts while one is drawn on the cell (the server also checks it is still lit), a hearth only once its tools are told. The order names the hearth only when one is faced; the server checks again before every run.
- Steps are sent and filled by column then row (`in_order`), so `run_seconds`' order and the fill agree.
- Ids: `cooking/tool.<Tool index>`, `cooking/step.<n>`, `cooking/place.<column × 10 + row>`, `cooking/source.<row>`, `cooking/food.<step × 100 + n>`, `cooking/liquid.<step × 100 + n>`, `cooking/faced.<step>`, `cooking/culture.0`. A canvas's id must not share a name with ids drawn inside it (`bind_panel` matches by name: `culture_space`, not `culture`).
