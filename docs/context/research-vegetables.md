# Research: vegetables, pulses and grains

Research for vegetables, pulses and grains (2026-11-17), approved by the owner. Made like the fruit research, and its code follows the fruit's. The review page with every finding and quote: https://claude.ai/artifact/VPrGvJo77w4FUC1NqqdjXL. Scripts and data (outside the repo): `~/.claude/research-tools/vegetables/` (`data/final.json`).

## Method and sources

- 85 plants across the nine biomes of the climate (`climate::Biome`): 51 vegetables, 17 pulses and 17 grains; 17 are only gathered wild.
- Left out as the game already has them as herbs: garlic, ramsons, mustard, fennel, watercress, wasabi, caper, fenugreek, chilli (and so sweet pepper), roselle, ginger, turmeric, galangal, stinging nettle, plantain, fireweed, angelica, cumin, borage, marsh mallow. When cooking comes, herbs are added to dishes and some of their effects carry over, as in potions.
- Each plant's Wikipedia article, its nutrition table (63 of 85 had one) and taxobox, read by Claude (Opus) for 29 facts with word-for-word quotes; a script checked every quote against its article (all passed). 60% of values are sourced, the rest estimated; wild plants' calories and yields are mostly estimates.
- Hand choices (units, effects, yields where missing) in `veg_overrides.py`.

## Owner's decisions

- Keep all 85.
- Healing (as fruit): 1 HP a second for kcal / 4 seconds. Raw, only what people eat raw commonly or sometimes heals, at a quarter of that; rarely or never raw, nothing. The full figure is for cooking, later.
- Grains are picked as a sheaf (30 g of grain, about 100 kcal) and heal nothing until milled (later; threshing will probably be skipped, going straight to milling).
- Pea, broad bean, runner bean and winged bean are picked green (fresh, a 100 g handful); the other pulses dry (a 30 g handful). Peanut is a pulse, not a nut.
- Items: one root, bulb, head or fruit; leaves, shoots, small roots and fresh pods by the 100 g handful; big ones eaten a serving (about 100 kcal) at a time (pumpkin, cassava, swede; agave heart, yam and taro as pieces).
- Harm when raw is kept as a real effect. Allergies, favism, ergot, mould and dirty water are left out.
- Exceptional for health: kale, spinach, carrot, sweet potato, cabbage, mountain sorrel, lamb's quarters, lentil, soybean, oats; a bonus only when cooked (as fruit's 1.5 times).
- Wild: crops only farmed are very rare in the wild (escapes); wild-only plants are common to uncommon as herbs; overall, wild vegetable patches are 1.5 times as common as wild fruit.
- Growth and picking follow herbs (`Growth`, `Regrow`): speed from real days to harvest (Fast to 60 days, Medium 120, Slow 240, then Very slow); dig (gone until it regrows: roots, heads, all grain), pick (peas, tomatoes, okra), cut (chicory, purslane), coppice (once a year: asparagus, rhubarb, cattail).
- Farms: each field grows one crop chosen by climate from the cultivated ones (no wild-only plants by default); the generic crop picture gives way to the crop's models by stage. Pasture stays grass.
- Lore effects are kept for fun: spinach Str +1 (Popeye), lettuce Agi -1 (sleepy latex). Carrot's Per +1 applies raw too.
- Tomato, cucumber, pumpkin, aubergine, okra, chayote and bottle gourd are vegetables here, though botanical fruits.

## The plants

Raw / cooked: HP healed raw now, and the full figure for cooking later. Harvest: items a full harvest gives before skills. Effects marked (raw), (cooked) or (both).

### Vegetables

