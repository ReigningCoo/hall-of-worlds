# Drinks: research notes

A short pass (2026-11-18) for the drinks and fermentation plan (`drinks.md`): casks, woods, drinks and how long they take, and vessels. From general reference knowledge (Wikipedia's articles on English brewery cask units, barrels, cooperage, fermentation, ale, cider, perry, wine and drinking vessels), checked for consistency, not a sourced extraction like the item research (`item-research-method` memory). Figures marked **est.** are Claude's estimates; the game rounds everything anyway.

## Casks

English ale and beer casks, from the imperial gallon (4.546 L). The game rounds each to a doubling-ish series in whole litres (owner):

| Cask | Gallons | Real litres | Game litres | Notes |
| ---- | ------- | ----------- | ----------- | ----- |
| Pin | 4.5 | 20.5 | 20 | Half a firkin |
| Firkin | 9 | 40.9 | 40 | The usual pub cask of real ale |
| Kilderkin | 18 | 81.8 | 80 | Two firkins |
| Barrel | 36 | 163.7 | 160 | Two kilderkins |
| Hogshead | 54 | 245.5 | 240 | A barrel and a half |
| Puncheon | 72 | 327 | 320 | Varies most by trade and region (wine puncheons 300 to 500 L) |
| Butt | 108 | 491 | 480 | Two hogsheads; also sherry's cask |
| Tun | 216 | 982 | 960 | Two butts; a measure as much as a cask |

**Empty weights (est.):** a 225 L Bordeaux barrique weighs about 45 to 50 kg empty and a 200 L spirit barrel about 50 to 55 kg; a wooden firkin about 12 to 18 kg. Scaling roughly with surface area gives the game's figures for oak: Pin 8, Firkin 14, Kilderkin 24, Barrel 45, Hogshead 60, Puncheon 80, Butt 105, Tun 150 kg (owner's range; a real tun would be heavier).

**Shape:** a cask's belly is wider than its heads (the bilge); old gaugers worked out its volume as π h (2R² + r²) / 3, R the belly's radius and r the heads'. Heads are commonly about 80 to 85% of the belly's width (**est.**). Small casks are slimmer for their height than large ones, which the game's cells follow.

## Woods

| Wood | Density (seasoned, est.) | In casks |
| ---- | ------------------------ | -------- |
| Oak (European and American white oak) | ~0.70 to 0.75 g/cm³ | The standard: tight grain, watertight; gives tannin and vanilla notes and lets a little air through, so wine and spirits mature in it |
| Chestnut | ~0.55 to 0.6 | Long used in Europe for wine and vinegar (balsamic sets); more porous, little flavour |
| Acacia (black locust) | ~0.75 to 0.8 | Used for white wines; neutral, keeps fruit and colour |
| Cherry | ~0.6 | Used in sets of casks for balsamic vinegar; gives a sweet, fruity note |
| Pine | ~0.5 | Resinous: taints what is kept in it (retsina is flavoured with pine resin on purpose); poor for keeping drinks |

The game weighs a cask by its wood's density against oak's (`Material::density`).

## Drinks and real durations

Fermentation times at cellar or room temperature; ageing afterwards is open-ended.

| Drink | Primary fermentation | Then | Strength | Notes |
| ----- | -------------------- | ---- | -------- | ----- |
| Ale | 3 to 7 days | 1 to 3 weeks conditioning in cask | 3 to 7% | Ale yeast; drunk young, spoils within weeks once opened |
| Cider (apples) | 1 to 3 weeks | Months maturing | 5 to 8% | Wild yeast on the fruit can start it alone |
| Perry (pears) | 1 to 3 weeks | Months | 5 to 8% | As cider; pears have unfermentable sorbitol, so perry stays sweeter |
| Wine (grapes) | 1 to 2 weeks | Months to years | 11 to 14% | Ages best in oak |
| Mead (honey) | Weeks to months | Months | 8 to 18% | Slow, as honey is poor in nutrients |

**Alcohol from sugar:** fermenting about 17 g of sugar per litre makes about 1% alcohol by volume; a gram of sugar is 4 kcal, so about 68 kcal per litre makes 1% (the plan's "kcal per litre ÷ 68"). Yeasts stop at their tolerance: wild and bread yeasts around 5 to 8%, ale yeasts 8 to 12%, wine yeasts 14 to 18%.

The game's durations (owner): 1 real hour is a game day; ale about 24 game days (a real day), cider and perry about 1.5 real days, wine about 4.

## Vessels

| Vessel | Usual size | Notes |
| ------ | ---------- | ----- |
| Tankard | ~0.5 L (a pint, 568 ml) | Pewter, wood or leather, often lidded |
| Cup | ~0.25 L | Wood, horn or pottery |
| Goblet | ~0.2 L | Stemmed; silver, pewter or glass |
| Drinking horn | ~0.3 to 0.5 L | Cattle horn; can't be set down full |
| Wine bottle | 0.75 L | Glass, corked |
| Jug | ~1 to 2 L | Pottery, for pouring |
| Flask | ~0.25 L | Flat, carried on the body |
| Waterskin | ~1 to 2 L | Leather, carried |

**Alcohol units:** a UK unit is 10 ml (8 g) of pure alcohol; the body clears about one an hour (the game's 1 unit per 10 real minutes at 70 kg is that, sped up). A pint of 4% ale is about 2.3 units; a 175 ml glass of 12% wine about 2.1.
