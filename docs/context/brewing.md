# Potion brewing: plan

Owner's request and decisions (2026-11-17). Herbs (`shared::herb`), bottles holding potions (`shared::potion`), the Alchemy and Herbalism skills and the Herbalist and Potion Shop buildings already exist. This adds base liquids, tools, the steps from herbs to a potion, the rules that decide what comes out, and drinking. Herb research: `research-herbs.md`.

## The steps

1. **Water:** an empty bottle filled at water with **Collect Water**.
2. **Oil:** vegetables, nuts or seeds pressed into a bottle with **Press Oil** (needs an oil press).
3. **Ointment:** herbs and one oil ground together with **Grind** (needs a mortar & pestle). Usable on its own at ¼ strength.
4. **Potion:** one ointment and one bottle of water with **Brew** (needs a cauldron; fire and heat later). The potion replaces the water in that bottle.
5. **Drinking:** one effect per stat at a time, cooldowns, and the empty bottle back.

## Owner's decisions

### Liquids
1. Bottles hold a **liquid**: a water, an oil, or a potion (today's `Contents`). Bottles stack by liquid (and, for potions, quality, which comes from the contents).
2. **Waters**, decided by the server where the bottle is filled:
   - **Cave Water** underground. Strengthens harmful doses (+25%, at least +1), leaves helpful ones alone.
   - **Salt Water** from the sea. Removes the strongest helpful dose.
   - **Moon Water** outside with no sun in the sky and at least one moon. Strengthens helpful doses (+25%, at least +1), leaves harmful ones alone.
   - **Water** anywhere else. Changes nothing.
   - Later: **Meltwater** (when there is snow or ice to fill from) removes the strongest harmful dose. Rain water only if something like puddles after rain is added.
3. **Oils:** Vegetable Oil, Nut Oil, Seed Oil (seeds from herbs whose part is Seeds, and from vegetables), kept generic. Each suits one row of the attributes: Vegetable Body, Nut Mind, Seed Spirit (Health counts as Body, Mana as Mind). Doses in its row are boosted (+25%, at least +1: first guess). More oils later, as fruit and vegetables bring more.

### Skills and abilities
4. **Press Oil, Grind and Brew** come from Alchemy, and are unusable (greyed, like any ability that can't be used now) without the tool carried. Everyone has them for now; Alchemy is to be learned later (TODO `todo-learn-alchemy`).
5. **Water skill** (`gathering/water`) and **Collect Water**: learned the first time a character gets an empty bottle (the first ability learned rather than given by kind). Usable while facing water (any water cell, river, sea) within reach (as for a shop door). Each bottle takes 10 s − ∛Water% (whole seconds: 10 down to 6), and it keeps filling empty bottles one after another until the character moves or is harmed, like eating. Practises Water.

### Ointment
6. One oil and a set of **different** herbs: only one of each herb counts towards strength. The most kinds of herb in a mix grows with Alchemy (first guess: 4 + ∛Alchemy%, so 4 to 8), as visible progress.
7. **Doses made:** 1 up to ∛Alchemy% + 1 (1 untrained, 5 at 100%). Each dose takes one full set of the herbs; extra copies of a herb make more doses, never more strength. One oil makes the whole batch.
8. Ointment is a counted item of its own, stacking by contents and quality. Used on oneself it gives ¼ of each dose (rounded down; doses at 0 dropped), lasting as a potion of its quality would.

### The rules (one shared function, whole numbers)
9. Each herb's effects: `change × potency` (`herb::potency`, so quality capped by Herbalism), **capped per herb at √Alchemy% + 1 steps** per effect.
10. Effects added per stat; opposites cancel.
11. The oil's row boosted (decision 3).
12. Each stat's total **capped at 4 × (√Alchemy% + 1)**.
13. **Herbalism cuts unwanted effects:** the wanted sign is the sign of the largest total; ∛Herbalism% steps are taken off the doses of the other sign, one at a time, round robin from the largest. That is the ointment's contents.
14. **Brewing** applies the water (decision 2). A potion forms when any dose reaches 4; doses of 2 or more go in, the rest are dropped (the herb research's rule).
15. **A mix that forms no potion** still uses up its ingredients and still gives practice.
16. **Quality:** score = strongest dose − next strongest (whatever its sign), divided by 3 (so purer potions are better): 2 or less Poor, 3–4 Common, 5–6 Uncommon, 7–8 Rare, 9–10 Epic, 11 or more Legendary; never Artifact.

### Drinking
17. **Health and Mana** are restored at once, 5 per step.
18. **Attributes** last 2 minutes, plus 30 s per quality above Common, less 30 s for Poor: Poor 90 s, Common 2 min, up to Legendary 4 min.
19. **One effect per stat:** a new potion's dose on a stat replaces the old one on that stat; the old potion's other stats carry on.
20. **Cooldown:** 30 s on drinking the same combination of stats and signs (whatever the strength). Different combinations can be chained, at a cost in ingredients.
21. The empty bottle goes back into the inventory, or onto the ground if there is no room.
22. Potions on the action bar: a slot holds a particular potion (its contents), not just "a bottle".

### Practice
23. Alchemy from Press Oil, Grind and Brew; Herbalism from Grind (the herbs handled); Water from Collect Water. Amounts first guesses.

## PRs

1. **Liquids in bottles** (built; and this plan into `docs/context/`, with `decisions.md` and the table in `CLAUDE.md`): `potion`'s liquid (waters, oils, potions), names, stacking, saves (today's potions load as potions), Give Item choosing a liquid, the client's liquid colours for water and oil. Herbalists and Potion Shops sell water bottles and oils.
2. **Abilities from skills and learned abilities; tools** (built): a rule for abilities known from a skill, abilities learned per character (saved), unusable without a tool; the oil press, mortar & pestle and cauldron as items with pictures (Carpenter, Herbalist / Potion Shop, Blacksmith); the Water skill. Needs 1.
3. **Collect Water** (built): learned on a first empty bottle; facing water within reach; which water by place and sky; filling until moved or harmed; practice. Needs 2.
4. **The brewing rules** (built, `shared::brewing`) in `shared` (decisions 6 to 16 and 18): one pure function with tests, no interface. Needs 1. Independent of 2 and 3.
5. **Press Oil** (built): vegetables, nuts or seeds into an empty bottle. Needs 2, and the Fruit and nuts and Vegetables chats' first PRs.
6. **Grind** (built): the mixing panel (herbs, an oil, doses), making ointment, ointment used on oneself; potions made before listed for making again. Needs 2 and 4.
7. **Brew:** ointment and water into a potion; practice for Alchemy and Herbalism; wiki page on brewing. Needs 6.
8. **Drinking:** decisions 17 to 22. Needs 1 (Give Item's potions can test it before 7).

## As built

- Collect Water (PR 3): each bottle is a wind-up like Heal's, with energy as long, so steps wait behind it unless it is called off; the client calls it off on any move (as for every wind-up) and the server on moving or harm (`Cancel::Moving`, `Cancel::Hurt`, as eating's effects). Facing is within 45° of the heading, reach from the water cell's middle (Claude's choices). Underground is any cave level or a basement; "outside" for Moon Water is the player's own cell being out in the open.

- The brewing rules (PR 4, `docs/context/shared/brewing.md`): a function per step (`grind`, `brew`, `quality`, `duration_seconds`, `on_skin`); the caller (Grind) passes one of each herb with `doses` sets wanted and takes them away. Claude's choices: a herb's steps round to the nearest (`change × potency`, so 1 × 80% gives 1), a helpful/harmful tie for the largest counts as helpful, Salt Water's tie takes the first stat, boosts keep a dose's sign.

- Press Oil (PR 5, `docs/context/shared/potion.md`; Claude's first guesses for the owner to tune): a bottle takes about ten handfuls by weight (nuts 200 g, seeds 300 g, vegetables 1 kg), 15 − ∛Alchemy% seconds (15 to 11), 8 Alchemy practice a bottle. Nuts give Nut Oil; vegetables whose part is seeds (beans, peanuts) and herbs whose part is Seeds give Seed Oil; any other vegetable (grain too) Vegetable Oil; other fruit and herbs nothing. Choosing: using it opens a requester listing what is carried with a bottle's worth (name, count, bottles of which oil); Press presses that, bottle after bottle like Collect Water, until too little is left, the empty bottles run out, or the player moves or is harmed.

- Grind (PR 6, `docs/context/shared/brewing.md`; Claude's choices and first guesses for the owner to tune): using Grind opens a mixing panel: a checkbox per carried herb stack (ticking one unticks any other stack of the same herb), the oil bottle, the doses there is enough of, what it would make, and the mixes made before (latest first, at most 20, saved on the sheet with what each made last; Use picks the best stack of each herb). What it would make shows its doses only once the character knows every effect of the herbs ticked, so grinding teaches nothing for free. Grinding takes 10 − ∛Alchemy% seconds (10 to 6), called off by moving or harm; it uses `doses` of each herb and one bottle of oil (the bottle comes back empty), and gives 8 Alchemy and 2 Herbalism a kind of herb, whatever comes of it. Ointment is a counted item (60 g a dose) in a small tin coloured as a potion of it would be; it is used like food (let go of on oneself), the latest ointment's effect replacing any earlier one; its Health and Mana come at once, 5 a step (decision 17).

## Later

- Brewing towards effects the character knows: pick the effects wanted, the panel suggests known herbs that would make it (`HerbKnowledge`).
- Meltwater, rain water, fire and heat for the cauldron, a cauldron placed rather than carried, Alchemy learned.
