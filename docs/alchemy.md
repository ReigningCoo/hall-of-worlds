---
title: Alchemy
nav_order: 6
parent: Gameplay
---

# Alchemy

Alchemy turns [herbs](herbs) into potions. Fill bottles with water and press oils, then use the **Alchemy** ability to work herbs and liquids through your apparatus, one step after another. The **Alchemy** skill makes every step better, each apparatus has its own skill under it, and **Herbalism** helps with the herbs.

## Tools

| Tool | Used for | Sold by |
| ---- | -------- | ------- |
| Oil Press | Press Oil, and Press Juice ([vessels](vessels)) | Carpenters |
| Mortar and Pestle | Alchemy: grinding herbs | Herbalists and potion shops |
| Retort | Alchemy: infusing liquids | Herbalists and potion shops |
| Alembic | Alchemy: distilling | Herbalists and potion shops |
| Crucible | Alchemy: calcining | Blacksmiths |
| Filter | Alchemy: filtering | Herbalists and potion shops |

An ability whose tool you don't carry is greyed out. A tool in a bag counts. Alchemy needs at least one apparatus. The first time you carry an apparatus, you learn its skill: **Grinding** for the Mortar and Pestle, **Infusing** for the Retort, **Distilling** for the Alembic, **Calcining** for the Crucible, **Filtering** for the Filter.

## Waters

Herbalists and potion shops sell bottles of water. The first time you have an empty bottle, you learn **Collect Water**: face water close by (a pond, a river, the sea) and use it to fill your empty bottles one after another, and then any [cask](casks) you carry, until you move or are hurt. In steady rain you can also stand outside, facing no water, and catch the rain. Practice in the **Water** skill makes it quicker.

Where you fill a bottle decides which water you get:

| Water | Where | What it does to a brew |
| ----- | ----- | ---------------------- |
| Water | Anywhere else | Nothing |
| Cave Water | Underground: caves and cellars | Makes harmful effects stronger |
| Moon Water | Outside at night, with a moon in the sky | Makes helpful effects stronger |
| Salt Water | The sea | Takes away the strongest helpful effect |
| Rain Water | Outside in steady rain, facing no water | Takes away the strongest harmful effect |

Water you face is always taken before the rain. Snow and sleet can't be caught.

## Oils

**Press Oil** fills empty bottles with oil from what you carry, about ten handfuls a bottle.

| Oil | Pressed from | Strengthens |
| --- | ------------ | ----------- |
| Neutral Oil | Vegetables | Nothing: it only carries the herbs |
| Grain Oil | Grain | Strength, Toughness, Agility and Health |
| Nut Oil | Nuts | Intellect, Perception, Will and Mana |
| Seed Oil | Seeds, beans and herbs used for their seeds | Presence, Empathy and Aura |

Herbalists and potion shops sell oils as well.

## Alchemy

**Alchemy** runs a process: a row of steps, each on an apparatus, with what goes into it. Using the ability opens the Alchemy panel, with your Inventory beside it.

### The Alchemy panel

- **Along the top:** every apparatus whose skill you know. Drag one down into the row below to add a step, or click it to add it at the end. Drop it between two steps to put it there.
- **The process row:** the steps, left to right. Drag a step along the row to move it, or off the row to take it out. A step past what your Alchemy allows has a red frame. Under each step are spaces for what goes into it: drag herbs, oils and bottles from your Inventory into them, and click one to take it out again. The space before the first step takes a stored Paste, Ointment or Powder to start from.
- **On the right:** **Start**, how many times to run it (`|<` once, `<` one fewer, `>` one more, `>|` as many as you have the ingredients for), and what one run makes, at the least quality it can be. Its effects show as **Unknown** until you know that effect on every herb that gives to it. The chance of success is under it.

While it runs, the panel is locked and its steps fade, then colour fills along them, from black through white and gold to the potion's own colour. What it made shows at the end of the row. **Stop**, moving or being hurt calls it off.

