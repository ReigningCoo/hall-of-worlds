//! Poisonous fungi carry their poison into a dish (owner, 2026-11-21): eating
//! it brings each one on as eating the fungus would (`fungus::poisoning`,
//! onset included), hidden until recognised. Only the server works this
//! out: a preview never shows it.

use super::{Portion, Source};
use crate::fungus::{Fungi, FungusDef, Trigger};

/// The fungi in these portions whose poison eating the dish brings on, each
/// once, in the order first met: those eating brings on (or opens alcohol's
/// window for, as the common ink cap), less those only harmful raw when the
/// dish was `cooked`.
pub fn poisons<'a>(portions: &[Portion], cooked: bool, fungi: &'a Fungi) -> Vec<&'a FungusDef> {
    let mut found: Vec<&FungusDef> = Vec::new();
    for portion in portions {
        let Portion::Food {
            source: Source::Fungus,
            key,
            ..
        } = portion
        else {
            continue;
        };
        let Some(def) = fungi.get(key) else {
            continue;
        };
        let Some(poison) = &def.poison else {
            continue;
        };
        let eaten = matches!(poison.trigger, Trigger::Eaten | Trigger::WithAlcohol);
        let cooked_out = poison.raw_only && cooked;
        if eaten && !cooked_out && !found.iter().any(|f| std::ptr::eq(*f, def)) {
            found.push(def);
        }
    }
    found
}
