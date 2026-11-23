# client: tutorial

Notes for new players, shown when the server says one is wanted (`ServerMessage::Tutorial`). The server decides when (it knows who is new); the client words and shows them. For now one: The Way Inn's, on a new character's first arrival there (The Way Inn plan PR 6), saying what the inn is, that its door opens onto the Hall of Worlds, and how to come back (off the walkway's edge halfway between two gates, or a click there).

## Files

- `crates/client/src/tutorial/mod.rs`: `TutorialPlugin`; `NOTES` (the markup: a `dialog` per note, `tutorial/way_inn`, its words and an Onward button), `spawn_notes` (Startup, on `dom::top_strip()`), `hear_notes` (keeps the note asked for in `Waiting`), `show_notes` (shows it once `common::LoadingStep` is `Ready`, so it never opens behind the loading screen), `close_notes` (Onward, or a press outside).

## Public API

- `TutorialPlugin`.

## Depends on

- `common` (`LoadingStep`), `connection` (`FromServer`), `dom` (`Template`, `Dom`, `UiEvent`, `document`, `top_strip`), `shared::protocol` (`Tutorial`).

## Used by

- Nothing.

## Notes

- Not a notice (`notices`): a note stays until the player closes it.
- The words are English in the markup, as other dialogs' are, not from `shared::lang` yet.