- **Mortar and Pestle** (first step only): one of each herb, ground dry into a **Paste**, with a bottle of oil into an **Ointment**, or with a bottle of pure alcohol into a **Tincture**. A herb's effects add up and opposite effects cancel out; how strong each is depends on its quality and your Herbalism, and Herbalism trims away effects you don't want. A tincture is already a liquid, 100 ml of it: its oil strengthens nothing, but every effect is a twentieth stronger, and on its own it makes a potion in one step.
- **Retort**: up to five bottles of water, oil, [drink](drinks) or pure alcohol, each acting on what came before (see the tables above; drinks and pure alcohol do nothing to the effects). Each bottle is 100 ml, and the result fills one bottle per 100 ml.
- **Alembic**: distils what came before, which must be a liquid. Feed it a bottle of a drink or pure alcohol, and the stronger that is, the more it boils away: drinks under 10% not at all, wine (10% to 19%) half, pure alcohol all but a tenth. The effects stay as strong as they were, so a potion comes out stronger in fewer bottles (never fewer than one). Salt Water leaves **Salt** behind, one for each bottle distilled.
- **Crucible**: calcines whatever came before, paste, ointment or liquid, into a **Powder**, three quarters as strong. Any liquid is driven off, so a potion becomes a powder again.
- **Filter**: put a Powder in it. An effect that both what came before and the Powder have is added together (opposite effects cancel); an effect only one of them has is taken out. Used well, it strengthens what you want and clears away the rest.

### Pure alcohol

Pour several drinks into a Retort as the first step, then distil them in an Alembic fed with something strong enough to reduce them, such as a bottle of wine. What comes over is **pure alcohol**, one bottle for each 100 ml left, as long as the drinks held 10 ml of alcohol for each: five bottles of 6% cider fed a wine make two. A smaller batch makes less each run, down to nothing, so use the count to run it again. Drinks alone in a Retort don't practise Infusing. Pure alcohol has no quality.

A stored Paste or Ointment can start a process instead of the Mortar, but loses a step of quality. A Powder never goes stale: starting from one loses nothing. You can rub an ointment on yourself, as you eat food, for a quarter of its strength; a paste or a powder does nothing by itself.

A process can be run several times in a row with one order, each run on its own. Moving or being hurt stops it.

### Chance and quality

With no Alchemy at all a one-step process always works; more Alchemy lets you take more steps safely, and each step beyond that halves the chance. So does each kind of herb beyond what your Alchemy allows. A process that fails still uses up everything put into it.

Everything stays at full strength until the end, where it is shared out over the bottles filled: more water makes more bottles, each weaker. A potion forms only when one of its effects is strong enough; weak effects are left out. If none forms, the ingredients are lost and the bottles come back empty.

The purer a potion, the better its quality: one strong effect makes a better potion than several of the same strength. Skill in the apparatus used adds to it when the process can't fail, and now and then a process turns out a step or two better than expected. A better potion lasts longer.

Each process that works teaches you one effect of one herb in it, so you learn more about herbs as you go.

## Drinking a potion

Drink a potion as you eat food: let go of it on yourself in the Character panel, or put it on the action bar and press its key. The bottle comes back empty, into your bags, or onto the ground if there is no room.

Health and Mana come at once. Other effects last a while, by the potion's quality:

| Quality | Lasts |
| ------- | ----- |
| Poor | 30 seconds |
| Common | 1 minute |
| Uncommon | 2 minutes |
| Rare | 4 minutes |
| Epic | 8 minutes |
| Legendary | 16 minutes |

 Each thing a potion changes is a separate effect, so a new potion's effect on, say, Strength replaces the old one's, while the old potion's other effects carry on.

After drinking a potion, you must wait 30 seconds before drinking another that changes the same things the same way, however strong it is. A potion that changes something else can be drunk at once.

A slot on the action bar holds one particular potion, and shows how many of it you carry.

## Practice

Every step practises its skill, whatever comes of it: Collect Water the Water skill, Press Oil Alchemy, and each apparatus its own skill (an Alembic every time), which also trains Alchemy. Herbs handled train Herbalism. A Retort practises Infusing only when it mixes liquids into something.
