# client: spell_design

The spell design panel (magic plan PR 9, `docs/context/magic-system.md`): opened by the Magic ability, a moving panel where the player builds a spell from steps (wheres, filters, effects), sees what it costs and what is wrong as it is built, and saves it in the spellbook with a name or the automatic one. Every rule is `shared::spell`'s (`check`, `spell_cost`, `step_cost`, `unlocked_wheres`, `auto_name`, `Spellbook::store` on the client's copy of the book), so the panel agrees with the server. Not the spellbook list (the Spells tab, `abilities/spellbook/`) or casting. Native and web.

## Files

- `crates/client/src/spell_design/mod.rs`: `SpellDesignPlugin`, `OpenSpellDesign`, `EditSpell`, the module list.
- `draft.rs`: `Draft` (resource: the panel's document, the steps as `shared::spell::Step`s with no filters on their wheres, the filters apart (`filters`: always the first step's, put on its where by `spell`; `set_steps` takes them off a design; forgotten with the last step; `filters_stranded` when the first step isn't a where), the player's own name kept from an edited spell, the Save requester and the name typed); every change, a where or effect at a time: `add_step`, `takes`, `add_to`, `append` (clicks: to the last step if it takes it, else a new step), `set_filter` (on only with a where first), `has_filter`, `shift`, `remove`, `remove_unit` (its step too once empty), `move_unit` (`Goes::Step` or `Goes::Gap`; put back if the step won't take it), `load`, `clear`; never past `MAX_STEPS`. Kept when the panel closes.
- `units.rs`: `Unit` (one where or one effect, as added), `units(step)` (a where step's where `count` times, an effect step's runs spelled out), `step_of(units, filters)` (back to a counted step; none when empty, mixed, two kinds of where, or more than `MOST_REPEATS`, 9, of one in a row), `with`, `filters_of`.
- `piece.rs`: `Piece` (`Where`, `Filter(FilterKind)`, `Effect`): names, tips, ids, icons (the skill's from `skills_page::skill_icon`; filters `delapouite/person`, `shaking-hands`, `angry-eyes`), and `look` (an `AbilityLook` on `attribute_colour` of what its skill leans on; wheres drawn round); `FilterKind` (Self, Friend, Enemy).
- `markup.rs`: `SPELL_DESIGN` (`spell_design/panel`: `words`, the canvases `pieces` and `steps`, `next`, `name`, `cost`, `problem`, `save`, `clear`; the `naming` dialog with `typed`, `really_save`, `cancel_save`), `WORDS`.
- `open.rs`: `open_panel` (`OpenSpellDesign` toggles it, with `inventory::OpenCharacter`; `EditSpell` loads a spell and opens it if closed), `bind_panel`, `notice_closed`, `tell_open` (into `common::OpenPanels` as Magic, so the ability shows as on).
- `rows.rs`: `draw_rows` (both canvases again when the draft, the skills or the drag's aim change): `PiecesRow` (Wheres unlocked, Filters, each greyed unless the first step is a where or once used, Effects in lines of 9: `PieceNode`), `StepsRow`.
- `steps.rs`: `draw_steps` (each step's box, `StepNode`: its wheres or effects one under another, first at the top (owner), each as often as added, `EntryNode`, and in the first step the filters used above them (`FilterNode`, red with no where there to filter); ">" between; the end box, `EndNode`), `step_wrong` (red edge: a locked where, a where last, filters over effects); green edge on the effect step a drag would join, the gold mark where it would go.
- `nodes.rs`: `spawn_piece` (wheres round and effects square as abilities, filters as octagons: owner, 2026-11-23), `spawn_filter` (`FilterNode`), `WRONG`, `spawn_words`, `spawn_insert_mark`, `id`.
- `octagon.rs`: `Octagon` (a fill and an edge), `OctagonMaterial` (`assets/shaders/octagon.wesl`, a regular octagon), `give_octagons`: the filters' frames.
- `places.rs`: `Places::insert_at` (the place in the row a drop at a point goes, by the steps' middles; none off the row).
- `spots.rs`: `Spot` (a piece, a step, a where or effect in a step, a filter in the first step, the end box), `DesignSpots` (`at` a point, `of` a node).
- `drag.rs`: `press_design` (`PreUpdate`, before `dom::DomInput::Presses` and `HoldObjects`: clicks and taps add a piece (a filter into the first step); a drag picks up a piece, a step, or one where, effect or filter in a step (a where or effect alone in it, the step; a filter dragged off the row comes off, dropped anywhere on the row into the first step) as `dom::HeldObject`, drawn as an ability) and `drop_on_design` (after `HoldObjects`: `DesignAim`, and the change on `dom::Dropped`), `DesignDrag`, `DesignAim`, `Dragging`.
- `side.rs`: `can_store` (filters with no where first; then the book's own `store` on a copy: the design's problem, "in your spellbook", full), `answer_panel` (Save opens the requester with the kept name; its Save sends `SpellbookAsk::Store` with `tidy_name`; Clear), `show_side` (the next unlock, the names, the cost, the problem or "Ready to save.", Save and Clear greyed when they can do nothing, the requester).
- `words.rs`: `problem_words` (each `DesignError`), `cost_words` ("Costs 32 Mana (4.8 × 6.7)": each step's share to one place), `next_words` (the next where at Magic ⌈n³ ÷ 6⌉%, and the Magic now).
- `debug_api.rs` (debug API only): `client.spell_design`.

## Public API

- `SpellDesignPlugin`.
- `OpenSpellDesign` (message): opens the panel, or closes it if open (`abilities`, on using Magic).
- `EditSpell(BookSpell)` (message): puts a spell on the panel in place of the design there, opening it if closed (`abilities`' Spells tab, on Remove).

## Depends on

- `inventory` (`OpenCharacter`), `character` (`CharacterSheet`), `skills_page` (`skill_icon`), `common` (`OpenPanels`, `Session`, `Wording`, `UiPointer`, `TouchGesture`, `PointerSystems`), `connection` (`ToServer`), `dom` (`Dom`, `UiEvent`, `HeldObject`, `Dropped`, `Finder`, `AbilityThing`, `Solid`, `DomInput`, `HoldObjects`), `widgets` (`spawn_ability`, `AbilityLook`, `attribute_colour`, `HoverTip`, `UiId`, `logical_rect`), `shared::spell`.

## Used by

- `abilities` (`OpenSpellDesign`, `EditSpell`).

## Notes

- Plan (owner, 2026-11-23): what can be used along the top, dragged down into a row of steps, a where step taking filters in the first place, an effect step effects in order; no counts (owner, 2026-11-23, while this PR was made): one used more than once is added again, and a step holds wheres or effects, never both (`shared::spell` still keeps counts underneath, the same thing to its rules); reordered, dragged off; the cost, the unlocks and what is wrong shown as it is built; Save asking for a name or keeping the automatic one; removing a spell from the book puts its design back here.
- Claude's choices: only unlocked wheres are shown, with a line saying when the next unlocks; a click or tap adds (so it all works by touch and the debug API) to the last step if it takes it (the same where again, any effect to effects), else as a new last step; a filter goes in the first step above its where, shown only once used (owner, 2026-11-23: only ever in the first step, placed first, and only with a where, as Friend and Enemy need one to do anything), taken out by dragging it off; moving the where away leaves them red over whatever is first, and saving refused, until a where is first again; a step holds one kind of where (two kinds would be two steps); at most 9 of one in a row; an effect dragged onto its own step goes to its end (how effects are reordered); a design already in the book can't be saved again ("This spell is in your spellbook."); the design stays after saving, for variants; `EditSpell` replaces whatever was on the panel. Opens at left 700, beside the Character panel, as both are used together.
- Ids: `spell_design/where.<n>`, `filter.<n>`, `effect.<n>` (the pieces, by their order in `Where::ALL`, Self/Friend/Enemy, `Effect::ALL`), `step.<n>`, `entry.<step × 100 + n>` (each where or effect as added), `filter_on.<n>` (a filter in the first step), `end.0`, and the markup's.
