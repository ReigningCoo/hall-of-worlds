# client: trade

The trade window (trading plan, `docs/context/trading.md`): one window for trading with shops, players and NPCs near enough. For now (PR 2) only its key, its button and an empty panel; the window for shops (PR 4) and for players (PR 6) fill it. Not the talk panel's shop Trade tab (`talk`), which it will replace.

## Files

- `crates/client/src/trade/mod.rs`: `TradePlugin`.
- `trade/button.rs`: `BUTTON` (`hud/trade`, at the bottom left right of the Quest Log's, 88 from the left), `TradeButton`, `spawn_button`, `bind_button` (tip "Trade"), `show_button` (hidden while connecting).
- `trade/icon.rs`: `trade_icon`, two arrows passing, drawn in code in the Character button's ink.
- `trade/markup.rs`: `TRADE`, a `panel moveable` (`trade/panel`) with only `trade/nobody` in it.
- `trade/open.rs`: `TradePanel` (the open document), `toggle_trade` (T, `common::Action::Trade`, or the button), `notice_closed` (its close button, or leaving the game).

## Public API

- `TradePlugin`. Nothing else yet.

## Depends on

- `common` (`Action::Trade`, `Preferences`, `GameView`, `Session`), `dom` (`Template`, `Pictures`, `document`, strips, `UiEvent`), `widgets` (`ButtonPressed`, `ButtonTooltip`, `UiId`).

## Used by

- `main.rs` only.
