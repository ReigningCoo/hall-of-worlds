### English: an item's popup: what it gives (its bonuses) and what it weighs.

## Descriptions under an item's name, for kinds that have one:
## `about-` and the kind in kebab case.

about-bag = A plain cloth bag with a drawstring. Tied to a belt while empty, its inside adds to the end of your inventory.
about-cask = A wooden cask, bound with iron hoops. It holds one liquid, by the litre.
about-dwarven-bread = Looks and feels like a lump of stone. To a dwarf it tastes of home, and it is delicious.
about-flour = Grain stone-ground into wholemeal flour, in an open hessian sack.
about-ointment = Herbs ground with oil. Let go of it on yourself to rub it on: it does a quarter of what a potion of it would.
about-pouch = A small pouch. Tied to a belt while empty, its inside adds to the end of your inventory.

## Herbs: one whose name isn't known, and each of its four effects, known
## ("Slightly raises Toughness") or not ("?"). $way is raises or lowers,
## $strength 1 (slight) to 3 (great).

herb-unknown = Unknown herb
herb-effect = { $way ->
    [lowers] { $strength ->
        [1] Slightly lowers
        [3] Greatly lowers
       *[other] Lowers
    }
   *[raises] { $strength ->
        [1] Slightly raises
        [3] Greatly raises
       *[other] Raises
    }
} { $stat }
herb-effect-unknown = ?

## A bonus: the amount with its sign ("+40", "−8"), then what it changes.

item-bonus = { $amount } { $stat }
item-bonus-percent = { $amount }% { $stat }

## Health or Mana given back over time, unsigned ("1", "0.5").

item-restores = Restores { $amount } { $stat } a second

## Health or Mana taken away over time (poison), signed ("−1").

item-drains = { $amount } { $stat } a second

## Health or Mana given or taken at once (ointment), signed ("+5", "−5").

item-cask-holds = Holds { $litres } of { $most } litres
item-at-once = { $amount } { $stat } at once

## Ointment whose doses come to nothing on the skin (a quarter of each).

item-too-weak = Too weak to do anything on the skin

## What food that doesn't suit the eater's kind does to them, before its line.

item-poisons = Poisons other kinds: { $line }

## How long an eaten thing's bonus lasts, after its line.

item-lasts = { $line } for { $seconds } s

## A whole fruit's or vegetable's servings, which eating it cuts it into.

item-servings = Cut into { $count } servings when eaten

## A vegetable that heals nothing raw: one rarely or never eaten raw (for
## cooking, later), or grain (for milling, later).

item-not-raw = Heals nothing raw
item-unmilled = Heals nothing until milled

## What ends it early, after its line.

item-ends = { $ends ->
    [moving] { $line }; stops on moving
    [hurt] { $line }; stops on being hurt
    [both] { $line }; stops on moving or being hurt
   *[nothing] { $line }
}

## What a bonus changes: the attributes, the most Health and Mana, skills.

stat-strength = Strength
stat-agility = Agility
stat-toughness = Toughness
stat-intellect = Intellect
stat-perception = Perception
stat-will = Will
stat-presence = Presence
stat-empathy = Empathy
stat-aura = Aura
stat-most-health = most Health
stat-most-mana = most Mana
stat-health = Health
stat-mana = Mana
stat-skill = { $skill }

## What wearing it asks of the wearer: an attribute at least this much
## ("Needs 110 Strength"), and how well it works for the viewer when they fall
## short ("Working at 49%": nothing at half, all at the requirement).

item-needs = Needs { $min } { $stat }
item-working = Working at { $percent }%

## Who food is for, from creature names in the plural ("Dwarves only",
## "Orcs and goblins only").

item-kinds-only = { $kinds } only

## The size a worn thing was made in, when not the viewer's own, and
## sizes' names ($size: tiny, small, medium, large, huge, giant).

item-size = Size: { $size } (you: { $yours })
item-size-name = { $size ->
    [tiny] Tiny
    [small] Small
    [large] Large
    [huge] Huge
    [giant] Giant
   *[medium] Medium
}

## Why food refused to the player's kind isn't eaten, under the clothes:
## the kinds it is for in the plural ("dwarves").

item-refused = Only { $kinds } can eat that.

## What a key remembers: where it was first found ("Found in Ashford (1204,
## −388)") and the door it has locked or unlocked ("Has unlocked a door in
## Ashford (1204, −388)"). $place is one of the key-place- phrases; $x and $y
## are world map points, with a true minus sign.

key-found = Found { $place }
key-turned = Has { $did ->
    [locked] locked
    [both] locked and unlocked
   *[unlocked] unlocked
} a door { $place }
key-place-in = in { $place } ({ $x }, { $y })
key-place-at = at ({ $x }, { $y })
key-place-hall = in the Hall of Worlds
key-place-elsewhere = { $place }, on another planet
