# Woodcutting plan: felling trees, logs and kindling

Any tree can be felled with a one-handed axe, for logs that can be split into kindling; the tree leaves a stump and grows back. Owner's decisions, 2026-11-18, planned in chat `local_9121fe1b`. Research: `docs/context/research-trees.md`.

## PRs

1. **Shared rules** (built): tree kinds (`shared::tree`), the woods as materials, the Log and Kindling items, Fell Tree and Cut Kindling with the Woodcutting skill, felling times, logs per tree, growth stages with their walking costs, damage fading (`shared::woodcutting`).
2. **The server** (built): felling (cancelled by moving or Esc, damage kept), the log pile dropped where the feller stands, felled trees saved per cell on world time, regrowing, Cut Kindling, walking costs on levels, changes sent to clients.
3. **Tree models** (built) per kind and stage, and stumps, in the fruit trees' style.
4. **The client:** damage flashes for everyone near, log and kindling pictures (bark sides, plain ends), the log pile model (1, 2, 3 or more), the abilities in use, Give Item, a debug tool to age felled trees, the wiki.

## Owner's decisions

- **Any tree** can be felled: wild `Cell::Tree`s (the wilderness, oases) and fruit trees with trunks (orchards, wild ones).
- **Fell Tree** is learned the first time a one-handed axe is wielded (only one-handed axes), and can be used while one is held (not just carried) and a tree is faced (Collect Water's rule: within reach, within 45° of the heading).
- **Time:** 60 s untrained with a basic axe, down to about 30 s at best, by ∛skill + ∛the axe's quality, scaled to fit. Any axe but a hatchet takes 50% longer, whatever the skill.
- **Cancelled** by moving or Esc; the tree keeps its damage, which fades after 24 hours of world time.
- **Felled:** the top is gone, a stump is left, and a pile of 1 to 12 logs (by kind and size, fewer for young trees) is dropped where the feller stands, by the usual drop rules.
- **Log:** 8 × 3 cells, weighing 30 × 30 × 100 cm of its wood, seasoned.
- **Cut Kindling:** the first log carried, else the first log on the ground while the inventory is open and the ground's storage is shown; 1 kg pieces keeping the wood, stacking by it.
- **Regrowing:** the stump is gone after 24 hours; the tree grows again from 5 days after felling, about a week a stage; its saved entry is dropped when full grown (the seed takes over again, checked on loading). All on world time (paused with the server, moved by the debug actions that change the time).
- **Walking:** a stump costs 1.5×, a sapling 1×, a young tree 1.5×, a half-grown one 3×; a grown tree blocks.

## Research questions (Claude's defaults, overturnable)

The owner hasn't answered the research's questions yet; these defaults were used (they are also in `TODO.local.md`):

1. **Acacia stays black locust** (the casks keep it); a new **Thorn** wood is added for the savanna acacias (umbrella thorn and camel thorn).
2. **Pine's weight stays 5** (500 kg/m³; Scots pine is 550).
3. **Banana, papaya and saguaro give no logs** when felled, but still leave a stump and grow back.
4. **The research's estimated densities** are used where nothing better was found (fig's 730 too).

## Claude's choices in PR 1 (overturnable)

- **Woods:** 55 new wood materials beside the five there were, one per wild kind's wood and one per group of fruit trees in the research (`Whitewood` for pawpaw, cacao, candlenut and mongongo; `Palm` for the palms at 6; `Tamarind` also for carob and jujube; `Lychee` for rambutan and mangosteen; `Sandalwood` for quandong; `Terminalia` for kakadu plum; the nut pines as Pine). Each has its seasoned weight and the research's heartwood colour.
- **Wild kinds:** all 27 of the research (core and extras). Fruit trees don't join the wild roll: wild fruit plants stay their own, rarer thing.
- **The roll:** conifer with the chance of the cover's `narrow`, else broadleaf (the other group if none suits); kinds by biome and yearly °C, weighted common 4, uncommon 2, rare 1, half within 3 °C outside their range; `lush` above 0.5 keeps to the rainforest kinds (tropical at 20 °C and over, else temperate); willow and alder common within reach of water, rare elsewhere; where nothing suits, the kind nearest by temperature. Tundra takes stunted birch, larch and spruce.
- **Stages and logs:** saplings and stumps can't be felled; a young tree gives a quarter of its logs, a half-grown one half (at least 1 each).
- **Damage** stays whole for 24 hours after the last blow, then is gone.
- **Practice:** 48 Woodcutting a tree felled (as Collect Water's 8 for 10 seconds), 8 a log cut into kindling.
- **Cut Kindling** also needs a one-handed axe held, takes 20 − ∛Woodcutting% seconds a log (20 to 16), and gives a piece per whole kilogram of the log.
- **Kindling** takes 2 × 1 cells.

## Claude's choices in PR 2 (overturnable)

- **Blows:** felling lands in blows of 2 seconds (`connections/fell_tree.rs`), each adding its share to the tree's damage and told to everyone who sees the tree (`ServerMessage::TreeChopped`, for PR 4's flash). Several players chopping the same tree add up; stopping (moving, Esc, harm) loses only the blow under way.
- **Cleared:** the days between the stump going and the sapling are a stage of their own (`TreeStage::Cleared`): open ground, nothing drawn.
- **Felled again:** a tree growing back can be felled once young (a quarter of its logs) or half grown (half); it starts again from a stump.
- **A fruit tree** counts as young in its first round (`Tree::fruit_stage`), and can't be picked while felled.
- **Wild kinds:** the wilderness's from the nearest world map point's climate and cover, with water near (a river or the sea within 4 cells); a site's (an oasis's) from its poi's point, by water for an oasis.
- **Cut Kindling** splits one log a use; a log on the ground is the one the client names in `UseAbility`'s `target` (PR 4 sends it only while the inventory is open with the ground shown).
- **Ageing felled trees** for testing: the debug menu's World Time moves them on (PR 4 may add a tool of its own).

## Claude's choices in PR 3 (overturnable)

- **Kinds sent:** the client can't roll a wild tree's kind itself (it needs the climate and the rivers near), so the server keeps each `Tree` cell's kind on its levels (`Level::tree_kind_at`: worked out once per wilderness block, `Wilderness::tree_kinds`, and per site level) and sends it with the cells (`LevelChunk::trees`); felling reads the same.
- **Shapes:** eleven crown shapes from the research's descriptions (spire, open cone, pine top, flat tiers, column, oval, dome, spreading, weeping, umbrella, shrubby) rather than a builder per kind; bark colours picked from the research's bark descriptions, leaves by kind.
- **Sizes:** grown trees drawn 2.2 to 4.6 m tall (`1.4 + 0.5 √m` of the real height), crowns 0.9 to 3 m across, so a wood's crowns touch; growing back at 0.22 (sapling), 0.42 (young) and 0.7 (half grown) of that.
- **Felled fruit trees** grow back as their own model in leaf, smaller (the same three sizes), and leave a stump in the fruit trees' bark; once grown they are their fruit plant again.
- **Remembered:** unlike other plants, trees are drawn where remembered, in grey, so woods stay on the map.
- **Clicks:** a click on a standing tree's crown sends the player to the free cell beside it nearest them; hover names a tree by its kind and how it has grown back ("Oak, sapling").
