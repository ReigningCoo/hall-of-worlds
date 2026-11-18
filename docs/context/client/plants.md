# client: plants

Herbs' plants as 3D models: built from each herb's `shared::herb::PlantLook` (shape, height, spread, leaf and flower colours and sizes) at each of the four stages, in a few variants each, with black outlines; and where a patch's plants stand in its cell. Placing them on the maps, and when, is `dungeon_map`'s (`herbs.rs`).

## Files

- `crates/client/src/plants/mod.rs`: `PlantsPlugin` (keeps `PlantModels`; `forget_on_new_table`). Tests in `tests.rs` (every herb builds at every stage, sizes, layouts in their cell).
- `crates/client/src/plants/triangles.rs`: `Triangles`: loose triangles with flat normals and colours, shaded once from a fixed light as they are added (`add` turns each away from the solid's middle); `into_mesh` with picture coordinates and a second set keeping the sight mask from counting anything as a raised top (`sight_mask.wesl`).
- `crates/client/src/plants/pieces.rs`: `Pieces` (fill and outline): `stem` (a box between square ends), `leaf` (two pyramids back to back, lying flat), `blob`, `blob_turned` (a rounded lump); each also added grown by `OUTLINE_WIDTH` (0.35 units) and black into the outline, an inverted hull.
- `crates/client/src/plants/plan.rs`: `Plan` (one plant's height, radius, leaf length, colours, flower size, `Growth`, lean, in world units; `count`, `leaned`, `stem_colour`), `Growth::of(stage)` (size 0.4, 0.65, 1, 1; flowers at 4), `shown_metres` (the exaggeration: `0.35 + 0.6 × m^0.7`), `plants_in_patch` (1 to 4, fewer the wider), `METRE`, `WOOD`, `colour` (sRGB to linear), `between`.
- `crates/client/src/plants/shapes.rs`: `build(shape, plan, rng)`, and helpers (`around`, `out`, `stem_radius`, `flower_head`, `bud`, `top`). In `shapes/`: `rosette.rs` (rosette, succulent), `upright.rs` (spike, clump), `woody.rs` (bush, tree: leaves as masses), `umbel.rs`, `vine.rs`, `grass.rs` (grass, aquatic), `fern.rs`, `creeper.rs`.
- `crates/client/src/plants/models.rs`: `PlantModels` (`get(def, stage, variant, meshes)`: built once, cached by herb key, stage and variant; a variant keeps its random numbers from stage to stage, so it grows rather than changes), `PlantMeshes { fill, outline }`, `VARIANTS` (4).
- `crates/client/src/plants/fruit.rs`: `FruitModels` (`get(def, stage, meshes)`: fruit plants' stand-in models (fruit plan PR 3, until PR 4's), cached by fruit key and `FruitStage`: a trunk (`stem`) and a crown (`blob`) for tree families, a mound for the rest, sized by family, 0.45 as big while young, six dots round the crown: pale flowers, the fruit's unripe or ripe colour). Tests in `fruit/fruit_tests.rs`.
- `crates/client/src/plants/layout.rs`: `patch_layout(look, seed)`: a patch's plants (`PlantSpot`: offset from the cell's middle, turn, scale, variant), in a ring round the middle.

## Public API

- `PlantsPlugin`, `PlantModels`, `FruitModels` (resources), `patch_layout`.

## Depends on

- `common` (`HerbTable`), `dungeon_map` (`CELL_SIZE`), `shared::herb` (`PlantLook`, `PlantShape`, `PartSize`, `Rgb`, `HerbDef`), `shared::fruit` (`FruitDef`, `FruitStage`, `PlantFamily`), `shared::rng`.

## Used by

- `dungeon_map` (`herbs/draw.rs`, `fruit.rs`).

## Notes

- Owner's: sizes may be exaggerated (small herbs drawn bigger, tall ones a little smaller; at most 2.2 m tall), patches of several plants, each varying by its seed (height, lean, hue, leaf count and angles), black outlines like every 3D model.
- Only looks, the same on every client (seeded from the herb's key and the patch's cell), never game rules.
- The maps' surfaces are unlit, so shading is baked into vertex colours; the outside light tints the plants' material (`dungeon_map::daylight::tint_outside`).
