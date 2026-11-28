# client: trade

The trade window (trading plan, `docs/context/trading.md`): one window for trading with shops, players and NPCs near enough. Who to trade with at the top (players, shops, NPCs, each nearest first; anyone set to Never greyed out), the own side (8 × 8: what is sold or offered), the middle (each side's worth, prices, who gains), and their side. With a shop (PR 4): the Buying box (8 wide, 4 rows or more as the server says, trading tweaks PR 2) above its whole stock, and along the bottom Complete trade with what is due at the right. With a player or an NPC (PR 6): their 8 × 8, seen but not touched, and the Trade toggle in the middle; a side locked in is drawn darker with a solid edge, the other's too, and "Asking..." shows over theirs (darkened) while they haven't answered. Once both lock in, only the receive box shows, with Take all, until emptied. A click on a shop's door opens the window on that shop (the talk panel's Trade tab is gone, Claude's choice for the owner to check). Another player asking (their setting Ask) brings up a requester: Allow or Deny.

## Files

- `crates/client/src/trade/mod.rs`: `TradePlugin` (all systems chained after `PlayerSystems::Travel`).
- `trade/button.rs`: `BUTTON` (`hud/trade`, at the bottom left right of the Quest Log's, 88 from the left), `TradeButton`, `spawn_button`, `bind_button` (tip "Trade"), `show_button` (hidden while connecting).
- `trade/icon.rs`: `trade_icon`, two arrows passing, drawn in code in the Character button's ink.
- `trade/markup.rs`: `TRADE`, a `panel moveable` (`trade/panel`): the `with` dropdown; `trading` holding the own side's `object-grid` (`own`, `OWN_COLUMNS` 8) in a `container`, the middle (`own_value`, `their_value`, `gain`, the `lock` toggle, `their_lock`), their side (`asking`, the Buying box `buying` under `buying_label`, the shop's grid `stock`, `STOCK_COLUMNS` 12, in a `scroll`, or a character's `theirs`, 8 × 8); the receive box (`received_frame`, `received`, `take_all`); `said` (a refusal or how a trade ended), `bottom` (`complete`, `due`). `ASKED`: the requester (`trade_asked/dialog`, `who`, `allow`, `deny`).
- `trade/state.rs`: `TradeWindow` (the document, `just_opened`, keepers by building (`protocol::Keeper`, with who they are), who is listed, the shop's whole stock or a character's offered things, the `StockGrid`s and their holders (`buying`: the Buying box, exactly as many rows as `ShopTrade::buying_rows`), the receive box (`received`, `receive`, `receiving()`), who is asking (`asked`), `said`, `dirty`; `forget_trade`, `their_grid`), `Partner` (`with`, `name`, `never`); `just_refused`.
- `trade/people.rs`: `Seen`, `people_near` (players, and NPCs: people kinds not unfriendly, within `shared::trade::in_reach`, nearest first, beggars before other NPCs), `numbered` ("Human 2").
- `trade/between.rs`: `hear_between` (`Between` opening or changing a trade with a character, `Asking`, `Asked`, `Received`, `Setting`), `placed` (a side's things where they were put), `follow_between` (the own side placed and locked as the server holds it).
- `trade/sides.rs`: `Mode` (`Shop`, `Between`, `Receiving`), `mode`, `show_sides` (which parts show; the toggle pressed, `selected`, while locked in), `draw_their_side` (their `GridShown`, with no `StockOf`, so nothing there can be picked up; `locked` while they are locked in or asked), `press_lock` (`Lock`, and Take all's `TakeAll`).
- `trade/asked.rs`: `spawn_asked`, `show_asked`, `answer_asked` (`Answer`; Deny, or a press outside).
- `trade/open.rs`: `open_window`, `toggle_trade` (T, `common::Action::Trade`, or the button; opening also opens the Character panel, `inventory::OpenCharacter`), `notice_closed` (its close button, or leaving the game). Closing sends `TradeAsk::Close` and forgets the trade.
- `trade/server.rs`: `read_server` (`Keepers`; `TradeStock`, opening the window if closed, as a door click asks, or a character's trade opening; `TradeNews`: `between`'s first, then `Opened` sets `common::Trading`'s keeper and terms, `State` what is sold and bought (into the Buying box where the server says), `Refused` and `Ended` words, `Ended` from whoever asked taking the requester down; a new level forgets it), `follow` (the sums worked out by `ShopTrade::totals`, the shop's side its stock but what is being bought, the own side what is sold as carried), `follow_carried`.
- `trade/shops.rs`: `shops_near` (doors within `shared::trade::shop_in_reach`, nearest first, named by their keeper as the talk panel names a giver, `quests::giver_title` ("Elien Radwell, tavern keeper"), numbered when alike), `show_shops` (the dropdown: players, shops, NPCs (`common::Other` with `HoverName`), Never greyed (`Dom::set_options_greyed`), the choice, disabled while anything is in the trade, the receive box holds anything or no one is near; on opening, the nearest shop is asked for), `choose_shop` (`OpenTrade`, never to someone greyed).
- `trade/grids.rs`: `bind_grids`, `note_gifts` (where things were let go on each grid; stock or the box's things let go on the Buying box sent as `Buy` with where they land; with a character, the own side's landing place sent as `Offer`), `draw_grids` (own, Buying box, shop's, receive box), `show_aim` (green where something held would land; none over the Buying box for new stock that can't be paid for).
- `trade/show.rs`: `Middle`, `middle` (worth by `shared::value::total`, prices from the totals; gain = worth received − worth given − coins paid; with a character only each side's worth), `show_middle` (also `due`, `said`, and Complete enabled only with something in the trade that can be paid for), `press_complete` (`TradeAsk::Complete`).
- `trade/truesight.rs`: `ask_again_with_truesight` (asks for the stock again, only while nothing is in the trade: asking starts it afresh).
- `trade/debug_api.rs` (debug API builds): `client.trade` (see `debug-api.md`).
- `trade/words.rs`: `coins_words` ("1 gold, 5 copper"), `due_words`, `refusal_words`, `ended_words`.

## Public API

- `TradePlugin`. The trade itself is `common::Trading` (with `common::own_side` and `receive_box`), which `inventory` draws and checks by; the setting is `common::TradeSetting`, which `system_menu` shows.

## Depends on

- `quests` (`giver_title`), `common` (`Wording`, `Action::Trade`, `Preferences`, `GameView`, `Session`, `Trading`, `TradeSetting`, `own_side`, `receive_box`, `Other`, `HoverName`, `CurrentDungeon`, `SpotPosition`, `Truesight`, `GaveItem`, `HeldOverGifts`, `HeldStock`), `inventory` (`Carried`, `OpenCharacter`), `player` (`Player`, `PlayerSystems`), `connection`, `dom`, `widgets` (`StockGrid`, `ButtonPressed`, `ButtonTooltip`, `UiId`), `shared::trade`, `shared::value`, `shared::protocol`.

## Used by

- `main.rs` only.

## Notes

- Nothing moves until Complete trade (owner): what is bought stays the shop's, in the Buying box and never in the inventory (completed, it goes wherever it fits, the rest into the receive box, which then shows), what is sold stays carried (marked with a bag). Closing the window, walking out of reach (the server's `Ended { OutOfReach }`), another level or disconnecting forgets the trade.
- Selling: anything carried let go on the own side or the shop's grid (`TradeAsk::Sell`, whole things only, not part of a stack); taken off the own side anywhere else, `Unsell`. Buying (trading tweaks PR 2): stock let go on the Buying box (`Buy` with where it lands, `grids.rs`); moved within it, `Buy` again; let go on the shop's grid, `Unbuy` (`inventory`'s `hold_items/trade.rs`); it can't go into the inventory. The server also takes `Buy` with no place (the first free one, growing the box), for PR 3's right-click.
- Sums are worked out on the client as the server does (`ShopTrade::totals`), so Complete is greyed and stock can't be dropped when unaffordable (coins + pending sales − pending buys).
- Completed, the window asks for the same shop again (the server ends a trade on completing), keeping what was paid or received in `said` until the next change; a refusal stays past the state that follows it (`just_refused`). Choosing another shop clears `said`.
- The window opens at 37% from the left, clear of the Character panel, which opens with it: overlapping, a press on one raises it and a drop lands on the wrong one.
- Asking `OpenTrade` again (another shop, truesight) starts the trade afresh on the server, so the dropdown is locked while anything is in it.
- With a character (PR 6): what is carried let go on the own side is offered where it lands (`inventory` doesn't send `Sell` then); moved there, offered again; taken off, `Unsell`. Any change unlocks that side (the server's rule). Their side has no `StockOf`, so nothing there can be picked up, and takes no gifts. The receive box's things are taken out by dragging into the inventory (`inventory` sends `Take` with the place) or Take all; nothing can be let go into it. While it holds anything the dropdown is disabled (the server refuses any trade: `Unclaimed`).
- Not built yet: offering more or less than is due (tips and haggling, `TradeAsk::Pay`, which the server takes); coins shown as words, not pictures.
