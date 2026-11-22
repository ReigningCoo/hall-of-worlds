# Recipes: plan

Owner's decisions (2026-11-21). Recipe scrolls teach real dishes; each character keeps a recipe book of what they learnt and of what they cooked themselves. Cooking first; Alchemy's recipes come later on the same items and book. Cooking's own plan: `cooking.md`.

## Owner's decisions

- **Every real dish has a scroll:** all of `cooking/dishes.ron` (219). Scrolls are 1 × 1, without quality, and stack by dish.
- **Reading a scroll** inserts its recipe into the character's recipe book and uses the scroll up. Copying scrolls and writing new ones is a later plan (below).
- **Own recipes:** every process a character cooks successfully is remembered, generic dishes too, named as it was classified. Cooking the same process again moves it to the top; at most 100 are kept, the oldest dropped. They are shown differently from learnt ones.
- **Removing** a recipe is allowed; a learnt one read from a scroll gives the scroll back (into the inventory, else at the feet).
- **Beyond your skill:** the panels note when a recipe's chance is under 100% for the character's Cooking skill.
- **Alchemy later:** scrolls carry a craft (`recipe::Craft`) and their art an emblem by craft, so Alchemy's fit in without new items.

## PRs

| # | PR | Needs | Order |
| - | -- | ----- | ----- |
| 1 | Shared data and the book: `shared::recipe` (the scroll item, `RecipeBook` on the sheet), `cooking::Recipe` and every real dish's recipe built and checked to cook into its dish, an order written down as a recipe, the beyond-skill helper (built) | none | first |
| 2 | Generic `${kind:…}` item tokens in text (`shared::item_text`), drawn red when the reader has none (built) | none | independent |
| 3 | The server, items, art, shops and rewards: reading and removing (`ClientMessage`), own recipes remembered on cooking, the scroll's picture (a rolled scroll with a chef's hat), scrolls sold and given as quest rewards (built) | 1 | in order |
| 4 | The reading panel: an unrolled scroll showing the recipe's grid and its written steps, foods missing in red, the beyond-skill note | 2, 3 | in order |
| 5 | The Recipes tab in the Cooking panel: the book listed (learnt and own told apart), Read, Make (filling the panel from what is carried, red ghosts for what is missing) and Remove | 4 | last |

## Later (not in this plan)

- Copying a scroll, and writing one from an own recipe (paper, ink, a skill), so recipes can be traded.
- Alchemy's recipes on the same scrolls and book.

## Generic item tokens (PR 2, built)

- `${kind:<kind>[:<option>…]}` in any text: a food as cooking wants it (`${kind:egg}` any egg, `${kind:vegetable:carrot}`), a cooking tool (`${kind:knife}`, `${kind:oven}`), water in a bottle (`${kind:bottle:water}`, `${kind:bottle:water:rain}`). `shared::item_text::KindToken` writes and reads them; every food and tool in the 219 dish recipes can be written (a test). Liquid foods (milk, honey, soy sauce) are foods too; water is `bottle:water`.
- The client draws one from a stand-in item (`KindToken::stand_in`, the usual food for "any"), the Oven by its icon, and names it on hover ("Any egg").
- Missing ones (PR 4, 5): put `missing="0 2"` on the element holding the words (`p`, `span`, `div`, `small`, `label`, `td`, `th`), counting the items and kinds written there from 0; their frames are red. Set it by id with `Dom::set`.

## Shared data and the book (PR 1, built)

`shared::recipe` (`docs/context/shared/recipe.md`) and `shared::cooking`'s recipes (`docs/context/shared/cooking.md`). Claude's choices, for the owner to tune:

- A scroll is "Recipe for <dish>", 20 g, Other; Give Item rolls any dish.
- A recipe counts foods by the item (a sack of flour for a grain, an egg, a herb as picked; meat as a 500 g Raw Meat and other butchered parts as 100 g, until those are items) and liquids (milk, honey, soy sauce, water) by the millilitre. Each input lists the foods it may be (the dish's alternatives, "any egg"), the first the usual one (a hen's egg, cow's milk, bees' honey, beef).
- A real dish's recipe is built from its pattern, not written by hand: the foods it must have, its usual ones, a common food for each role it still needs (honey for sweetness, lemon-like fruit for acid, the smallest seasoning), counts nudged until it holds nothing it forbids, every role it needs and its biggest role is one of them, and water for its wetness. A dish that joins rows puts all but its last two tools in the top row and the second-last alone below, starches on top. A test cooks every recipe with Common foods and checks it comes out as its dish.
- To make every dish reachable, the rules changed in one place: milk through a Sieve loses its whey and no longer counts as liquid (curds and cheeses come out dry). And `dishes.ron` changed where two dishes could never be told apart, or a dish could never be made: Baguette usually has salt, Sourdough rye; Butter, Ghee and Clotted Cream need Protein too (milk is mostly protein), Ghee is Dry and Clotted Cream and Panna Cotta Soupy; Labneh usually has salt; Halloumi needs Protein; Junket is Wet; Gulab Jamun and Pound Cake no longer forbid Protein (they hold milk and eggs); Poached Egg and Sticky Rice are Moist (Boiled Egg and Boiled Rice stay Dry); Crêpes are Soupy (Pancakes stay Wet); Hash Browns usually have onion; Biltong pepper and salt; Steak Tartare onion and salt; Dill Pickles garlic, Pickled Cucumbers no longer dill; Ratatouille onion and garlic; Pajeon egg; Kalguksu courgette; Pico de Gallo onion and coriander; Greek Salad cucumber and onion; Goulash onion and caraway.
- The book keeps learnt and own recipes together, latest first; a scroll already known is refused (kept). An own recipe is the order as cooked (`Inventory::cooking_recipe`, read before the run takes anything): its foods by what they are, no qualities, and the kind of intermediate starting each row.

## The server, items, art, shops and rewards (PR 3, built)

Claude's choices, for the owner to tune:

- A scroll is read by letting go of it on the clothes panel (as eating), carried or from the ground: `ClientMessage::Recipe(RecipeAsk::Insert)`, queued behind steps like eating. A scroll the book has is kept; its popup says "Already in your recipe book", else "Drop on yourself to add it to your recipe book". `RecipeAsk::Remove(entry)` is ready for PR 5's Remove button.
- PR 4's hook: after reading, the server sends `ServerMessage::RecipeInserted(entry)` (after `Sheet`); until the reading panel opens on it, a notice says "Recipe for Naan added to your recipe book".
- Every cooking run that works is remembered as an own recipe, named as its last output was classified.
- Scroll shops sell scrolls (3 in 5 of what they make), and inns and taverns sell only scrolls (which makes them shops). A quest's item reward may be a recipe (one in six of item rewards), given on hand-in as a scroll of a dish rolled from the prize seed.
- The picture: a parchment roll lying from bottom left to top right, tied with red ribbon, a white chef's hat standing on it (`item_art/scroll.rs`; Alchemy's angle and emblem go in `ScrollLook::style`).
