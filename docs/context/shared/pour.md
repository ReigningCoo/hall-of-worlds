# shared: pour

Pouring (drinks plan PR 2, `docs/context/drinks.md`): one rule for every container of a liquid, casks (`shared::cask`), vessels (`shared::vessel`) and bottles (`shared::potion`). The server decides (`ClientMessage::Pour`, `connections/pour.rs`); the client aims by the same rule (`inventory`'s `Target::Pour`).

## Files

- `crates/shared/src/pour.rs`: the module list and re-exports. Tests in `pour/tests.rs`.
- `pour/held.rs`: `BOTTLE_ML`, `Held`, `holding` (the same container holding something else).
- `pour/rule.rs`: `pour`, `Poured`, `PourRefused`, `pour_cost`.
- `pour/in_place.rs`: `Inventory::replace_carried`.

## Public API

- `BOTTLE_ML` (100): a bottle is full or empty, never in between.
- `Held { liquid, ml, room }`, `Held::of(item)`: what a cask, vessel or bottle holds and takes; `None` for anything else.
- `pour(from, into, tap) -> Result<Poured, PourRefused>` (one each, a stack's top): as much as there is, up to `into`'s room. Refused: `NotContainers`, `Itself`, `Empty`, `Potion` (potions stay in bottles), `Different` (never two liquids), `NeedsTap` (drawing from a cask without a tap carried), `Full` (no room, or a bottle that can't be emptied whole), `TooLittle` (not a bottle's worth to fill one). `PourRefused::words` (English, for logs and the client's red line).
- `Poured { from, into, ml }`: both after. Casks poured from or into are opened (`CaskForm::opened`); `into` is sealed (a wine bottle, or a bottle: `BottleForm::with_seal`) when it was empty and filled from a cask; anything poured from loses its seal.
- `pour_cost(ml)`: handling the liquid's weight (`energy::handling_cost`, a gram a millilitre), the same on both sides.
- `Inventory::replace_carried(item)`: puts a changed container back in place of the carried one with its id, if it is alone and of the same kind.

## Depends on

- `cask` (`CaskForm`), `vessel` (`VesselForm`), `potion` (`BottleForm`, `Liquid`), `item` (`Item`, `ItemKind`, `Inventory::find_mut`), `item_form`, `energy`, `serde`.

## Used by

- Server `connections/pour.rs` (also `side` and `put_back` for `drink_vessel.rs`), client `inventory` (`hold.rs` aims, `hold_items/pour.rs` lets go), `item_popup` (`Held` for how much is held).

## Notes

- A stack's top one is poured: the server takes it off and gives it back on its own (a new id for counted bottles); one alone changes where it lies, carried or on the ground. The client shows a carried one alone at once and waits for `Inventory` otherwise.
- The client aims only at carried containers, held from the inventory or from the ground; pouring into something lying on the ground is the server's already, but not aimed at yet.
