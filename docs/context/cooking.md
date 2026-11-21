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
| 6 | Food items: the Dish item (form, name, tint, diet labels in the popup), intermediates with rest clocks run by warmth, eating gives Well Fed, cultures as strains on the Yeast item, ferments forming wild without one | 5 | in order |
| 7 | Campfire and Ash (above) | none | independent |
| 8 | Hearths 1 × 1, 1 × 2, 1 × 3 on crafting bench rules; lit or by kindling; ash from cooking; towns' virtual 1 × 2; storage open to all | 4, 7 | in order |
| 9 | The server: the Cook ability, the order checked against what is carried and in reach (heat, tools, water from any source), run times, practice and sub-skills, intermediates out, storage space clocks | 6, 8 | in order |
| 10 | The client: the Cooking panel, the alchemy panel's process row grown into rows with join steps, drop spaces, preview (name, quality, chance, Well Fed, diet labels) | 9 | in order |
| 11 | Dish models by form (bowl, plate, loaf, flatbread, pie, noodles, dumplings, cake, wheel, jar, ...) tinted by ingredients, in `item_art` and on the ground | 6 | independent after 6 |
| 12 | Long processes: soy sauce and miso (koji, salt water, months in a cask, pressed), aged cheese; ageing raising quality | 10 | last |

Later (not in this plan): tavern stock, the forge burning food, lava, spoilage, warmth clocks for yeast and casks (`todo-warmth-clocks`), the alchemy bench's storage space.

## The culinary data (PR 1, built)

`shared::cooking` (`docs/context/shared/cooking.md`): `Ingredients`, built in from `cooking/ingredients.ron`, a row per raw food keyed by source (herb, fruit, vegetable, fungus; meat, milk, egg and honey by animal; livestock parts by part), found from an item's form. Claude's choices: the research's `animal_product` (meant "from an animal but not meat") is set for meat too, so vegan is simply no animal product; flour is looked up as its grain; the research's dish lists and notes stayed out of the table (the dish patterns come with PR 5).
