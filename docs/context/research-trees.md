# Research: trees and their woods

Research for the woodcutting plan (Fell Tree, logs of 30 × 30 × 100 cm, Cut Kindling, tree models, log pictures). It covers which wild trees to roll by climate, the wood of every fruit and nut tree with a trunk, each wood's seasoned weight, colours and bark, and tree sizes and shapes. It was done on 2026-11-18 as a non-code task, for the plan in `docs/context/woodcutting.md`, which records what was decided from it. The bark photos (for PR 4) are kept outside the repo, in `C:\Projects\research\woodcutting\bark\` (credits in `bark/credits.md`).

## Sources and how figures were reached

- **Density (seasoned, about 12% moisture):** for 58 woods it is The Wood Database's "Average Dried Weight", given at 12% (https://www.wood-database.com/<slug>/, the slugs are in the tables). For the rest it comes from PROTA/PROSEA, World Agroforestry, Wikipedia and papers (listed under "Densities found elsewhere" at the end). PROSEA figures are at 15% moisture, a few per cent above the 12% value. Where only a basic density (oven-dry mass ÷ green volume) was found, air-dry ≈ basic ÷ 0.83 (Chave et al. 2006's ratio). The **Src** column says which: **WD** for The Wood Database, **P** for PROTA/PROSEA, **L** for a paper, **W** for Wikipedia, **G** for a genus or related species used as a stand-in, **E** for Claude's estimate.
- **Log weight** = density × 0.09 m³ (one 30 × 30 × 100 cm log), rounded to whole kg. The game's materials keep weight in tenths of g/cm³ (`material/properties.rs`: Oak 7, Pine 5, Cherry and Chestnut 6, Acacia 8), so each density is also given that way.
- **Heartwood colours** are sRGB values Claude picked from The Wood Database's descriptions (quoted in short) and general knowledge. They are estimates for a picture, not measurements.
- **End grain:** porosity and ring notes come from The Wood Database's "Endgrain" field where it has one. Ring contrast is graded *strong*, *medium*, *faint* or *none*, from that note.
- **Bark** descriptions are Claude's, checked against the photos found. A **bark sRGB** for each wood can be measured from its photos by script once they are downloaded (see Bark pictures).
- **Tree sizes** are typical mature sizes, not records. The Wood Database's "Tree Size" ranges are tall forest-grown figures, so the lower half of each range was used. Fruit trees use the heights already in `fruits.ron` (`plant_look.height`), with trunk diameters estimated.

## Logs per tree

A real felled tree holds far more than 12 logs of 0.09 m³ (an oak 28 m tall with a 1.2 m trunk holds about 140), so the number is squeezed:

- stem volume V = 0.4 × π/4 × d² × h (form factor 0.4 for a tapering stem, branches left out)
- **logs = clamp(round(1.5 × log₂(V ÷ 0.09)), 1, 12)**

That gives 1 for small orchard trees (apple, quince, almond, lemon, juniper), 2 to 5 for most fruit trees, 7 to 11 for forest trees, and 12 for giants (Douglas fir, red cedar, cedar of Lebanon, mahogany, kapok, Brazil nut, chestnut, baobab). The figures are in the tables (**Logs**). A tree that is not full grown could scale the number down by its stage.

## Wild trees by climate

The world's `Cover` gives `trees` (how wooded), `narrow` (how conifer-like) and `lush` (how rainforest-like). The climate gives a `Biome` and a temperature in °C. Suggested roll for each wild `Cell::Tree`, from the position hash:

1. **Group:** conifer with chance `narrow`, otherwise broadleaf. A palm group joins in the warm and dry or the warm and wet (below).
2. **Kinds in the group** suiting the biome and yearly temperature (the **°C** range), weighted by **How common**: common 4, uncommon 2, rare 1. A kind slightly outside its range could count at half weight, so borders blend.
3. **Wet ground:** willow and alder only within a few cells of a river or the shore, where they are common. Elsewhere they are rare.
4. **`lush` above about 0.5** in warm places picks from the tropical rainforest kinds. In cool places it picks from the temperate rainforest ones.
5. **Fruit and nut trees** that are `Common` or `Uncommon` wild (`fruits.ron` `abundance`) can join the same roll as broadleaf, conifer or palm kinds, at their own weight. Then wild apples, oaks (acorns), beeches, walnuts, chestnuts, baobabs, marulas, coconuts and dates are the same trees as the orchards' and give the same wood.

The **core 20** cover every biome. The six **extras** add variety if wanted.

| Tree | Latin | Group | Biomes | °C (yearly) | Moisture | How common | Crown | Core |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Scots pine | *Pinus sylvestris* | conifer | BorealForest, Woodland, Grassland edges | −3 to 14 | dry to middling | common | young conical, old a tall bare trunk under a flat open top | core |
| Norway spruce | *Picea abies* | conifer | BorealForest, TemperateRainforest (cool) | −6 to 10 | middling to wet | common | narrow spire, drooping branches to the ground | core |
| Silver fir | *Abies alba* | conifer | BorealForest (mild), TemperateRainforest | 2 to 12 | wet | uncommon | regular cone, flat "stork's nest" top when old | core |
| European larch | *Larix decidua* | conifer (sheds needles) | BorealForest, mountains | −8 to 10 | middling | uncommon | open, airy cone; bare in winter | core |
| Douglas fir | *Pseudotsuga menziesii* | conifer | TemperateRainforest | 4 to 14 | wet | common there | very tall cone, long clean trunk | core |
| Western red cedar | *Thuja plicata* | conifer | TemperateRainforest | 4 to 13 | wet | uncommon | cone with drooping sprays, fluted, flared base | core |
| Cedar of Lebanon | *Cedrus libani* | conifer | Woodland (high) | 6 to 16 | dry to middling | rare | wide, in flat horizontal tiers | core |
| Mediterranean cypress | *Cupressus sempervirens* | conifer | Woodland, Desert edges | 12 to 20 | dry | uncommon | narrow flame-shaped column | extra |
| Silver birch | *Betula pendula* | broadleaf | BorealForest, Woodland, Tundra edge | −6 to 12 | middling | common | narrow, airy oval, weeping twigs | core |
| European aspen | *Populus tremula* | broadleaf | BorealForest, Grassland | −6 to 12 | middling | uncommon | slender, narrow and rounded | extra |
| Oak | *Quercus robur* | broadleaf | SeasonalForest, Woodland | 6 to 16 | middling | common | broad, spreading, crooked limbs | core |
| Beech | *Fagus sylvatica* | broadleaf | SeasonalForest, TemperateRainforest (mild) | 6 to 14 | middling to wet | common | dense high dome | core |
| Ash | *Fraxinus excelsior* | broadleaf | SeasonalForest | 6 to 14 | wet | uncommon | open dome, branches sweeping up | core |
| Sycamore maple | *Acer pseudoplatanus* | broadleaf | SeasonalForest, TemperateRainforest | 5 to 14 | middling to wet | uncommon | broad, dense dome | core |
| Small-leaved lime (linden) | *Tilia cordata* | broadleaf | SeasonalForest | 6 to 14 | middling | uncommon | tall dome to egg shape | extra |
| Wych elm | *Ulmus glabra* | broadleaf | SeasonalForest, Woodland | 5 to 14 | middling | uncommon | broad fan or vase | extra |
| White willow | *Salix alba* | broadleaf (wet) | near water, any temperate biome | 0 to 18 | wet | common by water, else rare | rounded, long hanging twigs | core |
| Black alder | *Alnus glutinosa* | broadleaf (wet) | near water, BorealForest, SeasonalForest | 0 to 14 | wet | common by water, else rare | narrow cone to oval | extra |
| Holm oak | *Quercus ilex* | broadleaf (evergreen) | Woodland | 12 to 20 | dry | common | dense, rounded, dark | core |
| Umbrella thorn acacia | *Vachellia tortilis* | broadleaf (thorn) | Savanna, Desert | 18 to 30 | dry | common | flat-topped umbrella | core |
| Camel thorn | *Vachellia erioloba* | broadleaf (thorn) | Savanna, Desert | 16 to 26 | dry | uncommon | wide umbrella, spreading | extra |
| Honey mesquite | *Prosopis glandulosa* | broadleaf (thorn) | Desert, warm Grassland | 14 to 24 | dry | common in desert | low, spreading, several stems | core |
| Mopane | *Colophospermum mopane* | broadleaf | Savanna | 20 to 28 | dry to middling | common | narrow, upright (mopane "cathedral" trees) | core |
| Teak | *Tectona grandis* | broadleaf | Savanna (wet side), TropicalRainforest (seasonal) | 22 to 28 | middling | uncommon | tall clean trunk, rounded crown | core |
| Mahogany | *Swietenia macrophylla* | broadleaf | TropicalRainforest | 22 to 28 | wet | uncommon | tall, umbrella crown, buttresses | core |
| Kapok | *Ceiba pentandra* | broadleaf (emergent) | TropicalRainforest, Savanna (wet) | 22 to 28 | wet | rare | emergent, flat tiers on a huge buttressed trunk | core |
| Ebony | *Diospyros crassiflora* | broadleaf (under the canopy) | TropicalRainforest | 22 to 28 | wet | rare | narrow, rounded | extra (dark wood) |

