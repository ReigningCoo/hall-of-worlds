# Research: livestock

Research for the animals people keep (2026-11-19), approved by the owner with the decisions below. Made like the vegetable and fungus research. The review page with every finding and quote: https://claude.ai/artifact/BSxips1oGvb65Fnh9AT7Gv. Scripts and data (outside the repo): `~/.claude/research-tools/livestock/` (`design.py` holds the proposals, the owner's `DECISIONS` and `GAME_RATES`; `data/collated.json` the checked facts). The table built from it: `shared::livestock` (`livestock/livestock.ron`, `livestock/parts.ron`; `docs/context/shared/livestock.md`).

## Method and sources

- 26 animals: 16 mammals (cattle and their relatives, sheep, goats, pigs, the horse family, camels, llamas, reindeer, the rabbit), 8 birds and 2 insects kept as objects (the honeybee's hive, the silkworm's rearing tray).
- Each animal's Wikipedia articles (the animal, its wild form, feral populations and breeds: 119 articles) read by Claude (Opus) for about 30 facts each with word-for-word quotes; a script checked every quote against its article. Topic articles on cuts of meat (beef, pork, lamb and mutton, primal cuts, veal), offal, hides and leather, rennet, bezoars, fats and fibres were read the same way, and sexual dimorphism per animal for the models.
- Proposals (`design.py`) are Claude's synthesis for the owner's review: kept biomes are the game's nine (`climate::Biome`), rarities a six-step scale, cut names one per quality from the cut tables. Game rates are about half of modern breeds' yields.
- The game's table follows the proposals with small changes: several cut names per quality (decision 4) added by Claude, a stage name for each sex, stage ends in game hours (a real month's growth a game day), egg chances from "most game days" (55 to 60%) and "every N game days" (100 ÷ N), and the silkworm's and honeybee's first stage named "larva" rather than "egg".

## Owner's decisions

1. Keep wild aurochs and wild dromedaries as very rare wild animals, though extinct or gone on Earth.
2. Growth: one real month of an animal's growth is one game day (one real hour): a calf grows up in about a day and a half, a chick in five hours.
3. Produce per game day, at lower yields than modern breeds (less selective breeding): a cow about 10 L of milk a game day (the game rates below).
4. Several cut names per quality, chosen by hash (sirloin, ribeye and T-bone all Epic).
5. Young animals' meat (veal, lamb, kid, suckling pig, squab) has its own name, its bottom quality a step higher, and a much lower yield (the table: 25%).
6. Butchering splits a carcass into portions with odds by real carcass shares (fillet about 3%, sirloin and rib about 16%), Butchery moving share up from Poor.
7. Furry hides (sheep, yak, llama, alpaca, reindeer, rabbit) are cured as pelts.
8. Raw hides rot after 7 hours of playtime carried (real time on the ground, like casks); salting with the game's Salt stops it until cured.
9. Organ drops: base + (95 − base) × Butchery%; the rare stones (ox bezoar, bezoar, os cordis) stay rare, base × 0.5 to 1.5 by skill.
10. Organs in Alchemy: to be decided. A bezoar may be the ingredient of a Cure Poison, with Toughness. The table keeps the proposed effects for then.
11. Animal fats can stand in for oils as ointment bases.
12. Blood is collected only into an empty bottle.
13. Variants (Muscovy under duck, Bactrian under dromedary, alpaca under llama, as the Coo under cattle) are their own creature kinds with their own rows, each marked as the variant of the other.
14. Bees and silkworms are objects (a hive, a rearing tray on mulberry leaves): no bee models, so no bee dimorphism.

