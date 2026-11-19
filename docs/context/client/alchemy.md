# client: alchemy

The Alchemy panel (Alchemy plan PR 4, `docs/context/alchemy.md`): opened by the Alchemy ability, a moving panel where the player builds a process from the apparatus whose sub-skills they know, puts carried stacks into each step's drop spaces, sees what one run makes at its least quality, and starts it. Every figure comes from `shared::alchemy` (`Inventory::check_alchemy`, `plan`, `preview`), the server's own rules. Native and web.

## Files

- `crates/client/src/alchemy/mod.rs`: `AlchemyPlugin`, `OpenAlchemy`, the module list.
- `bench.rs`: `Bench` (resource: the panel's document, the source, the steps (`Station`: apparatus, herbs, oil (or pure alcohol), alcohol (an Alembic's), powder (a Filter's), solid (a Retort's), liquids), the count, `Running`, what the last run made); `insert`, `shift`, `remove`, `stacks`, `keep_carried`. Kept when the panel closes.
- `spaces.rs`: `Place` (a drop space: `Source`, `Herb(step, n)`, `Oil(step)`, `Liquid(step, n)`, `Alcohol(step)`, `Powder(step)`, `Solid(step)`), `places` (a step's spaces: those filled and one more while there is room, `MOST_HERBS` herbs, `RETORT_UNITS` liquids, a Mortar's oil, a Retort's solid, an Alembic's alcohol, a Filter's Powder; a Crucible and a Pelican none), `in_place`, `takes` (what each takes: a carried, unworn Paste, Ointment or Powder; a herb stack not already in that Mortar; a bottle of oil or pure alcohol; a bottle of water, oil, a drink or pure alcohol; a bottle of a drink or pure alcohol; a Powder; a solid by `alchemy::Solid::of_item`), `put`, `SpaceIcons` (the faint pictures of empty spaces), `tip`.
- `order.rs`: `Bench::order` (an `AlchemyOrder`), `check` (one run, by `Inventory::check_alchemy`), `most_runs` (each stack's count over what a run uses, 1 to `MOST_REPEATS`).
- `markup.rs`: `ALCHEMY` (`alchemy/panel`, a `panel moveable`: `alchemy/words`, the canvases `alchemy/apparatus` and `alchemy/process`, `alchemy/steps`, `alchemy/start`, the count's `fewest`, `fewer`, `count`, `more`, `most`, the `object-slot` `alchemy/product`, `product_name`, `chance`, `effects`), `WORDS`, `NONE_KNOWN`.
- `open.rs`: `open_panel` (`OpenAlchemy` opens it, with `inventory::OpenCharacter`, or closes it), `bind_panel` (marks the canvases and the product's frame), `notice_closed` (its close button, leaving the game).
- `boxes.rs`: what the rows are drawn from: `spawn_box` (two cells across, the apparatus's picture by `item_art`, its border red past the steps allowed), `spawn_running` (a step's `Wash` and `Fill`), `spawn_picture`, `space_shown` (a `dom::SlotShown`), `spawn_insert_mark`, `id`; `BOX`, `GOLD`, `WASH`.
- `rows.rs`: `draw_rows` (both canvases drawn again when the process, what is carried, the skills or the drag's aim changes): `ApparatusRow`, `ProcessRow`, `ApparatusNode`, `StepNode`, `EndNode`, `SpaceNode`, `known_apparatus` (by the character's skills: unknown apparatus are not shown).
- `drag.rs`: `press_bench` (`PreUpdate`, before `dom::DomInput::Presses` and `HoldObjects`: a click or tap on an apparatus adds it at the end, on a filled space empties it; a drag picks up an apparatus or a step as `dom::HeldObject`, drawn as a kind of item is on the action bar), `drop_on_bench` (after `HoldObjects`: `BenchAim`, where it would go (`Places::insert_at`) and whether an item held over a space may go in; on `dom::Dropped`, the change), `BenchDrag`, `BenchAim`, `Dragging`, `BenchSpots`, `Places`.
- `side.rs`: `answer_panel` (Start sends `ClientMessage::Alchemy` and keeps what one run makes in `Running`; Stop sends `CancelAbility`; the count's buttons), `show_side` (the steps allowed, the count, Start's state, the product's frame at its least quality (it can't be picked up), its name, the chance, the effects known or Unknown), `ProductSlot`.
- `words.rs`: `product_item`, `product_name`, `chance_words`, `effect_words`, `product_colour` (its strongest dose's row's colour), `refused_words`.
- `run.rs`: `follow_run` (`AbilityStarted` and `AbilityLanded` of Alchemy: a run started, what it made (shown in the end box), the last run over or called off; given up if the server starts nothing within `START_WAIT`), `show_fill` (the wash over each step while running, and the colour fill along the steps by each step's share of `run_seconds`, from `common::OwnAbility`), `stage_colour` (black, white, gold, the product's colour).
- `debug_api.rs` (debug API only): `client.alchemy`.

## Public API

- `AlchemyPlugin`.
- `OpenAlchemy` (message): opens the panel, or closes it if open (`abilities`, on using Alchemy).

## Depends on

- `inventory` (`Carried`, `OpenCharacter`), `character` (`CharacterSheet`), `common` (`HerbTable`, `OwnAbility`, `Session`, `UiPointer`, `TouchGesture`, `PointerSystems`), `connection` (`ToServer`, `FromServer`), `dom` (`Dom`, `UiEvent`, `SlotShown`, `HeldObject`, `Dropped`, `Finder`, `AbilityThing`, `DomInput`, `HoldObjects`), `widgets` (`AbilityLook`, `HoverTip`, `UiId`, `item_picture`, `row_rgb`, `logical_rect`), `item_art` (`look_of`), `shared::alchemy`, `shared::brewing`.

## Used by

- `abilities` (`OpenAlchemy`).

## Notes

- Owner's design (2026-11-19): the apparatus known along the top, two cells across; dragged down into the process row, filling from the left, reordered by dragging or put between two; each step's drop spaces under it; Start, the count (`|<` `<` `>` `>|`) and the product at its least quality on the right, each effect Unknown until known on every herb giving to it; running locks it all and washes it out, colour filling along the steps through black, white and gold to the product's colour, the result at the end of the last apparatus; moving or harm calls it off (the server's, as for any wind-up).
- Claude's choices: a step dragged off the row is taken out; a click or tap adds an apparatus at the end and empties a filled space (so it all works by touch and by the debug API); an apparatus known but not carried is still shown, its tip saying so (`check_alchemy` refuses it); a source space before the first step takes a stored Paste, Ointment or Powder; herb spaces grow to `MOST_HERBS` (more kinds than Alchemy allows halve the chance); Start becomes Stop while running; what was made shows in the end box until the process changes. The panel is a moving panel, not a dialog, so the Character panel can stay open beside it to drag from.
- Running: one order runs `count` times on the server, each its own `AbilityStarted`; the fill restarts for each. The product's least quality is from Start; the shown result takes the quality the server rolled.
- Ids: `alchemy/kit.<apparatus index>` (top row), `alchemy/step.<n>`, `alchemy/end.0`, `alchemy/source.0`, `alchemy/herb.<step × 100 + n>`, `alchemy/oil.<step>`, `alchemy/liquid.<step × 100 + n>`, `alchemy/alcohol.<step>`, `alchemy/powder.<step>`, `alchemy/solid.<step>`.