Notes:
- **Tundra** has no full trees. Birch, larch, spruce and juniper reach its edge as stunted trees, so a tundra tree could be one of those at a smaller size (1 log).
- **Desert** trees: mesquite, umbrella thorn, cypress at the edges, and the fruit palms (date). Oases already have their own trees, which can be date palms and figs from `fruits.ron`.
- **Palms** (date, coconut, açaí) are the wild palm group: coconut on warm coasts, date in desert and oases, açaí in tropical wet ground.
- Each kind's range fits Earth's. The game's temperatures come from `ClimateSample::celsius()`, so the ranges can be used as they are.

## The woods

One row per wood, with wild trees first. **Logs** uses the size columns and the formula above. **Mat** is the weight as the game's materials keep it (tenths of g/cm³).

### Wild trees' woods

| Wood | Src | kg/m³ | Log kg | Mat | Height m | Trunk m | Logs | Heartwood sRGB | End grain | Bark |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Pine (Scots) | WD `scots-pine` | 550 | 50 | 6 (game: 5) | 25 | 0.6 | 7 | (196, 150, 105) light reddish brown, pale yellow sapwood | resinous conifer, strong late-wood rings | lower trunk grey-brown, deeply furrowed into plates; upper trunk thin, orange, flaking in papery scales |
| Spruce | WD `norway-spruce` | 405 | 36 | 4 | 35 | 0.8 | 9 | (232, 215, 185) creamy white | conifer, medium rings, little colour change | red-brown to grey, thin, in small round scales |
| Fir (silver) | WD `european-silver-fir` | 415 | 37 | 4 | 35 | 0.9 | 10 | (225, 205, 175) white to pale reddish | conifer, medium rings | smooth silver-grey, with resin blisters; old trunks finely cracked into small plates |
| Larch | WD `european-larch` | 575 | 52 | 6 | 30 | 0.7 | 9 | (200, 140, 90) yellow to reddish brown | conifer, strong rings | thick, red-brown to grey, deeply fissured into scaly plates, reddish in the cracks |
| Douglas fir | WD `douglas-fir` | 510 | 46 | 5 | 55 | 1.5 | 12 | (200, 130, 85) orange-brown | conifer, strong contrast of pale early wood and dark late wood | very thick, dark red-brown to grey, deep rough furrows, corky |
| Red cedar (western) | WD `western-red-cedar` | 370 | 33 | 4 | 45 | 2.0 | 12 | (165, 95, 65) reddish to pink-brown | conifer, medium rings | thin, red-brown weathering silver-grey, in long fibrous strips that peel |
| Cedar (of Lebanon) | WD `cedar-of-lebanon` | 520 | 47 | 5 | 30 | 1.5 | 12 | (205, 160, 115) straw to light reddish brown | conifer, medium rings | dark grey, cracked into small scaly plates |
| Cypress | WD `mediterranean-cypress` | 535 | 48 | 5 | 22 | 0.6 | 7 | (205, 165, 115) pale yellowish to reddish brown | conifer, faint to medium rings | thin, grey-brown, in long shallow fibrous ridges |
| Birch | WD `silver-birch` | 640 | 58 | 6 | 20 | 0.4 | 5 | (225, 200, 165) pale, light reddish heart, near white sapwood | diffuse-porous, faint rings | white, smooth and papery, black horizontal pores (lenticels); black, fissured diamonds at the base |
| Aspen | WD `european-aspen` | 450 | 41 | 5 | 18 | 0.45 | 6 | (220, 200, 165) light brown | diffuse-porous, faint rings | smooth grey-green to whitish, black diamond-shaped marks; dark and furrowed at the base |
| Oak | WD `english-oak` | 675 | 61 | 7 (game: 7) | 28 | 1.2 | 11 | (180, 140, 95) light to medium brown, olive cast | ring-porous, strong rings, wide rays | grey to dark grey, thick, deeply furrowed in a blocky, vertical pattern |
| Beech | WD `european-beech` | 710 | 64 | 7 | 30 | 1.0 | 10 | (215, 180, 140) pale straw, pink hue | diffuse-porous, faint rings, small rays | smooth, thin, silver-grey, sometimes faintly ringed |
| Ash | WD `european-ash` | 680 | 61 | 7 | 28 | 0.8 | 9 | (205, 175, 130) light brown, pale sapwood | ring-porous, strong rings | young smooth grey-green; older pale grey in a close net of diamond ridges |
| Maple (sycamore) | WD `sycamore-maple` | 615 | 55 | 6 | 25 | 0.8 | 9 | (230, 215, 185) creamy white (sapwood used) | diffuse-porous, faint rings | smooth grey when young; older pinkish grey, flaking in small square plates that show orange-brown under them |
| Lime (linden) | WD `european-lime` | 535 | 48 | 5 | 28 | 1.0 | 10 | (235, 220, 190) pale white to cream | diffuse-porous, very faint rings | grey-brown, shallow vertical fissures in a fine net |
| Elm | WD `wych-elm` | 605 | 54 | 6 | 25 | 0.8 | 9 | (170, 120, 85) light to medium reddish brown | ring-porous, strong rings | grey-brown, rough, long interlacing ridges |
| Willow | WD `white-willow` | 400 | 36 | 4 | 18 | 0.8 | 8 | (205, 170, 135) tan to pinkish brown | semi-ring-porous, faint rings | grey-brown, deeply and coarsely furrowed in a criss-cross net |
| Alder | WD `european-alder` | 535 | 48 | 5 | 20 | 0.4 | 5 | (200, 140, 100) light tan reddening to orange-brown when cut | diffuse-porous, faint rings | dark grey-brown to almost black, cracked into small square plates |
| Holm oak | WD `holm-oak` | 960 | 86 | 10 | 18 | 0.8 | 8 | (150, 95, 65) medium reddish brown | diffuse to semi-ring-porous, medium rings, wide rays | dark grey to black, finely cracked into small square plates |
| Acacia (umbrella thorn) | E (camel thorn 1185 WD, genus 800 to 1000) | about 900 | 81 | 9 | 8 | 0.4 | 3 | (130, 75, 50) dark reddish brown, yellow sapwood | diffuse-porous, faint rings | dark grey-brown to black, rough, deeply fissured, paired white thorns on twigs |
| Camel thorn | WD `camelthorn` | 1185 | 107 | 12 | 12 | 0.5 | 5 | (110, 55, 40) dark reddish brown | diffuse-porous, faint rings | grey to black, deeply furrowed, rough |
| Mesquite | WD `honey-mesquite` | 820 | 74 | 8 | 7 | 0.3 | 2 | (140, 80, 50) reddish brown, darkening | diffuse-porous, faint to medium rings | dark red-brown to grey, rough, in long shaggy strips |
| Mopane | WD `mopane` | 1075 | 97 | 11 | 15 | 0.5 | 6 | (110, 55, 40) dark reddish brown with black stripes | diffuse-porous, faint rings | dark grey, deeply and roughly fissured in long vertical ridges |
| Teak | WD `teak` | 655 | 59 | 7 | 30 | 0.9 | 10 | (170, 120, 65) golden to medium brown | ring-porous to semi-ring-porous, medium rings | light grey-brown, thin, fibrous, in shallow long fissures, peeling in strips |
| Mahogany | WD `honduran-mahogany` | 590 | 53 | 6 | 40 | 1.3 | 12 | (150, 80, 55) pinkish to dark reddish brown | diffuse-porous, faint rings (fine pale lines) | grey-brown, ridged and flaking in long plates; buttressed base |
| Kapok | P | 240 to 380, say 310 | 28 | 3 | 50 | 2.0 | 12 | (230, 220, 195) creamy white | diffuse-porous, no visible rings | smooth pale grey, young trunks studded with stout conical thorns; huge buttresses |
| Ebony | WD `gaboon-ebony` | 955 | 86 | 10 | 15 | 0.6 | 6 | (30, 25, 22) jet black heart, pale sapwood | diffuse-porous, no visible rings | thin, dark grey to black, rough, flaking in small pieces |

