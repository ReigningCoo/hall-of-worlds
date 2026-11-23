# Magic: plan

Owner's decisions (2026-11-23), worked out in conversation. Spells stop being abilities. One **Magic** ability, known by everyone from the start, opens a spell design panel. In it, a spell is built from **steps** the way an Alchemy process or a Cooking grid is: where it happens, and what it does there. Designed spells are kept in each character's **spellbook**, named, favourited, and dragged onto the action bar to be cast by hand or automatically.

Every rule is a whole-number formula in `shared` (a new `shared::spell`, beside `shared::magic`), so the panel's cost and preview always match what the server does. The client sends a spell's design; the server checks it against the caster's skills and does everything else. Mobs can cast designed spells by the same rules later.

Today's Heal becomes two spells: **Heal** (a Heal step alone, on yourself) and **Healing Missile** (Missile → Heal).

## A spell

- **Up to 10 steps** (a safety net against recursion; total Mana is the real limit).
- **A step** is either:
  - **a where step:** one where with a count (Missile ×2), plus, in the first step only, filters (below); or
  - **an effect step:** one or more effects with counts, read in order (so Void knows what comes after it).
- **A spell with no where before its first effect step** happens on the caster.
- **Steps resolve one at a time, about a second apart.** A missile's flight is its own delay: the next step starts when it lands.
- **What a step hits becomes the target of the next step.** The next where starts from each hit, facing away from where it came from, and follows the creature hit if it moves. If nothing was hit, the next where starts from the spot itself.
- Only the first where can make use of the caster's target. Later ones start from the earlier step's hits.

So:

| Spell | What happens |
| ----- | ------------ |
| Heal | You are healed |
| Missile → Heal | A missile flies to the target's spot and heals whoever it hits first |
| Touch → Missile → Fire | A missile flies on in the direction you face, burning what it hits |
| Missile → Surround → Fire | Fire round whoever the missile hits, but not them |
| Surround → Missile → Fire | 8 missiles fly outward from round you |
| Missile → Heal → Surround → Fire | Heals the target, then a second later burns everything round them |

## Wheres

Each unlocks in this order (see Unlocks). Its tier is also its cost (see Mana cost). Ranges and numbers are first guesses, to balance by playing.

| Where | Tier | What it hits | Stacked (×2, ×3 ...) |
| ----- | ---- | ------------ | -------------------- |
| Touch | 1 | Touching distance in the direction faced | A wider arc each |
| Missile | 2 | Flies at about 4× running speed to a fixed spot (the target's when cast; it doesn't follow), hitting the first creature in its way, stopped by walls | Faster each |
| Surround | 3 | A ring round its centre, not the centre itself | A larger outer ring each |
| Beam | 4 | A line to the target and everything along it, stopped by walls | More beams at angles either side |
| Point | 5 | Exactly the target, instantly, nothing in between; needs a clear line of sight | One more creature each: the nearest on the same side (friend or enemy) as the first |

- Only within the same where step does a count stack; the same where in two steps, with or without an effect between them, is two hops.
- Hits need a clear line from where the where starts (as sight does), so nothing reaches through walls.

## Filters

**Self**, **Friend** and **Enemy**: free, allowed only in the first step, and always together with a where. They limit what the first step can hit, and only that step: later steps hit everything as usual.

- **Self + Point ×2:** you and two friends. **Friend + Point ×2:** two friends, not you.
- **Friend + Surround → Heal:** heals only the friends round you.
- **A missile or beam with a filter** passes through what doesn't match: Enemy + Missile flies past a friend in the way.
- **A spell whose first step has no valid hit fizzles:** nothing happens, no Mana is paid, only the wind-up's time is lost. Enemy + Point ×2 with no enemy in sight fizzles.
- **Friends** are to be party members, hirelings and pets. Until parties exist, a friend is another player. **Enemies** are anything hostile to the caster. Neutral creatures (timid animals) are neither, so only unfiltered spells reach them.
- Filters never add the +1 for an extra part (Mana cost).

## Effects

