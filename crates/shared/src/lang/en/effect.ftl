### English: the bar of timed effects at the top of the window, and an
### effect's popup there.

## An effect's name: what was eaten (its kind's, fruit's or vegetable's name),
## a fruit's or vegetable's effects (its name), poison, the debug menu's
## bonus, a herb eaten raw, ointment, a potion's dose on one stat (its name),
## juice or cordial drunk (its fruit's name), a fermented drink's Mana, and
## the stage of drinking (`alcohol::Stage`).

effect-eaten = Eating: { $item }
effect-fruit = { $fruit }
effect-poison = Poisoned
effect-dev = Dev Bonus
effect-raw-herb = Raw herb
effect-ointment = Ointment
effect-potion = Potion: { $stat }
effect-juice = { $fruit } Juice
effect-cordial = { $fruit } Cordial
effect-drink = Drink
effect-alcohol = { $stage ->
    [tipsy] Tipsy
    [drunk] Drunk
    [very-drunk] Very drunk
    [collapsed] Collapsed
    [poisoned] Alcohol poisoning
   *[hangover] Hangover
}

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
