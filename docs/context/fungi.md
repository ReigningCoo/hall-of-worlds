# Fungi: plan and owner's decisions

Mushrooms, fungi and lichens in 9 PRs (owner, 2026-11-19): mostly underground, food first, many poisons, 36 used like herbs. The research, with every species and the owner's decisions on it: `research-fungi.md` (review page: https://claude.ai/artifact/1gGUDdPgtpYfDZ4QcM5BuA; scripts and data outside the repo in `~/.claude/research-tools/fungi/`). Each PR marks its step built here.

## Owner's decisions

1. Keep all 111 kinds (100 real, corn smut dropped; 12 invented). "Suggested"/"extra" was only for review.
2. Where: wood, dung and compost feeders grow **underground** (`deep`); truffles and buried lumps are **dug** (`dug`:
   cave earth floors, and the ground under their trees above); fungi on living tree roots or bark, and all lichens,
   **above ground only** (`surface`); a few underground only **near daylight** (`dim`: cave mouths' levels, skylights,
   holes to the surface).
3. Underground climate: a cave is at its land's yearly average temperature (herbs' measure, `Wilderness::habitat_at`);
   each kind has a range (`temp`). Rarer kinds likelier deeper; Dreamcap only on the deepest level. Basements and
   cellars grow the `deep` wood/soil kinds.
4. Niches on a level: `water` near pools and under drips, `lava` beside lava, `remains` in crypts and deepest caves,
   `wood` in basements, cellars and where roots come through, `dung` and `soil` on any cave floor. Above ground,
   wood-feeders also grow on **felled trees**: in practice on felled cells from a day after felling (stump gone) until a
   sapling (day 5); logs are items in piles, so left for later.
5. Above ground, wild patches a quarter as common as herbs, except lichens in Tundra and Boreal forest (common there).
   Underground, a patch on about 1 cave floor cell in 60.
6. Items: own table `shared::fungus` (as `shared::vegetable`), `ItemKind::Fungus`, counted, no quality. Units: `one`,
   `handful` (100 g), `piece` (large, eaten a serving at a time). Healing as vegetables: raw ¼ only when eaten raw
   commonly or sometimes; cooked figure kept for cooking.
7. 36 kinds used like herbs: four effects each (`herb`), accepted by Alchemy alongside herbs, Herbalism and knowledge
   the same. Deadly ones too (poison pastes, later coating weapons: a TODO).
8. Poisons: real effects when eaten (raw-only where marked `r`). Onset scaled to game time: soon (at once), delayed
   (about 10 game minutes), late (a game hour or more, a false recovery for the amatoxins). A delayed poison is
   **hidden** until recognised (no Medicine skill yet: knowing that fungus's poison, or a Healing check; magic later),
   and shown once recognised. When the delay ends it becomes a normal poison effect, shown as usual with the fungus as
   source. Deadly drains Health hard enough to kill unless healed, on every difficulty; Easy halves it.
9. Confusion: fly agaric, panther cap and Dreamcap lower Intellect, Perception, Agility and make the eater stumble
   (widen the alcohol stumble rule to other effects). Psilocybin kinds left out.
10. **Never give wrong information.** Unknown fungi show as "Unknown mushroom", or "Looks like X" when the player
    knows the name of a lookalike X (`lookalike` lists them). Picking teaches the name by a check (a new Mycology skill
    under gathering; Foraging until then).
11. Common ink cap: alcohol within 3 days after eating brings flushing and sickness, and eating it gives a **bonus**
    so it is a real choice (proposed: Toughness +1 and Will +1 for the same 3 days).
12. Glow: a growing patch gives light; range √(count) of glowing cells joined to it (a one-cell gap ignored), every
    joined cell getting that same light. Each kind its own colour, blending where a patch mixes them. Picked ones stop.
    Light is client-only today (`dungeon_map/mesh/light_grid.rs`).
13. Harm from handling, built here: touch blocked by anything worn on the hands (`worn_locations` has `Hands`),
    breathing by covering the mouth: a new `covers_mouth` check (litham `Headcloth.face_covered`, closed helmets
    `Face::closed`, aventail `Front::Closed`/`Ventail`) and a new **bandana** (a neckerchief worn over the mouth).
14. Keep all 12 invented kinds as named.
15. Corn smut dropped (fungal infections of crops maybe later). Termite mushroom as plain savanna/jungle patches.
    Manna lichen from general knowledge (no article).

## PRs

1. **Fungus table and items** (built): `shared::fungus` and `fungi.ron` generated once from `final_data.py` + `items.py`
   (then edited by hand), `ItemKind::Fungus`, eating raw (healing, food effects, raw poisons as plain immediate effects
   for now, 30 to 120 s by severity), `bonus::Source` additions (at the end), Give Item, item popup (healing and food
   effects, never the poison yet), effect bar names; `research-fungi.md`, context file `shared/fungus.md`.
2. **Pictures and wiki** (built): `item_art` pictures by `form` (cap and stem, bracket, ball, coral, cup, jelly,
   honeycomb, club, truffle, lichen tufts/beards/plates) and colours from `look`, with look details added to the table
   (cap shape, gills/pores/spines/ridges, stem, ring, volva, warts); `fungus_one` for PR 4's models; `cargo xtask wiki`
   page per fungus in `docs/fungi/` (raw food effects only, never poisons) with pictures, and that page's table.
3. **Where they grow and picking** (built): `FungusPatch` on `dungeon::Level` (as `HerbPatch`), placement underground
   (caves, basements, cellars) by niche, temperature, depth; dug kinds; `dim` by skylights and mouths; wilderness
   surface patches; felled cells; server picking, harvest, protocol.
4. **Models on the maps** (built): `plants::FungusModels` (a cell's patch by form and stage from `item_art::fungus_one`:
   caps clustering, brackets on a stub of wood, jelly and crusts on a fallen stick, lichen mats, beards hanging from a
   dead snag, truffles as a disturbed mound; simplified for the maps, black outlines, a grey copy) drawn by
   `dungeon_map/fungi.rs` (remembered in grey like trees), hover names, `client.fungus_showcase`.
5. **Glow**: client light from glowing patches as decision 12.
6. **Poisons**: effects that start later, hidden from the client until recognised (as `Sheet::as_sent` strips
   alignment), deadly drain (Easy halves), stumbles from other effects, ink cap and alcohol with its bonus.
7. **Knowing fungi**: "Unknown mushroom" / "Looks like X", a Mycology skill (`gathering/mycology`), names learned by a
   check on picking, effects learned as herbs'.
8. **Alchemy**: herb-like fungi as Mortar inputs (widen `herb_stacks`, `GroundHerb`, order checks, knowledge keys,
   client panel), deadly ones included.
9. **Harm from handling**: touch and breath harm on picking and handling, gloves and `covers_mouth`, the bandana.

Order: 1 first. Then 2, 3, 6, 7, 8 in any order (non-blocking). 4 needs 3; 5 needs 4; 9 needs 3.
