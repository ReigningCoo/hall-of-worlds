# shared: furnishings

Furnishings (tavern furniture plan PR 8): what castles and towers get besides tables, seats and crafting benches: bedrooms (a bed, a chest, a wardrobe), racks under their cellar casks and a wine rack beside them, a fireplace in the great hall, wall decorations, and where people live sconces, a chandelier and candles on tables. Also what layouts placed by code share (the Tavern Dev Area, The Way Inn later) and what generated racks start with. Stocking the racks is the server's (`sites/furnishings.rs`); drawing is client `dungeon_map`'s, as for any furniture and decor.

## Files

- `crates/shared/src/furnishings.rs`: the module, `Floor`, `furnish_rooms`, `is_lit`. Tests in `furnishings/tests.rs`.
- `furnishings/setting.rs`: `Setting` per kind.
- `furnishings/furnisher.rs`: what one level's furnishing keeps track of (rooms, usable cells, cask cells, `Reach`, ruin's losses, woods and stones by hash).
- `furnishings/wall.rs`: `against_wall`; spots against a wall (crate-private).
- `furnishings/cellar_racks.rs`, `hall.rs`, `bedroom.rs`, `decorate.rs`: each step.
- `furnishings/stock.rs`: `rack_fill`, `rack_bottle`.

## Public API

- `Floor { kind, seed, index, underground, ruin }`, `furnish_rooms(site_level, floor, casks)`: called by `site::Site::level` last, after `crafting_bench::place_benches`, with the level's cellar casks (`Site::cellar_casks`). Order: racks under the casks (each its size's 1-long rack or cradle with the cask's cell in it, a Tun's 2-long stand, back to the cask's wall) and perhaps a wine rack 1 to 3 long in the cellar room; on level 0 the great hall (the largest room, 20 cells or more) and its fireplace (1, 2 from 40 cells, an inglenook from 80; the longest that fits); bedrooms (6 to 80 cells, not the hall or cellar room: a bed, double from 16 cells half the time, its head to a wall, then a chest and a wardrobe of the same wood where they fit); then decor.
- `Setting { bedroom_upstairs_percent, bedroom_downstairs_percent, fireplace_percent, wine_rack_percent, cells_per_hanging, lit }`, `Setting::for_kind`: castles (70, 20, 90, 60, 10), stone towers (60, 20, 60, 30, 14), wooden towers (60, 20, 30, 20, 16), `lit` as `crafting_bench::always_lit`; `None` for every other kind.
- `is_lit(kind, ruin)`: lived in (a lit kind, not a ruin): sconces among the hangings, a chandelier over a hall of 30 cells or more, candles on 35% of tables, and fireplaces always burning (the server lights them once).
- `against_wall(furniture, at, (long, short), wall, material)`: a piece with its back to the wall on side `wall`, facing away, sized by `furniture::footprint`: for layouts placed by code.
- `rack_fill(capacity, rng)`: half to all of a generated rack's places; `rack_bottle(rng)`: a wine bottle as found on a generated rack (15% empty, else sealed, a rolled drink at a rolled quality, aged in its glass up to 120 game days).

## Depends on

- `furniture` (`Piece`, `Furniture`, `footprint`, `place::rooms`, `place::reach`), `cellar` (`CellarCask`), `cask`, `cask_rack`, `bottle_rack`, `fireplace`, `chest`, `decor`, `crafting_bench` (`always_lit`), `fermentation`, `vessel`, `dungeon`, `site` (`SiteLevel`), `door` (`Side`), `poi`, `rng`.

## Used by

- `site` (`Site::level`, the Tavern Dev Area's `against_wall`), server `sites/furnishings.rs` (`is_lit`, `rack_fill`, `rack_bottle`) and `sites/cellar.rs` (casks onto their racks).

## Notes

- All from hashes of `seed.derive("furnishings")`, never a generator's sequence, and placed after furniture, benches and cellar casks: no generator version or golden test moved. `cellar_casks` ignores pieces placed after it (`cellar/place.rs`'s `placed_before`), so the casks lie where they did before there were racks.
- Never cuts anything off (furniture's `Reach`); only on usable room floor (not by doors, stairs or arrivals), never on a cask's cell but its own rack's; wall pieces need a plain `Wall` behind and open floor in front. Ruins keep each piece, hanging and rug at 100 − 15 × ruin %.
- Decor is not saved: it is placed again each time the level is made, from the same hashes. Hangings are picked by the room (hall: trophies, shields, weapons, tapestries; cellar: hams, onions, herbs; bedrooms: tapestries, coat pegs), two cells apart, never behind a fireplace, rack, wardrobe or bench; rugs (2 × 2 or 3 × 2) in half the bedrooms and halls.
- All chances, counts and sizes are Claude's first guesses for the owner to tune.
