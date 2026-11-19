# Alchemy: plan

Owner's decisions (2026-11-19), from the plan doc the owner approved. Alchemy keeps the brewing plan's "what" (herb effects, the mortar's values, waters, oils, purity: `brewing.md`) and gives it a new "how": one **Alchemy** ability opens an Alchemy panel where the player builds a process from apparatus, step by step, and sees the result before starting. Every rule is a whole-number formula in `shared`, so the client's preview always matches what the server makes.

- Grind, Brew and the Cauldron leave alchemy (the cauldron is kept for cooking later). Press Oil and Press Juice stay separate abilities.
- Each apparatus has its own Alchemy sub-skill; the panel only shows apparatus whose skill is known, so more can be added later.
- The wiki page `docs/brewing.md` became **Alchemy** (`docs/alchemy.md`, PR 1); "brewing" is left for alcohol.
- The drinks plan's Alembic (its old PR 8) moved into this plan (PR 5).

## Oils, waters and durations (PR 1, built)

| Liquid | From | Effect on a mixture |
| ------ | ---- | ------------------- |
| Neutral Oil | Vegetables (not grain, not seeds) | None; old saves' and messages' Vegetable Oil loads as this |
| Grain Oil | Grains | Boosts the Body row (as Vegetable Oil did) |
| Nut Oil, Seed Oil | As before | As before (Mind, Spirit) |
| Rain Water | Collect Water outside on one's own cell in rain there (`shared::weather`), facing no water | Removes the strongest harmful dose (the opposite of Salt Water) |
| Water, Cave, Moon, Salt Water | As before | As before |

Potion durations by quality: Poor 30 s, Common 1 min, Uncommon 2, Rare 4, Epic 8, Legendary 16 (each double the one before).

Claude's choices in PR 1: rain counts from `RAIN_AT` (0.25 of `WeatherSample::rain`, steady rain, not a drizzle) and only above the sleet line (`weather::LIGHTNING_WARMEST`), sampled at the site's world map point as the drawn rain is; water faced wins over rain; Rain Water's tie takes the first stat, as Salt Water's; Grain Oil takes as much pressed as other vegetables did (1 kg a bottle); shops sell Neutral and Grain Oil, never Rain Water.

## The Alchemy panel (PR 4)

A process built left to right, with a locked preview of what it makes.

1. **Top row:** every apparatus whose sub-skill the character knows, each drawn 2 × 2 cells. Unknown apparatus are not shown at all.
2. **Process row:** drag an apparatus down into a row of boxes that fills from the left; drag within the row to reorder, or drop a new one between two others. Each step adds its own item drop spaces below it. The player may stop adding steps at any time.
3. **Right of the process:** a **Start** button; under it a **count** with `|<` `<` `>` `>|` (1, −1, +1, as many as the ingredients allow) to repeat the process without intervention; and an item frame that can't be picked up, showing the product at its **minimum** quality. Each effect shows as Unknown until the character knows that effect's value on every source material.
4. **Running:** everything locks and turns black and white, then colour fills along the steps like a progress bar, through the old colour stages (black, white, gold, the product's own colour). At the end of the last apparatus the result is given. Moving or being harmed calls it off, as Grind did.

## Apparatus and sub-skills

Six apparatus, each with its own sub-skill of Alchemy, learned the first time its apparatus is picked up. Practice goes to the sub-skills and flows up to Alchemy; each one used counts as a step.

| Apparatus | Sub-skill | Drop spaces | What it does |
| --------- | --------- | ----------- | ------------ |
| Mortar & pestle | Grinding | Herbs, one oil or pure alcohol | The mortar's values and decisions stay. Herbs alone make a **Paste** (effects, no way to apply it); with an oil an **Ointment**; with pure alcohol a **Tincture** (no row boost, every dose +5%). More herb kinds than Alchemy allows halve the chance per extra kind instead of being refused. |
| Retort | Infusing | Liquids up to 500 ml (100 ml units), 1 solid | Each liquid applies its effect to the previous step's output; liquids dilute but add volume, so paste + oil + water can make a potion. The solid can transform it (Retort solids). |
| Alembic | Distilling | Alcohol | Distils the previous step's output: the alcohol fed in sets how much it reduces by (rounded down); pure alcohol is the most efficient. A smaller batch gives less pure alcohol per run (use the count). Salt Water gives **Salt**. |
| Crucible | Calcining | None | Any input into a **Powder**, ¾ the strength of the matching Paste; a Powder never counts as old (coagulation merged into it). |
| Filter | Filtering | 1 Powder | Stats in both the input and the Powder add together; a stat in only one is removed. |
| Pelican | Circulating | None | Uses a step; +20% to the purity score before it becomes a quality. |

- **Infusing practice** only when the Retort makes something mixed (paste + oil, ointment + water); a first-step Retort of liquids alone, as for pure alcohol, gives none.
- **Pure alcohol:** several alcoholic drinks mixed in a first-step Retort, then distilled in the Alembic with enough alcohol to reduce it. The Alembic is to be tuned so this is worth doing.
- **Shops:** Retort, Alembic, Filter and Pelican where the Mortar is sold (herbalists, potion shops); Crucibles at blacksmiths.

## Steps, chance, dilution, quality, learning

Whole numbers throughout (doses kept in hundredths, rounded once at the end), so the preview matches the server.

- **Chance:** 100% + Alchemy%. Steps allowed: ⌈√Alchemy%⌉, at least 1. Each step beyond that halves the chance (50%, 25%, 12.5% …); so does each herb kind over the Mortar's limit. The server rolls it from a seed.
- **Failure:** the ingredients are used up, practice is still given, nothing is learned.
- **Dilution:** every step in between keeps everything at full amount and strength. Dilution by volume (100 ml units) applies only to the final result, which fills one bottle per 100 ml.
- **Quality:**
  1. Base from purity, as before (strongest dose minus the next, ÷ 3).
  2. Plus (steps × the mean ∛ of the sub-skills used) ÷ 10, in whole quality steps, only when the final chance is at least 100%.
  3. Minus 1 step for an "old" source (a stored Paste or Ointment), never below Poor. Powder never counts as old.
  4. On success the server may roll higher: 1% for +1 step, 0.1% for +2. The preview shows the minimum.
- **Learning:** each successful process teaches one effect of one ingredient: the strongest unknown effect on the product, on the ingredient that gave most to it (the first added on a tie). Knowledge is already kept per herb and effect; only what is taught changes.

## The rules (PR 2, built)

`shared::alchemy` (`docs/context/shared/alchemy.md`). Claude's choices:

- **Dilution:** whole doses stand for one 100 ml bottle. Each stat's total in hundredths is divided by (100 × the process's 100 ml units) and rounded once to the nearest step, halves away from 0; the potion threshold (4) applies after. An ointment and one water make one bottle as Brew did; paste + oil + water make two bottles, each half as strong.
- Each 100 ml unit applies its liquid once, in order. Waters' and oils' quarter boosts are exact in hundredths (at least a step), so Moon and Cave Water can come out up to half a step above Brew's.
- The Mortar comes first only (a stored Paste or Ointment can start a process instead); a Retort needs at least one liquid and takes only waters and oils so far. The Retort's solid place comes with PR 8.
- Quality's sub-skill steps: steps × the sum of each distinct apparatus's ∛sub-skill% ÷ (their count × 10), rounded down once.
- Learning chooses, among herbs whose effect on that stat is still unknown, the one giving most towards the dose's way.
- The Paste item itself (its kind, picture, popup) comes with PR 3, which gives it; PR 2 only makes it as a product.

## Retort solids and Darkvision (PR 8)

| Solid | Source | Effect |
| ----- | ------ | ------ |
| Salt | Distilling Salt Water | Body row +25%, Mind and Spirit −25% |
| Sulphur | Sold only, potion shops and herbalists | Mind row +25%, Body and Spirit −25% |
| Mercury | Sold only, potion shops and herbalists | Spirit row +25%, Body and Mind −25% |
| Carrot | Grown, bought | With Perception: **Darkvision**, seeing in the dark up to N cells, N from how pure and strong the Perception dose is. Every helpful effect becomes Darkvision; harmful effects stay. |

## PRs

1 stands alone, 2 to 4 go in order, and 6, 7 and 8 can land in any order once 3 has merged.

| PR | What | Needs |
| -- | ---- | ----- |
| 1 | Neutral and Grain Oil, Rain Water, durations by quality, the Alchemy wiki page; this plan (built) | None |
| 2 | The process rules in `shared`: steps, chance, dilution at the end, Paste, quality with sub-skills, luck, old sources, learning, the preview; the six sub-skills in the skill tree (built: `shared::alchemy`) | 1 |
| 3 | Server: the Alchemy ability, Mortar and Retort steps, the Retort item, the count, Infusing's rule; Grind, Brew and the Cauldron removed from alchemy | 2 |
| 4 | Client: the Alchemy panel (dragging, drop spaces, preview, count, colour fill) | 3 |
| 5 | Alembic: pure alcohol, tinctures, Salt from Salt Water (replaces the drinks plan's old PR 8) | 3 and drinks PR 6 (Ferment, merged) |
| 6 | Crucible and Filter, Powder | 3 |
| 7 | Pelican | 3 |
| 8 | Retort solids: Salt, Sulphur, Mercury, Carrot to Darkvision with Darkvision in the sight rules | 5 for Salt |

## Later

- Research interesting potions and what makes them (more transformations like Carrot to Darkvision); extends `todo-magical-potions`.
- Steps timed by the suns and moons (alchemical hours): a step under a given moon gives a bonus or is needed for a transformation.
- The Cauldron for cooking.
- Not for alchemy: a bain-marie.