### Fruit and nut trees' woods

Every plant in `fruits.ron` whose family has a trunk (`PlantFamily::has_trunk`: broadleaf trees, conifers, palms, giant herbs and column cacti), grouped where the woods are alike. The **Group** column suggests one material for several trees where they would look and weigh the same in game. Fruit trees' sizes are the orchard sizes from `fruits.ron`.

| Tree (fruits.ron key) | Group | Src | kg/m³ | Log kg | Height m | Trunk m | Logs | Heartwood sRGB | End grain | Bark |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| apple, crab_apple | Apple | WD `apple` | 830 | 75 | 6 to 7 | 0.3 | 1 to 2 | (175, 115, 85) light reddish to deeper red-brown | diffuse-porous, rings distinct | grey-brown, flaking in thin irregular scales |
| pear | Pear | WD `pear` | 690 | 62 | 11.5 | 0.35 | 3 | (200, 145, 115) pale pink to light reddish brown | diffuse-porous, very faint rings | dark grey, cracked into small square blocks ("alligator") |
| quince, medlar | Pear | G (pear; quince and medlar not found) | about 750 | 68 | 3.5 to 5 | 0.15 to 0.2 | 1 | (200, 150, 115) like pear | diffuse-porous, faint | quince: smooth grey-brown, flaking to show orange patches; medlar: grey-brown, scaly, cracked |
| rowan | Pear (Sorbus) | WD `rowan` | 770 | 69 | 10 | 0.3 | 2 | (195, 145, 110) light reddish brown (E) | diffuse-porous, faint | smooth, shiny silver-grey, horizontal lenticels |
| cherry | Cherry (game material) | WD `sweet-cherry` | 600 | 54 | 15 | 0.45 | 5 | (185, 115, 80) pinkish brown darkening to golden brown | semi-ring-porous, medium rings | shiny red-brown to purplish, smooth, ringed by wide horizontal bands of lenticels, peeling sideways in strips |
| plum, apricot, almond | Plum (Prunus) | WD `plum`, `apricot`; almond G | 795 (plum), 745 (apricot) | 72 / 67 | 3.75 to 8.5 | 0.2 to 0.3 | 1 to 2 | (180, 110, 80) plum's mixed brown with pink, red, purple streaks; apricot (190, 130, 90) | semi-ring to diffuse-porous, medium rings | dark grey-brown to purplish, smooth with lenticels when young, cracking into rough scales when old |
| walnut | Walnut | WD `english-walnut` | 640 | 58 | 17.5 | 0.9 | 8 | (110, 75, 50) pale to dark chocolate brown, darker streaks | semi-ring to diffuse-porous, medium rings | smooth olive-grey when young; old silvery grey with broad, deep fissures |
| pecan, hickory_nut | Hickory | WD `pecan`, `shagbark-hickory` | 735, 800 | 66 / 72 | 30 to 33 | 0.5 to 0.9 | 7 to 10 | (175, 125, 85) light to medium brown, reddish hue | ring-porous, strong rings | pecan: grey-brown, narrow scaly ridges; shagbark: grey, in long loose plates curling away from the trunk |
| sweet_chestnut, chinquapin | Chestnut (game material) | WD `sweet-chestnut`; chinquapin G | 590 | 53 | 17.5 to 27.5 | 0.5 to 1.5 | 6 to 12 | (170, 125, 80) light to medium brown, reddening | ring-porous, strong rings | grey-brown, deep fissures twisting in a spiral up the trunk |
| acorn | Oak (game material) | WD `english-oak` | 675 | 61 | 30 | 1.2 | 11 | as Oak above | as Oak | as Oak |
| beechnut | Beech | WD `european-beech` | 710 | 64 | 30 | 1.0 | 10 | as Beech above | as Beech | as Beech |
| olive | Olive | WD `olive` | 980 | 88 | 10 | 0.6 | 5 | (200, 160, 100) yellow to light reddish brown with dark brown to black streaks | diffuse-porous, faint rings, wild dark figure | pale silver-grey, gnarled, twisted, hollow and fissured when old |
| orange, lemon | Citrus | L (BioResources 2020) | 800 to 834 | 73 | 4.5 to 9.5 | 0.15 to 0.25 | 1 to 2 | (230, 205, 130) pale yellow | diffuse-porous, very faint rings | smooth grey-brown to green-grey, thin, thorns on younger growth |
| fig | Fig | L (Turkish trees, dense) | 730 (other sources give fig as light and soft) | 66 | 8.5 | 0.3 | 2 | (225, 210, 170) pale cream (E) | diffuse-porous, faint rings | smooth, pale silver-grey, like elephant skin, with folds at the branch joints |
| mulberry | Mulberry | WD `mulberry` | 690 | 62 | 15 | 0.4 | 5 | (180, 130, 60) golden brown, darkening | ring-porous, strong rings | orange-brown to grey-brown, shallow, twisting, net-like ridges |
| persimmon, jackalberry | Persimmon (Diospyros) | WD `persimmon`; P | 835, 800 to 900 | 75 | 5 to 16.5 | 0.4 | 2 to 5 | (215, 195, 160) wide pale sapwood with a small blackish heart (60, 45, 35) | semi-ring-porous, medium rings | dark grey to black, cracked deeply into small square blocks |
| pawpaw | Light pale wood | W (SG 0.40, probably oven-dry) | about 400 | 36 | 7 | 0.15 | 1 | (220, 215, 160) pale greenish yellow | diffuse-porous, faint | thin, smooth grey-brown with warty lenticels |
| date | Palm | L (basic 320, so about 390) | about 390 | 35 | 22.5 | 0.4 | 5 | (190, 150, 110) brown fibre bundles in a pale, soft body | monocot: no rings, dots of fibre thickest near the rim | covered in old leaf bases in a spiral, cut-off stubs; lower down a grey, diamond-patterned trunk |
| coconut | Palm | WD `red-palm` (coconut palm) | 820 (outer, dense part) | 74 | 20 | 0.35 | 5 | (150, 95, 65) reddish-brown fibres in a tan body | monocot: no rings, dark fibre dots, dense rim, soft core | smooth grey, ringed by old leaf scars, often leaning and curved |
| acai | Palm | L (outer zone 950, core much lighter) | E about 600 whole | 54 | 20 | 0.15 | 1 | (170, 130, 95) fibres in a pale body | monocot | slender, smooth grey, fine leaf-scar rings, several stems together |
| mango | Mango | WD `mango` | 675 | 61 | 35 | 1.0 | 10 | (200, 170, 120) light golden brown, often spalted with dark lines | diffuse-porous, faint | dark grey-brown, thick, cracked into flaking plates, oozes resin |
| jackfruit, breadfruit | Jackwood (Artocarpus) | P / L | 690 (jackfruit), 505 to 645 (breadfruit) | 62 / 52 | 20 | 0.6 | 7 | jackfruit (225, 170, 50) bright yellow, ageing to orange-brown; breadfruit (220, 195, 130) pale | diffuse-porous, faint | grey-brown, fairly smooth, warty, milky sap |
| avocado | Avocado | L (branch wood), G (Persea) | 540 | 49 | 14.5 | 0.5 | 5 | (210, 170, 130) pale pinkish brown (E) | diffuse-porous, faint | grey-green to grey-brown, shallowly cracked |
| lychee, rambutan, mangosteen | Lychee (red tropical fruitwood) | G (Nephelium 615 to 1110, Garcinia 690 to 1120 at 15%; lychee not found) | E about 850 | 77 | 12.5 to 19.5 | 0.35 to 0.5 | 4 to 6 | (165, 100, 75) red-brown; mangosteen darker (120, 60, 45) | diffuse-porous, faint | lychee: smooth grey-black; rambutan: grey-brown, smooth; mangosteen: dark brown to black, rough, yellow latex |
| durian | Durian | P | 420 to 690, say 560 | 50 | 37.5 | 1.0 | 11 | (150, 80, 60) dark red | diffuse-porous, faint | red-brown, rough, flaking irregularly; buttressed |
| cacao_pod | Light pale wood | L (branch wood 410) | about 410 | 37 | 9 | 0.2 | 1 | (225, 205, 175) pale (E) | diffuse-porous, faint | thin, smooth, grey-brown with pale patches; pods grow straight from the trunk (`FruitPosition::Trunk`) |
| guava | Guava | W (oven-dry 670, so about 700) | about 700 | 63 | 6.5 | 0.25 | 1 | (205, 170, 140) pinkish brown (E) | diffuse-porous, faint | smooth, thin, copper-brown, flaking off in patches to show green-grey beneath |
| tamarind | Tamarind | WD `tamarind` | 850 | 77 | 27.5 | 1.0 | 10 | (110, 55, 40) deep reddish brown, purplish; wide yellow sapwood (230, 205, 150) | diffuse-porous, faint | rough, dark grey, closely and deeply fissured |
| carob, jujube | Tamarind (hard red-brown) | L (carob 860), G (Ziziphus 535 to 1180) | 860 | 77 | 7.5 to 15 | 0.3 to 0.6 | 2 to 6 | carob (165, 90, 60) red-brown; jujube (170, 95, 70) | diffuse-porous, faint | carob: smooth grey-brown, rougher when old; jujube: dark grey-brown, rough, furrowed, thorny |
| cashew | Cashew | WD `cashew` | 560 | 50 | 10 | 0.4 | 4 | (205, 165, 120) light reddish brown (E) | diffuse-porous, faint | grey-brown, rough, low, crooked trunk |
| brazil_nut | Brazil nut | delta-intkey | 550 to 700, say 650 | 59 | 40 | 1.8 | 12 | (195, 140, 115) pinkish to light brown | diffuse-porous, faint | grey-brown, deeply fissured in long vertical ridges; straight, unbranched lower trunk |
| macadamia | Macadamia | WD `macadamia-nut` | 705 | 63 | 7 | 0.3 | 2 | (175, 110, 85) pale pink to reddish brown, large rays (lacewood figure) | diffuse-porous, faint rings, big rays | grey-brown, rough, slightly warty |
| candlenut | Light pale wood | L (SG 0.35) | about 350 | 32 | 22.5 | 0.6 | 7 | (235, 225, 200) white | diffuse-porous, faint | smooth grey-brown, faint vertical lines |
| kola_nut | Kola | not found | E about 600 | 54 | 16 | 0.5 | 6 | (210, 180, 110) dull yellow | diffuse-porous, faint | grey-green to grey-brown, smooth to slightly rough |
| pili_nut | Pili (Canarium) | G (390 to 780 at 15%) | E about 550 | 50 | 20 | 0.5 | 6 | (200, 160, 120) buff to reddish brown | diffuse-porous, faint | smooth grey, resinous |
| baobab | Baobab | P (about 210) | 210 | 19 | 15 | 5.0 | 12 | (220, 205, 170) pale, soft, spongy | no clear rings, spongy fibre | smooth, grey-pink to coppery, shiny, often wrinkled and bulging, scarred by elephants |
| marula | Marula | P | 590 | 53 | 13.5 | 0.7 | 7 | (205, 160, 130) pale pinkish to reddish brown | diffuse-porous, faint | grey, mottled, flaking in round patches to show paler bark beneath |
| quandong | Quandong (Santalum) | G (musk sandalwood 940) | E about 900 | 81 | 5 | 0.2 | 1 | (210, 160, 130) flesh-coloured | diffuse-porous, faint | dark grey to black, rough, cracked |
| kakadu_plum | Kakadu plum | G (Terminalia 640) | 640 | 58 | 10 | 0.3 | 2 | (200, 160, 110) pale brown (E) | diffuse-porous, faint | pale grey to cream, flaky, soft and corky |
| shea_nut | Shea | P | 720 | 65 | 11 | 0.6 | 6 | (170, 100, 70) reddish brown (E) | diffuse-porous, faint | thick, dark grey, deeply cracked into square, corky blocks (protects it from bush fires) |
| mongongo_nut | Light pale wood | W (SG 0.15) | 150 to 250, say 200 | 18 | 17.5 | 0.7 | 7 | (230, 215, 170) pale yellow to straw | diffuse-porous, faint | smooth pale grey, peeling in thin flakes |
| pistachio | Pistachio | WD `pistachio` (estimate there) | 865 | 78 | 7.5 | 0.3 | 2 | (170, 140, 90) olive green to orangish brown, dark stripes | ring to semi-ring-porous, medium rings | grey-brown to reddish grey, rough, in small plates |
| siberian_pine_nut | Pine (soft) | search summary, unverified | about 400 | 36 | 35 | 1.0 | 10 | (215, 170, 125) light reddish (E) | conifer, medium rings | grey-brown, smooth when young, later in scaly plates |
| stone_pine_nut | Pine | G (related pines about 500) | about 550 | 50 | 16 | 0.7 | 7 | (205, 155, 105) like Scots pine (E) | conifer, strong rings | red-brown to orange, deeply fissured into long flat plates; umbrella crown |
| juniper | Juniper (Cedar group) | L (562) | 560 | 50 | 5 | 0.15 | 1 | (180, 110, 90) reddish brown, aromatic | conifer, medium rings, no resin canals | red-brown, thin, peeling in long fibrous strips |

