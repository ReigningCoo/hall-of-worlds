# Milling plan: millers and flour

Grain is picked as 30 g sheaves (`shared::vegetable`, `Unit::Sheaf`) that heal nothing until milled. This plan adds flour and the towns' millers who make it. Flour is needed before the drinks plan's PR 9 (yeast starters fed with flour and water, `drinks.md`). Owner's decisions, 2026-11-18, planned in chat `local_e641f63a`.

## PRs

1. **Flour items** (built): `ItemKind::Flour` and `shared::flour`; a `flour_yield` per grain; name, cells and weight, the popup, Give Item, a picture, and a Flour section on each grain's wiki page.
2. **The miller** (built): the miller's building in towns and his own small stock of flour.

## Flour (PR 1)

- **One flour per grain**, all 17 (`set: Grain` in `vegetables.ron`), rather than one generic flour: rye flour and wheat flour are different items, so recipes can tell them apart later.
- **One sheaf makes one sack**, weighing the sheaf's 30 g × the grain's `flour_yield` percent, rounded. The sack is only the picture and the name ("Sack of Rye Flour"): the whole item is used up together, sack and all.
- **Counted**, stacking by grain like vegetables; **no quality**; **never goes stale** (owner: not yet, maybe never).
- **Yields**, stone-ground wholemeal, rough figures: wheat, rye, maize, teff 95; pearl millet, finger millet, amaranth 90; sorghum, quinoa 85; barley, spelt, proso millet 75; buckwheat, fonio 70; oats, rice, wild rice 65. Naked grains mill nearly whole; hulled ones (barley, spelt, oats, rice, buckwheat, fonio, the millets with hulls) lose their husks first. The owner gave wheat, rye, maize, barley, spelt, oats and rice; the rest are estimates from the same reasoning.
- **Its picture** is an open hessian sack, rectangular with its sides pushed out by the flour, its top rolled down, the flour heaped above the rim (owner). The flour's colour is the grain's inside with some of its skin's, as wholemeal keeps the bran.
- **The wiki** has no pages of flours' own: each grain's page gets a Flour section with the picture and yield (owner).

## Milling (recorded only: paid trade isn't built yet)

- **Milling is trade with the miller.** He buys a sheaf for 1 copper and mills it at a base of 1 minute a sheaf, by his quality: about 1.5 minutes a sheaf for a poor miller to 30 seconds for a fine one, on the shop make-time clock (`shared::shop`), one at a time in the order they came.
- Each sack appears in his trade view when done, to buy back for 2 copper. **Anyone** may buy it, not only whoever brought the sheaf.
- Sold sheaves don't count towards his own stock's limit.

## The miller (PR 2)

- **His building:** a miller's in towns, 9 × 9 with rounded corners, one storey, its door to the nearest road with a windmill sign, a tall cone roof, and the lower end of one sail above the door, fading with height (looks only; on the world map too). Placed after the town is finished, from the town's seed (`derive("miller")`), in free space inside the town (inside its walls), and skipped if none fits. Every town tries for one; a Town has a 20% chance of a second, a City 50% of a second and 25% of a third. The town generator's version goes up; new ids are appended.
- **His own stock:** at most 10 sacks, of flours suiting the town's climate (grains growing there, as `shared::produce` suits fields), 2–3 new each game day, with no top-up for time away.
- **As built (PR 2):** `BuildingKind::Miller` (`shared::site`, `town/miller.rs`; `TOWN_VERSION` up, `UNMILLED_TOWN_VERSION` kept for saved towns), his stock in `shared::shop` (`MILLER_MOST`, `tops_up`) and `shared::produce` (`flour_grain_for`), the door's windmill sign in the town atlas (game-icons' `delapouite/windmill`), the cone roof and fading sail in `dungeon_map/town/mill.rs` (a blended town surface, `TownBlend`), mills in the world map's town model (`poi_model/mill.rs`, `PoiLook::Town::millers`), and "<name>, miller" in the talk panel. Claude's choices: the mill's corners rounded with a radius of 4 cells; the door on the side nearest a road by walking; the world map shows as many mills as the town tries for (as it shows the buildings it aims for), so a town where none fitted still shows one; a grain where no field grain grows falls back to any grain.
