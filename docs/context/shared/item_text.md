# shared: item_text

Items written into text as `${item:<uuid>}`, and kinds of item as `${kind:<kind>[:<option>…]}`: writing them, reading text back as plain words, items and kinds, and what a kind is called and drawn as. Not how an item is shown (client) or found (server).

## Files

- `crates/shared/src/item_text.rs`: `ITEM_START`, `ITEM_END`, `Piece`, `Token`, `item_token`, `item_at`, `token_at`, `pieces`, `has_tokens`, `items_in`.
- `crates/shared/src/item_text/kind.rs`: `KIND_START`, `KindToken` (`Food(Wanted)`, `Tool(Tool)`, `Water(Option<Water>)`), `token`, `kind_at`; the words for sources, tools and waters.
- `crates/shared/src/item_text/kind_item.rs`: `KindToken::name` ("Any egg", "Carrot", "Knife", "Rain Water"; a fungus by key named), `KindToken::stand_in` (an item to draw it by, the same every time).

## Public API

- `item_token(id) -> String`: `${item:<uuid>}`, the UUID hyphenated in lower case.
- `item_at(text) -> Option<(Id, bytes)>`: the item written at the very start of `text`.
- `pieces(text) -> Vec<Piece>`: `Piece::Text(&str)`, `Piece::Item(Id)` and `Piece::Kind(KindToken)` in order; neighbouring words are one piece, none empty. `has_tokens(text)`: any item or kind in it.
- `Token` (`Item(Id)`, `Kind(KindToken)`, `token()` writes it), `token_at(text) -> Option<(Token, bytes)>`: what a text box takes as one whole.
- `KindToken`: `token()` (`${kind:egg}`, `${kind:vegetable:carrot}`, `${kind:knife}`, `${kind:bottle:water[:rain]}`), `name()`, `stand_in() -> Option<Item>` (nil id, rolled from one seed: the usual food of a source for "any" (`cooking::usual_food`), milk, honey, soy sauce and water in a bottle, a tool's item; `None` for the Oven, parts and foods no table has). `kind_at(text)`, `KIND_START`.
- `items_in(text) -> Vec<Id>`: each item once, in the order first written (for asking the server, `ClientMessage::LookUpItems`).
- `ITEM_START` (`${item:`), `ITEM_END` (`}`).

## Depends on

- `shared::id` (`Id`), `uuid`; for kinds `shared::cooking` (`Wanted`, `Source`, `Tool`, `food_name`, `usual_food`), `potion` (`Water`, `Liquid`, `BottleForm`), the food tables, `farmyard::new_egg`, `item_form::make_item`.

## Used by

- `shared::story` (a quest's gathering herb, `${kind:herb:<key>}`), client `widgets` (item text and text boxes), `dom` (whether writing has any), `item_text`; recipes' reading panel (recipes plan PR 4).

## Notes

- Only the exact shape counts: a hyphenated UUID (either case) between `${item:` and `}`. Anything else, a misspelt one included, is plain text, and a broken one doesn't hide a good one after it.
- A kind is as strict: parts of lower-case letters, digits and `_`, at most 80 bytes, a known first word and the right number of options; a well-shaped key no table has is still a kind (drawn as a "?"). New kinds of kind go in `KindToken` with their own first word.
- The text says only which item: what it is called and looks like comes from the server (`ItemsLookedUp`), live, as the reader may know it. `@name` is kept for mentioning players later.
