### English: the bar of timed effects at the top of the window, and an
### effect's popup there.

## An effect's name: what was eaten (its kind's name), poison, the debug
## menu's bonus, a herb eaten raw.

effect-eaten = Eating: { $item }
effect-poison = Poisoned
effect-dev = Dev Bonus
effect-raw-herb = Raw herb

## What ends it early, if anything does.

effect-ends = { $ends ->
    [moving] Ends if you move
    [hurt] Ends if you are hurt
    [both] Ends if you move or are hurt
   *[nothing] Nothing ends it early
}

## How long it has left, in whole minutes and seconds, counting down.

effect-ends-in = { $minutes ->
    [0] Ends in { $seconds } s
   *[other] Ends in { $minutes } min { $seconds } s
}
