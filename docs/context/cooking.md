# Cooking: plan

Owner's decisions (2026-11-20 and 21), from the research review and two rounds of answers. Cooking works like Alchemy (`alchemy.md`): tools placed in order, each step with its own sub-skill, a preview of what will come out, and whole-number rules in `shared` so the preview matches what the server makes. Unlike Alchemy, it has **rows**: each row is its own chain of steps, and a step can join rows, taking each row's output. It names what it makes from **rules of cooking**, not a recipe list. Research: `research-cooking.md`. The culinary data is built (`shared::cooking`, PR 1).

## The process

- Tools placed in order, each a step with its own Cooking sub-skill. The chance and step limit are Alchemy's: 100% + Cooking%, ⌈√Cooking%⌉ steps allowed, each extra step halving the chance; a join counts as a step and a half. More steps (complexity) add to quality.
- **Rows:** 2 to start, more allowed later. A step has a column, a top row and a height: a tall step spans the join of rows and takes all their outputs.
- **Tools:** Knife (cut; any knife or dagger from `shared::blade`), Pan (dry heat), Cauldron (wet heat; the existing item, kept from brewing for this), Bowl (mix; churning is a long Bowl step), Oven (bake, roast), Sieve (strain), and Alchemy's Mortar for spice pastes. The research backs all of them (`research-cooking.md`).
- **Rest is time, not a tool:** a row stopped early gives an intermediate item whose clock does the resting.
- Examples: cut (meat and vegetables) → Cauldron (water) → stew. Egg → Pan: fried egg; egg → Bowl → Pan: scrambled eggs; omelette and boiled egg likewise. Flour + water (+ egg) → Bowl: dough → Knife: noodles (one shape). Milk → Bowl (churned): butter; warm milk + acid or rennet → Sieve: fresh cheese; aged cheese later, on cask clocks. Flour + fat in a Pan: a roux, the base of white sauces.

```
row 1  [Bowl: wheat flour, egg, water] → [Knife] → [Cauldron: water, salt] ─┐
                                                                           ├─ [Bowl] → Noodles with Ragù
row 2  [Knife: onion, carrot]         → [Cauldron: minced beef, wine, milk] ┘
```

## Rules of cooking

- **Roles:** every raw food has culinary roles (Starch, Protein, Vegetable, Fat, Sweet, Acid, Nut, Aromatic, Herb, Spice) and properties: whether it needs cooking, texture, functions (thickens, binds, makes dough, ...), flavours, strength and the most of a dish it should be (`shared::cooking::IngredientDef`).
- **What a dish is:** its tool steps (and joins) plus its role mix, each ingredient adding its weight to its main role in full and its other roles at half. A table of real dishes as patterns names it with the **real name** when it is close to the true recipe or style (Carbonara, Paneer), otherwise a **generic name** from its form and its biggest ingredients ("Mutton and Turnip Stew"). Herbs and spices under their max share never change what a dish is (leeway).
- **Medicinal herbs** go in as a bitter seasoning: they give their effects but lower the dish's balance.
- **Poisonous fungi** carry their hidden poison into the dish (`shared::fungus` poisons, onset included); Mycology still protects.
- **Ash** goes in the Cauldron with maize as an alkali (nixtamal: hominy, masa).
- **Quality:** the ingredients' quality, plus steps × the sub-skills (∛, as Alchemy), plus **balance** (as Alchemy rewards purity): the roles the dish's form wants that are present, less anything over its max share, less medicinal bitterness. Capped at Rare away from a proper hearth.

## Results