| Name | Latin | Biomes | Wild | Item | Raw / cooked HP | Growth | Picking | Harvest | Effects |
| ---- | ----- | ------ | ---- | ---- | --------------- | ------ | ------- | ------- | ------- |
| Turnip | *Brassica rapa subsp. rapa* | Boreal, Tundra, Seasonal forest | uncommon | one | 4 / 15 | Fast | dig | 1-3 | - |
| Swede | *Brassica napus var. napobrassica* | Boreal, Tundra | very rare | whole, 4 servings | 5 / 20 | Medium | dig | 1-3 | - |
| Kale | *Brassica oleracea var. sabellica* | Tundra, Boreal, Seasonal forest | very rare | handful (100 g) | 2 / 10 | Fast | pick | 1-3 | Tou +1 (both) |
| Cabbage | *Brassica oleracea var. capitata* | Boreal, Seasonal forest | very rare | one | 10 / 40 | Medium | dig | 1-3 | Tou +1 (both) |
| Radish | *Raphanus sativus* | Boreal, Seasonal forest | uncommon | handful (100 g) | 1 / 4 | Fast | dig | 1-2 | - |
| Mountain sorrel (wild only) | *Oxyria digyna* | Tundra | common | handful (100 g) | 1 / 5 | Fast | pick | 1 | Tou +1 (raw) |
| Alpine bistort (wild only) | *Bistorta vivipara* | Tundra | common | handful (100 g) | 5 / 21 | VerySlow | dig | 1 | - |
| Eskimo potato (wild only) | *Hedysarum alpinum* | Tundra, Boreal | common | one | 3 / 10 | VerySlow | coppice (once a year) | 1-2 | - |
| Fiddlehead (wild only) | *Matteuccia struthiopteris* | Boreal, Temperate rainforest | common | handful (100 g) | 0 / 8 | VerySlow | coppice (once a year) | 1 | Health -1 (raw) |
| Carrot | *Daucus carota subsp. sativus* | Seasonal forest, Woodland | uncommon | one | 2 / 9 | Medium | dig | 2-4 | Per +1 (both) |
| Parsnip | *Pastinaca sativa* | Seasonal forest | uncommon | one | 9 / 36 | Slow | dig | 1-3 | - |
| Beetroot | *Beta vulgaris* | Seasonal forest, Woodland | rare | one | 4 / 16 | Fast | dig | 1-3 | - |
| Onion | *Allium cepa* | Seasonal forest, Woodland, Desert | very rare | one | 4 / 15 | Medium | dig | 1-3 | Pre -1 (raw) |
| Leek | *Allium ampeloprasum* | Seasonal forest | rare | one | 6 / 25 | Slow | dig | 1-3 | - |
| Potato | *Solanum tuberosum* | Seasonal forest, Grassland | very rare | one | 0 / 36 | Medium | dig | 2-4 | Health -1 (raw) |
| Spinach | *Spinacia oleracea* | Seasonal forest | very rare | handful (100 g) | 1 / 5 | Fast | pick | 1-2 | Str +1 (both) |
| Rhubarb | *Rheum rhabarbarum* | Seasonal forest, Boreal | very rare | one | 1 / 5 | VerySlow | coppice (once a year) | 1-2 | Tou -1 (both) |
| Celery | *Apium graveolens* | Seasonal forest, Woodland | rare | one | 5 / 19 | Slow | dig | 1-3 | - |
| Skirret | *Sium sisarum* | Seasonal forest | rare | one cluster | 7 / 30 | Slow | dig | 1-3 | Tou -1 (both) |
| Burdock (wild only) | *Arctium lappa* | Seasonal forest, Grassland | common | one | 0 / 45 | Slow | dig | 1-3 | - |
| Wild carrot (wild only) | *Daucus carota* | Grassland, Seasonal forest, Woodland | common | handful (100 g) | 2 / 7 | Medium | dig | 1 | - |
| Cattail (wild only) | *Typha latifolia* | Seasonal forest, Grassland, Boreal | common | one | 4 / 16 | VerySlow | coppice (once a year) | 1-2 | - |
| Lamb's quarters (wild only) | *Chenopodium album* | Seasonal forest, Grassland, Woodland | common | handful (100 g) | 3 / 10 | Fast | pick | 1-3 | - |
| Dandelion (wild only) | *Taraxacum officinale* | Grassland, Seasonal forest | common | handful (100 g) | 3 / 11 | Medium | pick | 1-2 | Tou +1 (both) |
| Camas (wild only) | *Camassia quamash* | Temperate rainforest, Grassland | common | handful (100 g) | 0 / 25 | VerySlow | coppice (once a year) | 1-3 | Tou -1 (raw) |
| Wapato (wild only) | *Sagittaria latifolia* | Temperate rainforest, Seasonal forest | common | handful (100 g) | 6 / 22 | Slow | coppice (once a year) | 1-3 | - |
| Sea kale | *Crambe maritima* | Temperate rainforest, Seasonal forest | rare | one | 0 / 2 | VerySlow | coppice (once a year) | 1 | - |
| Jerusalem artichoke | *Helianthus tuberosus* | Grassland, Seasonal forest | uncommon | one | 2 / 9 | Slow | coppice (once a year) | 1-3 | Tou -1 (both) |
| Prairie turnip (wild only) | *Pediomelum esculentum* | Grassland | uncommon | one | 2 / 8 | VerySlow | dig | 1 | - |
| Pumpkin | *Cucurbita pepo* | Grassland, Seasonal forest, Woodland | rare | whole, 12 servings | 0 / 23 | Medium | dig | 1-3 | - |
| Tomato | *Solanum lycopersicum* | Grassland, Woodland | very rare | one | 2 / 7 | Medium | pick | 1-3 | - |
| Lettuce | *Lactuca sativa* | Woodland, Seasonal forest | very rare | one | 4 / 14 | Medium | dig | 1-2 | Agi -1 (both) |
| Globe artichoke | *Cynara cardunculus var. scolymus* | Woodland | very rare | one | 3 / 10 | Slow | coppice (once a year) | 1 | - |
| Asparagus | *Asparagus officinalis* | Woodland, Seasonal forest | rare | handful (100 g) | 1 / 4 | VerySlow | coppice (once a year) | 1 | Pre -1 (both) |
| Chicory | *Cichorium intybus* | Woodland, Grassland | uncommon | handful (100 g) | 1 / 5 | Medium | cut | 1-3 | - |
| Cucumber | *Cucumis sativus* | Woodland, Savanna | rare | one | 3 / 12 | Fast | pick | 1-2 | - |
| Purslane (wild only) | *Portulaca oleracea* | Woodland, Desert, Savanna | common | handful (100 g) | 1 / 4 | Fast | cut | 1-2 | - |
| Salsify | *Tragopogon porrifolius* | Woodland, Grassland | uncommon | one | 3 / 13 | Slow | dig | 1-3 | - |
| Agave heart (wild only) | *Agave americana* | Desert | common | piece, 5 servings | 0 / 25 | VerySlow | dig | 2-4 | Health -2 (raw) |
| Desert onion (wild only) | *Allium macropetalum* | Desert | uncommon | handful (100 g) | 2 / 8 | VerySlow | dig | 1 | Pre -1 (raw) |
| Okra | *Abelmoschus esculentus* | Savanna, Desert, Tropical rainforest | very rare | handful (100 g) | 2 / 7 | Fast | pick | 1-3 | - |
| Aubergine | *Solanum melongena* | Savanna, Tropical rainforest, Woodland | rare | one | 0 / 19 | Slow | pick | 1-3 | - |
| Bottle gourd | *Lagenaria siceraria* | Savanna, Tropical rainforest | rare | one | 0 / 40 | Fast | pick | 1-2 | - |
| Cassava | *Manihot esculenta* | Tropical rainforest, Savanna | rare | whole, 13 servings | 0 / 23 | VerySlow | dig | 2-4 | Health -3 (raw) |
| Taro | *Colocasia esculenta* | Tropical rainforest | uncommon | one | 0 / 69 | VerySlow | dig | 1-3 | Health -2 (raw) |
| Yam | *Dioscorea alata* | Tropical rainforest, Savanna | rare | piece, 4 servings | 0 / 19 | VerySlow | dig | 2-4 | Health -1 (raw) |
| Sweet potato | *Ipomoea batatas* | Tropical rainforest, Savanna | very rare | one | 0 / 47 | Slow | dig | 2-4 | Per +1 (both) |
| Arrowroot | *Maranta arundinacea* | Tropical rainforest | rare | one | 0 / 28 | VerySlow | dig | 1-3 | Tou +1 (cooked) |
| Chayote | *Sechium edule* | Tropical rainforest | rare | one | 4 / 16 | Slow | pick | 1-3 | - |
| Bamboo shoot | *Phyllostachys edulis* | Tropical rainforest | uncommon | one | 0 / 42 | VerySlow | coppice (once a year) | 1 | Health -2 (raw) |
| Water spinach | *Ipomoea aquatica* | Tropical rainforest | uncommon | handful (100 g) | 1 / 4 | Fast | cut | 1-3 | - |