The plan, in 8 PRs: 1 the tables (`shared::livestock`); 2 body parts and differences between the sexes; 3 to 5 new kinds with models (cattle's relatives, pigs, sheep and goats; the horse family, camels, llamas, reindeer and the rabbit; birds); 6 life stages and castrates' looks; 7 wild animals from the table; 8 kept animals on farms and in towns. Production (butchery, the items, hides, milking, shearing, hives, trays) is a later task.

## The animals

### Where they live and what they eat

| Animal | Kind | Kept in | Wild form | Wild | Feral | Diet | Favourites | Water |
| ------ | ---- | ------- | --------- | ---- | ----- | ---- | ---------- | ----- |
| Cattle | Cattle | Grassland, Woodland, SeasonalForest, Savanna, BorealForest | aurochs (extinct on Earth) | very rare (Grassland, Woodland) | uncommon herds on islands, moors | grazer | clover, grain, salt | Wades: stands in ponds to cool |
| Highland cattle | Coo | Tundra, BorealForest, Grassland, TemperateRainforest | none (aurochs) | very rare (feral) (hills, moors) | semi-wild folds on rough grazing | grazer, some browse | fresh grass, grain, salt | Swims: once swum between islands |
| Water buffalo | - | Savanna, TropicalRainforest, SeasonalForest, Grassland | wild water buffalo | rare (TropicalRainforest, Savanna (wetlands)) | common feral herds | grazer, water plants | fresh green grass, rice, sugarcane | Prefers: wallows in mud daily, grazes under water |
| Yak | - | Tundra (high), Grassland (cold steppe), BorealForest | wild yak | rare (Tundra above the trees) | occasional, joins wild herds | grazer | salt; refuses grain | Drinks: rolls in dry soil, eats snow |
| Sheep | - | Grassland, Woodland, SeasonalForest, Savanna, Desert edge, Tundra | mouflon | rare (mountains in Grassland, Woodland) | uncommon (island mouflon) | grazer, crops close | grain, clover, salt | Drinks: avoids getting wet |
| Goat | - | Grassland, Savanna, Woodland, Desert, SeasonalForest | wild goat (bezoar ibex) | uncommon (rocky mountains, any warm biome) | very common on islands | browser | grain, browse, salt | Drinks: dislikes getting wet |
| Pig | - | SeasonalForest, Woodland, TemperateRainforest, Grassland, TropicalRainforest | wild boar | common (SeasonalForest, Woodland, BorealForest, TemperateRainforest) | very common | omnivore, roots | acorns and beech mast, grain, scraps | Likes: wallows in mud to cool, swims well |
| Horse | Horse | Grassland, Savanna, Woodland, SeasonalForest, BorealForest | wild horse (Przewalski's, tarpan) | rare (Grassland steppe) | common (mustangs, moor ponies) | grazer | oats, apples, carrots, salt | Swims: rolls in dust and mud |
| Donkey | - | Desert, Savanna, Grassland, Woodland | African wild ass | very rare (Desert) | common (burros) | mixed feeder | salt, grain, carrots | Drinks: dislikes wet ground, rolls in dust |
| Mule | - | Grassland, Savanna, Desert, Woodland, SeasonalForest | none (a hybrid) | never | rare strays, sterile | grazer | grain, fruit, salt | Drinks: dislikes deep water |
| Dromedary | - | Desert, Savanna | none living | very rare (feral) (Desert) | large feral herds in places | browser | acacia, saltbush, salty plants | Drinks: drinks 100 L at once, no water for 10 days |
| Bactrian camel | - | Desert (cold), Grassland (steppe) | wild Bactrian camel | very rare (cold Desert) | rare | mixed feeder | salty plants, salt | Drinks: eats snow, months without drinking |
| Llama | - | Grassland (mountain), Desert, Tundra (alpine) | guanaco | common (Grassland and Desert in mountains) | rare | grazer | grain, salt | Swims: dust baths |
| Alpaca | - | Grassland (mountain), Tundra (alpine) | vicuña | uncommon (high Grassland) | very rare | grazer | salt, tender leaves | Wades: lies in shallow water in heat |
| Reindeer | - | Tundra, BorealForest | wild reindeer (caribou) | common (Tundra, BorealForest) | fairly common, herds roam free | mixed: lichen in winter, leaves in summer | mushrooms, lichen, salt | Swims: strong swimmer, crosses lakes |
| Rabbit | - | Grassland, Woodland, SeasonalForest, Savanna | European rabbit | common (Grassland, Woodland, SeasonalForest) | domestic ones rarely last | grazer | young shoots, clover, roots, fruit | Avoids: never swims by choice |
| Chicken | Chicken | every biome where people live | red junglefowl | uncommon (TropicalRainforest edges) | often free-ranging | omnivore, scratches | grain, ripe fruit, insects | Drinks: dust-bathes |
| Duck | - | Grassland, SeasonalForest, Woodland, TemperateRainforest, TropicalRainforest, BorealForest | mallard | very common (any biome by water) | common on ponds | omnivore, dabbles | grain, slugs, snails | Prefers: lives on ponds, needs water to preen |
| Muscovy duck | - | TropicalRainforest, Savanna, SeasonalForest | wild Muscovy | common (TropicalRainforest, Savanna by water) | common | omnivore | grain, corn | Likes: bathes, but perches in trees |
| Goose | - | Grassland, SeasonalForest, Woodland, BorealForest, Tundra | greylag goose, swan goose | common (BorealForest, Tundra, Grassland wetlands) | common | grazer | short young grass, grain | Likes: swims and bathes, grazes on land |
| Turkey | - | Woodland, SeasonalForest, Grassland | wild turkey | common (Woodland, SeasonalForest) | occasional | omnivore | acorns and nuts, grain | Drinks: dust-bathes |
| Guineafowl | - | Savanna, Grassland, Woodland, Desert edge | same species | common (Savanna) | often | forager | ticks, beetles, millet | Drinks: dust-bathes |
| Pigeon | - | towns in any biome; cliffs | rock dove | very common (feral) (towns, sea cliffs) | very common | grain eater | grain, seed | Drinks: bathes in puddles |
| Quail | - | Grassland, Savanna, SeasonalForest | common and Japanese quail | common (Grassland, Savanna) | released for hunting | grain eater | millet, sweet water | Drinks: dust-bathes |
| Honeybee | - | any biome with flowers but Tundra | wild colonies | common (hollow trees in Woodland, SeasonalForest) | swarms leave hives often | nectar feeder | blossom, sugar syrup | Drinks: fetches water to cool the hive |
| Silkworm | - | where mulberry grows (kept indoors) | wild silk moth (no use) | never | never: cannot fly or live alone | leaf specialist | fresh white mulberry leaves only | Avoids: damp brings disease |

### Produce and work

| Animal | Real yields | Game rate (a game day) |
| ------ | ----------- | ---------------------- |
| Cattle | milk 15-22 L a day for 10 months after calving; dung; pulls ploughs and carts | milk 10 L |
| Highland cattle | a little rich milk (rarely milked); shaggy hair combed in spring; dung | milk 3 L (rarely milked) |
| Water buffalo | milk 5-8 L a day, 7-8% fat (mozzarella); ploughs paddies, turns mills | milk 4 L |
| Yak | milk 1-2 L a day, 6-7% fat (butter tea); down combed in spring 0.1-0.5 kg, hair shorn 0.5-1 kg; packs 60-100 kg | milk 1 L |
| Sheep | wool 2-5 kg a year, shorn once; milk 0.5-2 L a day; dung | milk 0.5 L; wool 2 kg a shearing |
| Goat | milk about 3 L a day; mohair (Angora) or cashmere (combed); clears brush; small packs | milk 1.5 L |
| Pig | dung; roots out truffles and clears ground; bristles at slaughter | - |
| Horse | riding, war, ploughing, carts, packs 70-100 kg; mare's milk 1-2 L (kumis); tail hair (bow strings) | milk 1 L |
| Donkey | packs 50-70 kg, carts, mills; milk 1-2 L; guards sheep; sires mules | milk 0.5 L |
| Mule | packs up to 90 kg, draught, riding; outworks horses | - |
| Dromedary | milk about 9 L a day; hair 1 kg a year; packs 150-300 kg, riding | milk 5 L |
| Bactrian camel | hair and down 5 kg a year, combed; packs 170-250 kg; milk 2-5 L | milk 2 L |
| Llama | fleece 1.5-3 kg a year, shorn; packs 30-60 kg; guards sheep | fleece 1.5 kg a shearing |
| Alpaca | fine fleece 2.3-4.5 kg a year, shorn; guards sheep | fleece 2 kg a shearing |
| Reindeer | a little very rich milk (20% fat); pulls sledges, packs 30-40 kg; antler velvet | milk 0.1 L |
| Rabbit | Angora wool 0.5-1 kg a year; droppings (garden manure) | Angora wool 0.5 kg a shearing |
| Chicken | eggs, up to 300 a year at 50-60 g; droppings (best manure); eats insects | an egg most game days (heritage hens about 180 a year) |
| Duck | eggs about 200 a year at 70-80 g; eats slugs and paddy pests | an egg most game days |
| Muscovy duck | eggs 60-120 a year; eats flies | an egg every 3 game days |
| Goose | eggs up to 100 a year at 120-170 g; down 0.1-0.2 kg plucked at moult; guards with its alarm | an egg every 3 to 4 game days (spring) |
| Turkey | eggs 50-100 a year at 80-90 g (rarely eaten) | an egg every 4 to 5 game days |
| Guineafowl | eggs about 100 a year at 40 g, thick shells; eats ticks; loud guard | an egg every 3 game days |
| Pigeon | carries messages; dovecote dung (manure, saltpetre) | - |
| Quail | eggs up to 300 a year at 10 g; song | an egg most game days |
| Honeybee | honey (a colony makes far more than its 50-100 kg yearly need), beeswax, propolis, royal jelly; pollination | honey about 0.1 kg a game day in the flowering months |
| Silkworm | cocoons (one filament 300-900 m; 4,000-7,000 cocoons a kg of raw silk); pupae eaten | a tray's cocoons once grown |

### Life stages

Young and juvenile end at these real months; in the game a real month is a game day (24 game hours).

| Animal | Young | Juvenile | Adult (female / male / castrate) | Young until | Grown at |
| ------ | ----- | -------- | -------------------------------- | ----------- | -------- |
| Cattle | calf | heifer / young bull | cow / bull / ox (castrated) | 9 months | 30 months |
| Highland cattle | calf | heifer / young bull | cow / bull / steer | 8 months | 36 months |
| Water buffalo | calf | heifer / young bull | cow / bull / bullock | 8 months | 36 months |
| Yak | calf | heifer / young bull | dri (cow) / yak (bull) / steer | 12 months | 42 months |
| Sheep | lamb | hogget | ewe / ram / wether | 4 months | 14 months |
| Goat | kid | doeling / buckling | doe / buck / wether | 3 months | 15 months |
| Pig | piglet | gilt / young boar | sow / boar / barrow | 2 months | 8 months |
| Horse | foal | filly / colt | mare / stallion / gelding | 6 months | 48 months |
| Donkey | foal | young | jenny / jack | 7 months | 24 months |
| Mule | foal | molly / john colt | molly / john | 5 months | 48 months |
| Dromedary | calf | young camel | cow / bull / gelding | 15 months | 48 months |
| Bactrian camel | calf | young camel | cow / bull / gelding | 18 months | 48 months |
| Llama | cria | tui | female / macho / gelding | 6 months | 24 months |
| Alpaca | cria | tui | hembra / macho / gelding | 6 months | 18 months |
| Reindeer | calf | yearling | cow / bull / härk (draught) | 5 months | 24 months |
| Rabbit | kit | young | doe / buck | 1 months | 5 months |
| Chicken | chick | pullet / cockerel | hen / cock / capon | 1.5 months | 5 months |
| Duck | duckling | young duck | duck / drake | 2 months | 6 months |
| Muscovy duck | duckling | young duck | duck / drake | 2.5 months | 6 months |
| Goose | gosling | young goose | goose / gander | 2 months | 12 months |
| Turkey | poult | jenny / jake | hen / tom | 2 months | 7 months |
| Guineafowl | keet | young | hen / cock | 2 months | 6 months |
| Pigeon | squab | squeaker | hen / cock | 1 months | 5 months |
| Quail | chick | young | hen / cock | 0.7 months | 1.5 months |
| Honeybee | egg and larva | pupa | worker / drone / queen | 0.3 months | 0.7 months |
| Silkworm | egg | larva (5 instars), then cocoon | moth | 0.5 months | 2 months |

### Meat

One name per quality as researched; the table (`livestock.ron`) adds more per quality, one chosen by hash (decision 4). "-": never.

| Animal | Meat | Young | Poor | Common | Uncommon | Rare | Epic | Legendary |
| ------ | ---- | ----- | ---- | ------ | -------- | ---- | ---- | --------- |
| Cattle | beef | veal | beef trimmings | beef shin | chuck steak | rump steak | sirloin | fillet steak |
| Highland cattle | beef | veal | beef trimmings | beef shin | chuck steak | rump steak | sirloin | fillet steak |
| Water buffalo | buffalo | buffalo veal | buffalo trimmings | buffalo shank | buffalo chuck | buffalo round | buffalo sirloin | - |
| Yak | yak | yak veal | - | yak shank | yak brisket | yak round | yak sirloin | yak fillet |
| Sheep | mutton | lamb | scrag end | breast of lamb | lamb shoulder | leg of lamb | loin chops | rack of lamb |
| Goat | goat | kid | goat neck | goat breast | goat shoulder | leg of goat | goat loin chops | goat rack |
| Pig | pork | suckling pig | trotters | ham hock | pork shoulder | pork belly | ham | pork tenderloin |
| Horse | horse | colt | horse trimmings | horse shank | horse chuck | horse rump | horse sirloin | horse fillet |
| Donkey | donkey | donkey foal | donkey stewing meat | donkey shoulder | donkey haunch | donkey loin | - | - |
| Mule | mule | mule foal | mule stewing meat | mule shoulder | mule steak | - | - | - |
| Dromedary | camel | camel calf | camel heel | camel neck | camel shoulder | camel leg | camel loin | camel hump |
| Bactrian camel | camel | camel calf | camel heel | camel neck | camel shoulder | camel leg | camel loin | camel hump |
| Llama | llama | cria | llama neck | llama shoulder | llama leg | llama loin | - | - |
| Alpaca | alpaca | cria | alpaca neck | alpaca shoulder | alpaca leg | alpaca loin | - | - |
| Reindeer | venison | reindeer calf | venison shank | venison shoulder | haunch of venison | saddle of venison | venison backstrap | - |
| Rabbit | rabbit | young rabbit | rabbit belly | rabbit foreleg | rabbit hind leg | saddle of rabbit | - | - |
| Chicken | chicken | spring chicken | chicken back | chicken wing | drumstick | chicken thigh | chicken breast | chicken oysters |
| Duck | duck | duckling | duck neck | duck wing | duck leg | duck thigh | duck breast | - |
| Muscovy duck | duck | duckling | duck neck | duck wing | duck leg | duck thigh | magret | - |
| Goose | goose | green goose | goose neck | goose wing | goose leg | goose thigh | goose breast | - |
| Turkey | turkey | young turkey | turkey neck | turkey wing | turkey drumstick | turkey thigh | turkey breast | - |
| Guineafowl | guineafowl | keet | - | guineafowl wing | guineafowl leg | guineafowl breast | - | - |
| Pigeon | pigeon | squab | - | - | pigeon legs | pigeon breast | - | - |
| Quail | quail | quail | - | - | quail legs | quail breast | - | - |
| Honeybee | bee brood | bee brood | bee brood | - | - | - | - | - |
| Silkworm | silkworm pupae | silkworm pupae | silkworm pupae | - | - | - | - | - |

Carcass shares by quality (beef's cut tables), each animal's kept for the qualities it has and scaled to 100: Poor 12%, Common 15%, Uncommon 30%, Rare 24%, Epic 16%, Legendary 3%.

### Hides

| Animal | Hide | Cured | Qualities | Notes |
| ------ | ---- | ----- | --------- | ----- |
| Cattle | cowhide | cow leather | Poor-Epic | calfskin from calves: Uncommon-Legendary (finest, and vellum later); thick 4-6 mm, shoes, belts, saddles |
| Highland cattle | shaggy cowhide | cow leather, or a hairy hide kept as a rug | Poor-Epic | calfskin as cattle |
| Water buffalo | buffalo hide | buffalo leather | Poor-Rare | thick 5-7 mm: soles, harness, shields |
| Yak | yak hide | yak leather, or the hairy hide as a robe | Poor-Rare | boots, bags, coracles, tents |
| Sheep | sheepskin | sheepskin with the wool on, or thin sheep leather | Poor-Epic (pelt), Poor-Uncommon (leather) | lambskin Uncommon-Epic; pelts graded by seeds and burrs in the wool |
| Goat | goatskin | goat leather | Common-Epic | kidskin from kids: Rare-Legendary (kid gloves); wineskins, drums, parchment |
| Pig | pigskin | pig leather | Poor-Rare | soft and tough: gloves, bookbinding |
| Horse | horsehide | horse leather | Common-Legendary | Legendary only as shell cordovan, the rump's under layer, at high skill |
| Donkey | donkey hide | donkey leather | Poor-Uncommon | boiled for ejiao gelatine |
| Mule | mule hide | mule leather | Poor-Uncommon | tougher than a horse's: straps, harness |
| Dromedary | camel hide | camel leather | Poor-Uncommon | poor leather: sandals, bags, water skins |
| Bactrian camel | camel hide | camel leather | Poor-Uncommon | ropes, bags, harness |
| Llama | llama skin | llama pelt with fleece, or leather | Common-Rare (pelt), Poor-Uncommon (leather) | neck skin for shoe soles |
| Alpaca | alpaca skin | alpaca pelt with fleece | Common-Rare | rugs, blankets |
| Reindeer | reindeer hide | reindeer pelt (hollow hair), or buckskin-like leather | Common-Epic (pelt), Common-Rare (leather) | clothing, boots, tents |
| Rabbit | rabbit skin | rabbit fur | Poor-Rare | fur only, no leather; felt for hats; Rex and Angora a step better |
| Birds | - | - | - | no leather: the skin is eaten crisp; feathers and down instead |
| Honeybee | - | - | - | - |
| Silkworm | - | - | - | - |

### Organs

| Organ | From | Food | Effect (proposed) | Other use | Base % | Quests |
| ----- | ---- | ---- | ----------------- | --------- | ------ | ------ |
| liver | all | pâté, liver sausage; calf's liver prized | Health +2 (weakness, blood loss) | - | 70 | common target ('a fresh calf's liver') |
| foie gras | fattened duck or goose only | the finest liver | Health +1, Toughness +1 | - | 100 | Legendary food; feast quests |
| heart | all | grilled, stewed | Will +1 (courage) | - | 60 | a prize bull's heart |
| kidneys | mammals | pies, stews | Toughness +1 | - | 60 | - |
| tongue | mammals, duck | pressed, smoked (reindeer) | Presence +1 (speech, breath) | - | 50 | - |
| brain | mammals | fried; calf's brain a delicacy | Intellect +1 | brain-tanning softens hides (buckskin) | 30 | - |
| sweetbreads | young mammals only | delicacy for the sick | Toughness +1 (recovery) | - | 30 | - |
| tripe | ruminants (cattle, buffalo, yak, sheep, goat, camels, llama, alpaca, reindeer) | stews, soups; haggis | Toughness +1; eases a hangover | - | 70 | - |
| rennet stomach | suckling calf, lamb or kid only | - | Health +1 (indigestion) | cheese-making: each kind's rennet suits its own milk | 50 | cheese-makers' orders |
| gizzard | birds | confit, giblets | Toughness +2 from the dried lining (digestion) | - | 80 | - |
| blood | all (needs an empty bottle) | black pudding, blood soup | Health +2 (blood loss) | sacrifice, rituals (Aura) | 80 | rituals |
| bile | cattle, buffalo, yak, sheep, goat, pig, most birds; never horse, donkey, mule, camels, llama, alpaca, reindeer, pigeon (no gall bladder) | - | Will +2 (TCM: seat of courage) | ox gall soap, watercolour | 40 | painters, launderers |
| gallstone (ox bezoar) | old cattle, buffalo, yak | - | Health +3 (fever, convulsions) | worth a fortune | 1 | prime quest target |
| bezoar | goats (wild best), camels, llama, alpaca | - | cures poison (universal antidote) | status jewel, set in rings | 1 | prime quest target |
| testicles | intact males only | lamb fries, criadillas | Strength +2 (virility) | - | 95 | - |
| marrow | mammals' long bones | roasted bones, ossobuco | Toughness +1 (restorative) | - | 90 | - |
| os cordis | old cattle (a bone in the heart) | - | Aura +2 (amulet) | - | 2 | prime quest target |
| wishbone | birds | - | Aura +1 (luck) | wishing, oracles | 90 | children's errands |
| shoulder blade | sheep, cattle, goat | - | Mana +1 (scapulimancy) | hoe blade, divination | 90 | seers |
| antler velvet | reindeer while antlers grow | - | Strength +2 (tonic) | - | 60 | seasonal |
| scent glands | billy goat | - | Presence +1 (love charm) | - | 20 | - |
| thyroid | pig, cattle, sheep | - | Health +2, Agility +1 (organotherapy) | - | 20 | physicians |
| cockscomb | cock only | garnish | Presence +1 | - | 95 | - |
| lungs | mammals | haggis, faggots (Poor) | - | - | 70 | - |
| bladder | mammals | - | - | a waterproof pouch, a ball | 50 | - |
| intestines | all | sausage casings, chitterlings | - | catgut: bow and harp strings, sutures (Health +2 bandage) | 60 | - |

### Fats

| Fat | From | Notes |
| --- | ---- | ----- |
| leaf lard | pig | the best lard, from round the kidneys |
| lard | pig | fatback and the rest |
| suet | cattle, sheep, goat | hard kidney fat; rendered into tallow |
| tallow | rendered suet | candles, soap, leather dressing |
| goose fat | goose | the finest of fats; chest rub |
| duck fat | ducks | confit |
| schmaltz | chicken | cooking |
| hump fat | camels | prized cooking fat, salve for aches |
| fat tail | fat-tailed sheep | cooking fat |

### Other parts

| Part | From | Uses |
| ---- | ---- | ---- |
| bones | all | bone tools (needles, awls, combs, handles, dice, flutes), bone meal (fertiliser), bone char (a filter: decolours sugar, cleans water of poisons), bone ash, stock and broth, glue |
| horn | cattle, buffalo, yak, sheep, goats | drinking horns (the game has them as vessels), horn bows with sinew, combs, handles, buttons, spoons, windows, glue |
| antlers | reindeer (shed each year: found as well as butchered) | the best bone for tools; hartshorn jelly |
| hooves | horse, cattle, donkey, camels | glue |
| tusks | boar | carving, charms |
| sinew | mammals | thread, bowstrings, bow backing, lashings; shrinks tight as it dries |
| feathers | birds | fletching, quill pens (goose), stuffing, ornament (guineafowl spots, turkey) |
| down | goose (best), duck | bedding, quilted padding |
| hair | horse tail and mane, yak tail, pig bristles, cattle | bow strings for instruments, brushes, rope, fly whisks |
| wool and fleece | sheep, goat (mohair, cashmere), alpaca, llama, yak down, camel, Angora rabbit | spun into yarn for cloth; lambswool finest, old rams coarsest |

### Looks (for the models, PRs 2 to 6)

| Animal | Sexes differ | How | Castrate | Young |
| ------ | ------------ | --- | -------- | ----- |
| Cattle | marked | bull: bigger, heavy neck and dewlap, thick short horns; cow: udder, finer lyre horns | ox: tall and rangy, used for draught | leggy calf, big ears, horn buds |
| Highland cattle | minor | bull: heavier, horns thicker and level; cow: longer upswept horns, udder | steer between the two | fluffy calf, no horns |
| Water buffalo | minor | bull: heavier neck, thick horn bases; cow: udder | bullock like a cow in build | hairier calf; swamp calves grey |
| Yak | marked | bull: far bigger, wide sweeping horns, big hump, ground-length skirt; cow: small upright horns | steer near bull frame | woolly calf, no hump or skirt |
| Sheep | marked | ram: big spiral horns (by breed), heavy head; ewe: small or no horns, udder | wether near ewe build | curly lamb, long tail, horn buds |
| Goat | minor | buck: bigger horns, fuller beard and mane, smell; doe: udder | wether tall and lean | big-eared kid with horn buds |
| Pig | minor | boar: tusks, bigger head and shoulders, bristly mane; sow: teat rows | barrow like a sow | piglets; wild boarlets striped |
| Horse | minor | stallion: crested neck, heavier forehand; mare: finer neck, udder | gelding like a mare | gangly foal, fuzzy coat, short mane; greys born dark |
| Donkey | minor | jack: heavier neck and jaw; jenny: udder | - | fluffy foal with huge ears |
| Mule | none | one model for both | john usually gelded | gangly foal, long ears |
| Dromedary | minor | bull: bigger, thick neck, pink palate sac in rut; cow: udder | - | leggy pale calf, small hump |
| Bactrian camel | minor | bull: heavier mane and beard; cow: udder | gelding large | fluffy pale calf, floppy humps |
| Llama | none | one model; males have fighting teeth | - | leggy fluffy cria |
| Alpaca | none | one model; colour and Huacaya/Suri fleece instead | - | leggy fluffy cria |
| Reindeer | marked | bull: big branching antlers, shed after the rut, pale neck mane; cow: small antlers kept through winter | härk: large, keeps antlers | plain reddish-brown calf, antler buds |
| Rabbit | minor | doe: dewlap; buck: broader head | - | blind hairless kits, then fluffy round-faced young |
| Chicken | strong | cock: comb and wattles large, hackles, sickle tail, spurs, upright; hen: small comb, round, plain | capon: plump, smaller comb | yellow downy chick, gangly pullet |
| Duck | strong | drake (mallard colours): green head, neck ring, chestnut breast, curled tail; duck: speckled brown; white breeds alike | - | yellow and brown downy duckling |
| Muscovy duck | strong | drake twice the size, red caruncled face, bill knob, crest; duck: feathered face | - | yellow downy duckling |
| Goose | minor | gander taller, thicker neck, bigger bill knob (Chinese geese) | - | yellow-grey downy gosling |
| Turkey | strong | tom: twice the size, breast beard, long snood, fanned tail display, spurs; hen: smaller, plainer | - | buff downy poult, bare head |
| Guineafowl | none | one model; cock's wattles slightly bigger | - | striped brown keet |
| Pigeon | none | one model; cock puffs and bows to court | - | sparse yellow-down squab |
| Quail | minor | one mesh; cock rufous breast and cheeks, hen spotted pale breast (texture) | - | striped chick |
| Honeybee | none | never seen one by one: a hive object, bees no bigger than dots (owner, 2026-11-19) | - | - |
| Silkworm | minor | female moth much bulkier; male feathery antennae | - | pale larva with a tail horn, white cocoon |

### Pets (for the pets task, later)

| Animal | Kept as pets |
| ------ | ------------ |
| Cattle | rare: miniature zebu; sacred cows in India |
| Highland cattle | occasional: kept for looks on estates |
| Water buffalo | rare: prized festival buffalo |
| Yak | rare |
| Sheep | occasional: bottle-raised lambs |
| Goat | common: pygmy goats |
| Pig | occasional: miniature pigs |
| Horse | very common: riding and companions |
| Donkey | common and growing |
| Mule | occasional |
| Dromedary | rare |
| Bactrian camel | rare |
| Llama | common in Europe and America |
| Alpaca | common on hobby farms |
| Reindeer | rare |
| Rabbit | very common: third most popular pet in the UK |
| Chicken | common: named backyard hens, silkies, bantams |
| Duck | fairly common |
| Muscovy duck | occasional |
| Goose | occasional: hand-raised Chinese geese |
| Turkey | rare |
| Guineafowl | occasional: tick-eaters |
| Pigeon | common: fancy breeds and racing |
| Quail | growing |
| Honeybee | hobby beekeeping |
| Silkworm | classroom |

