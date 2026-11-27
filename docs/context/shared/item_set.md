# shared: item_set

Item sets (owner, 2026-11-27; quality bonuses PR 2): a fixed pool of sets, each rolled from its number alone, and the bonuses wearing their pieces gives. Not how sets are shown (the item popup's set section, `client: item_popup`'s `set.rs`) or which planet pays which piece (`quest::set_item_of`).

## Files

- `crates/shared/src/item_set.rs`: the module list, re-exports, `POOL` (1,024), `MIN_PIECES` 2, `MAX_PIECES` 7.
- `item_set/set.rs`: `ItemSet { number, name, quality, pieces, bonuses }` (`of(number)`, taken round the pool; `bonuses_at(worn)`); placeholder names ("The Gilded Regalia").
- `item_set/kinds.rs`: which kinds may be pieces (`worn_kinds`, `weapons`): every worn or held kind with quality that is kept (not counted, not a bag), but whole suits of armour, a horse's armour and the lantern; `ItemKind::ALL`'s order.
- `item_set/theme.rs`: `Theme::{Layer, Place}` (owner, 2026-11-27): a set's worn pieces all on one layer at different places (a second ring allowed), or all at one place on different layers; layers weighted Skin, Middle, Outer 3 each, Over 1; places only where worn kinds cover two layers or more.
- `item_set/roll.rs`: the pool seed, a set's quality (Common to Legendary), theme and pieces: worn pieces of its theme first, then weapons (0, 1 or 2 asked for, 6:3:1, different kinds), topped up to 2 weapons when the worn ones fall short; every set rolled once and kept (`OnceLock`).
- `item_set/bonuses.rs`: `SetBonus { worn, bonus }`, `SET_BONUS_FACTOR` (2); one at each count from 2 up and one more at the whole set, each a different stat from `bonus::GEAR_STATS` suiting any piece, or a skill a piece's kind suits (`bonus::gear_skills` on a stand-in: armour's Defence, footwear's Kick).
- `item_set/piece.rs`: `SetPiece { set, piece }` (`whole`, `kind`, `quality`, `fitting(kind, quality, id)`: a piece of that kind from an id-picked start, of that quality if any).
- `item_set/worn.rs`: `worn_pieces(worn)` (different pieces of each set worn, only on items of the piece's kind), `pieces_worn(worn, set)` (which of one set's pieces, for the popup), `set_bonuses(worn)`.
- `item_set/saved.rs`: `SavedSet` (crate only): reading `Item::set`; a save's old `set: true`, or a piece whose set now has another kind there, becomes `SetPiece::fitting` (deterministic, from the id), messages always `Option<SetPiece>`.

## Public API

- `ItemSet`, `SetPiece`, `SetBonus`, `set_bonuses`, `worn_pieces`, `pieces_worn`, `POOL`, `MIN_PIECES`, `MAX_PIECES`, `SET_BONUS_FACTOR`.

## Depends on

- `bonus` (`GEAR_STATS`, `gear_skills`, `places_of`, `Bonus`), `item` (`Item`, `ItemKind`, `Quality`, `WearDef`), `rng`, `id`, `creature` (`Layer`, `Location`), `serde`.

## Used by

- `item` (`Item::set`, `Saved`), `item_form` (`make_item`: an order's `set` becomes `fitting`), `bonus` (`worn_totals`; set items' `own_count`), `sheet` (`Sheet::totals`), `quest` (`set_item_of`), server `quests/prize.rs`, client `item_popup` (`set.rs`).

## Notes

- Everything is rolled, never saved but `SetPiece`: changing the pool seed, the kinds a piece may be (`ItemKind::ALL`, a kind's layer or places), `GEAR_STATS` or the purposes changes every set (and every planet's set item); saved pieces are then fitted again on loading, and the protocol version must go up so client and server agree.
- Set bonuses count in full, with no requirements, in `Sheet::totals` and `Inventory::worn_totals`; the same piece worn twice counts once, and a piece on an item of another kind not at all.
- A kind no set has (whole suits, a horse's armour, lanterns, bags) can't be a set item: Give Item's Set and old saves leave it with none.
- A torso piece's own lining (`Item::padded`) also takes Middle, so a lined cuirass and a gambeson of one place-set clash when worn together; the sets go by kinds only.
