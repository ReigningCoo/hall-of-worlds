# client: plants

Herbs' plants as 3D models: built from each herb's `shared::herb::PlantLook` (shape, height, spread, leaf and flower colours and sizes) at each of the four stages, in a few variants each, with black outlines; and where a patch's plants stand in its cell. Fruit plants' models too, by family from each fruit's `FruitPlantLook`, at their four stages. Placing them on the maps, and when, is `dungeon_map`'s (`herbs.rs`, `fruit.rs`).

## Files

- `crates/client/src/plants/mod.rs`: `PlantsPlugin` (keeps `PlantModels`; `forget_on_new_table`). Tests in `tests.rs` (every herb builds at every stage, sizes, layouts in their cell).
- `crates/client/src/plants/triangles.rs`: `Triangles`: loose triangles with flat normals and colours, shaded once from a fixed light as they are added (`add` turns each away from the solid's middle); `into_mesh` with picture coordinates and a second set keeping the sight mask from counting anything as a raised top (`sight_mask.wesl`).
- `crates/client/src/plants/pieces.rs`: `Pieces` (fill and outline): `stem` (a box between square ends), `leaf` (two pyramids back to back, lying flat), `blob`, `blob_turned` (a rounded lump), `ball` (rounder: 7 round, 3 rings; turned about the upright); each also added grown by `OUTLINE_WIDTH` (0.35 units) and black into the outline, an inverted hull.
- `crates/client/src/plants/plan.rs`: `Plan` (one plant's height, radius, leaf length, colours, flower size, `Growth`, lean, in world units; `count`, `leaned`, `stem_colour`), `Growth::of(stage)` (size 0.4, 0.65, 1, 1; flowers at 4), `shown_metres` (the exaggeration: `0.35 + 0.6 × m^0.7`), `plants_in_patch` (1 to 4, fewer the wider), `METRE`, `WOOD`, `colour` (sRGB to linear), `between`.
- `crates/client/src/plants/shapes.rs`: `build(shape, plan, rng)`, and helpers (`around`, `out`, `stem_radius`, `flower_head`, `bud`, `top`). In `shapes/`: `rosette.rs` (rosette, succulent), `upright.rs` (spike, clump), `woody.rs` (bush, tree: leaves as masses), `umbel.rs`, `vine.rs`, `grass.rs` (grass, aquatic), `fern.rs`, `creeper.rs`.
- `crates/client/src/plants/models.rs`: `PlantModels` (`get(def, stage, variant, meshes)`: built once, cached by herb key, stage and variant; a variant keeps its random numbers from stage to stage, so it grows rather than changes), `PlantMeshes { fill, outline }`, `VARIANTS` (4).
- `crates/client/src/plants/fruit.rs`: fruit plants' models (fruit plan PR 4): `FruitModels` (`get(def, stage, variant, meshes)`: cached by fruit key, `FruitStage` and variant (`VARIANTS`)), `fruit_plant_size(def)` (how tall and how far out a grown one is drawn, for clicks), `build` (a builder per `PlantFamily`; two random sequences per variant, one for the shape and one for flowers and fruit, so a plant keeps its shape from stage to stage), `around`, `out`. Tests in `fruit/fruit_tests.rs` (every fruit at every stage and variant, shape kept, within its click size). In `fruit/`:
  - `look.rs`: `Look` (one plant's measures in world units from `FruitPlantLook`, 0.45 as big while young; leaf and flower colours and sizes; fruit's half width from its `FruitLook` size, at least 10 cm on trunk plants; what shows, `Show::{Leaves, Flowers, Fruit(colour)}`; `wide`, spread over height in life), `drawn_metres` (trunk plants `0.6 + 1.1 √m` tall up to 4.4 m and up to 3 m wide; others as herbs, low ones up to 0.6 m, up to 1.6 m wide).
  - `dress.rs`: `Dress`: flowers and fruit by `FruitPosition` (clusters on a stalk, single, at tips), fruit by `FruitShape` (round, oval, long, pod, pear as two balls) hanging, or sitting (`sits`: cacti, the pineapple) or lying on the ground; `over` spreads them over a mass of leaves.
  - `tree.rs` (broadleaf: trunk, branches, a crown of balls round, spreading or upright by `wide`; fruit on the trunk for cacao and jackfruit), `conifer.rs` (tiers narrowing up, or an umbrella crown when `wide` > 0.55; cones near the top for `Tips`), `palm.rs` (curving trunk, drooping fronds, bunches under the crown), `shrub.rs` (shrubs; bramble canes arching), `vine.rs` (climbing vines on a two-post trellis; ground vines' runners with fruit lying beside them), `low.rs` (low mats; low herbs with flowers and fruit on stems of their own), `rosette.rs` (the pineapple: fruit sitting on a stalk with its leafy crown), `cactus.rs` (pads standing on each other's edges; a rounded column with arms, fruit at the tips), `giant.rs` (banana with a hanging bunch and purple heart, by a `Long` fruit; else papaya, fruit round the trunk top).
- `crates/client/src/plants/layout.rs`: `patch_layout(look, seed)`: a patch's plants (`PlantSpot`: offset from the cell's middle, turn, scale, variant), in a ring round the middle.

## Public API

- `PlantsPlugin`, `PlantModels`, `FruitModels` (resources), `patch_layout`, `fruit_plant_size`.

## Depends on

- `common` (`HerbTable`), `dungeon_map` (`CELL_SIZE`), `shared::herb` (`PlantLook`, `PlantShape`, `PartSize`, `Rgb`, `HerbDef`), `shared::fruit` (`FruitDef`, `FruitStage`, `PlantFamily`, `FruitPlantLook`, `FruitPosition`, `FruitShape`), `shared::rng`.

## Used by

- `dungeon_map` (`herbs/draw.rs`, `fruit.rs`, `fruit_picking.rs`).

## Notes

- Owner's: sizes may be exaggerated (small herbs drawn bigger, tall ones a little smaller; at most 2.2 m tall), patches of several plants, each varying by its seed (height, lean, hue, leaf count and angles), black outlines like every 3D model.
- Only looks, the same on every client (seeded from the herb's key and the patch's cell), never game rules.
- The maps' surfaces are unlit, so shading is baked into vertex colours; the outside light tints the plants' material (`dungeon_map::daylight::tint_outside`).
- Fruit plants (fruit plan PR 4): trees kept within reason for a camera looking down (4.4 m at most); fruit drawn larger than life so it shows. Fruit on plants is built here from balls in the fruit's own ripe or unripe colour, not from `item_art`'s fruit pictures (another mesh kit, drawn for close views), so the two share colours and shape kinds rather than meshes.
