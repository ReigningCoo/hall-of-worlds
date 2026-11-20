# Research: mushrooms, fungi and lichens

Research for fungi (2026-11-19), approved by the owner; the plan built from it is `fungi.md`, and its code follows the vegetables' (`shared::vegetable`). The review page with every finding and quote, and the owner's answers: https://claude.ai/artifact/1gGUDdPgtpYfDZ4QcM5BuA. Scripts and data (outside the repo): `~/.claude/research-tools/fungi/` (`items.py` the candidates with their Latin names, articles, groups and biomes; `final_data.py` the game values per kind; `data/collated.json` the findings with checked quotes).

## Method and sources

- The owner's brief: mostly underground, very few above ground unless the biome calls for it; mainly food, noting which can be used like a herb, and poisons. Confusion-type effects are fine (fly agaric, panther cap); psilocybin mushrooms are left out as recreational; lichens are in; yeast and ergot are left out. Some invented cave kinds fill out the underground.
- 100 real candidates, each with its Wikipedia article, read by Claude (Opus) for its facts with word-for-word quotes (`extract.py`); a script checked every quote against its article, nutrition box and taxobox (`collate.py`). Names were chosen by Google Ngram counts of their spellings (`ngram.py`). Manna lichen has no article: drawn from general knowledge.
- Claude chose the game values from the findings (`final_data.py`: where, niches, temperature, abundance, unit, kcal, raw, food, herb, poison, growth, regrow, glow, lookalikes, look), and the owner approved them. Corn smut was dropped afterwards, leaving 99 real and 12 invented: 111.
- `fungi.ron` was generated once from those files (the script stayed outside the repo), with each kind's form, colours and size chosen by hand from its description; edit it by hand from now on.

## Owner's decisions

The full list is in `fungi.md`. In short:

