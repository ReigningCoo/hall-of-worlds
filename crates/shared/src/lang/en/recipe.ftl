# The recipe reading panel (recipes plan PR 4): a recipe written out as
# steps, in the order they are worked (column by column, the top row
# first). Foods, liquids and tools are kinds of item in the text, drawn as
# pictures: $item, $tool and the list $foods hold them.

# A food put in: how many, and which.
recipe-food = { $count } { $item }
# A liquid put in, by the millilitre (milk, honey, soy sauce, water).
recipe-liquid = { $ml } ml of { $item }

# What a step starts from ($start): the first step of a row with nothing
# before it (first), what its row has made so far (row), both rows brought
# together (join), or the kind of intermediate the row starts from (source,
# named in $source). $added says whether foods ($foods) are put in too.
recipe-start = { $start ->
    [row] Take what you have so far
    [join] Bring both rows together
    [source] Take your { $source }
   *[first] Take { $foods }
}
recipe-added = { $added ->
    [yes] , add { $foods }
   *[no] {""}
}
# What is done with the tool ($tool-kind), shown as $tool.
recipe-use = { $tool-kind ->
    [knife] cut it with a { $tool }
    [pan] fry it in a { $tool }
    [cauldron] boil it in a { $tool }
    [bowl] mix it in a { $tool }
    [oven] bake it in an { $tool }
    [sieve] strain it through a { $tool }
   *[mortar] pound it in a { $tool }
}
# One whole step, numbered.
recipe-step = { $number }. { recipe-start }{ recipe-added }, and { recipe-use }.

# The last line: what it all makes.
recipe-makes = That makes { $dish }.

# Under a recipe's own name: where the dish is from, or that the reader
# wrote it down themselves by cooking it.
recipe-cuisine = A dish from { $cuisine } cooking
recipe-own = Your recipe

# When the recipe has more steps than the reader's Cooking skill allows:
# how many more ($over), and the chance that leaves ($chance, a whole
# percentage).
recipe-beyond = { $over ->
    [one] One step beyond your Cooking skill: a { $chance }% chance.
   *[other] { $over } steps beyond your Cooking skill: a { $chance }% chance.
}