- **Dish items:** real food, eaten later anywhere, bought and sold; models by form (bowl, plate, loaf, ...) tinted by the ingredients. Taverns can stock them later. No spoilage for now.
- **Well Fed** on eating: Health and/or Mana regen for 20 minutes at Poor, +10 a quality step; the amount from the ingredients; attribute bonuses from them (vegetables' cooked effects, fruit, herbs). One Well Fed at a time.
- **Diet labels** shown on food, worked out from the ingredients' flags: vegan, vegetarian, gluten free, lactose free, nut free (more later). Oats count as gluten free, coconut not as a nut, chestnut and acorn as tree nuts.
- **Intermediates:** any row can stop early, giving an item (dough, noodles, roux, stock, butter, fresh cheese, yoghurt, ...) that can start a later row from a source space. Each carries a clock, as yeast and casks do: stable (dried noodles), improving then declining (dough proving), or changing state (fresh noodles drying).
- **Warmth clocks:** rest clocks run by warmth: too cold slows them, too hot speeds them. Cooking only for now; yeast and casks later (`todo-warmth-clocks`).
- **Cultures:** ferments form by themselves, wild, poorer and slower; a matching culture improves them but is never needed. Cultures are strains on the Yeast item (yoghurt, koji), sampled from a good product as yeast is. Warm milk left to rest becomes yoghurt on its own. Koji (soy sauce, miso): ordinary yeast works, a koji culture does better.
- **Long processes:** soy sauce (soybeans + water + heat → Sieve → rest → salt water → rest → press), miso, aged cheese: long, but quality matters a lot.

## Heat and places

- Steps without heat work anywhere; heat steps need real heat. Water from every source (bottles, vessels, casks, water faced).
- **Hearths:** furniture on crafting bench rules (`shared::crafting_bench`): 1 × 1 a stove or oven only; 1 × 2 a stove and tools; 1 × 3 adds a storage space for leaving food to prove or rest (drawers in its look). Always lit in buildings with NPC spaces (towers, castles); elsewhere it needs kindling. Hearths give ash only from cooking (by heat time).
- **Towns** give a virtual 1 × 2 hearth: stove and tools, no storage, tools fixed (town buildings can't be entered). Hearths found elsewhere (towers, castles, huts) are open to everyone, storage included. Player homes, locks and personal storage come later.
- **Campfire:** a real object anyone lights (Make Campfire, learned on first carrying kindling; no tinderbox), 5 to 10 minutes a kindling, about 5 kindling at most, fed while burning, giving light and leaving Ash when it burns out.
- Later: the forge (too hot: a chance to burn food), lava.

## Ingredients still to add

- About 15 kitchen herbs, spices and vegetables (PR 2): basil, parsley, dill, coriander, oregano, marjoram, tarragon, chives, nutmeg, anise, caraway, sesame, poppy, courgette, sweet pepper (built). Claude's choices: every new herb has four effects like the rest (mild, from their kitchen and old medicinal uses; the format stays as it is), the poppy's are only its seeds' (no opium); sesame is a herb (seeds) whose main role is Nut, not flagged a nut allergen; paprika waits for ground spice items (sweet pepper is a vegetable, chilli stays a herb). Facts from Wikipedia (heights, natives, climates) and USDA-style kcal; abundance, growth and effects are game choices.
- Milk, Egg and Honey items (PR 3, built: `shared::farmyard`), hooked to livestock production later. Claude's choices: milk and honey are liquids by animal (`Liquid::Milk`, `Honey`) in bottles, vessels and casks, milk drinkable for nothing yet; eggs counted, stacking by animal, no quality; with no farm shops or general stores yet, herbalists sell eggs and bottles of milk and honey, of an animal kept in the town's biome. `RawMeat` stays generic; cuts have no items yet.

## PRs

| # | PR | Needs | Order |
| - | -- | ----- | ----- |
| 1 | Plan, research and culinary roles: `shared::cooking`'s table of each raw food's roles, functions, cooking need, strength, max share and diet flags (built) | none | first |
| 2 | New kitchen ingredients (above); plant looks, wiki pages regenerated (built) | 1 | independent |
| 3 | Milk, Egg and Honey items (per animal, counted or by the ml as fits), sold by farms' and general shops (built: `shared::farmyard`; herbalists sell them for now) | 1 | independent |
| 4 | Shared rules I: the grid (rows, columns, joins), tools and their sub-skills, the Cooking skill, chance and step limit (a join 1.5 steps), balance, quality with history carried by intermediates, heat needs and the Rare cap away from a hearth (built) | 1 | in order |
| 5 | Shared rules II: classifying and naming (the dish patterns table, real names when close, generic otherwise, herb and spice leeway), Well Fed, diet labels, poisons carried through (built) | 4 | in order |
| 6 | Food items: the Dish item (form, name, tint, diet labels in the popup), intermediates with rest clocks run by warmth, eating gives Well Fed, cultures as strains on the Yeast item, ferments forming wild without one (built: `shared::food`) | 5 | in order |
| 7 | Campfire and Ash (above) (built: `shared::campfire`) | none | independent |
| 8 | Hearths 1 × 1, 1 × 2, 1 × 3 on crafting bench rules; lit or by kindling; ash from cooking; towns' virtual 1 × 2; storage open to all (built: `shared::crafting_bench`) | 4, 7 | in order |
| 9 | The server: the Cook ability, the order checked against what is carried and in reach (heat, tools, water from any source), run times, practice and sub-skills, intermediates out, storage space clocks (built: `cooking::CookOrder`, server `connections/cook.rs`) | 6, 8 | in order |
| 10 | The client: the Cooking panel, the alchemy panel's process row grown into rows with join steps, drop spaces, preview (name, quality, chance, Well Fed, diet labels) (built: client `cooking`) | 9 | in order |
| 11 | Dish models by form (bowl, plate, loaf, flatbread, pie, noodles, dumplings, cake, wheel, jar, ...) tinted by ingredients, in `item_art` (built; on the ground left as bags) | 6 | independent after 6 |
| 12 | Long processes: soy sauce and miso (koji, salt water, months in a cask, pressed), aged cheese; ageing raising quality (built: `shared::food`'s `long.rs`, Press Oil's `Pressing::Moromi`) | 10 | last |

Later (not in this plan): tavern stock, the forge burning food, lava, spoilage, warmth clocks for yeast and casks (`todo-warmth-clocks`), the alchemy bench's storage space.

## The culinary data (PR 1, built)

`shared::cooking` (`docs/context/shared/cooking.md`): `Ingredients`, built in from `cooking/ingredients.ron`, a row per raw food keyed by source (herb, fruit, vegetable, fungus; meat, milk, egg and honey by animal; livestock parts by part), found from an item's form. Claude's choices: the research's `animal_product` (meant "from an animal but not meat") is set for meat too, so vegan is simply no animal product; flour is looked up as its grain; the research's dish lists and notes stayed out of the table (the dish patterns come with PR 5).

## Campfire and Ash (PR 7, built)

`shared::campfire` (`docs/context/shared/campfire.md`). Claude's choices, for the owner to tune: Make Campfire lights a fire on the ground faced (bare ground, grass, a floor or a stone or dirt road; never boards, fields, gardens, covered ground, water, furniture, trees, stairs) or feeds the fire faced, a piece of the first kindling carried at a time (5 s to light, 1 s to feed); a piece burns 5 minutes for the lightest wood to 10 for the densest, in real time by the server's clock (paused with it); a fire holds 5 unburnt; it lights 6 cells round (embers 1) and goes when it burns out, leaving one Ash (20 g, counted, no quality) a piece fed on the ground at its cell. Fires don't block, harm or spread, and can't be put out or moved. Ash is drawn as Salt's heap in grey.

## Food items (PR 6, built)

`shared::food` (`docs/context/shared/food.md`). Claude's choices, for the owner to tune: a Dish is counted in servings of about 300 g, each eaten whole for Well Fed (one at a time, the newest replacing it); intermediates are Dough, Fresh and Dried Noodles, Roux, Stock, Butter, Fresh Cheese, Warm Milk and Yoghurt, found from what an output is (milk heated alone is Warm Milk, milk worked in a Bowl alone is churned Butter); dough is a step better from 1 to 4 hours' rest and a step worse each 8 hours after 12; fresh noodles dry in 12 hours; warm milk sets into yoghurt in 8 hours with a yoghurt culture, 12 with another strain, 16 wild (and a step poorer); clocks stop at freezing, run at their pace at 20 °C and twice it at 40 °C; carried food rests at 20 °C, food on the ground at the land's yearly temperature. Cultures are `yeast::Strain`s (Yeast, Yoghurt Culture, Koji): only bread yeast ferments casks, a yoghurt culture is never fed (a new one is taken from each yoghurt). Intermediates are not eaten by themselves, and a Dish is not on the action bar yet. Pictures are models by form and by kind of intermediate, in the tint with an accent for pieces (PR 11, `item_art/dish/`); on the ground they are still bags. No trade exists yet, so buying and selling waits for it.

## Hearths (PR 8, built)

`shared::crafting_bench` (`docs/context/shared/crafting_bench.md`): hearths are crafting benches of the crafts Stove (1 × 1), Hearth (1 × 2) and Range (1 × 3), of stone. Claude's choices, for the owner to tune: new kitchen tool items Pan, Bowl and Sieve (cooking's tools by `cooking::Tool::of_item`; blacksmiths sell the Pan, carpenters the Bowl and Sieve); the Oven has no item, it is a lit hearth's own (for PR 9 to check); a Stove holds no tools, a Hearth and Range one each of Pan, Cauldron, Bowl, Sieve, Knife and Mortar and Pestle, starting with a Pan and Cauldron and perhaps more; a Range's storage holds 8 things (dishes and intermediates only), resting in real time at the land's warmth as on the ground; towers, castles and forts (households) have hearths always lit, huts and cottages a hearth lit and fed with kindling by Make Campfire (burning as a campfire, no Ash); one craft of hearth a site, on ⌈levels ÷ 5⌉ levels (huts mostly Stoves, cottages any, towers Hearths or Ranges, castles and forts Ranges); `hearth_ash` gives one Ash per 450 s of heat used (for PR 9); towns give a virtual Hearth (`town_hearth`, `town_tools`) that PR 9 uses. The client draws a stove with a firebox under a hood, a counter, drawers on a Range, and a small fire with light in a lit one.

## The server (PR 9, built)

`shared::cooking`'s order (`CookOrder`, `Inventory::check_cooking`) and the server's `connections/cook.rs`. Claude's choices, for the owner to tune: the Cook ability comes from Cooking, given to everyone for now (as Alchemy), needs no tool (the order is checked), and is asked for by the panel (PR 10); the kitchen is the hearth asked for and faced (its tools, storage, and heat and Oven only while lit), else a town's virtual hearth anywhere in the town, else a lit campfire faced (Fire: no Oven, capped at Rare), else cold; tools on a hearth or in a town are used before carried ones; food goes in by the item (each its own weight), water, milk and honey by the millilitre from a carried bottle (whole bottles, given back empty), vessel or cask, or from water faced; an intermediate starts a row from what is carried or the hearth's storage, used whole; a carried culture (any Yeast item) goes into an output that ferments and is used up then; steps take 10 s (Knife, Bowl, Sieve, Mortar), 20 s (Pan), 30 s (Cauldron) or 40 s (Oven) less the sub-skill's ∛, a join half again; each step practises its sub-skill 8 a run, whatever comes of it; a lit real hearth (not a town's) leaves `hearth_ash` of the run's heat time as Ash given to the cook, at least one a run with a heat step; a failure uses everything up and makes nothing. Kindling burns on hearths' and campfires' own clocks (PRs 7 and 8), so cooking takes none itself.

## The panel (PR 10, built)

Client `cooking` (`docs/context/client/cooking.md`). Claude's choices, for the owner to tune: all seven tools shown along the top, red when not to hand (the Knife and Oven drawn as icons); a click or tap on a tool adds it at the end of the top row, a drag puts it on any empty place of the grid (two rows, as many columns as used plus one, at least 3); a click or tap on a step makes it join both rows (where the place above or below is free) or splits it again; a step dragged off the grid is taken out; each step has up to 6 food spaces and 2 liquid spaces, growing as they fill, and a water faced space while water is faced; dropping a stack already in a step adds one more of it (100 ml for a liquid, up to what it holds); a click on the water faced adds 100 ml, back to none past 1 litre; one culture space for the whole process; the preview shows every output (the first in the frame) with its least quality, diet labels and Well Fed (or that an intermediate rests); the kitchen is found as the server finds it and named under the grid. Hearths' storage can start a row only when dragged from somewhere that holds it (nothing on the panel lists it yet).

## Long processes (PR 12, built)

`shared::food` (`docs/context/shared/food.md`, `long.rs`). Claude's choices, for the owner to tune: salt is the Salt item (from Alchemy's Alembic), now a cooking food (`cooking::Source::Pantry`), with soy sauce a liquid (`Liquid::SoySauce`) that seasons dishes; soybeans boiled in a Cauldron and strained are Cooked Soybeans, growing into Soybean Koji in 48 h with a koji culture (72 h with bread yeast, 96 h wild and a step poorer), and a koji culture can be taken from koji as from yoghurt (no action does it yet); koji salted (at least 5% of the foods' weight) in a cold step is Moromi when kept wet (water at least half the foods' weight in that step), else Miso; a fresh cheese salted the same way is a Cheese; moromi, miso and cheese age a step better each 30, 20 and 15 game days (an hour each, at the warmth's pace), 3 steps at most, wherever they rest (not only in a cask); Press Oil (the oil press) presses a moromi whole into a bottle of soy sauce per 100 g of its soybeans at its quality, needing that many empty bottles, the mash not kept. Left over: rice koji, sampling a culture as an action, moromi kept in a cask, the pressed mash.
