# Research: cooking

Research for cooking (2026-11-21), reviewed by the owner; the plan built on it is `cooking.md`. The game's data is `crates/shared/src/cooking/ingredients.ron` (`shared::cooking`, `docs/context/shared/cooking.md`); this page is the summary. The full review page (every ingredient and dish, filterable) was a private artifact; the scripts and data stayed outside the repo.

## Method and sources

- **Ingredients:** every raw food the game has (468) gathered from the shared crate's tables: 82 herbs, 101 fruits and nuts (herbs' fruits included), 85 vegetables, pulses and grains, 111 fungi, meat, milk, eggs and honey by animal (44), and 45 livestock parts.
- Claude (Opus) gave each, in batches of 12, its culinary roles, main role, how often it is eaten, cooking need, texture, functions, usual methods, flavours, strength (1 to 5), the most of a dish it should be (max share, %) and five diet flags as a careful allergen label would set them, with a few dishes and a note. Then a pass of hand revisions.
- **Dishes:** a catalogue of 244 well-known real dishes in 14 categories (soups, stews, porridges, breads, noodles, dumplings, eggs, roasts, fried, salads, sauces, sweets, dairy, preserved), each written as the game's tool steps by row, the roles it needs and forbids, specific ingredients, how much liquid, its form, how it keeps, and a generic name pattern.
- Sources are general culinary knowledge, not quoted articles: strengths, shares and how well known a dish is are first guesses to tune.

## Findings

- **Roles work.** Nearly every ingredient has one main role: Vegetable 99, Protein 72, Sweet 58, Herb 44, Starch 38, Spice 27, Acid 24, Nut 19, Fat 15, Aromatic 13. 59 have none: inedible parts and medicinal-only fungi (reishi).
- **Use:** Staple 58, Common 135, Occasional 129, Rare 61, Medicinal only 38, Inedible 47.
- **The tools hold up.** Steps across the dishes: Bowl 128, Cauldron 126, Knife 115, Pan 78, Rest 63, Oven 53, Mortar 27, Sieve 15. The Oven is plainly needed (breads, pies, roasts); the Mortar earns its place (pesto, curry pastes, dips); the Sieve is small but needed for dairy and draining. Rest became time on an intermediate's clock rather than a tool.
- **Dishes are short:** 23 take one step, 106 two, 93 three, 20 four, 2 more. Alchemy's step limit (⌈√skill%⌉) fits: a novice manages two, a good cook four or five.
- **Two rows are real but not common:** 32 dishes join two preparations (noodles and a sauce, pies' filling and pastry, stews with dumplings): a mark of a skilled cook, not needed for everyday food.
- **Diet flags came out clean** after one fix: asafoetida was marked gluten (shops cut it with flour), but the raw herb has none. Honey is an animal product, not meat; bees give no meat.
- **Forms:** Bowl 68, Plate 23, Jar 19, Pudding 13, Sauce 13, Egg dish 13, Flatbread 11, Noodles 11, Dumplings 10, Cake 9, Pie 9, Paste 9, Loaf 8, Ball 6, Block 6, Strips 6, Sausage 3, Pat 1, other 6: about a dozen models would cover them.
- **Keeping:** declines 93, stable 62, improves 60, changes state 20, improves then declines 9.
- **Intermediates** (dishes mostly used in other dishes): butter, ghee, paneer, ricotta, quark, egg pasta dough, boiled rice, bacon, mayonnaise, béchamel, bone broth, fruit jam.

## Gaps

- Kitchen herbs and spices (basil, parsley, dill, coriander, oregano, marjoram, tarragon, chives, nutmeg, anise, caraway, sesame, poppy), courgette and sweet pepper (paprika) are missing: the catalogue leans on them (cooking PR 2).
- Milk, eggs and honey have production rates but no items (cooking PR 3). Meat is one generic `RawMeat` item; cuts by quality have no items.
- Made from other things, so cookable here: cream, butter, ghee, cheese, yoghurt, stock, gelatin, breadcrumbs, filo, masa, bulgur, semolina.
- Imports to swap: soy sauce and miso (a long process, cooking PR 12), olive oil (Press Oil already takes olives), coconut milk (Press Juice on coconut).
- Fish left out on purpose: the game has none yet.

## Owner's decisions on the research (2026-11-21)

Add the missing kitchen ingredients; medicinal herbs as bitter seasoning; poisonous fungi keep their poison in dishes; Rest is clock time; intermediates as proposed plus a roux; milk, egg and honey items now; real names when close, generic otherwise; oats gluten free, coconut no nut, chestnut and acorn tree nuts; ash in nixtamal. The rest is in `cooking.md`.

## The dish catalogue in short

244 dishes, 17 or 18 per category, from many cuisines (Indian and Italian 20 each, French 18, English 17, Chinese 15, American 11, British 10, Greek and Japanese 8, Mexican, Levantine and German 7, and some 40 more). For example:

| Dish | Steps (row: tool + added) | Form |
| ---- | ------------------------- | ---- |
| Naan | 1: Bowl (wheat flour, water, milk, salt) → Rest → Oven | Flatbread |
| Chapati | 1: Bowl (wheat flour, water, salt) → Knife → Pan | Flatbread |
| Baguette | 1: Bowl (wheat flour, water, salt) → Rest → Oven | Loaf |
| Paneer | 1: Cauldron (milk, an acid) → Sieve → Rest | Block |
| Irish stew | 1: Knife (a protein, a starch, an aromatic) → Cauldron (water) | Bowl |
| Jiaozi | 1: Bowl (wheat flour, water); 2: Knife (a protein, a vegetable, an aromatic); a Knife joining both → Cauldron (water) | Dumplings |

Each also gives the roles it needs and forbids (a chapati forbids Sweet), its liquid (none, little, some, lots) and a generic name pattern for near misses ("Unleavened <Grain> Flatbread"). The table comes into the game with cooking PR 5.