- Keep all 111, the 12 invented ones as named.
- Wood, dung and compost feeders grow underground; truffles and buried lumps are dug; fungi on living roots or bark, and every lichen, grow above ground only; a few grow underground only near daylight.
- A cave is at its land's yearly average temperature; rarer kinds likelier deeper, the Dreamcap only on the deepest level; basements and cellars grow the underground wood and soil kinds. Niches place them on a level; wood feeders also come up on felled trees above ground.
- Above ground a quarter as common as herbs (lichens common in Tundra and Boreal forest); underground a patch on about 1 cave floor cell in 60.
- Items: their own table, counted, no quality; one, a handful (100 g) or a large piece eaten a serving (about 200 g) at a time. Healing as vegetables: raw only for what is eaten raw commonly or sometimes, at a quarter.
- 36 kinds used like herbs (four effects each, in Alchemy alongside herbs, deadly ones too).
- Poisons are real effects when eaten (raw only where marked), with onsets soon, delayed (about 10 game minutes) or late (a game hour or more, with a false recovery for the death cap's poison), hidden until recognised; deadly kills unless healed, halved on Easy.
- Confusion from fly agaric, panther cap and the Dreamcap (Intellect, Perception, Agility down, stumbling).
- Never wrong information: unknown fungi show as "Unknown mushroom" or "Looks like X"; a Mycology skill teaches names by a check on picking.
- Common ink cap: alcohol within 3 days brings sickness, and it gives a bonus for those days.
- Glowing patches light their surroundings, each kind in its own colour.
- Handling and breathing harm, blocked by gloves and by covering the mouth (a new bandana).

## Game values

Where: deep (anywhere underground), dug (buried in cave earth, and under its trees above), dim (underground near daylight), surface (above ground only). °C: the yearly average temperatures it grows in. Raw / cooked HP: Health healed eating a serving raw now, and the full figure for cooking later. Food and herb effects as in `fungi.ron`; raw marks an effect or poison only raw. Poisons: severity, the real onset, then the effects; `handling` marks harm from touching or breathing (PR 9), `old` from old ones.

### Underground (deep) (34)

| Name | Latin | Niches | Biomes | °C | Abundance | Item | Raw / cooked HP | Food | Herb | Poison |
| ---- | ----- | ------ | ------ | -- | --------- | ---- | --------------- | ---- | ---- | ------ |
| Button mushroom | *Agaricus bisporus* | dung, soil | Seasonal forest, Grassland, Woodland | 2 to 26 | Common | handful (100 g) | 1 / 5 | - | - | - |
| Oyster mushroom | *Pleurotus ostreatus* | wood | Seasonal forest, Temperate rainforest, Boreal | 0 to 26 | Common | handful (100 g) | 0 / 8 | - | - | - |
| Velvet shank | *Flammulina velutipes* | wood | Boreal, Seasonal forest | -4 to 18 | Common | handful (100 g) | 0 / 9 | - | - | mild, 1-3 h: Health -1 (raw) |
| Shiitake | *Lentinula edodes* | wood | Seasonal forest, Temperate rainforest | 8 to 24 | Uncommon | handful (100 g) | 0 / 8 | - | Tou +2, Health +1, Str +1, Pre -1 | mild, 1 day: Pre -1 (raw) |
| Lion's mane | *Hericium erinaceus* | wood | Seasonal forest, Temperate rainforest | 4 to 20 | Rare | piece (400 g, 2 servings) | 0 / 17 | Str +1 (cooked) | Int +2, Wil +1, Tou +1, Health +1 | - |
| Nameko | *Pholiota microspora* | wood | Boreal, Seasonal forest | 0 to 16 | Uncommon | handful (100 g) | 0 / 3 | - | - | - |
| Pioppino | *Cyclocybe aegerita* | wood | Seasonal forest, Woodland | 10 to 22 | Uncommon | handful (100 g) | 0 / 7 | - | - | - |
| Beech mushroom | *Hypsizygus tessellatus* | wood | Seasonal forest, Boreal | 0 to 16 | Uncommon | handful (100 g) | 0 / 9 | - | - | - |
| Deer mushroom | *Pluteus cervinus* | wood | Seasonal forest, Boreal | 0 to 18 | Common | handful (100 g) | 0 / 7 | - | - | - |
| Wine cap | *Stropharia rugosoannulata* | wood, soil | Seasonal forest, Woodland | 4 to 22 | Uncommon | one (120 g) | 0 / 9 | - | - | - |
| Wood blewit | *Clitocybe nuda* | soil | Seasonal forest, Boreal | 0 to 16 | Uncommon | handful (100 g) | 0 / 7 | Str +1 (cooked) | - | mild, 1-3 h: Health -1 (raw) |
| Shaggy ink cap | *Coprinus comatus* | soil, dung | Grassland, Seasonal forest | 0 to 20 | Common | handful (100 g) | 0 / 6 | - | - | - |
| Common ink cap | *Coprinopsis atramentaria* | wood, soil | Seasonal forest, Grassland | 2 to 22 | Common | handful (100 g) | 0 / 6 | - | Tou +1, Wil +1, Health -1, Per -1 | moderate, with alcohol, for 3 days: Health -2, Wil -1 |
| Straw mushroom | *Volvariella volvacea* | soil | Tropical rainforest, Savanna | 20 to 32 | Common | handful (100 g) | 0 / 8 | - | - | - |
| Honey fungus (glows) | *Armillaria mellea* | wood, roots | Seasonal forest, Boreal, Temperate rainforest | -2 to 18 | Common | handful (100 g) | 0 / 6 | Str +1 (cooked) | - | mild, 1-4 h: Health -1 (raw) |
| Wood ear | *Auricularia auricula-judae* | wood, water | Seasonal forest, Temperate rainforest, Tropical rainforest | 2 to 30 | Common | handful (100 g) | 0 / 6 | - | Health +1, Per +1, Tou +1, Str -1 | mild, hours: Health -1 (raw) |
| Snow fungus | *Tremella fuciformis* | wood, water | Tropical rainforest | 18 to 30 | Uncommon | handful (100 g) | 0 / 6 | - | Tou +1, Pre +1, Health +1, Emp +1 | - |
| Witch's butter | *Tremella mesenterica* | wood, water | Seasonal forest, Temperate rainforest | 2 to 18 | Common | handful (100 g) | 0 / 5 | Tou +1 (cooked) | - | - |
| Dryad's saddle | *Cerioporus squamosus* | wood | Seasonal forest | 2 to 18 | Uncommon | piece (300 g, 2 servings) | 0 / 11 | - | - | - |
| Train wrecker | *Neolentinus lepideus* | wood | Boreal, Seasonal forest | -2 to 16 | Uncommon | one (80 g) | 0 / 6 | - | - | - |
| Cellar cup | *Peziza domiciliana* | wood, soil | Seasonal forest, Woodland | 4 to 24 | Uncommon | handful (100 g) | 0 / 6 | - | - | moderate, lung illness, after weeks: Health -2, handling |
| Dry rot | *Serpula lacrymans* | wood | Seasonal forest, Boreal | 4 to 22 | Uncommon | piece (200 g) | 0 / 15 | - | - | - |
| Bitter oyster (glows) | *Panellus stipticus* | wood | Seasonal forest, Boreal | 0 to 18 | Common | handful (100 g) | 0 / 7 | - | Health +2, Tou -1, Str -1, Per +1 | mild, hours: Health -1 (cooked) |
| Glowing mycena (glows) | *Mycena chlorophos* | wood, water | Tropical rainforest | 18 to 30 | Uncommon | handful (100 g) | 0 / 7 | - | - | - |
| Funeral bell | *Galerina marginata* | wood | Boreal, Seasonal forest, Temperate rainforest | -4 to 22 | Common | handful (100 g) | 0 / 7 | - | - | deadly, 6-12 h, false recovery: Health -3, Tou -3 |
| Sulphur tuft | *Hypholoma fasciculare* | wood | Seasonal forest, Boreal, Temperate rainforest | 0 to 20 | Common | handful (100 g) | 0 / 7 | - | - | moderate, 5-10 h: Health -2, Per -1, Agi -1 |
| Jack-o'-lantern mushroom (glows) | *Omphalotus olearius* | wood, roots | Woodland, Seasonal forest | 8 to 22 | Uncommon | handful (100 g) | 0 / 7 | - | - | moderate, 1-3 h: Health -2, Tou -1 |
| Ghost fungus (glows) | *Omphalotus nidiformis* | wood | Temperate rainforest, Woodland | 10 to 22 | Uncommon | handful (100 g) | 0 / 7 | - | - | moderate, 30 min-2 h: Health -2, Tou -1 |
| Dead man's fingers | *Xylaria polymorpha* | wood, remains | Seasonal forest | 0 to 20 | Common | one (5 g) | 0 / 0 | - | - | - |
| Corpse finder | *Hebeloma radicosum* | remains, soil | Seasonal forest, Boreal | 0 to 18 | Rare | handful (100 g) | 0 / 7 | - | - | mild, 1-3 h: Health -2, Tou -1 |
| Split gill | *Schizophyllum commune* | wood | Tropical rainforest, Savanna, Seasonal forest | 8 to 30 | Common | handful (100 g) | 0 / 8 | - | - | mild, lung infection, rare: Health -1, handling |
| Turkey tail | *Trametes versicolor* | wood | Seasonal forest, Boreal, Temperate rainforest | 0 to 26 | Common | handful (100 g) | 0 / 7 | - | Tou +2, Health +1, Str -1, Wil +1 | - |
| Reishi | *Ganoderma lingzhi* | wood | Seasonal forest, Tropical rainforest | 8 to 26 | Rare | one (80 g) | 0 / 6 | - | Tou +2, Health +1, Mana +1, Aur +1 | - |
| Artist's bracket | *Ganoderma applanatum* | wood | Seasonal forest, Boreal, Temperate rainforest | 0 to 22 | Common | piece (500 g, 3 servings) | 0 / 12 | - | Tou +1, Health +1, Pre +1, Str -1 | - |

### Dug (11)

| Name | Latin | Niches | Biomes | °C | Abundance | Item | Raw / cooked HP | Food | Herb | Poison |
| ---- | ----- | ------ | ------ | -- | --------- | ---- | --------------- | ---- | ---- | ------ |
| Black truffle | *Tuber melanosporum* | roots, soil | Woodland, Seasonal forest | 8 to 18 | Rare | one (50 g) | 1 / 3 | - | - | - |
| White truffle | *Tuber magnatum* | roots, soil | Seasonal forest | 8 to 16 | VeryRare | one (60 g) | 1 / 4 | - | - | - |
| Summer truffle | *Tuber aestivum* | roots, soil | Seasonal forest, Woodland | 6 to 18 | Uncommon | one (50 g) | 1 / 3 | - | - | - |
| Desert truffle | *Terfezia* | roots, soil | Desert, Grassland | 16 to 28 | Uncommon | one (100 g) | 0 / 15 | Str +1 (cooked) | - | - |
| Deer truffle | *Elaphomyces granulatus* | roots, soil | Boreal, Seasonal forest | -2 to 16 | Common | one (10 g) | 0 / 0 | - | Health +2, Tou +1, Emp +1, Str +1 | - |
| Poria | *Wolfiporia extensa* | roots, wood | Seasonal forest, Woodland | 6 to 20 | Uncommon | piece (1000 g, 5 servings) | 0 / 40 | - | Tou +2, Health +2, Wil +1, Mana +1 | - |
| Umbrella polypore | *Polyporus umbellatus* | roots, wood | Seasonal forest | 4 to 18 | Rare | piece (500 g, 3 servings) | 0 / 12 | Str +1 (cooked) | Health +2, Tou +1, Str +1, Agi -1 | - |
| Tiger milk mushroom | *Lignosus rhinocerus* | wood | Tropical rainforest | 20 to 30 | VeryRare | one (80 g) | 0 / 6 | - | Tou +2, Health +2, Str +1, Wil +1 | - |
| Caterpillar fungus | *Ophiocordyceps sinensis* | insects, soil | Tundra, Grassland | -6 to 6 | VeryRare | one (1 g) | 0 / 0 | - | Str +2, Tou +1, Health +1, Mana +1 | mild, after weeks of use: Health -1 |
| Scarlet caterpillar club | *Cordyceps militaris* | insects, soil | Seasonal forest, Boreal | 2 to 18 | Uncommon | one (1 g) | 0 / 0 | Str +1 (cooked) | Tou +1, Str +1, Health +1, Agi +1 | - |
| Termite mushroom | *Termitomyces titanicus* | termites | Savanna, Tropical rainforest | 20 to 30 | Uncommon | piece (1000 g, 5 servings) | 0 / 17 | Str +1 (cooked) | - | - |

### Near daylight (dim) (2)

| Name | Latin | Niches | Biomes | °C | Abundance | Item | Raw / cooked HP | Food | Herb | Poison |
| ---- | ----- | ------ | ------ | -- | --------- | ---- | --------------- | ---- | ---- | ------ |
| Poison fire coral | *Trichoderma cornu-damae* | wood, soil | Temperate rainforest, Tropical rainforest | 12 to 26 | VeryRare | one (5 g) | 0 / 0 | - | - | deadly, hours, blood failing at a week: Health -3, Tou -3, Agi -2, Pre -2 |
| Rock tripe | *Umbilicaria* | rock | Tundra, Boreal | -16 to 10 | Common | handful (100 g) | 0 / 12 | Str +1 (cooked) | - | mild, hours: Health -1 (raw) |

### Above ground (52)

| Name | Latin | Niches | Biomes | °C | Abundance | Item | Raw / cooked HP | Food | Herb | Poison |
| ---- | ----- | ------ | ------ | -- | --------- | ---- | --------------- | ---- | ---- | ------ |
| Chanterelle | *Cantharellus cibarius* | roots | Boreal, Seasonal forest, Temperate rainforest | -2 to 18 | Uncommon | handful (100 g) | 0 / 9 | - | - | - |
| Porcini | *Boletus edulis* | roots | Boreal, Seasonal forest | -4 to 16 | Uncommon | one (300 g) | 6 / 22 | Str +1 (cooked) | - | - |
| Morel | *Morchella esculenta* | soil, roots | Seasonal forest, Grassland, Boreal | 0 to 18 | Rare | handful (100 g) | 0 / 7 | Str +1 (cooked) | - | mild, a few hours: Health -1 (raw) |
| Black trumpet | *Craterellus cornucopioides* | roots | Seasonal forest | 4 to 16 | Uncommon | handful (100 g) | 0 / 7 | Str +1 (cooked) | - | - |
| Hedgehog mushroom | *Hydnum repandum* | roots | Boreal, Seasonal forest | -2 to 16 | Common | handful (100 g) | 0 / 7 | - | - | - |
| Bay bolete | *Imleria badia* | roots | Boreal, Seasonal forest | -2 to 14 | Common | one (100 g) | 2 / 7 | - | - | - |
| Birch bolete | *Leccinum scabrum* | roots | Boreal, Tundra | -8 to 12 | Common | one (80 g) | 0 / 5 | - | - | mild, 1-4 h: Health -1 (raw) |
| Slippery jack | *Suillus luteus* | roots | Boreal | -4 to 14 | Common | one (60 g) | 0 / 3 | - | - | - |
| Saffron milk cap | *Lactarius deliciosus* | roots | Boreal, Woodland | -2 to 16 | Common | one (80 g) | 0 / 5 | - | - | - |
| Matsutake | *Tricholoma matsutake* | roots | Boreal, Temperate rainforest | -2 to 14 | VeryRare | one (150 g) | 0 / 9 | - | - | - |
| Caesar's mushroom | *Amanita caesarea* | roots | Woodland, Seasonal forest | 12 to 20 | Rare | one (120 g) | 2 / 9 | Str +1 (cooked) | - | - |
| Charcoal burner | *Russula cyanoxantha* | roots | Seasonal forest | 2 to 16 | Common | one (80 g) | 0 / 5 | - | - | - |
| Lobster mushroom | *Hypomyces lactifluorum* | roots | Boreal, Seasonal forest | -2 to 14 | Uncommon | one (200 g) | 0 / 15 | - | - | - |
| Hen of the woods | *Grifola frondosa* | roots, wood | Seasonal forest | 4 to 18 | Uncommon | piece (500 g, 3 servings) | 0 / 12 | - | - | - |
| Chicken of the woods | *Laetiporus sulphureus* | trees, wood | Seasonal forest, Woodland | 4 to 22 | Common | piece (300 g, 2 servings) | 0 / 11 | Str +1 (cooked) | - | mild, a few hours: Health -1 (raw) |
| Beefsteak fungus | *Fistulina hepatica* | trees | Seasonal forest, Woodland | 6 to 18 | Uncommon | piece (400 g, 2 servings) | 3 / 12 | Str +1 (cooked) | - | - |
| Cauliflower fungus | *Sparassis crispa* | roots | Boreal, Seasonal forest | 0 to 16 | Uncommon | piece (1000 g, 5 servings) | 0 / 15 | - | - | - |
| Blusher | *Amanita rubescens* | roots | Seasonal forest, Boreal | 0 to 16 | Common | one (80 g) | 0 / 6 | Str +1 (cooked) | - | moderate, hours to days: Health -2 (raw), Str -1 (raw) |
| Chaga | *Inonotus obliquus* | trees | Boreal, Tundra | -10 to 6 | Uncommon | one (150 g) | 0 / 7 | - | Tou +2, Health +1, Str +1, Wil +1 | moderate, after months of tea: Health -1 |
| Tinder fungus | *Fomes fomentarius* | trees, wood | Boreal, Seasonal forest | -4 to 16 | Common | piece (600 g, 3 servings) | 0 / 0 | - | Health +2, Tou +1, Per -1, Agi -1 | - |
| Birch polypore | *Fomitopsis betulina* | trees | Boreal | -8 to 12 | Common | one (200 g) | 0 / 15 | - | Health +2, Tou +1, Str -1, Per +1 | - |
| Agarikon | *Laricifomes officinalis* | trees | Boreal, Temperate rainforest | -2 to 10 | VeryRare | one (150 g) | 0 / 11 | - | Tou +2, Health +1, Aur +1, Mana +1 | - |
| Scarlet elf cup | *Sarcoscypha austriaca* | wood | Seasonal forest, Temperate rainforest | 2 to 14 | Uncommon | handful (100 g) | 0 / 7 | - | - | - |
| Field mushroom | *Agaricus campestris* | soil | Grassland, Seasonal forest | 2 to 22 | Common | handful (100 g) | 1 / 5 | - | - | - |
| Horse mushroom | *Agaricus arvensis* | soil | Grassland | 2 to 18 | Uncommon | one (120 g) | 0 / 7 | - | - | - |
| Fairy ring champignon | *Marasmius oreades* | soil | Grassland | 0 to 20 | Common | handful (100 g) | 0 / 8 | - | - | - |
| St George's mushroom | *Calocybe gambosa* | soil | Grassland, Seasonal forest | 2 to 16 | Uncommon | one (100 g) | 0 / 7 | - | - | - |
| Parasol mushroom | *Macrolepiota procera* | soil | Grassland, Woodland | 4 to 20 | Uncommon | one (100 g) | 0 / 7 | - | - | - |
| Giant puffball | *Calvatia gigantea* | soil | Grassland, Seasonal forest | 2 to 20 | Uncommon | piece (2000 g, 10 servings) | 0 / 15 | - | Health +2, Tou +1, Per -1, Agi -1 | mild, a few hours: Health -1, old |
| Common puffball | *Lycoperdon perlatum* | soil | Seasonal forest, Grassland, Boreal | -2 to 22 | Common | handful (100 g) | 0 / 7 | - | Health +1, Tou +1, Per -1, Str +1 | moderate, hours, from breathed spores: Health -2, handling, old |
| King oyster | *Pleurotus eryngii* | soil, roots | Grassland, Woodland | 10 to 22 | Uncommon | one (100 g) | 0 / 8 | Tou +1 (cooked) | - | - |
| Death cap | *Amanita phalloides* | roots | Seasonal forest, Woodland | 6 to 20 | Uncommon | one (60 g) | 0 / 4 | - | - | deadly, 6-12 h, false recovery: Health -3, Tou -3, Int -1 |
| Destroying angel | *Amanita virosa* | roots | Boreal, Seasonal forest | -2 to 16 | Uncommon | one (50 g) | 0 / 3 | - | - | deadly, 6-12 h, false recovery: Health -3, Tou -3 |
| Fly agaric | *Amanita muscaria* | roots | Boreal, Seasonal forest, Tundra | -6 to 16 | Common | one (100 g) | 0 / 7 | - | Mana +2, Int -2, Per -1, Agi -1 | moderate, 30-90 min, over by a day: Int -2, Per -2, Agi -2, Health -1 |
| Panther cap | *Amanita pantherina* | roots | Seasonal forest, Woodland | 2 to 18 | Uncommon | one (80 g) | 0 / 5 | - | Mana +1, Int -2, Per -2, Agi -1 | severe, 1-3 h, over by a day: Health -3, Int -2, Per -2, Agi -2 |
| Deadly webcap | *Cortinarius rubellus* | roots | Boreal | -4 to 12 | Rare | handful (100 g) | 0 / 7 | - | - | deadly, 2-20 days: Health -3, Tou -3 |
| Fool's webcap | *Cortinarius orellanus* | roots | Seasonal forest | 4 to 16 | Rare | handful (100 g) | 0 / 7 | - | - | deadly, 2-20 days: Health -3, Tou -3 |
| False morel | *Gyromitra esculenta* | soil, roots | Boreal | -4 to 12 | Uncommon | handful (100 g) | 0 / 7 | - | - | deadly, 6-12 h: Health -3, Agi -2, Int -2 |
| Ivory funnel | *Clitocybe dealbata* | soil | Grassland | 2 to 18 | Common | handful (100 g) | 0 / 7 | - | - | severe, 15-30 min, over in 2 h: Health -3, Per -2, Tou -2 |
| Yellow stainer | *Agaricus xanthodermus* | soil | Seasonal forest, Grassland | 2 to 20 | Common | one (80 g) | 0 / 5 | - | - | mild, 1-2 h: Health -2, Tou -1 |
| Devil's bolete | *Rubroboletus satanas* | roots | Seasonal forest, Woodland | 10 to 18 | Rare | one (500 g) | 0 / 37 | - | - | moderate, a few hours: Health -2, Tou -1 |
| Brown roll-rim | *Paxillus involutus* | roots | Boreal, Seasonal forest | -4 to 16 | Common | one (80 g) | 0 / 6 | - | - | deadly, after repeated meals: Health -3, Tou -2 |
| Deadly fibrecap | *Inosperma erubescens* | roots | Seasonal forest | 4 to 16 | Uncommon | one (30 g) | 0 / 2 | - | - | deadly, 15-30 min: Health -3, Per -2, Tou -2 |
| Common earthball | *Scleroderma citrinum* | roots | Seasonal forest, Boreal | 0 to 18 | Common | one (60 g) | 0 / 4 | - | - | mild, 30 min-3 h: Health -2, Per -1 |
| Iceland moss | *Cetraria islandica* | rock, soil | Tundra, Boreal | -16 to 4 | Common | handful (100 g) | 0 / 25 | Str +1 (cooked) | Health +2, Tou +1, Str +1, Pre +1 | - |
| Reindeer lichen | *Cladonia rangiferina* | rock, soil | Tundra, Boreal | -18 to 4 | Common | handful (100 g) | 0 / 22 | - | Tou +1, Health +1, Str +1, Agi -1 | mild, hours: Health -1 (raw) |
| Old man's beard | *Usnea* | trees | Boreal, Temperate rainforest | -6 to 16 | Common | handful (100 g) | 0 / 15 | - | Health +2, Tou +1, Wil +1, Str -1 | - |
| Tree lungwort | *Lobaria pulmonaria* | trees | Temperate rainforest, Seasonal forest | 2 to 14 | Rare | handful (100 g) | 0 / 15 | - | Health +2, Tou +1, Str +1, Pre +1 | - |
| Horsehair lichen | *Bryoria fremontii* | trees | Boreal | -6 to 10 | Common | handful (100 g) | 0 / 20 | Str +1 (cooked) | Tou +1, Health +1, Agi +1, Str +1 | moderate, hours, some patches only: Health -2 |
| Manna lichen | *Lecanora esculenta* | rock | Desert, Grassland | 14 to 28 | Uncommon | handful (100 g) | 3 / 12 | Str +1 (raw) | - | - |
| Wolf lichen | *Letharia vulpina* | trees | Boreal | -6 to 10 | Uncommon | handful (100 g) | 0 / 17 | - | Health +1, Tou +1, Pre +1, Str -1 | severe, hours: Health -3 |
| Oakmoss | *Evernia prunastri* | trees | Seasonal forest, Woodland | 4 to 18 | Common | handful (100 g) | 0 / 12 | - | Pre +2, Emp +1, Health +1, Tou -1 | mild, skin, hours: Health -1, handling |

### Invented (12)

| Name | Latin | Niches | Biomes | °C | Abundance | Item | Raw / cooked HP | Food | Herb | Poison |
| ---- | ----- | ------ | ------ | -- | --------- | ---- | --------------- | ---- | ---- | ------ |
| Glowcap (glows) | (invented) | wood, water, soil | - | -4 to 30 | Common | handful (100 g) | 2 / 7 | - | - | - |
| Lantern shelf (glows) | (invented) | wood | - | 0 to 30 | Uncommon | piece (400 g, 2 servings) | 0 / 15 | - | - | moderate, 1-3 h: Health -2, Tou -1 |
| Guano cap | (invented) | dung | - | 0 to 30 | Common | handful (100 g) | 0 / 7 | Str +1 (cooked) | - | - |
| Grave cap | (invented) | remains | - | 0 to 26 | Uncommon | handful (100 g) | 0 / 7 | - | - | deadly, 6-12 h, false recovery: Health -3, Tou -3 |
| Stonebread | (invented) | soil, water | - | -2 to 30 | Uncommon | piece (1500 g, 8 servings) | 0 / 42 | Str +1 (cooked), Tou +1 (cooked) | - | - |
| Dripjelly | (invented) | water, rock | - | 0 to 26 | Common | handful (100 g) | 1 / 5 | - | Tou +1, Health +1, Pre +1, Str -1 | - |
| Sleepdust puffball | (invented) | soil | - | 0 to 26 | Uncommon | one (40 g) | 0 / 3 | - | Wil +1, Health +1, Agi -2, Per -1 | mild, at once, a few minutes: Agi -2, Per -1, handling |
| Dreamcap | (invented) | soil, remains | - | -2 to 30 | VeryRare | one (20 g) | 0 / 1 | - | Mana +3, Int -2, Per -1, Wil -1 | moderate, 30 min, a few hours: Int -2, Per -2, Mana +1 |
| Emberbloom (glows) | (invented) | lava, rock | - | -10 to 40 | Rare | one (30 g) | 0 / 2 | - | Tou +2, Health +1, Str +1, Agi -1 | - |
| Cave pearl | (invented) | soil, water | - | 0 to 28 | VeryRare | one (20 g) | 0 / 1 | - | - | - |
| Root beard | (invented) | roots | - | -6 to 24 | Uncommon | handful (100 g) | 0 / 10 | - | Health +2, Tou +1, Wil +1, Str -1 | - |
| Bleeding shelf | (invented) | wood, remains | - | 0 to 26 | Rare | one (150 g) | 0 / 11 | - | Health +2, Str +1, Tou -1, Emp -1 | mild, 1-3 h: Health -1 |
