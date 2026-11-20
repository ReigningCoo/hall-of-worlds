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
5. **Glow** (built): client light from glowing patches as decision 12, in `dungeon_map`'s fixed lights (`fixed_lights.rs`, shared with lava's light later): only full-grown patches glow, so picked ones (back to stage 3) stop until grown again; light reaches from every cell of a group whatever is seen, but remembered cells are drawn grey as ever; the models of glowing ones are drawn in full colour in the dark (`HerbMaterials::glow`).
6. **Poisons** (built): effects laid on to start later (`bonus::Later` on `Sheet::later`, started by
   `Sheet::forget_ended`), hidden from the client until recognised (`Sheet::as_sent` leaves out those not known):
   recognised on eating by knowing that fungus's poison (`FungusKnowledge::knows_poison`, learned by suffering it)
   or a Healing check (`fungus::recognised`), and shown on the effect bar as "Starts in" when known. Onsets: soon
   at once, delayed 10 game minutes, late 60; a false recovery is 30 s of sickness, 60 game minutes eased, then the
   worst. Deadly: Health drained by `bonus::Stat::DeadlyDrain` at 3 × the eater's most over its 120 s (Easy half),
   with no floor, the server killing at 0 (`die_of_poison`); dying cures poisons. Confusion: fly agaric, panther
   cap and Dreamcap (`Poison::confuses`) stumble one move in 8 (`Sheet::stumble_one_in`). Common ink cap: Toughness
   and Will +1 for 3 game days (`Source::FungusWindow`), alcohol drunk then bringing its poison at once.
7. **Knowing fungi** (built): `FungusKnowledge` widened as `HerbKnowledge` (names, herb effects, poisons; knowing
   the name knows the poison), shown as "Unknown mushroom" / "Unknown lichen" / "Looks like X" (`Shown`,
   `client: common`'s `fungus_name`) in hover names, item popups, the effect bar, notices and the held item; a
   Mycology skill (`gathering/mycology`, Intellect and Perception) practised by picking, whose check (20% + Mycology
   + half Foraging, at most 95%) teaches the name; it also counts towards recognising a poison on eating (with
   Healing). A popup tells what eating does only once the fungus is known by name, its poison then too. Herb
   effects are kept for PR 8 to teach. The Vitals message no longer tells when a hidden poison starts
   (`Sheet::told_limits`).
8. **Alchemy** (built): the 36 fungi with herb effects, deadly ones included, are Mortar inputs beside herbs:
   `brewing::Ingredient` (Herb or Fungus) on `GroundHerb` (`GroundHerb::fungus`, Common as fungi have no quality),
   `HerbStack`, `alchemy::Contribution` and `Lesson`, so a herb's and a fungus's keys never clash; `herb_stacks`
   offers whole fungi (not servings); Herbalism practised as for herbs; the preview and lessons read and teach
   `FungusKnowledge`'s effects (`alchemy::Known`). Poison pastes coating weapons are for later (a TODO).
9. **Harm from handling** (built): a `Touched` or `Breathed` poison (oakmoss; common puffball, cellar cup, split gill,
   Sleepdust puffball) harms whoever picks it or grinds it in a Mortar (`fungus::handling`, `Guard`, `Guarded`;
   server `connections/fungus_harm.rs`), from its onset as if eaten, hidden until recognised. Touch is kept off by
   anything worn on the Hands (`Inventory::hands_covered`), breath by `Item::covers_mouth` (`Inventory::mouth_covered`:
   a litham, a closed helmet, an aventail `Closed` or `Ventail`, a set with either, and the new **Bandana**, a
   `NeckwearForm` drawn as a neck roll, the litham's face cloth and a knot behind). A known fungus's popup says how it
   harms and "Gloves protect" or "Cover the mouth". Herbs harming whoever handles them stay a TODO (no herb data for it).

All 9 built.

Order: 1 first. Then 2, 3, 6, 7, 8 in any order (non-blocking). 4 needs 3; 5 needs 4; 9 needs 3.
