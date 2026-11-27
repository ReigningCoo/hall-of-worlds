### English: an item's popup: what it gives (its bonuses) and what it weighs.

## Descriptions under an item's name, for kinds that have one:
## `about-` and the kind in kebab case.

about-bag = A plain cloth bag with a drawstring. Tied to a belt while empty, its inside adds to the end of your inventory.
about-cask = A wooden cask, bound with iron hoops. It holds one liquid, by the litre. Drawing from it needs a tap.
about-crucible = A clay crucible that takes the fire: Alchemy's apparatus for calcining whatever came before into a Powder, three quarters as strong. Carrying one teaches Calcining.
about-dwarven-bread = Looks and feels like a lump of stone. To a dwarf it tastes of home, and it is delicious.
about-pelican = A glass flask whose two curved arms bend back into its belly: Alchemy's apparatus for circulating what came before, its vapours running back down again and again. It changes nothing in what it holds but leaves it a fifth purer, which can lift it to the next quality. Carrying one teaches Circulating.
about-filter = A glass funnel lined with paper: Alchemy's apparatus for filtering what came before through a Powder. What both hold adds together; the rest is taken out. Carrying one teaches Filtering.
about-egg = A whole egg in its shell, kept for cooking.
about-dish = Cooked food, a serving at a time. Eat it for Well Fed.
about-intermediate = Part-made food, to finish later in a new row. Some change as they rest: warmth hurries them, cold slows them.
about-recipe = A rolled scroll of a dish's recipe, to read into your recipe book. Reading it uses it up.
about-flour = Grain stone-ground into wholemeal flour, in an open hessian sack.
about-ointment = Herbs ground with oil. Let go of it on yourself to rub it on: it does a quarter of what a potion of it would.
about-paste = Herbs ground dry in a mortar. It does nothing by itself: it is a start for Alchemy, mixed with an oil or a water in a retort.
about-pouch = A small pouch. Tied to a belt while empty, its inside adds to the end of your inventory.
about-powder = Calcined in a crucible. It does nothing by itself: it is a start for Alchemy, or put in a filter. Unlike a paste, it never goes stale.
about-retort = A glass retort on a ring stand: Alchemy's apparatus for infusing liquids into what came before. Carrying one teaches Infusing.
about-tap = A brass tap, knocked into a cask to draw from it. Carry it, and let go of a cask on a vessel or bottle to pour.
about-yeast = A crock of live yeast. Let go of flour on it to feed it, with fresh water carried. Fed every half day or so it keeps, and fed on time it can get better; fed sooner, it splits in two, with an empty bottle for the second. Left unfed, it weakens a step each day, down to Poor. Between feeds it can start one thing without being used up.
about-vessel = For drinking from or carrying a drink. Let go of it on another vessel, a bottle or a cask to pour; on yourself to drink.

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

## Fungi: one whose name isn't known (never wrong information), or that looks
## like $fungus, a fungus whose name is known.

fungus-unknown = Unknown mushroom
lichen-unknown = Unknown lichen
fungus-looks-like = Looks like { $fungus }

## A known fungus's poison: when it starts ($onset soon, delayed or late), a
## line for each effect ($line), deadly, a false recovery; the common ink cap's
## window after eating and what alcohol drunk in it brings.

fungus-onset = { $onset ->
    [soon] Poisonous: at once
    [late] Poisonous: an hour or more after eating
   *[delayed] Poisonous: about 10 minutes after eating
}
fungus-poison = Poison: { $line }
fungus-deadly = Deadly unless healed
fungus-false-recovery = Seems to wear off, then comes back worse
fungus-window = For 3 days after eating: { $line }
fungus-with-alcohol = Alcohol within 3 days brings: { $line }

## A known fungus whose poison comes from handling it: by touch ($guard hands)
## or by breathing its spores ($guard mouth), and when it starts ($onset soon,
## delayed or late); then what keeps it off.

fungus-handled = { $guard ->
    [mouth] Its spores harm when breathed
   *[hands] Harms bare hands that touch it
}{ $onset ->
    [soon] {""}
    [late] , an hour or more later
   *[delayed] , about 10 minutes later
}
fungus-guard = { $guard ->
    [mouth] Cover the mouth
   *[hands] Gloves protect
}

## A bonus: the amount with its sign ("+40", "−8"), then what it changes.

item-bonus = { $amount } { $stat }
item-bonus-percent = { $amount }% { $stat }

## Health or Mana given back over time, unsigned ("1", "0.5").

item-restores = Restores { $amount } { $stat } a second

## Health or Mana taken away over time (poison), signed ("−1").

