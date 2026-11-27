# client: spell_popup

A spell's popup on every spell picture (owner, 2026-11-27): the action bar's spell slots, the Spells tab's pictures and the spell design panel's preview. Resting the pointer (or a finger) there opens a `dom` `popup hover` with the spell's name (its own or automatic), its key on the bar, and `shared::spell::tip_lines`: its cost and each step with numbers for the player as their skills are now, before the target resists. Not the pictures themselves (`abilities`, `spell_design`) nor how popups behave (`dom`).

## Files

- `crates/client/src/spell_popup/mod.rs`: `SpellPopupPlugin`, the module list. Tests (the markup is good; spells found).
- `spell_popup/subject.rs`: `Subject` (`Book(key, key's short name)`, `Preview`), `pointed` (by `dom::Hit`: an `AbilityCell` holding `BarThing::Spell`, its slot's key from `AbilitySlotShown`, or the element with id `spell_design/preview`), `spell_of` (the book's spell and own name, or `spell_design::PreviewSpell`; none for a spell no longer in the book).
- `spell_popup/popup.rs`: `SPELL_POPUP` (`spell_popup/box` with `name`, `key`, `lines` and the `line` template, wrapping at 360), `SpellPopup` (the one open and where the pointer was, the picture rested on), `forget_closed`, `follow_pointer` (as `effect_bar`'s: opens after `REST`, 0.15 s, in place of one not locked; none while something is held; keeps it open through `dom::KeptOpen`).
- `spell_popup/place.rs`: `keep_on_screen` (past the bottom, as over the bar, it goes above the pointer; past the right edge, in).

## Public API

- `SpellPopupPlugin` only.

## Depends on

- `dom` (`Hit`, `AbilityCell`, `AbilitySlotShown`, `KeptOpen`, `Popup`, `HeldObject`, `Dom`, `ClosePopups`, `BuildDocuments`), `spell_design` (`PreviewSpell`), `character` (`CharacterSheet`: the book and skills), `common` (`Wording`, `interface_area`), `widgets` (`UiId`, `logical_rect`), `shared::spell` (`tip_lines`, `auto_name`).

## Used by

- Nothing: the pictures' owners leave their looks unnamed for spells (`abilities`' bar and Spells tab, `spell_design`'s preview), so no plain tip shows beside it.

## Notes

- Order: `PostUpdate`, after `dom::ClosePopups`, before `dom::BuildDocuments`, as the effect bar's popup.
- Words are first guesses in `shared/src/lang/en/spell-tip.ftl`, for the owner to change.
