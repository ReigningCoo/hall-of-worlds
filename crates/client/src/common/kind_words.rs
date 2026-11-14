//! Kinds of creature named together in the player's language, in the plural:
//! "dwarves", "orcs and goblins" (`creature-<kind>` in the language files).

use shared::creature::CreatureKind;
use shared::lang::{Args, Lang, Selector};

/// These kinds in the plural, as a list: "orcs and goblins".
pub fn kinds_words(kinds: &[CreatureKind], lang: &Lang) -> String {
    let names: Vec<String> = kinds
        .iter()
        .map(|kind| {
            let id = format!("{}-{}", kind.fact(), kind.key());
            let words = lang.render(&id, &Args::new().count("count", 2));
            words.unwrap_or_else(|error| error.text)
        })
        .collect();
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    lang.rules().list(&names)
}
