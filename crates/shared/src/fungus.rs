//! Mushrooms, fungi and lichens: what each one is (`FungusDef`: where it grows
//! and on what, its food, its herb effects, its poison, whether it glows, what
//! it is mistaken for, its look), the table of them (`Fungi`, built in), fungi
//! as items (`FungusForm`; a big piece is cut into servings when eaten, `cut`)
//! and what eating one raw does (`eating`). Where they grow, picking, models,
//! glow, delayed poisons, knowing them, Alchemy and harm from handling come in
//! later PRs of the fungi plan (`docs/context/fungi.md`). Research and the
//! owner's decisions: `docs/context/research-fungi.md`.

mod cut;
mod def;
mod dev;
mod eating;
mod form;
mod look;
mod poison;
mod table;
mod tag;
mod traits;

pub use cut::cut;
pub use def::{FungusDef, Unit};
pub use dev::dev_fungus;
pub use eating::{eating, effect_ticks, healing, poison_ticks, raw_ticks};
pub use form::{FungusForm, new_item};
pub use look::{Cap, Form, FungusLook, MOST_COLOURS, Stem, Under};
pub use poison::{Onset, Poison, Severity, Trigger};
pub use table::Fungi;
pub use tag::FungusTag;
pub use traits::{Group, Niche, Where};

#[cfg(test)]
mod tests;
