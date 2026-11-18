# client: effect_bar

The bar of timed effects at the top middle of the window: an icon for each effect on the character that has an end (eating, a herb eaten raw, poison, the debug menu's Bonus), its shade growing down from the top as it runs out, and a `hover` popup saying what it does and when it ends. Effects for good (`end: None`, as worn ones would be) never show. Not the effects' rules (`shared::bonus`) nor the bonuses' wording (`common::bonus_line`, shared with `item_popup`). Native and web.

## Files

- `crates/client/src/effect_bar/mod.rs`: `EffectBarPlugin`, the module list.
- `common::ServerTick` (in `common/action_tick.rs`, kept here by `take_clock`; also read by `abilities` for drinking's cooldowns): the tick the server last said in `ServerMessage::Clock`, and when by `Time`; `now(elapsed)` counts on at `energy::ACTION_TICKS_PER_SECOND`, never back.
- `effect_bar/looks.rs`: `timed` (effects with an end not yet reached), `is_harmful` (drains, or less of anything), `colour` (calm blue-green; sickly yellow-green when harmful), `icon_path` (every food `delapouite/eating`, fruit too, a fruit's effects `lorc/shiny-apple`, a vegetable's `delapouite/carrot`, a raw herb `delapouite/herbs-bundle`, ointment `delapouite/covered-jar`, Poison `lorc/poison-bottle`, Dev `lorc/magic-swirl`; listed in `xtask/icons.txt`), `gone` (0 at its start, 1 at its end).
- `effect_bar/bar.rs`: `EFFECT_BAR` (markup: a row as wide as its icons, 12 down in the middle of `dom::top_strip()`, holding a `canvas`), `spawn_bar`, `bind_canvas`, `draw_icons` (draws them again whenever which effects show changes: `EffectIcon(source)` on each `Solid` frame, an action bar slot's size), `show_shades` (each `EffectShade` over its frame, `gone` of its height, from the top).
- `effect_bar/words.rs`: `name` ("Eating: Bread", "Eating: Apple" for a fruit and the fruit's name for its effects, by `fruit::Fruits::by_tag`, the same for a vegetable by `vegetable::Vegetables::by_tag`, "Poisoned", "Dev Bonus"), `bonus_lines`, `ends_early` ("Ends if you move or are hurt"), `ends_in` ("Ends in 12 s", "Ends in 4 min 30 s", rounded up), through `effect.ftl`.
- `effect_bar/popup.rs`: `EFFECT_POPUP` (`popup hover`: `name`, `bonuses`, `ends`, `left`), `EffectPopup` (the one open and the icon rested on), `forget_closed`, `follow_pointer` (opens after `REST`, 0.15 s, on an icon found by `dom::Hit`, in place of one not locked; keeps it open through `dom::KeptOpen`), `count_down` (the time left, once a second; closes it when its effect goes).

## Public API

- `EffectBarPlugin`.

## Depends on

- `character` (`CharacterSheet`: its `effects`), `connection` (`FromServer`: `Clock`), `common` (`Wording`, `amount_of`, `bonus_line`, `written`), `dom` (`Template`, `document`, `top_strip`, `Solid`, `Hit`, `KeptOpen`, `Popup`, `Dom`, `UiEvent`, `HeldObject`, `BuildDocuments`, `ClosePopups`), `widgets` (`ABILITY`, `ITEM_EDGE`, `spawn_outlined_icon`, `UiId`), `shared::bonus`, `shared::lang`.

## Used by

- Nothing. `notices` sits below it (72 down).

## Notes

- The client has no action clock of its own: the server sends `Clock { tick }` after every `Sheet`, and `ServerTick` counts on from it by `Time` (virtual), which stops while single player is paused, as the server's clock does. Nothing shows before the first `Clock`.
- An icon goes at its end by the client's tick, or at once when a `Sheet` comes without it (moving or being hurt ended it, or another food took its place: one food counts at a time).
- Only the icons take the pointer (the form has no height and the row is only as wide as they are), so the rest of the top of the map is never covered.
