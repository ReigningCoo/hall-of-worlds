# shared: cellar

Cellar casks (drinks plan PR 9, `docs/context/drinks.md`): castles and towers may have one room of larger casks, and what each holds when first found. Where they stand is shared, from hashes; the casks themselves are items the server makes (`server` `sites/cellar.rs`) and leaves lying on the ground, so they block no one and anyone may tap, fill, ferment in or carry them off.

## Files

- `crates/shared/src/cellar.rs`: the module list and re-exports. Tests in `cellar/tests.rs`.
- `cellar/place.rs`: `CellarCask`, `cellar_percent`, `cellar_level`, `cellar_casks`.
- `cellar/contents.rs`: `stock`.

## Public API

- `cellar_percent(kind)`: castles 80, stone towers 50, wooden towers 40, others 0.
- `cellar_level(kind, seed, levels, has_basement)`: the level holding the casks, if the site has a cellar: its basement (the last level) if any, else level 0. Castles and towers have no basements, so it is their ground floor, played set into the wilderness.
- `CellarCask { at, size, wood }`; `cellar_casks(site_level, kind, seed, index)`: 3 to 8 casks in one room, the one with three places or more hashed highest; on room floor beside a plain `Wall`, never beside a door, within a cell of stairs, where anyone arrives or under furniture, a cell apart from each other (`furniture`'s `rooms_of`, `usable_cells`). Castles' sizes Barrel to Tun, towers' Kilderkin to Butt; woods mostly oak. `Site::cellar_casks(index, level)` checks the level first.
- `stock(cask, rng) -> CaskForm`: 20% empty (opened), 15% vinegar (a fruit's or malt, opened, a tenth to nine tenths full), the rest drink: barley ale 35%, else a rolled fruit's (`rolled_drink`), fermented at Poor 15, Common 55, Uncommon 22, Rare 8; aged up to 120 game days in the cask's wood (`made`, `aged`, quality by `fermentation::aged`); 30% of drinks opened and partly drunk, some way towards vinegar, the rest sealed and full.

## Depends on

- `cask` (`CaskForm`, `CaskTime`, `CaskSize`), `fermentation`, `furniture` (`place::rooms`), `dungeon`, `site` (`SiteLevel`, `Exit`), `poi`, `material`, `vegetable`, `item` (`Quality`), `potion` (`Liquid`), `rng`.

## Used by

- `site` (`Site::cellar_casks`), server `sites/cellar.rs`.

## Notes

- From `seed.derive("cellar")` and, for contents, the server's `derive("cellar contents")` sequence; placed after furniture and changing no cell, so no generator version or golden test moved.
- All numbers are Claude's first guesses for the owner to tune.
