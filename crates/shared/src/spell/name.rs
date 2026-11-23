//! A spell's automatic name, worded through `shared::lang` (`lang/en/spell.ftl`):
//! the first effect step's first outcome, shaped by the where just before it
//! (Heal, Healing Missile, Ball of Fire). The client renders it; the server
//! never does.

use super::combination::outcomes;
use super::design::{Spell, Step};
use crate::lang::{Args, Lang};

/// The automatic name, in `lang`'s language. A spell with no effect is "Spell".
pub fn auto_name(spell: &Spell, lang: &Lang) -> String {
    let mut place = None;
    for step in &spell.steps {
        match step {
            Step::Where(step) => place = Some(step.place),
            Step::Effects(step) => {
                let Some(&(outcome, _)) = outcomes(step).first() else {
                    continue;
                };
                let part = format!("spell-part-{}", outcome.key());
                let args = Args::new()
                    .text("noun", rendered(lang, &part, &Args::new()))
                    .text(
                        "adjective",
                        rendered(lang, &format!("{part}.adjective"), &Args::new()),
                    );
                let id = match place {
                    Some(place) => format!("spell-name-{}", place.key()),
                    None => "spell-name-self".to_owned(),
                };
                return rendered(lang, &id, &args);
            }
        }
    }
    rendered(lang, "spell-name-empty", &Args::new())
}

/// A message rendered, or as much of it as could be.
fn rendered(lang: &Lang, id: &str, args: &Args) -> String {
    lang.render(id, args).unwrap_or_else(|error| error.text)
}
