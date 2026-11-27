# A spell's tip (shared::spell::tip_lines): its cost, each step with its
# numbers for the caster now, before the target resists. The parts' nouns are
# spell.ftl's `spell-part-…`.

spell-tip-cost = Costs { $mana } Mana
spell-tip-step = { $step }. { $what }
spell-tip-then = , then{" "}
spell-tip-effects = { $step ->
    [one] 1. On you:
   *[other] { $step }. Then:
}
spell-tip-filters = Filters: { $filters }
spell-tip-filter-self = Self
spell-tip-filter-friend = Friend
spell-tip-filter-enemy = Enemy
spell-tip-note = Numbers are yours now, before the target resists.

## A where, with how many times it is stacked.

spell-tip-times = { $times ->
    [one] {""}
   *[other] {" "}×{ $times }
}
spell-tip-touch = Touch{ $suffix }: within arm's reach, a { $degrees }° arc
spell-tip-missile = Missile{ $suffix }: up to { $cells } cells, { $speed } cells a second, the first in its way
spell-tip-surround = Surround{ $suffix }: everyone within { $cells ->
    [one] a cell
   *[other] { $cells } cells
} round
spell-tip-beam = Beam{ $suffix }: { $beams ->
    [one] a beam
   *[other] { $beams } beams { $degrees }° apart
} of { $cells } cells
spell-tip-point = Point{ $suffix }: { $creatures ->
    [one] one creature
   *[other] { $creatures } creatures
} within { $cells } cells

## An outcome of an effect step and what it does.

spell-tip-outcome = { $noun }{ $suffix }: { $facts }
spell-tip-outcome-bare = { $noun }{ $suffix }

spell-tip-harms = { $amount } harm
spell-tip-heals = heals { $amount } Health
spell-tip-dries = dries the wet, who take half
spell-tip-lights = lights fireplaces
spell-tip-douses = puts out fires
spell-tip-wets = wet for { $seconds } s
spell-tip-chills = { $percent }% speed for { $seconds } s if it takes hold
spell-tip-freezes = freezes water within { $cells ->
    [one] a cell
   *[other] { $cells } cells
} for { $seconds } s
spell-tip-clouds = a cloud of steam within { $cells ->
    [one] a cell
   *[other] { $cells } cells
} for { $seconds } s
spell-tip-knocks = knocks back { $cells } { $plural ->
    [one] cell
   *[other] cells
}
spell-tip-pulls = pulls in { $cells } { $plural ->
    [one] cell
   *[other] cells
}
spell-tip-poisons = { $amount } harm a second for { $seconds } s
spell-tip-lasts = { $part ->
    [earth] held still for { $seconds } s if it takes hold
    [stoneskin] { $amount }% less harm taken for { $seconds } s
    [curse] blows { $amount }% weaker for { $seconds } s if it takes hold
    [illusion] { $amount }% more Defence for { $seconds } s
    [warding] { $amount }% more resistance to magic for { $seconds } s
    [haste] { $amount }% faster for { $seconds } s
    [slow] { $amount }% slower for { $seconds } s
    [blessing] { $amount }% more to every attribute for { $seconds } s
    [weakness] { $amount }% less to every attribute for { $seconds } s
    [light] light reaching { $amount } cells for { $seconds } s
    [darkness] no light for { $seconds } s
    [calm] calmed for { $seconds } s
    [fear] frightened for { $seconds } s
    [levitate] floating for { $seconds } s
    [teleport] may teleport up to { $range } cells for { $seconds } s
    [anchor] can't be teleported for { $seconds } s
   *[other] for { $seconds } s
}
spell-tip-ends = { $ends ->
    [poison] cures poison
    [curse] lifts curses
    [illusion] ends illusions
    [spells] ends every spell's effect
    [levitation] brings floating things down
    [light] ends a Light
   *[darkness] ends Darkness
}
spell-tip-identifies = reveals what is secret about what is worn, and secret doors
spell-tip-conceals = hides what is secret again
spell-tip-gives-mana = up to { $amount } Mana, shared out
spell-tip-drains-mana = drains { $amount } Mana