### Pulses

| Name | Latin | Biomes | Wild | Item | Raw / cooked HP | Growth | Picking | Harvest | Effects |
| ---- | ----- | ------ | ---- | ---- | --------------- | ------ | ------- | ------- | ------- |
| Pea | *Pisum sativum* | Seasonal forest, Boreal, Woodland | rare | handful (100 g) | 2 / 9 | Medium | pick | 1-2 | - |
| Broad bean | *Vicia faba* | Seasonal forest, Woodland | very rare | handful (100 g) | 2 / 8 | Medium | pick | 1-2 | Health -1 (raw) |
| Lentil | *Lens culinaris* | Woodland, Grassland | very rare | handful (30 g) | 0 / 26 | Medium | dig | 1-2 | - |
| Chickpea | *Cicer arietinum* | Woodland, Grassland, Desert | very rare | handful (30 g) | 0 / 27 | Medium | dig | 1-2 | - |
| Kidney bean | *Phaseolus vulgaris* | Grassland, Seasonal forest | rare | handful (30 g) | 0 / 25 | Medium | dig | 1-2 | Health -2 (raw) |
| Runner bean | *Phaseolus coccineus* | Seasonal forest, Temperate rainforest | rare | handful (100 g) | 0 / 7 | Medium | pick | 1-3 | Health -2 (raw) |
| Grass pea | *Lathyrus sativus* | Woodland, Grassland | rare | handful (30 g) | 0 / 26 | Medium | dig | 1-2 | Agi -1 (both) |
| Lupin bean | *Lupinus albus* | Woodland | rare | handful (30 g) | 0 / 28 | Slow | dig | 1-3 | Health -2 (raw) |
| Tepary bean | *Phaseolus acutifolius* | Desert | uncommon | handful (30 g) | 0 / 26 | Medium | dig | 1 | Health -1 (raw) |
| Mesquite pod (wild only) | *Prosopis glandulosa* | Desert, Savanna | common | handful (30 g) | 7 / 26 | VerySlow | coppice (once a year) | 1-3 | - |
| Cowpea | *Vigna unguiculata* | Savanna, Desert | uncommon | handful (30 g) | 0 / 25 | Medium | pick | 1 | - |
| Peanut | *Arachis hypogaea* | Savanna, Tropical rainforest, Grassland | very rare | handful (30 g) | 11 / 43 | Medium | dig | 1-3 | - |
| Bambara groundnut | *Vigna subterranea* | Savanna | rare | handful (30 g) | 0 / 28 | Slow | dig | 1-2 | - |
| Lima bean | *Phaseolus lunatus* | Tropical rainforest, Savanna | rare | handful (30 g) | 0 / 25 | Medium | pick | 1-3 | Health -2 (raw) |
| Soybean | *Glycine max* | Grassland, Seasonal forest | very rare | handful (30 g) | 0 / 33 | Medium | dig | 1-3 | Health -1 (raw) |
| Mung bean | *Vigna radiata* | Tropical rainforest, Savanna | rare | handful (30 g) | 0 / 26 | Medium | dig | 1 | - |
| Winged bean | *Psophocarpus tetragonolobus* | Tropical rainforest | rare | handful (100 g) | 3 / 12 | Medium | pick | 1-3 | - |

