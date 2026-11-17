# Research: fruit and nuts

Research for fruit plants and nuts (2026-11-17), approved by the owner. The game's data is `crates/shared/src/fruit/fruits.ron` (`shared::fruit`, `docs/context/shared/fruit.md`); this page is the summary and the reasons. The full review page, with every finding and its quote, was a private artifact; the scripts and data stayed outside the repo.

## Method and sources

- 73 fruits across the nine biomes of the climate (`climate::Biome`), at least two a biome, and 22 nuts as a second set; 6 herbs that bear fruit (bilberry, elder, hawthorn, juniper, bearberry, wintergreen) researched as fruit too.
- One Wikipedia article each, read by Claude for facts with word-for-word quotes, every quote checked by a script against the article; where an article was silent, Claude estimated from general knowledge (59% of values are backed by a quote; yields, unit masses and wild berries' calories are mostly estimates). Folk uses only: whether they work in real life doesn't matter, and nothing here is medical advice.
- Effects only from eating the fruit or nut itself, and only where a source backs it or it is well known; remedies from bark, leaves or roots, and drinks, left out.
- How common, from the sources' words: abundant or common is Common, local or scarce Uncommon, rare or endangered Rare, never wild Orchard only. Wild plants are very rare overall: this only says which are rarer still.
- Harvest: a mature plant's yield in servings a year, put into tiers of 1, 1-2, 1-3, 2-4 or 3-6 items a full harvest.
- Temperatures are the climate code's measure (yearly average °C), from each article's climate answer; a few were widened by hand so every biome grows some fruit at its hottest and coldest (crowberry and bog blueberry -11 to 9, date to 40, coconut to 40, avocado into temperate rainforest).
- Looks (shape, size, ripe and unripe colour, skin) were turned from words into values by a script, for the pictures and models (first guesses, to adjust by eye).

## Owner's decisions (2026-11-17)

- Keep all 95 fruits and nuts.
- Item sizes: a unit under 25 kcal comes as a handful (100 g of fruit, or 20 g of nut kernels); otherwise one fruit. A fruit over 300 kcal is picked whole and split into a stack of servings when eaten, one eaten (slice models needed).
- Healing: 1 Health a second for kcal ÷ 4 seconds (100 kcal: 25 Health); an apple is 94 kcal, 23 Health over 23 s. The 25 exceptional for health heal 1.5 times as long.
- Harmful raw (rowan, olive, medlar, breadfruit, acorn, cashew, candlenut, shea nut, elder, juniper): eaten raw they heal nothing, and their harm applies. Kola nut stays, as a stimulant.
- Effects last as long as the healing would.
- Bilberry, elder, hawthorn, juniper, bearberry and wintergreen stay herbs for potions, and get the fruit's healing and effects too (as well as the raw herb's dose, read as "both").
- Growth: four stages (young, leafy, flowering, fruiting, with the fruit unripe then ripe); picking puts the plant back to leafy; speed by plant form (low plants and canes fast, shrubs and vines medium, trees slow, giants very slow).
- Harvest tiers of 1 to 6 items, with Orchards and Foraging adding. Wild plants are very rare; farms get orchards (farm generators to be tweaked later; vegetables come next).
- Drinks (wine, cider, liqueur, beer) are left for brewing, in the cooking tasks.

## The fruit and nuts

Item: what one item is. kcal: one item (a whole fruit all of it). Health: one serving, × 1.5 for the exceptional (*). Effects: Str(ength), Agi(lity), Tou(ghness), Int(ellect), Per(ception), Wil(l), Pre(sence); Health − drains it.

