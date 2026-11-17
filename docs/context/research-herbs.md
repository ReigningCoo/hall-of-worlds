# Research: herbs

Research for herbs and potion making (2026-11-16), approved by the owner. The game's data is `crates/shared/src/herb/herbs.ron` (`shared::herb`, `docs/context/shared/herb.md`); this page is the summary and the reasons.

## Method and sources

- 82 herbs chosen across the nine biomes of the climate (`climate::Biome`), plus marsh, which the climate doesn't have yet: those that grow there are kept as data for later.
- Each herb's Wikipedia article for what it is, where it grows, its traditional uses and how common it is; Nicholas Culpeper's *Complete Herbal* (1653, public domain) for traditional virtues, where it covers the herb (46 of them). Folk uses only: whether they work in real life doesn't matter, and nothing here is medical advice.
- Where a source gave no temperature, the yearly average range was estimated from where the herb grows, in the climate code's own measure (yearly average °C).
- Plant looks (shape, heights, colours, leaf and flower sizes) and the weight of one harvested unit were worked out from each herb's description, for the client's models (first guesses, to adjust by eye).

## Owner's decisions

- Effects: each herb has four, on the nine attributes or Health or Mana, each raised or lowered weakly (1), moderately (2) or strongly (3).
- Growth: four stages, in bands of Fast (2 game days for all four), Medium (4), Slow (8) and Very slow (16). Harvesting: cut back to stage 2, picked back to stage 3, dug up (gone), or coppiced back to stage 1. Each herb has a rule for harvesting early, at stage 3.
- How common: common, uncommon or rare where it can grow.
- Herbs are never cursed, blessed or magical. Quality is Poor to Uncommon normally, anything higher significantly rarer than other items; Foraging makes picked herbs better (replaced 2026-11-17: quality is the plant's rarity, and Foraging and the Herbs skill give more); quality affects potency only when the user has enough Herbalism to use it.
- Herbs stack by herb and quality only, each looking slightly different by a seed of its own.
- Potions (to come): herbs mixed, effects added (weak 1, moderate 2, strong 3, opposites cancel); a potion forms when any effect reaches 4, and every other effect at 2 or more goes in too. Potions restore Health and Mana; attribute effects last a time and end at once.
- Knowledge: players learn each herb on its own, and it only changes what is shown (the popup shows the effects known). Picking or buying teaches a herb's name; eating it raw shows its first effect; Herbalism practice, making potions, books and training show more. An unknown herb shows as "Unknown herb" (look-alikes possible).
- The list can grow by modding (`HOW_MODS`).
- No hallucinogens or recreational drugs, and no strong links to religion.
- Left for later: drying and spoiling, seeds, the herbalist's trade, handling poisonous herbs, cave mushrooms, seasons.

## The herbs

Temperatures are yearly averages in °C. Effects: Str(ength), Agi(lity), Tou(ghness), Int(ellect), Per(ception), Wil(l), Pre(sence), Emp(athy), Aur(a).

| Herb | Latin | Biomes | How common | °C | Part used | Growth | Harvest | Effects |
| ---- | ----- | ------ | ---------- | -- | --------- | ------ | ------- | ------- |
| Roseroot | *Rhodiola rosea* | Tundra, Boreal | rare | -8 to 6 | Root | Very slow | dig | Tou +2, Wil +2, Int +1, Emp -1 |
| Labrador tea | *Rhododendron groenlandicum* | Tundra, Boreal, Marsh | common | -8 to 6 | Leaves | Very slow | cut | Health +1, Tou +1, Per -2, Int -1 |
| Bearberry | *Arctostaphylos uva-ursi* | Tundra, Boreal, Woodland | common | -8 to 10 | Leaves | Slow | cut | Aur +2, Health +1, Tou +1, Str -1 |
| Angelica | *Angelica archangelica* | Tundra, Boreal, Marsh, Seasonal forest | uncommon | -6 to 10 | Root and stems | Medium | dig | Tou +2, Health +1, Per +1, Pre -1 |
| Arctic willow | *Salix arctica* | Tundra, Boreal | common | -12 to 2 | Bark and leaves | Slow | coppice | Health +2, Tou +1, Per +1, Emp -1 |
| Juniper | *Juniperus communis* | Boreal, Tundra, Grassland, Woodland | common | -8 to 14 | Berries (cones) | Very slow | pick | Int +2, Tou +1, Aur +1, Health -1 |
| Bilberry | *Vaccinium myrtillus* | Boreal, Seasonal forest, Tundra | common | -4 to 10 | Berries and leaves | Slow | pick | Per +2, Health +1, Tou +1, Str -1 |
| Fireweed | *Chamaenerion angustifolium* | Boreal, Tundra, Grassland | common | -6 to 12 | Leaves and flowers | Medium | cut | Tou +1, Str +1, Emp +2, Agi -1 |
| Wintergreen | *Gaultheria procumbens* | Boreal, Seasonal forest | uncommon | 0 to 12 | Leaves | Slow | cut | Health +2, Agi +2, Pre +1, Tou -2 |
| Devil's club | *Oplopanax horridus* | Boreal, Temperate rainforest | uncommon | 0 to 10 | Inner root bark | Very slow | dig | Aur +3, Health +1, Agi +1, Mana +1 |
| Yarrow | *Achillea millefolium* | Grassland, Tundra, Boreal, Woodland, Seasonal forest | common | -4 to 18 | Flowering tops | Medium | cut | Health +3, Tou +1, Mana +2, Aur +1 |
| Echinacea | *Echinacea purpurea* | Grassland | common | 4 to 16 | Root | Very slow | dig | Tou +3, Health +1, Per -1, Mana -1 |
| Chamomile | *Matricaria chamomilla* | Grassland, Woodland | common | 6 to 20 | Flowers | Fast | pick | Emp +2, Health +2, Int +1, Agi -1 |
| Wormwood | *Artemisia absinthium* | Grassland, Desert, Woodland | common | 4 to 18 | Leaves and tops | Medium | cut | Tou +2, Str +1, Health -2, Agi -1 |
| Mugwort | *Artemisia vulgaris* | Grassland, Seasonal forest, Woodland | common | 2 to 18 | Leaves and tops | Medium | cut | Mana +2, Aur +2, Agi +1, Health -1 |
| Mullein | *Verbascum thapsus* | Grassland, Woodland, Desert | common | 4 to 20 | Leaves and flowers | Slow | cut | Tou +2, Health +1, Aur +1, Per -1 |
| Red clover | *Trifolium pratense* | Grassland, Seasonal forest | common | 4 to 18 | Flowers | Fast | pick | Health +1, Per +1, Pre +1, Tou -1 |
| St John's wort | *Hypericum perforatum* | Grassland, Woodland, Seasonal forest | common | 4 to 18 | Flowering tops | Medium | cut | Wil +2, Aur +2, Health +1, Per -1 |
| Plantain | *Plantago major* | Grassland, Seasonal forest, Temperate rainforest, Woodland | common | 0 to 22 | Leaves | Fast | cut | Health +2, Tou +1, Agi +1, Per +1 |
| Eyebright | *Euphrasia officinalis* | Grassland, Tundra | uncommon | -4 to 12 | Whole herb in flower | Medium | dig | Per +3, Int +1, Tou +1, Str -1 |
| Mustard | *Sinapis alba / Brassica nigra* | Grassland, Woodland, Seasonal forest | common | 6 to 20 | Seeds | Fast | dig | Tou +2, Per +1, Int +1, Emp -1 |
| Rosemary | *Salvia rosmarinus* | Woodland, Grassland | common | 10 to 22 | Leaves | Very slow | cut | Int +3, Per +1, Tou +1, Emp +1 |
| Sage | *Salvia officinalis* | Woodland, Grassland, Desert | uncommon | 8 to 20 | Leaves | Slow | cut | Health +2, Int +2, Tou +1, Agi -1 |
| Thyme | *Thymus vulgaris* | Woodland, Grassland | uncommon | 8 to 20 | Leaves | Medium | cut | Wil +2, Tou +2, Aur +1, Health +1 |
| Lavender | *Lavandula angustifolia* | Woodland, Grassland | uncommon | 8 to 20 | Flowers | Slow | pick | Emp +2, Wil +1, Health +2, Pre +1 |
| Lemon balm | *Melissa officinalis* | Woodland, Seasonal forest, Grassland | common | 6 to 20 | Leaves | Fast | cut | Emp +2, Wil +1, Int +1, Tou +1 |
| Fennel | *Foeniculum vulgare* | Woodland, Grassland, Desert | common | 10 to 22 | Seeds and leaves | Medium | cut | Per +2, Health +1, Aur +1, Wil +1 |
| Rue | *Ruta graveolens* | Woodland, Desert | uncommon | 10 to 22 | Leaves | Slow | cut | Aur +3, Per +2, Health -2, Pre -1 |
| Bay laurel | *Laurus nobilis* | Woodland, Seasonal forest | common | 12 to 22 | Leaves | Very slow | cut | Pre +2, Mana +2, Int +1, Aur -1 |
| Saffron crocus | *Crocus sativus* | Woodland, Desert, Grassland | rare | 10 to 22 | Stigmas | Fast | pick | Emp +2, Pre +2, Int +1, Health -2 |
| Garlic | *Allium sativum* | Woodland, Grassland, Seasonal forest | uncommon | 4 to 22 | Bulb | Slow | dig | Tou +3, Str +2, Pre -2, Mana -1 |
| Ramsons | *Allium ursinum* | Seasonal forest, Temperate rainforest, Marsh | common | 4 to 14 | Leaves | Fast | cut | Str +1, Health +1, Tou +2, Pre -1 |
| Foxglove | *Digitalis purpurea* | Seasonal forest, Temperate rainforest, Woodland | common | 4 to 16 | Leaves | Slow | cut | Health -3, Tou +2, Per -2, Str -1 |
| Valerian | *Valeriana officinalis* | Seasonal forest, Marsh, Grassland | common | 2 to 16 | Root | Slow | dig | Wil +2, Agi -2, Int -1, Pre -1 |
| Stinging nettle | *Urtica dioica* | Seasonal forest, Temperate rainforest, Grassland, Boreal, Marsh | common | 0 to 16 | Young leaves | Fast | cut | Str +2, Tou +2, Health +1, Agi -1 |
| Betony | *Betonica officinalis* | Seasonal forest, Grassland, Woodland | common | 4 to 16 | Leaves and tops | Medium | cut | Aur +2, Int +1, Wil +1, Health +1 |
| Ginseng | *Panax quinquefolius* | Seasonal forest | rare | 4 to 14 | Root | Very slow | dig | Str +3, Int +1, Health +1, Wil -1 |
| Hawthorn | *Crataegus monogyna* | Seasonal forest, Woodland, Grassland | common | 4 to 16 | Flowers and berries | Very slow | pick | Aur +3, Tou +1, Health +2, Emp +2 |
| Comfrey | *Symphytum officinale* | Seasonal forest, Marsh, Temperate rainforest, Grassland | common | 2 to 16 | Root and leaves | Medium | cut | Health +3, Agi +1, Str +1, Tou -2 |
| Vervain | *Verbena officinalis* | Seasonal forest, Grassland, Woodland | common | 6 to 18 | Tops | Medium | cut | Aur +3, Mana +2, Wil +1, Agi -1 |
| Wild ginger | *Asarum caudatum / europaeum* | Temperate rainforest, Seasonal forest | common | 4 to 14 | Rhizome | Slow | dig | Per +2, Tou +1, Str -1, Health -2 |
| Yerba buena | *Clinopodium douglasii* | Temperate rainforest | common | 6 to 16 | Leaves | Fast | cut | Health +1, Pre +1, Tou +1, Emp +1 |
| Self-heal | *Prunella vulgaris* | Temperate rainforest, Seasonal forest, Grassland, Marsh | common | 2 to 20 | Flowering tops | Fast | cut | Health +2, Tou +1, Per +1, Agi +1 |
| Wasabi | *Eutrema japonicum* | Temperate rainforest | rare | 6 to 16 | Rhizome | Very slow | dig | Per +3, Tou +1, Health -1, Int +1 |
| Aloe | *Aloe vera* | Desert, Savanna | common | 18 to 30 | Leaf gel | Very slow | cut | Health +3, Pre +1, Aur +1, Str -2 |
| Creosote bush | *Larrea tridentata* | Desert | common | 14 to 28 | Leaves | Very slow | cut | Tou +2, Agi +1, Pre -1, Health -1 |
| Cumin | *Cuminum cyminum* | Desert, Savanna | uncommon | 14 to 28 | Seeds | Medium | dig | Str +1, Pre +1, Tou +1, Emp +1 |
| Asafoetida | *Ferula assa-foetida* | Desert | rare | 10 to 24 | Resin from the root | Very slow | dig | Tou +2, Aur +2, Health +1, Pre -3 |
| Henna | *Lawsonia inermis* | Desert, Savanna | uncommon | 20 to 32 | Leaves | Slow | cut | Pre +2, Aur +1, Health +1, Emp +1 |
| Devil's claw | *Harpagophytum procumbens* | Savanna, Desert | uncommon | 18 to 28 | Side tubers | Slow | dig | Agi +2, Health +2, Str +1, Tou -1 |
| Roselle | *Hibiscus sabdariffa* | Savanna, Tropical rainforest | common | 20 to 30 | Calyces | Medium | pick | Tou +1, Health +1, Pre +1, Str -1 |
| Lemongrass | *Cymbopogon citratus* | Savanna, Tropical rainforest | common | 20 to 30 | Stalks | Medium | cut | Tou +1, Emp +1, Pre +1, Agi -1 |
| Ashwagandha | *Withania somnifera* | Savanna, Desert, Grassland | uncommon | 18 to 30 | Root | Medium | dig | Str +1, Wil +2, Agi -1, Health -1 |
| Neem | *Azadirachta indica* | Savanna, Tropical rainforest, Desert | common | 21 to 32 | Leaves | Very slow | cut | Tou +2, Health +1, Wil +1, Str +1 |
| Ginger | *Zingiber officinale* | Tropical rainforest, Savanna | uncommon | 20 to 30 | Rhizome | Medium | dig | Tou +2, Str +1, Agi +1, Aur +1 |
| Turmeric | *Curcuma longa* | Tropical rainforest, Savanna | uncommon | 20 to 30 | Rhizome | Slow | dig | Health +1, Tou +1, Aur +1, Pre +1 |
| Cardamom | *Elettaria cardamomum* | Tropical rainforest | uncommon | 20 to 28 | Seed pods | Very slow | pick | Pre +2, Emp +1, Tou +1, Int +1 |
| Black pepper | *Piper nigrum* | Tropical rainforest | uncommon | 22 to 30 | Berries | Very slow | pick | Tou +1, Health +1, Per +2, Pre -1 |
| Vanilla | *Vanilla planifolia* | Tropical rainforest | rare | 22 to 30 | Pods (cured) | Very slow | pick | Pre +2, Aur +1, Emp +2, Health -1 |
| Cinnamon | *Cinnamomum verum* | Tropical rainforest | common | 22 to 30 | Inner bark | Very slow | coppice | Tou +2, Pre +1, Aur +1, Health -1 |
| Clove | *Syzygium aromaticum* | Tropical rainforest | uncommon | 22 to 30 | Flower buds | Very slow | pick | Health +2, Pre +2, Tou +1, Emp +1 |
| Guarana | *Paullinia cupana* | Tropical rainforest | common | 22 to 30 | Seeds | Very slow | pick | Agi +2, Per +1, Str +1, Mana -1 |
| Gotu kola | *Centella asiatica* | Tropical rainforest, Marsh, Savanna | common | 16 to 30 | Leaves | Fast | cut | Int +2, Health +1, Tou +1, Agi -1 |
| Chilli | *Capsicum annuum* | Tropical rainforest, Savanna, Desert | common | 16 to 30 | Fruit | Medium | pick | Health +1, Tou +1, Per -2, Emp -1 |
| Bog myrtle | *Myrica gale* | Marsh, Boreal, Temperate rainforest | uncommon | 0 to 12 | Leaves | Slow | cut | Tou +1, Pre +1, Health +1, Wil -1 |
| Meadowsweet | *Filipendula ulmaria* | Marsh, Grassland, Seasonal forest | common | 2 to 16 | Flowers | Medium | pick | Health +2, Emp +1, Agi +1, Pre +1 |
| Marsh mallow | *Althaea officinalis* | Marsh, Grassland | common | 6 to 18 | Root | Slow | dig | Health +2, Tou +1, Pre +1, Agi +1 |
| Water mint | *Mentha aquatica* | Marsh, Seasonal forest, Grassland, Temperate rainforest | uncommon | 4 to 18 | Leaves | Fast | cut | Pre +2, Int +1, Tou +1, Mana -1 |
| Sweet flag | *Acorus calamus* | Marsh, Seasonal forest, Boreal | common | 2 to 18 | Rhizome | Slow | dig | Tou +1, Pre +1, Int +2, Health -2 |
| Bogbean | *Menyanthes trifoliata* | Marsh, Tundra, Boreal | uncommon | -6 to 12 | Leaves | Medium | cut | Wil +1, Agi -1, Tou +1, Health +1 |
| Sundew | *Drosera rotundifolia* | Marsh, Tundra, Boreal | uncommon | -6 to 12 | Whole plant | Slow | dig | Tou +2, Wil +1, Emp +1, Pre -1 |
| Water hemlock | *Cicuta virosa / maculata* | Marsh, Grassland | uncommon | 0 to 16 | Root | Slow | dig | Health -3, Agi -3, Wil -1, Int -2 |
| Purple loosestrife | *Lythrum salicaria* | Marsh | common | 2 to 18 | Tops | Medium | cut | Health +2, Per +2, Pre +1, Emp +1 |
| Caper | *Capparis spinosa* | Desert, Woodland | common | 14 to 28 | Flower buds | Slow | pick | Tou +1, Emp +1, Pre +1, Agi +1 |
| Fenugreek | *Trigonella foenum-graecum* | Desert, Grassland, Savanna | uncommon | 12 to 28 | Seeds and leaves | Medium | dig | Str +1, Tou +1, Health +1, Pre -1 |
| Galangal | *Alpinia galanga* | Tropical rainforest | uncommon | 20 to 30 | Rhizome | Slow | dig | Health +1, Tou +1, Str +1, Emp +1 |
| Watercress | *Nasturtium officinale* | Marsh, Grassland, Seasonal forest | common | 4 to 18 | Leaves | Fast | cut | Tou +2, Int +1, Str +1, Per +1 |
| Borage | *Borago officinalis* | Grassland, Woodland | common | 8 to 22 | Flowers and leaves | Fast | cut | Emp +3, Wil +1, Health +1, Tou -1 |
| Elecampane | *Inula helenium* | Grassland, Seasonal forest, Marsh | uncommon | 4 to 16 | Root | Slow | dig | Tou +2, Mana +2, Health +1, Aur -1 |
| Lady's mantle | *Alchemilla vulgaris* | Grassland, Tundra, Boreal | common | -4 to 12 | Leaves | Fast | cut | Health +3, Aur -1, Mana +2, Pre +1 |
| Elder | *Sambucus nigra* | Seasonal forest, Woodland, Grassland | common | 4 to 16 | Flowers and berries | Very slow | pick | Tou +2, Health -1, Pre +1, Aur +2 |
| Moonwort | *Botrychium lunaria* | Grassland, Tundra | uncommon | -6 to 10 | Fronds | Slow | cut | Mana +3, Health +2, Aur +1, Tou +1 |
