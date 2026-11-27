# shared: item_set

Item sets (owner, 2026-11-27; quality bonuses PR 2): a fixed pool of sets, each rolled from its number alone, and the bonuses wearing their pieces gives. Not how sets are shown (the item popup's set section is PR 3) or which planet pays which piece (`quest::set_item_of`).

## Files

- `crates/shared/src/item_set.rs`: the module list, re-exports, `POOL` (1,024), `MIN_PIECES` 2, `MAX_PIECES` 7.
- `item_set/set.rs`: `ItemSet { number, name, quality, pieces, bonuses }` (`of(number)`, taken round the pool; `bonuses_at(worn)`); placeholder names ("The Gilded Regalia").
- `item_set/roll.rs`: the pool seed, a set's quality (Common to Legendary) and pieces (prize kinds, `quest::PRIZE_KINDS`, worn in different places, a second ring allowed, never rings with gauntlets).
- `item_set/bonuses.rs`: `SetBonus { worn, bonus }`, `SET_BONUS_FACTOR` (2); one at each count from 2 up and one more at the whole set, each a different stat from `bonus::GEAR_STATS` suiting any piece, or a skill a piece's kind suits (`bonus::gear_skills` on a stand-in: armour's Defence, footwear's Kick).
- `item_set/piece.rs`: `SetPiece { set, piece }` (`whole`, `kind`, `quality`, `fitting(kind, quality, id)`: a piece of that kind from an id-picked start, of that quality if any).
- `item_set/worn.rs`: `worn_pieces(worn)` (different pieces of each set worn, only on items of the piece's kind), `set_bonuses(worn)`.
- `item_set/saved.rs`: `SavedSet` (crate only): reading `Item::set`; a save's old `set: true` becomes `SetPiece::fitting`, messages always `Option<SetPiece>`.

## Public API

- `ItemSet`, `SetPiece`, `SetBonus`, `set_bonuses`, `worn_pieces`, `POOL`, `MIN_PIECES`, `MAX_PIECES`, `SET_BONUS_FACTOR`.

## Depends on

- `bonus` (`GEAR_STATS`, `gear_skills`, `places_of`, `Bonus`), `item` (`Item`, `ItemKind`, `Quality`), `quest` (`PRIZE_KINDS`), `rng`, `id`, `creature` (`Location`), `serde`.

## Used by

- `item` (`Item::set`, `Saved`), `item_form` (`make_item`: an order's `set` becomes `fitting`), `bonus` (`worn_totals`; set items' `own_count`), `sheet` (`Sheet::totals`), `quest` (`set_item_of`), server `quests/prize.rs`.

## Notes

- Everything is rolled, never saved but `SetPiece`: changing the pool seed, `PRIZE_KINDS`' order, `GEAR_STATS` or the purposes changes every set (and every planet's set item).
- Set bonuses count in full, with no requirements, in `Sheet::totals` and `Inventory::worn_totals`; the same piece worn twice counts once, and a piece on an item of another kind not at all.
- A kind no set has (hats, say) can't be a set item: Give Item's Set and old saves leave it with none.
