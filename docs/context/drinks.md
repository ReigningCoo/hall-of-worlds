# Drinks and fermentation: plan

Owner's request and decisions (2026-11-18). Bottles, waters and oils (`shared::potion`), the oil press and Collect Water (`brewing.md`), fruit, vegetables and grains with their kcal (`shared::fruit`, `shared::vegetable`) and bakers already exist. This adds casks, vessels and pouring, juice and cordial, yeast, fermentation, vinegar, drinking alcohol, the alembic and casks in cellars. Research: `research-drinks.md`.

## Owner's decisions

### Clocks
1. One real hour is one game day.
2. Casks and drinks age on **world time** while on the ground, and on **game time while logged in** while in someone's pack.
3. Yeast goes by **24 real hours of playtime** in the pack (paused while logged out), and by real time on the ground.
4. Durations are real ones, in game days: ale about 24 game days (about a real day), cider and perry about 1.5 real days, wine about 4.
5. Ageing in oak raises quality, up to a cap per drink.

### Casks
6. Eight sizes, doubling from the English casks: Pin 20 L, Firkin 40, Kilderkin 80, Barrel 160, Hogshead 240, Puncheon 320, Butt 480, Tun 960.
7. Woods: oak (improves with age), chestnut and acacia (neutral), cherry (a little flavour), pine (taints).
8. The liquid adds its weight (about a kilogram a litre).
9. Big casks simply exist as items for now; how they are moved or used is settled later.
10. No ownership: anyone may use a cask in the world.

### Vessels and pouring
11. Vessels: tankard, cup, goblet, horn, wine bottle, jug, flask, waterskin.
12. A **tap** item, and one **Pour** rule between any containers.
13. A sealed cask never spoils. Once opened, a **best-before** countdown runs (longer for stronger drinks, none over 20%), shown in the popup, with a notice near the end. Adding vinegar speeds it.
14. Opened juice ferments wild into Poor cider instead of spoiling.
15. A bottle filled from a cask starts sealed.

### Fermentation
16. Strength ≈ kcal per litre ÷ 68, capped by the yeast's tolerance by its quality; leftover sugar makes the drink sweet; under 1% nothing ferments.
17. Grains ferment directly.
18. More yeast starts a cask faster. Time grows with the cask's size and falls with the yeast's quality and amount.

### Juice and cordial
19. Juice is pressed (**Press Juice**, with the oil press): a serving gives 1 Mana a second for 30 s.
20. Cordial (juice and water) makes 2 servings of 1 Mana a second for 20 s each.
21. An alcoholic drink gives twice its juice's effect, plus units of alcohol.

### Alcohol
22. **Units** on the sheet (1 unit = 10 ml of pure alcohol), worn off at **1 unit per 10 real minutes at 70 kg**, scaled by the kind's body mass; thresholds scale with mass too.
23. Stages: **tipsy** −33% speed; **drunk** −66% speed, Mind-row penalties, the odd **stumble** (seeded, so the client predicts it) that stops travel; **very drunk** collapses; **poisoning** harms; a **hangover** after.
24. Stacks fully with load penalties.
25. A **Tolerance** skill raises the thresholds.

### Yeast
26. Bought from bakers, nearly always Common.
27. Each yeast is its own item in a stack, the ones needing feeding shown first; feeding a stack feeds them all.
28. Each yeast can be used in one thing between feeds without being used up.
29. It loses a quality step for each 24 h unfed (Poor stays Poor for ever).
30. An **early feed** (under 12 h) resets its progress and splits it into two identical yeasts; the split uses that use up, so it needs 12+ hours and an on-time feed before it can split again.
31. **On-time feeds** (12 to 24 h) add progress points by the feeder's **Fermenting** skill, kept on the item. Fermenting is a new skill.

### Alembic
32. Heat is assumed to exist for now. Pure alcohol's use in potions is left for later (`todo-magical-potions`).

## PRs

1. **Casks** (built; and this plan into `docs/context/`, with `decisions.md` and the table in `CLAUDE.md`): casks as items in 8 sizes and 5 woods, holding a liquid by the litre, weights; models in `item_art` and on the ground; carpenters sell them; Collect Water fills them; wiki and Give Item.
2. **Vessels, tap and pouring.** Needs 1.
3. **Juice and cordial** (Press Juice). Needs 2.
4. **Yeast** (independent): the item, feeding, splitting, quality loss, the Fermenting skill.
5. **Fermentation rules in `shared`** (independent): strength, sweetness, tolerance, time by cask, yeast and amount, ageing by wood.
6. **Ferment and vinegar.** Needs 1 to 5.
7. **Drinking alcohol**: units, stages, stumbles, Tolerance. Needs 6.
8. **Alembic.** Needs 6.
9. **Feeding starters.** Needs the miller and flour plan.
10. **Casks in castle and tower cellars**: one room at most, casks not blocking, random contents and ages. Needs 6.

## As built

- Casks (PR 1, `docs/context/shared/cask.md`; Claude's choices and first guesses for the owner to tune): one `ItemKind::Cask` with its form (size, wood, liquid, litres); the woods are new materials (Oak, Chestnut, Acacia, Cherry, Pine), each cask weighing oak's figure for its size by its wood's density against oak's. Cells lying in the pack grow with size, from 2 × 3 (Pin) to 7 × 8 (Tun). Give Item asks for a size, a wood (as its material) and optionally a water or oil (then full); potions are refused. Carpenters sell them, the small sizes far more often. Collect Water fills empty bottles first, then pours 5 to 10 litres (by the Water skill) every 5 seconds into a carried cask that is empty or holds the same water, the one already started first; each pour practises Water as a bottle does. A pile on the ground holding a cask shows the biggest cask standing there instead of a sack (`ground::Load::Cask`, remembered like bags). The popup says how many litres it holds.