| Name | Latin | Set | Biomes | °C | Item | kcal | Health | Effects | How common | Harvest | Plant |
| ---- | ----- | --- | ------ | -- | ---- | ---- | ------ | ------- | ---------- | ------- | ----- |
| Cloudberry | *Rubus chamaemorus* | Fruit | Tundra, BorealForest | -8 to 6 | Handful | 46 | 17* | Tou +1 | Common | 1 | low herb |
| Crowberry | *Empetrum nigrum* | Fruit | Tundra, BorealForest | -11 to 9 | Handful | 47 | 12 | - | Common | 1 | low mat |
| Lingonberry | *Vaccinium vitis-idaea* | Fruit | Tundra, BorealForest | -10 to 7 | Handful | 54 | 14 | Tou +1 | Common | 1 | low mat |
| Bog blueberry | *Vaccinium uliginosum* | Fruit | Tundra, BorealForest | -11 to 9 | Handful | 55 | 14 | - | Common | 1 | low mat |
| Arctic bramble | *Rubus arcticus* | Fruit | Tundra, BorealForest | -8 to 6 | Handful | 44 | 11 | - | Common | 1 | low herb |
| Cranberry | *Vaccinium oxycoccos* | Fruit | BorealForest, Tundra | -5 to 8 | Handful | 45 | 11 | Tou +1 | Common | 1 | low mat |
| Rowan | *Sorbus aucuparia* | Fruit | BorealForest, Woodland | -2 to 10 | Handful | 46 | 0 (harmful raw) | Health -1 | Common | 1-3 | broadleaf tree |
| Sea buckthorn | *Hippophae rhamnoides* | Fruit | BorealForest, Grassland | 0 to 12 | Handful | 71 | 27* | Tou +1 | Common | 1-3 | shrub |
| Saskatoon | *Amelanchier alnifolia* | Fruit | BorealForest, Grassland | -2 to 10 | Handful | 83 | 21 | - | Common | 1-3 | shrub |
| Blueberry | *Vaccinium corymbosum* | Fruit | BorealForest, SeasonalForest | 4 to 15 | Handful | 57 | 21* | Per +1 | Common | 1-3 | shrub |
| Wild strawberry | *Fragaria vesca* | Fruit | Woodland, BorealForest, SeasonalForest | 0 to 15 | Handful | 37 | 9 | - | Common | 1 | low herb |
| Chokecherry | *Prunus virginiana* | Fruit | Grassland, Woodland | 0 to 15 | Handful | 44 | 11 | - | Common | 1-2 | shrub |
| Wild plum | *Prunus americana* | Fruit | Grassland, Woodland | 2 to 20 | Handful | 39 | 10 | - | Common | 1-3 | shrub |
| Buffaloberry | *Shepherdia argentea* | Fruit | Grassland | 2 to 12 | Handful | 51 | 13 | - | Common | 1-2 | shrub |
| Sloe | *Prunus spinosa* | Fruit | Grassland, Woodland | 4 to 15 | Handful | 41 | 10 | - | Common | 1-2 | shrub |
| Apricot | *Prunus armeniaca* | Fruit | Grassland, Desert | 8 to 16 | Handful | 44 | 11 | - | Uncommon | 1-3 | broadleaf tree |
| Melon | *Cucumis melo* | Fruit | Grassland, Desert | 18 to 30 | One | 246 | 62 | - | Uncommon | 1-2 | ground vine |
| Rose hip | *Rosa canina* | Fruit | Grassland, Woodland | 5 to 15 | Handful | 97 | 36* | Tou +1, Health -1 | Common | 1-2 | shrub |
| Blackberry | *Rubus fruticosus* | Fruit | Woodland, SeasonalForest | 7 to 16 | Handful | 43 | 11 | - | Common | 1-2 | canes |
| Raspberry | *Rubus idaeus* | Fruit | Woodland, BorealForest | 3 to 14 | Handful | 53 | 13 | - | Common | 1-2 | canes |
| Crab apple | *Malus sylvestris* | Fruit | Woodland, SeasonalForest | 5 to 12 | Handful | 66 | 17 | - | Rare | 1-3 | broadleaf tree |
| Olive | *Olea europaea* | Fruit | Woodland | 15 to 20 | Handful | 117 | 0 (harmful raw) | Health -1 | Common | 1-3 | broadleaf tree |
| Fig | *Ficus carica* | Fruit | Woodland, Desert | 12 to 22 | One | 36 | 9 | - | Common | 1-3 | broadleaf tree |
| Grape | *Vitis vinifera* | Fruit | Woodland, SeasonalForest | 10 to 20 | Handful | 67 | 17 | - | Uncommon | 1-3 | vine |
| Pomegranate | *Punica granatum* | Fruit | Woodland, Grassland, Desert | 13 to 25 | One | 137 | 34 | - | Uncommon | 1-3 | shrub |
| Quince | *Cydonia oblonga* | Fruit | Woodland | 8 to 18 | One | 170 | 42 | - | Uncommon | 1-3 | broadleaf tree |
| Medlar | *Mespilus germanica* | Fruit | Woodland | 8 to 15 | Handful | 38 | 0 (harmful raw) | Health -1 | Uncommon | 1-3 | broadleaf tree |
| Apple | *Malus domestica* | Fruit | SeasonalForest, Woodland | 5 to 15 | One | 94 | 23 | - | Uncommon | 2-4 | broadleaf tree |
| Pear | *Pyrus communis* | Fruit | SeasonalForest, Woodland | 7 to 15 | One | 100 | 25 | - | Uncommon | 2-4 | broadleaf tree |
| Cherry | *Prunus avium* | Fruit | SeasonalForest, Woodland | 7 to 14 | Handful | 56 | 14 | - | Common | 1-3 | broadleaf tree |
| Plum | *Prunus domestica* | Fruit | SeasonalForest, Woodland | 7 to 14 | Handful | 43 | 11 | - | Orchard only | 1-3 | broadleaf tree |
| Pawpaw | *Asimina triloba* | Fruit | SeasonalForest | 8 to 18 | One | 132 | 33 | Int -1 | Common | 1-3 | broadleaf tree |
| Persimmon | *Diospyros virginiana* | Fruit | SeasonalForest | 8 to 20 | Handful | 98 | 25 | Health -1 | Common | 1-3 | broadleaf tree |
| Mulberry | *Morus alba* | Fruit | SeasonalForest, Woodland | 8 to 25 | Handful | 42 | 10 | - | Common | 1-3 | broadleaf tree |
| Gooseberry | *Ribes uva-crispa* | Fruit | SeasonalForest, BorealForest | 3 to 12 | Handful | 43 | 11 | - | Common | 1-2 | shrub |
| Blackcurrant | *Ribes nigrum* | Fruit | SeasonalForest, BorealForest | 0 to 10 | Handful | 61 | 23* | Tou +1 | Common | 1-2 | shrub |
| Redcurrant | *Ribes rubrum* | Fruit | SeasonalForest, BorealForest | 2 to 12 | Handful | 55 | 14 | - | Common | 1-2 | shrub |
| Salmonberry | *Rubus spectabilis* | Fruit | TemperateRainforest | 4 to 11 | Handful | 44 | 11 | - | Common | 1 | canes |
| Huckleberry | *Vaccinium parvifolium* | Fruit | TemperateRainforest, BorealForest | 5 to 12 | Handful | 44 | 11 | - | Common | 1 | shrub |
| Thimbleberry | *Rubus parviflorus* | Fruit | TemperateRainforest | 2 to 12 | Handful | 49 | 12 | - | Common | 1 | shrub |
| Salal | *Gaultheria shallon* | Fruit | TemperateRainforest | 8 to 14 | Handful | 58 | 15 | - | Common | 1-2 | shrub |
| Oregon grape | *Mahonia aquifolium* | Fruit | TemperateRainforest, SeasonalForest | 5 to 14 | Handful | 33 | 8 | - | Common | 1-2 | shrub |
| Hardy kiwi | *Actinidia arguta* | Fruit | TemperateRainforest, SeasonalForest | 2 to 14 | Handful | 58 | 22* | - | Common | 1-3 | vine |
| Date | *Phoenix dactylifera* | Fruit | Desert | 13 to 40 | One | 30 | 7 | Str +1 | Uncommon | 2-4 | palm |
| Prickly pear | *Opuntia ficus-indica* | Fruit | Desert, Savanna, Grassland | 15 to 25 | One | 34 | 8 | Health -1 | Common | 1-3 | pad cactus |
| Saguaro fruit | *Carnegiea gigantea* | Fruit | Desert | 18 to 24 | One | 30 | 8 | - | Common | 1-3 | column cactus |
| Jujube | *Ziziphus jujuba* | Fruit | Desert, Grassland | 10 to 22 | Handful | 70 | 26* | Wil +1 | Common | 1-3 | broadleaf tree |
| Quandong | *Santalum acuminatum* | Fruit | Desert, Savanna | 15 to 25 | Handful | 50 | 19* | - | Common | 1-3 | broadleaf tree |
| Watermelon | *Citrullus lanatus* | Fruit | Desert, Savanna | 18 to 30 | Whole, 11 servings | 1080 | 25 | Tou +1 | Uncommon | 1-3 | ground vine |
| Carob | *Ceratonia siliqua* | Fruit | Desert, Woodland | 15 to 22 | One | 35 | 9 | - | Common | 2-4 | broadleaf tree |
| Baobab | *Adansonia digitata* | Fruit | Savanna | 20 to 30 | One | 170 | 64* | Tou +1 | Common | 2-4 | broadleaf tree |
| Marula | *Sclerocarya birrea* | Fruit | Savanna | 18 to 28 | Handful | 28 | 10* | Tou +1 | Common | 2-4 | broadleaf tree |
| Tamarind | *Tamarindus indica* | Fruit | Savanna, TropicalRainforest | 22 to 30 | Handful | 102 | 25 | - | Common | 2-4 | broadleaf tree |
| Jackalberry | *Diospyros mespiliformis* | Fruit | Savanna | 20 to 28 | Handful | 78 | 19 | - | Common | 2-4 | broadleaf tree |
| Kakadu plum | *Terminalia ferdinandiana* | Fruit | Savanna | 25 to 29 | Handful | 36 | 14* | Tou +2 | Common | 1-3 | broadleaf tree |
| Guava | *Psidium guajava* | Fruit | Savanna, TropicalRainforest | 18 to 28 | One | 92 | 34* | Tou +1 | Common | 1-3 | broadleaf tree |
| Wild custard apple | *Annona senegalensis* | Fruit | Savanna | 17 to 30 | Handful | 38 | 10 | - | Common | 1-2 | shrub |
| Banana | *Musa acuminata* | Fruit | TropicalRainforest, Savanna | 22 to 30 | One | 87 | 22 | Agi +1 | Uncommon | 1-3 | giant herb |
| Mango | *Mangifera indica* | Fruit | TropicalRainforest, Savanna | 21 to 30 | One | 143 | 36 | - | Uncommon | 2-4 | broadleaf tree |
| Papaya | *Carica papaya* | Fruit | TropicalRainforest, Savanna | 21 to 30 | One | 271 | 102* | - | Uncommon | 1-3 | giant herb |
| Durian | *Durio zibethinus* | Fruit | TropicalRainforest | 24 to 30 | Whole, 7 servings | 647 | 23 | Pre -1 | Uncommon | 2-4 | broadleaf tree |
| Rambutan | *Nephelium lappaceum* | Fruit | TropicalRainforest | 22 to 30 | Handful | 31 | 8 | - | Common | 1-3 | broadleaf tree |
| Mangosteen | *Garcinia mangostana* | Fruit | TropicalRainforest | 24 to 28 | One | 18 | 4 | - | Uncommon | 1-3 | broadleaf tree |
| Jackfruit | *Artocarpus heterophyllus* | Fruit | TropicalRainforest | 19 to 29 | Whole, 34 servings | 3325 | 24 | - | Uncommon | 3-6 | broadleaf tree |
| Lychee | *Litchi chinensis* | Fruit | TropicalRainforest | 20 to 26 | Handful | 43 | 11 | - | Uncommon | 2-4 | broadleaf tree |
| Passion fruit | *Passiflora edulis* | Fruit | TropicalRainforest, Savanna | 18 to 28 | One | 26 | 7 | - | Uncommon | 1-3 | vine |
| Breadfruit | *Artocarpus altilis* | Fruit | TropicalRainforest | 24 to 28 | Whole, 16 servings | 1545 | 0 (harmful raw) | Health -1 | Orchard only | 2-4 | broadleaf tree |
| Avocado | *Persea americana* | Fruit | TropicalRainforest, SeasonalForest | 16 to 26 | One | 224 | 84* | - | Uncommon | 2-4 | broadleaf tree |
| Açaí | *Euterpe oleracea* | Fruit | TropicalRainforest | 24 to 28 | Handful | 21 | 5 | - | Common | 1-3 | palm |
| Cacao pod | *Theobroma cacao* | Fruit | TropicalRainforest | 21 to 30 | One | 112 | 28 | - | Uncommon | 1-2 | broadleaf tree |
| Pineapple | *Ananas comosus* | Fruit | TropicalRainforest, Savanna | 20 to 27 | Whole, 6 servings | 550 | 23 | - | Uncommon | 1-2 | rosette |
| Orange | *Citrus × sinensis* | Fruit | TropicalRainforest, Woodland, Savanna | 17 to 27 | One | 65 | 24* | Tou +1 | Orchard only | 1-3 | broadleaf tree |
| Lemon | *Citrus × limon* | Fruit | Woodland, Savanna | 15 to 25 | One | 24 | 9* | Tou +1 | Orchard only | 1-3 | broadleaf tree |
| Siberian pine nut | *Pinus sibirica* | Nut | BorealForest | -5 to 4 | Handful | 134 | 34 | - | Common | 2-4 | conifer |
| Stone pine nut | *Pinus pinea* | Nut | Woodland | 13 to 19 | Handful | 114 | 28 | - | Common | 2-4 | conifer |
| Hazelnut | *Corylus avellana* | Nut | Woodland, SeasonalForest, BorealForest | 5 to 15 | Handful | 126 | 47* | Int +1 | Common | 1-3 | shrub |
| Beaked hazelnut | *Corylus cornuta* | Nut | TemperateRainforest, BorealForest | 0 to 12 | Handful | 126 | 32 | - | Common | 1-3 | shrub |
| Acorn | *Quercus robur* | Nut | Woodland, SeasonalForest | 5 to 14 | Handful | 77 | 0 (harmful raw) | Health -1 | Common | 2-4 | broadleaf tree |
| Walnut | *Juglans regia* | Nut | SeasonalForest, Woodland | 8 to 16 | Handful | 131 | 49* | Int +1 | Uncommon | 2-4 | broadleaf tree |
| Sweet chestnut | *Castanea sativa* | Nut | SeasonalForest, Woodland | 8 to 15 | Handful | 43 | 11 | - | Common | 2-4 | broadleaf tree |
| Beechnut | *Fagus sylvatica* | Nut | SeasonalForest | 6 to 12 | Handful | 115 | 29 | - | Common | 1-3 | broadleaf tree |
| Pecan | *Carya illinoinensis* | Nut | SeasonalForest | 12 to 21 | Handful | 138 | 34 | - | Common | 2-4 | broadleaf tree |
| Hickory nut | *Carya ovata* | Nut | SeasonalForest | 5 to 18 | Handful | 131 | 33 | - | Common | 2-4 | broadleaf tree |
| Chinquapin | *Chrysolepis chrysophylla* | Nut | TemperateRainforest | 7 to 14 | Handful | 76 | 19 | - | Common | 1-3 | broadleaf tree |
| Almond | *Prunus dulcis* | Nut | Grassland, Woodland, Desert | 14 to 20 | Handful | 116 | 43* | - | Rare | 2-4 | broadleaf tree |
| Pistachio | *Pistacia vera* | Nut | Grassland, Desert | 14 to 20 | Handful | 112 | 42* | - | Uncommon | 2-4 | broadleaf tree |
| Mongongo nut | *Schinziophyton rautanenii* | Nut | Desert, Savanna | 19 to 24 | Handful | 122 | 46* | - | Common | 3-6 | broadleaf tree |
| Shea nut | *Vitellaria paradoxa* | Nut | Savanna | 24 to 32 | Handful | 120 | 0 (harmful raw) | Health -1 | Common | 2-4 | broadleaf tree |
| Cashew | *Anacardium occidentale* | Nut | Savanna, TropicalRainforest | 24 to 28 | Handful | 111 | 0 (harmful raw) | Health -2 | Common | 2-4 | broadleaf tree |
| Brazil nut | *Bertholletia excelsa* | Nut | TropicalRainforest | 24 to 27 | Handful | 132 | 49* | Tou +1, Health -1 | Uncommon | 3-6 | broadleaf tree |
| Coconut | *Cocos nucifera* | Nut | TropicalRainforest, Savanna | 24 to 40 | Whole, 13 servings | 1204 | 23 | - | Uncommon | 2-4 | palm |
| Macadamia | *Macadamia integrifolia* | Nut | TropicalRainforest | 18 to 25 | Handful | 148 | 37 | - | Uncommon | 2-4 | broadleaf tree |
| Candlenut | *Aleurites moluccanus* | Nut | TropicalRainforest | 20 to 28 | Handful | 124 | 0 (harmful raw) | Health -2 | Common | 3-6 | broadleaf tree |
| Kola nut | *Cola nitida* | Nut | TropicalRainforest, Savanna | 24 to 28 | One | 21 | 5 | Per +2, Wil -1 | Uncommon | 2-4 | broadleaf tree |
| Pili nut | *Canarium ovatum* | Nut | TropicalRainforest | 24 to 28 | Handful | 144 | 36 | - | Common | 3-6 | broadleaf tree |
| Bilberry | *Vaccinium myrtillus* | Herb fruit | BorealForest, SeasonalForest, Tundra | -2 to 10 | Handful | 45 | 17* | Per +1, Pre -1 | Common | 1 | low mat |
| Elder | *Sambucus nigra* | Herb fruit | SeasonalForest, Woodland, Grassland | 5 to 14 | Handful | 66 | 0 (harmful raw) | Health -1 | Common | 1-3 | shrub |
| Hawthorn | *Crataegus monogyna* | Herb fruit | SeasonalForest, Woodland, Grassland | 4 to 15 | Handful | 68 | 17 | Tou +1 | Common | 1-3 | shrub |
| Juniper | *Juniperus communis* | Herb fruit | BorealForest, Tundra, Grassland, Woodland | -8 to 14 | Handful | 202 | 0 (harmful raw) | Health -1 | Common | 1-2 | conifer |
| Bearberry | *Arctostaphylos uva-ursi* | Herb fruit | Tundra, BorealForest, Woodland | -8 to 12 | Handful | 70 | 17 | - | Common | 1 | low mat |
| Wintergreen | *Gaultheria procumbens* | Herb fruit | BorealForest, SeasonalForest | 2 to 14 | Handful | 46 | 12 | Pre +1 | Common | 1 | low mat |
