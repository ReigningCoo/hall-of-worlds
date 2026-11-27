# client: trade

The trade window (trading plan, `docs/context/trading.md`): one window for trading with shops, players and NPCs near enough. Built for shops (PR 4): who to trade with at the top, the own side (8 × 8: what is being sold), the middle (each side's worth and price, who gains), the shop's side (its whole stock), and along the bottom Complete trade with what is due at the right. Players and NPCs come in PR 6. A click on a shop's door opens it on that shop (the talk panel's Trade tab is gone, Claude's choice for the owner to check).

## Files

- `crates/client/src/trade/mod.rs`: `TradePlugin` (all systems chained after `PlayerSystems::Travel`).
- `trade/button.rs`: `BUTTON` (`hud/trade`, at the bottom left right of the Quest Log's, 88 from the left), `TradeButton`, `spawn_button`, `bind_button` (tip "Trade"), `show_button` (hidden while connecting).
- `trade/icon.rs`: `trade_icon`, two arrows passing, drawn in code in the Character button's ink.
- `trade/markup.rs`: `TRADE`, a `panel moveable` (`trade/panel`): the `with` dropdown, the own side's `object-grid` (`own`, `OWN_COLUMNS` 8) in a `container`, the middle (`own_value`, `their_value`, `gain`), the shop's grid (`stock`, `STOCK_COLUMNS` 12) in a `scroll`, `said` (a refusal or how a trade ended), `complete` and `due`.
- `trade/state.rs`: `TradeWindow` (the document, `just_opened`, keepers by building, the shops listed, the shop's whole stock, both `StockGrid`s and their holders, `said`, `dirty`; `forget_trade`, `their_grid`), `Shop`; `just_refused`.
- `trade/open.rs`: `open_window`, `toggle_trade` (T, `common::Action::Trade`, or the button; opening also opens the Character panel, `inventory::OpenCharacter`), `notice_closed` (its close button, or leaving the game). Closing sends `TradeAsk::Close` and forgets the trade.
- `trade/server.rs`: `read_server` (`Keepers`; `TradeStock`, opening the window if closed, as a door click asks; `TradeNews`: `Opened` sets `common::Trading`'s keeper and terms, `State` what is sold and bought, `Refused` and `Ended` words; a new level forgets it), `follow` (the sums worked out by `ShopTrade::totals`, the shop's side its stock but what is being bought, the own side what is sold as carried), `follow_carried`.
- `trade/shops.rs`: `shops_near` (doors within `shared::trade::shop_in_reach`, nearest first, named by kind, numbered when alike), `show_shops` (the dropdown's options, choice, and disabled while anything is in the trade or no one is near; on opening, the nearest shop is asked for), `choose_shop` (`OpenTrade`).
- `trade/grids.rs`: `bind_grids`, `note_gifts` (where things were let go on either grid), `draw_grids`, `show_aim` (green where something held would land).
- `trade/show.rs`: `Middle`, `middle` (worth by `shared::value::total`, prices from the totals; gain = worth received − worth given − coins paid), `show_middle` (also `due`, `said`, and Complete enabled only with something in the trade that can be paid for), `press_complete` (`TradeAsk::Complete`).
- `trade/truesight.rs`: `ask_again_with_truesight` (asks for the stock again, only while nothing is in the trade: asking starts it afresh).
- `trade/debug_api.rs` (debug API builds): `client.trade` (see `debug-api.md`).
- `trade/words.rs`: `coins_words` ("1 gold, 5 copper"), `due_words`, `refusal_words`, `ended_words`.

## Public API

- `TradePlugin`. The trade itself is `common::Trading` (with `common::own_side`), which `inventory` draws and checks by.

## Depends on

- `common` (`Action::Trade`, `Preferences`, `GameView`, `Session`, `Trading`, `own_side`, `CurrentDungeon`, `SpotPosition`, `Truesight`, `GaveItem`, `HeldOverGifts`, `HeldStock`), `inventory` (`Carried`, `OpenCharacter`), `player` (`Player`, `PlayerSystems`), `connection`, `dom`, `widgets` (`StockGrid`, `ButtonPressed`, `ButtonTooltip`, `UiId`), `shared::trade`, `shared::value`, `shared::protocol`.

## Used by

- `main.rs` only.

## Notes

- Nothing moves until Complete trade (owner): what is bought stays the shop's (drawn unpaid in the inventory by `inventory`), what is sold stays carried (marked with a bag). Closing the window, walking out of reach (the server's `Ended { OutOfReach }`), another level or disconnecting forgets the trade.
- Selling: anything carried let go on the own side or the shop's grid (`TradeAsk::Sell`, whole things only, not part of a stack); taken off the own side anywhere else, `Unsell`. Buying: stock let go in the inventory (`Buy` with the place); an unpaid thing moved is `Unbuy` then `Buy`, let go on the shop's grid `Unbuy`. Both are `inventory`'s (`hold_items/trade.rs`).
- Sums are worked out on the client as the server does (`ShopTrade::totals`), so Complete is greyed and stock can't be dropped when unaffordable (coins + pending sales − pending buys).
- Completed, the window asks for the same shop again (the server ends a trade on completing), keeping what was paid or received in `said` until the next change; a refusal stays past the state that follows it (`just_refused`). Choosing another shop clears `said`.
- The window opens at 37% from the left, clear of the Character panel, which opens with it: overlapping, a press on one raises it and a drop lands on the wrong one.
- Asking `OpenTrade` again (another shop, truesight) starts the trade afresh on the server, so the dropdown is locked while anything is in it.
- Not built yet: offering more or less than is due (tips and haggling, `TradeAsk::Pay`, which the server takes); coins shown as words, not pictures.