### Grains

| Name | Latin | Biomes | Wild | Item | Raw / cooked HP | Growth | Picking | Harvest | Effects |
| ---- | ----- | ------ | ---- | ---- | --------------- | ------ | ------- | ------- | ------- |
| Wheat | *Triticum aestivum* | Grassland, Woodland, Seasonal forest | very rare | sheaf (30 g) | 0 / 25 | Medium | dig | 2-4 | - |
| Barley | *Hordeum vulgare* | Grassland, Boreal, Seasonal forest | rare | sheaf (30 g) | 0 / 26 | Medium | dig | 1-2 | Tou +1 (cooked) |
| Oats | *Avena sativa* | Seasonal forest, Boreal, Temperate rainforest | uncommon | sheaf (30 g) | 0 / 29 | Medium | dig | 1-3 | Tou +1 (cooked) |
| Rye | *Secale cereale* | Boreal, Seasonal forest | rare | sheaf (30 g) | 0 / 25 | VerySlow | dig | 1-3 | - |
| Spelt | *Triticum spelta* | Seasonal forest, Woodland | very rare | sheaf (30 g) | 0 / 25 | VerySlow | dig | 1-3 | - |
| Buckwheat | *Fagopyrum esculentum* | Boreal, Grassland | rare | sheaf (30 g) | 0 / 26 | VerySlow | dig | 1-2 | - |
| Wild rice (wild only) | *Zizania palustris* | Boreal, Seasonal forest | common | sheaf (30 g) | 0 / 27 | Medium | dig | 1 | - |
| Proso millet | *Panicum miliaceum* | Grassland, Desert | uncommon | sheaf (30 g) | 0 / 29 | Medium | dig | 1-3 | - |
| Maize | *Zea mays* | Grassland, Seasonal forest, Savanna | very rare | sheaf (30 g) | 0 / 27 | Medium | dig | 1-3 | - |
| Sorghum | *Sorghum bicolor* | Savanna, Desert, Grassland | uncommon | sheaf (30 g) | 0 / 25 | Slow | dig | 1-3 | - |
| Pearl millet | *Cenchrus americanus* | Savanna, Desert | rare | sheaf (30 g) | 0 / 27 | Medium | dig | 1 | - |
| Finger millet | *Eleusine coracana* | Savanna | very rare | sheaf (30 g) | 0 / 23 | Medium | dig | 1-2 | - |
| Teff | *Eragrostis tef* | Savanna, Grassland | very rare | sheaf (30 g) | 0 / 28 | Medium | dig | 1-3 | - |
| Fonio | *Digitaria exilis* | Savanna | uncommon | sheaf (30 g) | 0 / 26 | Fast | dig | 1-2 | - |
| Rice | *Oryza sativa* | Tropical rainforest, Savanna | rare | sheaf (30 g) | 0 / 27 | Slow | dig | 1-3 | - |
| Quinoa | *Chenopodium quinoa* | Grassland, Tundra | rare | sheaf (30 g) | 0 / 28 | Slow | dig | 1-3 | - |
| Amaranth | *Amaranthus cruentus* | Grassland, Savanna | rare | sheaf (30 g) | 0 / 28 | Medium | dig | 1-2 | - |
