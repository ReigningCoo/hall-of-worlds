# shared: shop

What town shops have in stock, and working it out when one is opened. Only the server will call it, as the seed is secret. It makes no items, since ids are the server's: it says which old stock to keep and what to add. The server calls it when a shop is opened (`sites::open_trade`).

## Files

- `crates/shared/src/shop.rs`: `Shop`, `sells`, `NewStock`, `Restocked`, `catch_up`, `REPLACED_AFTER`, `TOP_UP_AFTER`, `DOOR_REACH` (360 units from the door's middle: any cell beside it), `beside_door(spot, door)` (the one rule both sides use). Tests. In `shop/`: `shop.rs` (`Shop`), `sells.rs`, `armoursmith.rs` (the armoursmith's long list: every armour piece and set), `bottled.rs` (what shops' bottles hold), `catch_up.rs`.

## Public API

- `sells(BuildingKind)`: what it sells, with weights; empty if it isn't a shop. Herbalists and potion shops sell Sulphur and Mercury (Retort solids, one at a time). Vessels (drinks PR 2) at carpenters (with taps), leatherworkers, jewellers and herbalists. Blacksmiths sell the Keyring, herbalists the Herb Pouch, eggs (`shared::farmyard`, until towns have farm shops or general stores; the animal by the town's land, `shared::produce`) and yeast (until towns have bakers; `shared::yeast`, nearly always Common by `yeast::shop_quality` in `catch_up`), fruit and vegetables (potion shops some fruit: which ones, by the town's climate, is `shared::produce`, chosen by the server); herbalists and potion shops sell bottles. Brewing's tools: carpenters sell the Oil Press and casks (`shared::cask`, the small sizes far more often), herbalists and potion shops the Mortar and Pestle, the Retort, the Alembic, the Filter and the Pelican (Alchemy's apparatus), blacksmiths the Crucible, the Cauldron and the Pan, carpenters the Bowl and the Sieve (cooking's kitchen tools). Millers sell only flour (which grain, by the town's climate, is `shared::produce`'s `flour_grain_for`).
- `bottled(BuildingKind)`: the liquids its bottles hold (herbalists and potion shops: Water, Cave Water, Moon Water and the four oils, never Salt Water or Rain Water; herbalists milk and honey too); `stock_request(shop, item kind, seed)`: the `RollRequest` a new stock item is given its form by, a bottle's liquid picked from its seed, a vessel one the shop makes (`vessel::vessel_for_shop`), anything else `NONE`.
- `Shop { kind, quality, trades, of_kind }`: `target`, `minimum` (80% of it), `tops_up`, `make_minutes(quality, kind_percent)`, `replaced_after`.
- `catch_up(seed, shop, stock qualities, last_made, last_visit, now) -> Restocked { keep, new, last_made }`.

## Depends on

- `shared::item` (the server suits fruit and vegetables with `shared::produce`), `shared::item_form` (`RollRequest`), `shared::potion` (`LiquidKind`), `shared::rng`, `shared::site` (`BuildingKind`).

## Used by

- `server` (`sites`), client `talk`.

## Notes

- Make time: the quality's base × the kind's `ItemDef::make_percent` × 150% to 50% by shop quality ÷ √(shops of the kind in town). While time is filled the kind isn't known, so it uses the average for what the shop sells.
- Quality odds are `Quality::odds` weighted up with shop quality and trades, stopping at Legendary: artifacts only come from a questline.
- One thing is made at a time. At its count, each one finished pushes one out, by `sell_weight` (Common 100 to Legendary 50), so selling keeps pace with making, rivals included, and stock stands still during a slow make.
- A roll is hashed from the minute making began, so what is part made stays the same however often a shop is looked at.
- Past `replaced_after` (90 days ÷ √rivals) old stock is gone and only that much time is run, so a long absence costs no more.
- Reopening after `TOP_UP_AFTER` (a game day) below `minimum` adds extras up to a random count between it and the target, using none of the shop's time.
- A miller has his own setting (owner, 2026-11-18; `MILLER_MOST` and `MILLER_SACKS_AT_*` in `shop.rs`): at most 10 sacks, made at 2 a game day for the worst miller to 3 for the best (not by rivals or the flour, which has no quality), and never topped up on reopening (`tops_up`). Players' sheaves milled for them, when paid trade exists, are apart from this (`docs/context/milling.md`).
- All numbers are first guesses. Design: `decisions.md`, 2026-11-02, item quality.