item-drains = { $amount } { $stat } a second
item-drains-deadly = { $amount } { $stat } a second, deadly

## Health or Mana given or taken at once (ointment), signed ("+5", "−5").

## A yeast's clock, one line each in a stack: hours to the next feed, or
## (once a feed is on time) hours until it weakens; $poor is yes when it
## can't weaken any more. "used since fed" when it has started something.

yeast-feed-in = Feed in { $hours } h
yeast-feed-now = { $poor ->
    [yes] Feed now
   *[no] Feed now: weakens in { $hours } h
}
yeast-used = used since fed
## A starter begun from flour and water, hours until it is ready.
yeast-starting = Starter: ready in { $hours } h
yeast-more = and { $count } more

## Cooked food (cooking): its biggest foods, its diet labels, and how an
## intermediate is resting, in hours of rest at its usual pace.

food-made-with = Made with { $foods }
food-diet = { $labels }
food-turns = { $into } in { $hours } h
food-wild = setting wild
food-cultured = with a culture
food-proving = Proving: at its best in { $hours } h
food-proved = At its best for { $hours } h more
food-resting = Over-proving in { $hours } h
food-over-proved = Over-proved
food-ageing = Ageing: { $steps ->
    [0] better
   *[other] { $steps } steps better, more
  } in { $hours } h
food-aged = Fully aged

item-holds = Holds { $amount } of { $most }
item-sealed = sealed
item-opened = opened
item-sealed-bottle = Sealed

## A drink in a cask (drinks plan PR 6): its strength ("5.8%"), sweetness
## ("dry", "off-dry", "medium", "sweet") and quality; game days until it is
## ready, until an opened one turns to vinegar, or until opened juice
## ferments by itself.

item-drink = { $strength }, { $sweetness }, { $quality }
item-fermenting = { $days ->
    [one] Fermenting: ready in a day
   *[other] Fermenting: ready in { $days } days
}
item-turns-vinegar = { $days ->
    [one] Turns to vinegar in a day
   *[other] Turns to vinegar in { $days } days
}
item-turns-wild = { $days ->
    [one] Ferments by itself in a day
   *[other] Ferments by itself in { $days } days
}
item-at-once = { $amount } { $stat } at once

## Ointment whose doses come to nothing on the skin (a quarter of each).

item-too-weak = Too weak to do anything on the skin

## What food that doesn't suit the eater's kind does to them, before its line.

item-poisons = Poisons other kinds: { $line }

## How long an eaten thing's bonus lasts, after its line.

item-lasts = { $line } for { $seconds } s

## A whole fruit's, vegetable's or fungus's servings, which eating it cuts it
## into.

item-servings = Cut into { $count } servings when eaten

## A vegetable or fungus that heals nothing raw: one rarely or never eaten raw
## (for cooking, later), or grain (for milling, later).

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
stat-darkvision = cells of Darkvision
stat-defence = Defence
stat-blow-harm = harm from your blows
stat-harm-taken = harm from blows taken
stat-resistance = resistance to magic
stat-speed = speed
stat-light = cells of light
stat-skill = { $skill }

## A quality bonus raising a skill: signed levels, each one percentage point
## ("+1 level in Sword", "+2 levels in Melee").

item-skill-levels = { $amount } { $count ->
    [one] level
   *[other] levels
} in { $skill }

## A worn or held thing's bonus with what the stat does: "+5% Strength:
## carrying and how hard you hit".

item-bonus-does = { $line }: { $does }

## A set's bonuses in an item's popup come under how many of its different
## pieces must be worn for them: "2 worn:".

item-set-worn = { $count } worn:

## What each stat does, after a worn or held thing's bonus: worded to suit
## a bonus either way, as some are negative.

does-strength = carrying and how hard you hit
does-agility = how quick your attacks are, and defending
does-toughness = your Health
does-intellect = reasoning, learning and arcane magic
does-perception = noticing things, and defending
does-will = your Mana
does-presence = commanding and persuading
does-empathy = reading others, and divine magic
does-aura = standing up to magic
does-most-health = how much Health you can have
does-most-mana = how much Mana you can have
does-restore-health = Health coming back while it refills
does-restore-mana = Mana coming back while it refills
does-darkvision = seeing in the dark
does-defence = how well you defend
does-blow-harm = how much your blows hurt
does-harm-taken = how much blows hurt you
does-resistance = how much harmful magic does to you
does-speed = how fast you act and move
does-light = how far your light reaches
does-skill = how well you fight

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

## A recipe scroll (recipes plan PR 3): whether the reader's recipe book has
## it, or how to read it in.

recipe-known = Already in your recipe book
recipe-drop-on-self = Drop on yourself to add it to your recipe book
