---
title: Magic
nav_order: 5
parent: Gameplay
---

# Magic

Every character can cast spells, and every spell is one you design yourself. **Magic**, an ability everyone knows from the start, opens the spell design panel. There you build a spell from **steps**, give it a name, and keep it in your **spellbook**. From the spellbook you drag spells onto the action bar and cast them like any ability. See [Fighting](fighting) for the action bar and targets.

New characters' spellbooks start with **Heal** (a Heal on yourself) and **Healing Missile** (a missile that heals whoever it hits first).

## Designing a spell

Magic sits on the action bar from the start; if you take it off, the Spells tab (P) has a **Design a Spell** button too. In the design panel, click a where, filter or effect along the top, or drag it down, to add it to the spell. Drag steps, and the wheres and effects in them, to move them; drag them off the row, or right-click them, to take them out. Beside the spell's name and cost is the picture it will have on the action bar, greyed until it can be saved.

**Cast** tries the spell as it is, saved or not. It goes at your target just as a spell on the action bar would, walking you into reach first, and it is greyed whenever a spell on the bar would be: when your skills don't allow it, when you haven't the Mana, or in The Way Inn for a spell that isn't on yourself.

## Steps

A spell is a row of up to ten steps, cast one after another, about a second apart.

- A **where** step says where the spell happens: one or more of the wheres below.
- An **effect** step says what it does there: one or more of the effects below, read in order.
- A spell that starts with an effect step happens on you.
- Whatever a step hits is where the next step starts from: Missile, then Surround, then Fire burns everything round whoever the missile hit, but not them.
- Different wheres in the same step each start from what the one before hit, and the effects after the step land on all of them: Missile and Surround together, then Fire, burns the one hit and everything round them.
- The same where or effect twice in a step makes it stronger, as casting it twice would.

## Wheres

You learn the wheres in this order, as your Magic skill grows: at first only Touch, then Missile soon after, the rest much later.

| Where | What it reaches | Twice or more in a step |
| ----- | --------------- | ----------------------- |
| Touch | Anyone within arm's reach in front of you | A wider arc each time |
| Missile | Flies to the spot your target stood on when you cast, hitting the first creature in its way; walls stop it | Faster each time |
| Surround | A ring round its middle, but not the middle itself | A wider ring each time |
| Beam | A line to the target and everyone along it; walls stop it | More beams, fanning out |
| Point | Exactly the target, at once, if you can see it | One more creature each time, the nearest on the same side as the first |

Nothing reaches through walls.

## Filters

**Self**, **Friend** and **Enemy** go above the first where, and only there. They cost nothing.

- **Self** aims the first where at you, whoever you have targeted: Self and Surround is a ring round you.
- **Friend** and **Enemy** aim it at your target if it is one, else at the nearest one in reach; with nobody of that side about, the spell fizzles. They also limit what the first step can hit: an Enemy missile flies past a friend in the way.
- For now, a friend is another player, and an enemy anything hostile to you. Timid animals are neither.

A spell whose first step hits nothing fizzles: you lose the moment spent casting it, but not the Mana.

## Effects

Each effect has its own skill, and does more the better you are at it. **Void** reverses every effect after it in its step; a second Void puts them back.

| Effect | What it does | Reversed |
| ------ | ------------ | -------- |
| Fire | Burns; dries the wet; lights a fireplace that has kindling | Cold: burns with cold |
| Water | Makes you wet (fire then does half the harm); puts out fires | Ice: chills (a third slower) and freezes water to walk on |
| Air | Knocks away from where the spell came from | Pulls towards it |
| Earth | Holds still for a few seconds | Stoneskin: less harm from blows |
| Identify | Finds out whether what someone wears is magical, blessed or cursed, for everyone; finds secret doors near where it lands | Conceal: hides what is worn again |
| Curse | Weakens a creature's blows | Lifts curses, on what you wear too |
| Illusion | Harder to hit | Ends illusions |
| Warding | Better at resisting magic | Dispel: ends every spell's effect, good and bad |
| Heal | Restores Health | Harm |
| Poison | Harm every second for a while | Cures poison |
| Teleport | Your next move goes at once to a cell you click: one you can see, in a clear line and not too far | Held in place: no teleporting for a while |
| Light | Glows, lighting the way; cast where there is nobody, the spot glows for a while | Darkness: no light of your own, a lantern's swallowed too |
| Haste | Faster | Slow |
| Calm | A creature stops being unfriendly | Fear: a creature flees from you |
| Blessing | Every attribute up | Weakness: every attribute down |
| Levitate | Float over holes and water | Brings floating things down |
| Mana | Gives Mana, never more than the spell cost | Drains Mana, giving you back what it took, up to what the spell cost |

**Fire and Water together** in one step make **steam**: it scalds (half again a fire's harm), puts fires out, and leaves a cloud for a few seconds that hides what is behind it.

What an effect leaves on you for a while shows in the bar at the top of the screen, with what it does and how long it has left. Calm and Fear only work on creatures, not on other players. When a Levitate ends over water or a hole, you can still jump out.

## Resisting

Harmful magic is resisted, so some of it always gets through and some is lost. Harm, cold, poison and drained Mana are resisted by Aura, with the Warding skill. Being held still, calmed and frightened are resisted by Will, with Warding; curses, slowing and weakening by Aura, with Warding. A Warding spell makes you better at both. Taking an effect away is never resisted.

Spells never kill you or your friends: harm from your own spells stops at 1 Health.

## Mana

Every spell has a cost in Mana, shown in the panel as you build it. An effect costs less the better its skill; a where costs more the further it reaches, less with its own skill. The steps' costs are **multiplied together**, so long spells soon cost a great deal: Heal costs 5 at first, a Healing Missile 10, and a missile that heals and then burns everything round its target 150. The whole cost is paid when the first step lands. A spell you can't afford can't be cast.

## Casting

A spell winds up before its first step; moving or Esc calls it off until then. After that the steps go on whatever you do.

Right-click a spell on the action bar (or hold a finger on it) to have it cast by itself: one that only helps, when its first target is down to a quarter of their Health; one that only harms, when an enemy you have targeted is in reach. A spell that does both, or neither, is cast by hand.

No spell reaches anyone else in [The Way Inn](the-way-inn).

## Your spellbook

The Spells tab (P) lists your spells, favourites first. Click the star to make one a favourite, Rename to give it your own name, and Remove to take it out of the book and back into the design panel to change it. A spell without a name of yours is named from its steps: Heal, Healing Missile, Ball of Fire. You can keep up to 100 spells.

## Skills

Magic's skills are in four branches: **Arcane** (Fire, Water, Air, Earth, Void, Teleport, Haste, Levitate, Mana), **Divine** (Healing, Warding, Blessing, Curse, Poison, Calm), **Insight** (Identify, Illusion, Light) and **Form** (the five wheres). An effect's skill grows only when it changes something: Health restored, harm done, a fire lit, something found out. A where's skill grows when an effect it carried did something.