### Trunks without true wood

- **banana, papaya** (`GiantHerb`): the "trunk" is a pseudostem of leaf sheaths (banana) or soft pith (papaya). Felling one should give **no logs**, perhaps fibre later. Their bark: banana smooth green with brown dried sheaths; papaya smooth grey-green, marked by leaf scars in a diamond pattern.
- **saguaro_fruit** (`ColumnCactus`): its woody ribs (about 430 kg/m³, unverified) were used for building in the desert. Felling one could give 1 rib bundle rather than logs. Its skin: pleated green, with spines along the ridges.

## Suggested materials

There are five wood `Material`s now. To cover everything without a material per tree, the **Group** column above suggests these woods (density for the material's weight):

- **Already there:** Oak (675; the game has 7), Chestnut (590; game 6), Cherry (600; game 6), Pine (550; game 5) and Acacia (see the gap below).
- **Wild trees (20 core):** Spruce 4, Fir 4, Larch 6, Douglas Fir 5, Red Cedar 4, Cedar 5, Birch 6, Beech 7, Ash 7, Maple 6, Willow 4, Holm Oak 10 (or just Oak), Mesquite 8, Mopane 11, Teak 7, Mahogany 6, Kapok 3. Extras: Cypress 5, Aspen 5, Lime 5, Elm 6, Alder 5, Camel Thorn 12, Ebony 10.
- **Fruit and nut trees:** Apple 8, Pear 7, Plum 8, Walnut 6, Hickory 7, Olive 10, Citrus 8, Fig 7, Mulberry 7, Persimmon 8, Palm 4 to 8, Mango 7, Jackwood 6, Lychee 9, Tamarind 9, plus Baobab 2 and a "light pale wood" for pawpaw, cacao, candlenut and mongongo (about 3). The rest (avocado, durian, guava, cashew, Brazil nut, macadamia, kola, pili, marula, quandong, kakadu plum, shea, pistachio, juniper) could have woods of their own or join the closest group by weight and colour.

## Gaps and decisions needed

1. **Acacia means two woods.** The casks' Acacia is black locust (*Robinia pseudoacacia*, about 770 kg/m³, pale yellow-green to golden brown), as `research-drinks.md` says, but the savanna's acacias are thorn trees (*Vachellia*), heavy and dark red-brown (camel thorn 1185). Either: the savanna trees give a new material (say "Thorn" or "Camel Thorn"), and Acacia stays black locust (which could join the temperate wild trees as an uncommon broadleaf, `wood-database.com/black-locust`); or Acacia becomes the savanna wood and the casks' one is renamed Locust. Claude would keep Acacia as black locust and add the savanna ones as a new material.
2. **Pine's weight:** the game has Pine at 5 (500 kg/m³). Scots pine at 12% is 550, so about 6. The casks' weights would change slightly if it moved. Leaving it at 5 is within the usual ±10%.
3. **No figure found** for lychee, kola, quince, medlar, stone pine and quandong, and only a genus figure for jujube, kakadu plum, rambutan, mangosteen and pili. The tables use those genus figures or estimates (E, G). Good enough for log weights.
4. **Umbrella thorn's density** was not found. 900 is an estimate between related *Vachellia*.
5. **Fig wood:** the only figure (730) is from a study that calls its trees unusually dense. Fig is usually called light and soft, so 500 to 600 may suit a game better.
6. **Palms** have no rings. A palm log's end is fibre dots on a pale ground, dense at the rim. Coconut wood is dense outside (820) and soft inside, and date palm is light (about 390). One "Palm" material at about 6 would do.
7. **Banana, papaya and saguaro** have no true wood (above): decide whether felling gives nothing, fibre or ribs.
8. **Heartwood colours are estimates** from descriptions. If they look wrong in the log pictures, sampling the colour of the open-licensed end-grain photos on Commons would be the next step.

## Bark pictures

See `bark/credits.md` (outside the repo, beside the photos) for the files, sources, authors and licences.

| Tree | Photos | Licences |
| --- | --- | --- |
| pine | `pine_1.jpg`, `pine_2.jpg` | CC BY 3.0, CC BY-SA 4.0 |
| spruce | `spruce_1.jpg`, `spruce_2.jpg` | CC BY-SA 3.0, CC BY-SA 3.0 rs |
| fir | `fir_1.jpg`, `fir_2.jpg` | CC BY-SA 3.0, CC BY-SA 4.0 |
| larch | `larch_1.jpg`, `larch_2.jpg` | CC BY-SA 4.0, Public domain |
| birch | `birch_1.jpg`, `birch_2.jpg` | CC BY-SA 3.0, CC0 |
| aspen | `aspen_1.jpg` | CC BY 4.0 |
| oak | `oak_1.jpg`, `oak_2.jpg` | CC BY-SA 3.0, CC BY-SA 4.0 |
| beech | `beech_1.jpg`, `beech_2.jpg` | CC BY-SA 4.0, Public domain |
| ash | `ash_1.jpg`, `ash_2.jpg` | CC BY-SA 4.0 |
| maple | `maple_1.jpg`, `maple_2.jpg` | CC BY-SA 4.0 |
| lime | `lime_1.jpg`, `lime_2.jpg` | CC BY-SA 3.0 |
| elm | `elm_1.jpg`, `elm_2.jpg` | CC BY-SA 3.0, CC BY-SA 3.0 rs |
| willow | `willow_1.jpg`, `willow_2.jpg` | CC BY-SA 3.0 |
| alder | `alder_1.jpg` | CC BY-SA 3.0 |
| douglas_fir | `douglas_fir_1.jpg`, `douglas_fir_2.jpg` | CC BY 3.0, CC BY-SA 3.0 |
| red_cedar | `red_cedar_1.jpg`, `red_cedar_2.jpg` | CC BY 3.0, CC BY-SA 4.0 |
| cedar | `cedar_1.jpg`, `cedar_2.jpg` | CC BY 4.0, CC BY-SA 3.0 |
| cypress | `cypress_1.jpg`, `cypress_2.jpg` | CC BY-SA 4.0 |
| holm_oak | `holm_oak_1.jpg`, `holm_oak_2.jpg` | CC BY-SA 2.5, CC BY-SA 3.0 |
| mesquite | `mesquite_1.jpg` | CC BY 2.5 |
| mahogany | `mahogany_1.jpg`, `mahogany_2.jpg` | CC BY-SA 3.0 |
| teak | `teak_1.jpg` | Public domain |
| mopane | `mopane_1.jpg` | CC BY-SA 4.0 |
| kapok | `kapok_1.jpg`, `kapok_2.jpg` | CC BY 3.0, CC BY-SA 2.0 |
| apple | `apple_1.jpg`, `apple_2.jpg` | CC BY-SA 4.0 |
| crab_apple | `crab_apple_1.jpg` | CC BY-SA 4.0 |
| pear | `pear_1.jpg`, `pear_2.jpg` | CC BY-SA 2.0, CC BY-SA 4.0 |
| quince | `quince_1.jpg` | CC BY-SA 4.0 |
| rowan | `rowan_1.jpg` | CC BY 4.0 |
| cherry | `cherry_1.jpg`, `cherry_2.jpg` | CC BY-SA 4.0, CC0 |
| plum | `plum_1.jpg` | CC BY 2.0 |
| apricot | `apricot_1.jpg`, `apricot_2.jpg` | CC BY-SA 3.0, CC BY-SA 4.0 |
| almond | `almond_1.jpg`, `almond_2.jpg` | CC BY-SA 4.0 |
| walnut | `walnut_1.jpg` | CC BY-SA 4.0 |
| pecan | `pecan_1.jpg`, `pecan_2.jpg` | CC BY-SA 3.0, Public domain |
| hickory | `hickory_1.jpg`, `hickory_2.jpg` | CC BY 3.0 us, CC BY-SA 3.0 |
| chestnut | `chestnut_1.jpg`, `chestnut_2.jpg` | CC BY-SA 4.0 |
| chinquapin | `chinquapin_1.jpg` | CC BY 4.0 |
| olive | `olive_1.jpg`, `olive_2.jpg` | CC BY-SA 2.0, CC BY-SA 4.0 |
| mulberry | `mulberry_1.jpg`, `mulberry_2.jpg` | CC BY-SA 2.5, CC BY-SA 4.0 |
| persimmon | `persimmon_1.jpg`, `persimmon_2.jpg` | CC BY-SA 4.0 |
| pawpaw | `pawpaw_1.jpg` | CC BY-SA 3.0 |
| citrus | `citrus_1.jpg` | CC BY-SA 4.0 |
| date_palm | `date_palm_1.jpg`, `date_palm_2.jpg` | CC BY-SA 4.0 |
| coconut | `coconut_1.jpg`, `coconut_2.jpg` | CC BY 2.0 |
| mango | `mango_1.jpg` | Public domain |
| jackfruit | `jackfruit_1.jpg`, `jackfruit_2.jpg` | CC BY 3.0 us, CC BY-SA 3.0 |
| guava | `guava_1.jpg`, `guava_2.jpg` | CC BY 2.0, CC BY-SA 4.0 |
| tamarind | `tamarind_1.jpg`, `tamarind_2.jpg` | CC BY-SA 4.0 |
| cashew | `cashew_1.jpg`, `cashew_2.jpg` | CC BY-SA 3.0 |
| candlenut | `candlenut_1.jpg` | CC BY-SA 3.0 au |
| baobab | `baobab_1.jpg`, `baobab_2.jpg` | CC BY 3.0 us, CC0 |
| marula | `marula_1.jpg` | CC BY-SA 4.0 |
| jujube | `jujube_1.jpg` | CC BY-SA 4.0 |
| quandong | `quandong_1.jpg` | CC BY-SA 3.0 |
| shea | `shea_1.jpg` | CC BY-SA 4.0 |
| siberian_pine | `siberian_pine_1.jpg`, `siberian_pine_2.jpg` | CC BY-SA 2.0 fr |
| stone_pine | `stone_pine_1.jpg`, `stone_pine_2.jpg` | CC BY-SA 3.0 au, CC BY-SA 4.0 |
| juniper | `juniper_1.jpg`, `juniper_2.jpg` | CC BY-SA 4.0 |
| camelthorn | `camelthorn_1.jpg` | CC BY-SA 4.0 |
| medlar | `medlar_1.jpg` | CC BY-SA 4.0 |
| fig | `fig_1.jpg`, `fig_2.jpg` | CC BY-SA 4.0 |
| lychee | `lychee_1.jpg` | CC BY-SA 3.0 |
| mangosteen | `mangosteen_1.jpg` | CC BY 3.0 us |
| brazil_nut | `brazil_nut_1.jpg` | CC BY 2.0 |
| carob | `carob_1.jpg` | CC BY 3.0 |
| saguaro | `saguaro_1.jpg` | CC BY 2.0 |
| banana | `banana_1.jpg` | CC BY 3.0 us |
| papaya | `papaya_1.jpg` | CC0 |

110 photos of 69 trees were chosen from contact sheets of Commons search results (`bark <latin name>`, then `<latin name> trunk` for the ones left empty). Close-ups that fill the frame were preferred, and cut ends, leaves, drawings and wrong trees were left out. **No usable open photo** was found for acacia, ebony, jackalberry, lemon, acai, breadfruit, avocado, rambutan, durian, cacao, macadamia, kola, pili, kakadu_plum, mongongo, pistachio. Their bark can follow the description in the tables, or the photo of the group's other tree: lemon as orange, jackalberry as persimmon, umbrella thorn as camel thorn, rambutan and durian as lychee, açaí as coconut, ebony and kola from the description.

Most files are not downloaded yet, because Wikimedia's image server answered HTTP 429 after the first few. `credits.md` has every page link, so the rest can be fetched later with `scripts/bark_get.py`, which also measures each photo's mean colour (the middle half) as a base bark colour for the textures.

## Sources

- The Wood Database, https://www.wood-database.com/ (each wood's page by the slug in the tables): weights at 12%, tree sizes, colours and end grain.
- PROTA, https://prota.prota4u.org/, and PROSEA, https://prosea.prota4u.org/ and https://plantuse.plantnet.org/: baobab, marula, kapok, shea, jackalberry, durian, and the genera *Nephelium*, *Garcinia*, *Ziziphus*, *Canarium* and *Artocarpus*.
- The other sources for single woods are listed with their figures under "Densities found elsewhere" below: papers on fig, carob, citrus, date palm, açaí, juniper and avocado; World Agroforestry; CTAHR (candlenut); Wikipedia (guava, pawpaw, mongongo).
- Chave, J. et al. (2006), *Regional and phylogenetic variation of wood density across 2456 neotropical tree species*, Ecological Applications 16(6), for the conversion from basic density to air-dry density.
- `docs/context/research-drinks.md` (casks' woods), `crates/shared/src/fruit/fruits.ron` (fruit trees' heights and families), `crates/shared/src/material/properties.rs` (the game's wood weights).

## Densities found elsewhere

The woods The Wood Database lacks.

"Air-dry" means about 12% moisture content unless stated. PROSEA figures are given at 15% moisture content, which reads a few per cent higher than 12% (close enough to use as air-dry). "Basic" means oven-dry mass over green volume (air-dry ≈ basic ÷ 0.83). Where a figure is a proxy from a related species or the genus, it says so.

| species | air-dry kg/m³ (or basic, labelled) | source URL | notes |
| --- | --- | --- | --- |
| fig (Ficus carica) | 730 (air-dry D12); oven-dry 610, basic 540 | https://dergipark.org.tr/en/download/article-file/1264327 | Turkish trees (Aydın, Kahramanmaraş). The authors call the density remarkably high. |
| guava (Psidium guajava) | about 670, **oven-dry** (not 12%) | https://en.wikipedia.org/wiki/Psidium_guajava | Wikipedia cites a Nigerian roof-truss study. The 12% figure would be a little higher. |
| jackfruit (Artocarpus heterophyllus) | about 690 at 12% | https://npn.rngr.net/publications/ttsm/species/Artocarpus%20heterophyllus.pdf/at_download/file | USDA Tropical Tree Seed Manual. Heartwood bright yellow (boiled chips give a yellow dye). Strong, hard, durable. Others: PROSEA gives 420–710 at 15% (https://plantuse.plantnet.org/en/Artocarpus_(PROSEA_Timbers)), and a Ugandan study gives basic density 458 (https://journal.naro.go.ug/index.php/ujas/article/view/324). |
| breadfruit (Artocarpus altilis) | 505–645 at 15% | https://plantuse.plantnet.org/en/Artocarpus_(PROSEA_Timbers) | Agroforestree gives the same range and says the wood is very light and soft, but quite resistant (https://growables.org/information/TropicalFruit/BreadfruitAgroforestry.htm). |
| avocado (Persea americana) | 440–640, mean 540, at 12% (branch wood) | https://publicaciones.inecol.mx/Record/oai:oai.myb.ojs.inecol.mx:article-1153/Description?lng=ga | Mexican study of branch wood. The USDA sheet for the genus Persea gives air-dry 30–41 pcf (about 480–655) and basic SG 0.39–0.54 (https://dendro.cnre.vt.edu/DENDROLOGY/woodtech/persea.pdf). |
| lychee (Litchi chinensis) | not found | https://growables.org/information/TropicalFruit/LycheeAgroForest.htm | Only described in words: very hard, durable, "nearly indestructible" but brittle. Flora Malesiana says hard, takes a fine polish. No number in any reliable source (game wikis list 880; don't use it). |
| rambutan (Nephelium lappaceum) | **genus proxy:** Nephelium 615–1110 at 15% | https://prosea.prota4u.org/view.aspx?id=5965 | Genus heartwood is brown, pale purple-red or pale greyish-brown. N. lappaceum splits when seasoned. There is no species figure. |
| durian (Durio zibethinus) | 420–690 at 15% | https://prosea.prota4u.org/view.aspx?id=4230 | PROSEA species record. Heartwood dark red; the genus is pinkish-brown to deep red-brown (https://prosea.prota4u.org/view.aspx?id=4212). Old senile Philippine trees measured relative density (basic) 0.34 (https://philjournalsci.dost.gov.ph/physical-and-mechanical-properties-of-selected-fruit-bearing-and-underutilized-tree-species-in-the-philippines). |
| mangosteen (Garcinia mangostana) | **genus proxy:** Garcinia 690–1120 at 15% | https://plantuse.plantnet.org/en/Garcinia_(PROSEA_Timbers) | There is no species figure. PROSEA (fruits) says dark red, heavy, coarse and very strong. Agroforestree says dark brown, heavy, "almost sinks in water" (https://apps.worldagroforestry.org/treedb2/AFTPDFS/Garcinia_mangostana.PDF). A GWDD-style SG of 0.81 is quoted by worldspecies.org, but I couldn't check it. |
| baobab (Adansonia digitata) | about 210, air-dry | https://prota.prota4u.org/protav8.asp?p=Adansonia+digitata | PROTA says only that the air-dry wood is light. The trunk is soft, spongy and water-storing; that is general knowledge, not stated in this source. No real timber. |
| marula (Sclerocarya birrea) | 590, air-dry | https://prota.prota4u.org/protav8.asp?p=Sclerocarya+birrea | Heartwood pale pinkish to pale reddish-brown; thin sapwood paler. |
| carob (Ceratonia siliqua) | 860 (air-dry D12); oven-dry 810, basic 710 | https://dergipark.org.tr/en/download/article-file/176338 | Turkish Mediterranean trees. Janka hardness is high (110 N/mm² on the end grain). |
| jujube (Ziziphus jujuba) | not found. **Genus proxy:** Ziziphus 535–1180 at 15% (mainly Z. mauritiana) | https://prosea.prota4u.org/view.aspx?id=6511 | Z. jujuba sources say only that the wood is dense and hard. UF/IFAS lists its specific gravity as unknown. |
| kakadu plum (Terminalia ferdinandiana) | not found. **Proxy:** Terminalia sericocarpa (damson) 640 at 12% | https://www.business.qld.gov.au/industries/farms-fishing-forestry/forests-wood/properties-timbers/damson | No wood data found for T. ferdinandiana itself. |
| jackalberry (Diospyros mespiliformis) | (640–)800–900 at 12% | https://prota.prota4u.org/protav8.asp?p=Diospyros+mespiliformis | Heartwood pinkish grey to pinkish brown, darkening to dark brown. Some boles have a blackish (ebony-like) core up to 25 cm wide. |
| shea (Vitellaria paradoxa) | about 720 at 12% | https://prota.prota4u.org/protav8.asp?p=Vitellaria+paradoxa | Moderately heavy and hard. Colour not stated. |
| mongongo (Schinziophyton rautanenii) | about 150 (SG 0.15, basis not stated) | https://en.wikipedia.org/wiki/Mongongo | Balsa-like: very light, very soft, but strong (Janka 140 lbf). PROTA gives no density; it says heartwood pale yellow to straw-coloured, coarse texture (https://prota.prota4u.org/protav8.asp?p=Schinziophyton+rautanenii). |
| kola (Cola nitida) | not found | https://apps.worldagroforestry.org/treedb2/AFTPDFS/Cola_nitida.PDF | Agroforestree: sapwood pinkish-white, heartwood dull yellow; used for furniture, boats and carving. PROTA has no wood data, and the African Wood Density Database has other Cola species but not C. nitida. |
| pili (Canarium ovatum) | not found. **Genus proxy:** Canarium (360–)390–780(–815) at 15% | https://plantuse.plantnet.org/en/Canarium_(PROSEA_Timbers) | Genus heartwood nearly white or buff to pale pinkish- or reddish-brown. USDA's Canarium sheet gives basic SG 0.35–0.52 (https://www.fpl.fs.usda.gov/documnts/TechSheets/Chudnoff/SEAsian_Oceanic/htmlDocs_SEAsian/Canariumspp.html). |
| candlenut (Aleurites moluccanus) | SG 0.35 (basis not stated, probably basic), so about 350 | https://www3.ctahr.hawaii.edu/gsp/doc/Forestry/Little_Skolmen_CFT/CFT_Aleurites_moluccana.pdf | Little & Skolmen: white, lightweight, soft, often blue-stained, not durable; Hawaiians used it for canoes and net floats. An Indonesian study gives air-dry about 345 (https://aminer.org/pub/619b98d91c45e57ce9ffa7bf/changes-in-the-density-specific-gravity-and-dimensional-stability-of-candlenut-wood). |
| Brazil nut (Bertholletia excelsa) | 550–700 (basis not stated) | https://www.delta-intkey.com/wood/en/www/lecbeexc.htm | DELTA: heartwood pinkish to light brown, no streaks. IBAMA gives apparent density 700–750 (moisture basis not stated). Young plantation trees: basic 550, apparent 610 (https://core.ac.uk/works/276269008). |
| cacao (Theobroma cacao) | about 410 (branch wood, basis not clearly stated) | https://edepot.wur.nl/501084 | Wageningen MSc thesis, Côte d'Ivoire. The same for all six branch orders. No trunk figure found. |
| pawpaw (Asimina triloba) | about 396 (SG 0.3969, likely oven-dry) | https://en.wikipedia.org/wiki/Asimina_triloba | Pale greenish yellow, sapwood lighter; light, soft, coarse-grained and spongy. UF/IFAS lists the SG as unknown, so this is a single old figure. |
| quince (Cydonia oblonga) | not found | https://www.wsl.ch/land/products/dendro/species.php?code=CYOB | WSL: anatomically hardly distinguishable from apple, pear and hawthorn. Those would be the natural proxies, but none is sourced here. |
| medlar (Mespilus germanica) | not found | https://en.wikipedia.org/wiki/Mespilus_germanica | Fine-textured and very hard. |
| date palm (Phoenix dactylifera) | 320 **basic** (infradensity) | https://link.springer.com/10.1007/s00107-021-01777-0 | Wet density 460, saturated 690. Not true timber: vascular bundles in soft ground tissue, denser towards the outside (https://ishs.org/ishs-article/1371_67/). |
| açaí palm (Euterpe oleracea) | about 950, outer (peripheral) zone only | https://jtfs.frim.gov.my/jtfs/article/view/2823 | The core is much lighter, and there is no whole-stem figure. Fibrous palm stem, similar to coconut. |
| sweet orange (Citrus sinensis) | 800–827 air-dry | https://bioresources.cnr.ncsu.edu/wp-content/uploads/2020/07/BioRes_15_3_6840_Sahin_ABECS_Determ_Wood_Props_Weathering_Citrus_Limon_Wood_16786.pdf | 827 is from Berti et al. 2018, as cited by Sahin 2020. The fig paper above cites orange at D12 800, oven-dry 760, basic 600 (Kesik 2017). An Iranian study gives basic 570–780 (https://bioresources.cnr.ncsu.edu/wp-content/uploads/2020/06/BioRes_15_3_5696_HassanpoorTichi_KD_Eval_Phys_Anatom_Biometric_Citrus_Trees_Longit_Transverse_17403.pdf). |
| lemon (Citrus limon) | 834 air-dry | same Sahin 2020 URL as orange | Mainly yellow; dense and hard (Janka 138 N/mm² end grain). |
| Siberian pine (Pinus sibirica) | about 391 (basis not stated; **unverified**) | https://yadda.icm.edu.pl/agro/element/bwmeta1.element.agro-6931a55d-b99c-404c-962b-89f13d9a7fb6 | Only a search summary of a 2017 Polish study ("very light and very soft", weaker than Scots pine); the page refused access (403). |
| stone pine (Pinus pinea) | not found | https://revues.cirad.fr/index.php/BFT/article/view/31267 | Elaieb et al. 2015 (Tunisia) measured it, but the figures weren't reachable. Related pines (P. pinaster, P. sylvestris, P. radiata) are about 500 at 12%; that is an inference, not sourced for P. pinea. |
| common juniper (Juniperus communis) | 562 air-dry (550 at 12% in a trade table) | https://agro.icm.edu.pl/agro/element/bwmeta1.element.agro-73a9a617-7719-4116-b938-38133c2fb78c/c/2015_02_151au.pdf | Polish study: slightly denser than Scots pine. The 550 figure is from https://espen.de/en/?p=18484, and the CRC Handbook via MFA gives 0.56 g/cm³. |
| kapok (Ceiba pentandra) | (200–)240–380(–450) at 12% | https://prota.prota4u.org/protav8.asp?p=Ceiba+pentandra | Creamy white, streaked yellow, brown, pink or grey; heartwood not distinct from sapwood. Particularly weak and soft. |
| quandong (Santalum acuminatum) | not found | https://wtlandcare.org/plants/santalum-acuminatum/ | Heartwood hard, close-grained, flesh-coloured; sapwood creamy pink; pleasant smell when cut; suits fine cabinetwork. These words are from a search summary of landcare and botanic pages, not checked. "Silver" and "brown quandong" (Elaeocarpus, 495–610 at 12%) are unrelated, so don't use them. |
| saguaro ribs (Carnegiea gigantea) | about 430, dry (basis loose; **unverified**) | https://www.inaturalist.org/guide_taxa/1490311 | No primary study found. Wikipedia says the ribs are similar in composition to hardwoods. A cactus skeleton of woody ribs, not timber. |
