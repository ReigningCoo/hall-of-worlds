# shared: trade

The rules every trade goes by (trading plan PR 3, `docs/context/trading.md`), shared so the client can say beforehand what the server will do: a shop's rates for a character, what it pays and asks, what is due, tips and haggling, and completing a shop trade all at once; trades between characters (PR 5): sides, offers, locking in, and the receive box. Not the trade window (client `trade`).

## Files

- `crates/shared/src/trade.rs`: the module list and re-exports; `REACH_CELLS` (5), `REACH` (units), `shop_in_reach(at, door)`. Tests in `trade/tests.rs`.
- `trade/rates.rs`: `Rates { buys, sells }` (percent of value), `Rates::for_shop(standing, trading)`, `BUYS_AT` 50, `SELLS_AT` 150, `MOST_STANDING` and `MOST_SKILL` (25 each), `standing_percent`, `skill_percent`.
- `trade/price.rs`: `shop_pays(value, rates, normally_sold)`, `shop_asks(value, rates)`, `NOT_SOLD_PERCENT` (50), `normally_sold(shop kind, item kind)` (`shop::sells`).
- `trade/settle.rs`: `due(asks, pays)`, `haggle_room(due, trading)`, `judge(due, paying, trading)`, `Payment` (`Exact`, `Tip { copper, generous }`, `Haggled { copper }`), `HAGGLE_PERCENT`, `TIP_PERCENT`, `TRADE_PRACTICE`.
- `trade/shop_trade.rs`: `ShopTrade { with, buying, selling, paying }` (`new`, `has`, `totals`), `Buy { item, to, x, y, rotation }`, `Totals { asks, pays, due }` (`affordable`), `Refusal`, `for_sale(item)`.
- `trade/between.rs` (PR 5): `SIDE_CELLS` (8), `Offer { item, x, y, rotation }`, `Side { offers, locked }` (`has`, `offer`, `take_back`, `still_carried`, `hand_over`), `Between { with, mine, theirs }` (a trade as one side is told it), `for_trade(item)` (anything but quest items or what holds one; coins can). Tests in `trade/between_tests.rs`.
- `trade/received.rs` (PR 5): `Received { items }` (the receive box: `is_empty`, `take(place, carried)`, `take_all`), `TradingSetting` (`Allow`, `Ask` the default, `Never`).
- `trade/complete.rs`: `complete(...) -> Completed { inventory, stock, totals, paid, payment }`.
- Coins carried and paid: `item/inventory/pay.rs` (`Inventory::coins_carried`, `pay_coins`, `receive_coins`, `PayRefused`; see `shared/item.md`).

## Public API

- `Rates::for_shop(Standing, trading %)`: 50% and 150% moved towards 100% by standing (Stranger 0 to Champion 25, evenly) and Trading (25 at 100%), added.
- `shop_pays`, `shop_asks`: fine units (`shared::value`) from a stack's `value::total`; not normally sold pays half again less, after everything; resold at the normal price.
- `due(asks, pays)`: copper the player pays (negative: is paid), rounded up when paying and **down** when paid.
- `judge(due, paying, trading)`: what is paid and how: more than due a tip (`generous` past 5% of due), less a haggle allowed up to `haggle_room` (5% of due at 100% Trading, in step with the skill), else `Refusal::TooShort`.
- `ShopTrade::totals(stock, carried, rates, shop kind)`: worked out on a copy, refusing what has gone or isn't for sale (coins, quest items, anything holding either); something sold inside something sold is refused.
- `complete(trade, stock, carried, (rates, kind), trading, limits, made_at, new_id)`: all or nothing, on copies: sold things into the stock, coins paid highest first with change, what is bought where asked (else wherever it fits), the shop's coins last.
- `Refusal`: `NotStocked`, `NotCarried`, `NotForSale`, `AlreadyIn`, `TooShort`, `CantAfford`, `NoRoom`, `NotHere`, `Nothing` (nothing bought or sold and nothing given: a gift alone, a tip with nothing traded, completes), and for trades with characters `Never`, `Busy`, `Unclaimed` (the receive box isn't empty), `Denied`.
- `in_reach(a, b)`: two characters within `REACH`.
- `Side::offer(offer, carried)`: carried, tradeable, not inside (or holding) something else offered, fitting in 8 × 8 beside the rest; moving one already there; any change unlocks the side. `Side::hand_over`: all of it taken out of a copy of the inventory, or nothing.
- `Received::take` puts one thing where asked (`Inventory::add`); `take_all` gives everything that fits (`Inventory::give`), the rest staying.

## Depends on

- `shared::value` (`total`, `copper_shown`, `FINE_PER_COPPER`), `item` (inventory, coins), `quest` (`Standing`), `shop` (`sells`), `site` (`BuildingKind`), `spot`.

## Used by

- `shared::protocol` (`TradeAsk`, `TradeNews`, `ClientMessage::SetTrading`), `server` (`connections/trade.rs` and its `deal_*.rs`, `sites` reach, `storage::TradeSave`).

## Notes

- Nothing moves until completed: what is bought stays in the shop's stock (with the place it is to go), what is sold stays carried. Closing, walking out of reach, another level or disconnecting just forgets the trade.
- Rounding a shop's payment down (Claude's, against the owner's "rounded up"): otherwise a pinch of anything sells for a whole copper again and again.
- Tips, practice and standing are the server's (`connections/trade/settle.rs`): a tip up to 5% practises Trading, more unpractises it (`Skills::unpractise`); every 1,000 copper tipped is a point of standing (`quests::tipped`).
- All figures Claude's first guesses, to balance, but the owner's 50/150/25/25/5%.