Each effect has its own skill (Skill tree) and does more the better that skill. Several of the same in a step make it stronger, exactly as casting it that many times. Mixed together in one step they may make something else, from a combination table (`shared::spell`, data like Alchemy's `TRANSFORMATIONS`): fire + water = steam is the first row, the rest filled in by the owner in the effects PRs. A mixture not in the table does each effect.

**Void** reverses every effect after it in its step; a second Void puts them back to normal. Void with nothing after it does nothing.

| Effect | What it does | Reversed |
| ------ | ------------ | -------- |
| Fire | Flame damage; lights things that burn (campfires, fireplaces, lanterns) | Cold damage |
| Water | Gets things wet; puts out fires | Ice (freezes water to walk on, slows) |
| Air | Knocks back from the centre | A vacuum, pulling towards the centre |
| Earth | Paralysis for a time | Stoneskin: less harm from blows for a time |
| Identify | Learns about the target; cast on yourself, reveals what is secret about what you wear (magical, blessed, cursed); cast at the ground, finds secret doors and traps | Conceal: hides those secrets again |
| Curse | On a creature, less harm done by its blows for a time; on an item, it can't be taken off | Lifts curses |
| Illusion | Harder to hit: more Defence for a time | Removes illusions |
| Warding | Protection against magical effects, stronger the more stacked | Removes every effect, good and bad |
| Heal | Restores Health | Direct damage |
| Poison | Damage over time | Removes poison |
| Teleport | The target's next move goes there at once: a known cell in sight and in range, on floor, checked by the server | Prevents teleporting for a time |
| Light | Lights the place for a time | Darkness |
| Haste | Actions take less energy for a time | Slow |
| Calm | Calms a creature (the way into taming animals later) | Fear |
| Blessing | Attributes up for a time | Weakness: attributes down |
| Levitate | Floats over holes and water for a time | Brings floating things down |
| Mana | Gives Mana | Drains Mana; never giving back more than the spell cost |

- **Combat has no hit or miss roll**, so Curse lowers the harm a creature's blows do, and Illusion raises its Defence.
- **Effects for a time** are timed effects (`shared::bonus`), shown in the effect bar.
- **Resisting:** harmful magic is contested by the target's Aura, plus their Warding; paralysis, Calm, Fear and illusions by Will (the ratio contest from `abilities-skills.md`).
- **Friendly fire is allowed, never to death:** harm from a spell never takes the caster or a friend below 1 Health. Players can't kill each other with spells (no PvP yet).
- **Curse on an item** needs a way to target items, which comes later; casting on yourself covers what you wear for now.
- **Heal's amount** keeps today's line by Healing (12 Health at 0%, 120 at 100% for one Heal), whatever it costs.

## Mana cost

All in thousandths of a point, as Mana is kept.

- **An effect** costs 5, less the cube root of its skill's percentage, never below 1: 5 at 0%, 3 at 8%, 2 at 27%, 1 from 64%.
- **An effect step** costs its effects added up, plus 1 for each effect after the first: Heal 5, Heal ×2 11.
- **A where step** costs its tier × its count, plus 1 for each extra. Its Form skill takes off up to 50%. It never costs less than 1, so Touch → Heal is never cheaper than Heal.
- **The spell** costs its steps multiplied together, paid all at once when the first step lands:

| Spell | Cost at 0% skill |
| ----- | ---------------- |
| Heal | 5 |
| Heal ×2 | 11 |
| Missile → Heal | 2 × 5 = 10 |
| Point ×3 → Heal | (5 × 3 + 2) × 5 = 85 |
| Missile → Heal → Surround → Fire | 2 × 5 × 3 × 5 = 150 |

A median character starts with about 20 Mana (`shared::vitals`), so long spells come with Mana grown and skills learned.

## Unlocks

The number of wheres a caster can use is ⌊∛(Magic% × 6)⌋, Magic being the `magic` node's own value. Practice halves as it flows up, so Magic gets a quarter of what a leaf gets; Form practice feeds it too, which roughly halves these.

| Unlock | Magic | Casts at 8 practice, one leaf |
| ------ | ----- | ----------------------------- |
| Touch | 0.17% | 1 |
| Missile | 1.3% | 2 |
| Surround | 4.5% | about 46 |
| Beam | 10.7% | about 600 (under an hour) |
| Point | 20.8% | about 4,500 |

Filters are known from the start.

## Practice

- **An effect's skill is practised only when it changes something:** Health restored, harm done, a fire lit or put out, a secret found, an effect added or removed. Heal on yourself while whole teaches nothing.
- **A where's Form skill** is practised when an effect it carried did something.
- 8 a time, as Heal is now, at most once per skill every 5 seconds (`skill::Windows`). Familiar foes don't reduce it, as now.

## Casting

- A wind-up before the first step, as other abilities have; cancelled by moving or Esc until it lands. After that the steps go on whatever the caster does.
- Refused when there isn't enough Mana (the bar shows it as unusable).
- **The Way Inn:** anything beyond self is refused there, as fighting is.
- **Automatic casting**, worked out from the effects:
  - helpful effects only: when the first step's target (yourself, or a friend) is below a quarter of their Health;
  - harmful effects only: when an enemy is targeted and in reach;
  - mixed: by hand only.

## Spellbook, panel and action bar

- **Spellbook** on the sheet, kept by the server like the recipe book (`shared::recipe`'s `RecipeBook`): each spell with a name (the player's own, or an automatic one) and a favourite mark. Favourites stay at the top of the list. 100 spells (first guess).
- **Automatic names** from the steps, worded through `shared::lang`: Heal, Healing Missile, Ring of Fire.
- **Editing:** removing a spell puts its design back into the panel; adding it again stores it anew.
- **New characters' books** start with Heal and Healing Missile, and bars holding the Heal ability hold the Heal spell instead.
- **The design panel**, as Alchemy's and Cooking's: what can be used along the top (the wheres unlocked, the filters, the effects), dragged down into a row of steps, a where step taking filters in the first place and a count, an effect step taking effects in order with counts; reordered, dragged off; the cost, the unlocks and what is wrong shown as it is built; Save, which asks for a name or keeps the automatic one.
- **The action bar** takes spells from the spellbook (a new `BarThing::Spell`), by hand or automatic.
- **On screen:** each where drawn as it happens (sparkling bolts in flight, beams, rings and spheres, a flash at a point), coloured by its effects.

## Skill tree

Magic's Wild branch goes. Draft of the new branch (attributes as `skill/branches/magic.rs`):

| Branch | Leans on | Leaves |
| ------ | -------- | ------ |
| Arcane | Intellect | Fire, Water, Air, Earth, Void, Teleport, Haste, Levitate, Mana |
| Divine | Empathy | Healing, Warding, Blessing, Curse, Poison, Calm |
| Insight | Perception, Intellect | Identify, Illusion, Light |
| Form | Intellect, Will | Touch, Missile, Surround, Beam, Point |

Wild's practice moves when saves load: Beasts to Social's Animals, Plants to Earth, Weather to Air. Illusion's moves with it to Insight.

## PRs

1. **Skill tree:** Form and Insight, the new leaves, Wild gone with its practice moved; this plan, the decision and the context table. Shared only (built).
2. **The rules** (`shared::spell`): a spell's steps, wheres, filters and effects as data, checking a design (unlocks, 10 steps, filters only first and with a where), costs, automatic names, the combination table, the automatic casting rule. Needs 1 (built).
3. **Where hits** (shared): Touch's arcs, Beam's lines, Surround's rings, Point's targets, a missile's path, filters and sides, line of sight; pure functions both sides use. Needs 2.
4. **Spellbook and the Magic ability** (shared and server): the book on the sheet, messages to store, rename, favourite and remove spells, new characters' book. Needs 2.
5. **Casting on the server:** steps a second apart, Self, Touch, Beam, Surround and Point, Heal as an effect, fizzling, Mana, practice, the inn, never to death; the Heal ability gone and bars moved onto the Heal spell, and the client's bar taking spells (by hand and automatic). Needs 3 and 4.
6. **Missiles in flight** (server, client following them); Healing Missile in new books. Needs 5.
7. **Drawing wheres** (client): bolts, beams, rings, points, sparkles. Needs 6.
8. **The spellbook page** (client): the list with favourites at the top, names, removing, dragging to the bar. Needs 5; any order with 7.
9. **The design panel** (client). Needs 8.
10. **Effects: Void, Fire, Water** (and cold, ice, steam), resisting by Aura and Warding's place in it. Needs 5.
11. **Effects: Air, Earth, Poison, Curse, Illusion, Warding.** Needs 10.
12. **Effects: Identify, Teleport, Light, Haste, Calm, Blessing, Levitate, Mana**, and a players' wiki page on magic. Needs 10; any order with 11.

## Left for later

- Curse and Identify on items chosen from the inventory.
- Parties, hirelings and pets as friends.
- Mobs designing and casting spells.
- Learning rare effects (Teleport, say) from books or trainers, if not every effect should be open from the start.
- The rest of the combination table.
