//! Mushrooms, fungi and lichens: what each one is (`FungusDef`: where it grows
//! and on what, its food, its herb effects, its poison, whether it glows, what
//! it is mistaken for, its look), the table of them (`Fungi`, built in), fungi
//! as items (`FungusForm`; a big piece is cut into servings when eaten, `cut`)
//! and what eating one raw does (`eating`); where they grow (underground by
//! niche, warmth and depth, above ground by biome, on felled trees) and patches
//! picked (`FungusPatch`, `harvest`); poisons starting later, hidden until
//! recognised, deadly ones, confusion and the ink cap with alcohol
//! (`poisoning`); what a character knows of each (`FungusKnowledge`: names
//! by a Mycology check on picking, shown as "Unknown mushroom" or "Looks like
//! X" until then, `Shown`). Alchemy and harm from handling come in later PRs of the fungi plan (`docs/context/fungi.md`). Research and the
//! owner's decisions: `docs/context/research-fungi.md`.

mod cut;
mod def;
mod dev;
mod eating;
mod felled;
mod form;
mod knowledge;
mod look;
mod naming;
mod patch;
mod place;
mod poison;
mod poisoning;
mod table;
mod tag;
mod traits;
mod underground;

pub use cut::cut;
pub use def::{FungusDef, Unit};
pub use dev::dev_fungus;
pub use eating::{
    WINDOW_BONUS, WINDOW_TICKS, eating, effect_ticks, healing, poison_ticks, raw_ticks,
    with_alcohol,
};
pub use felled::{FELLED_PERCENT, felled_patch};
pub use form::{FungusForm, new_item};
pub use knowledge::FungusKnowledge;
pub use look::{Cap, Form, FungusLook, MOST_COLOURS, Stem, Under};
pub use naming::{MYCOLOGY, NAME_BASE, NAME_MOST, Shown, name_chance, names_on_picking};
pub use patch::{FungusPatch, harvest};
pub use place::{DEEPEST_ONLY, Spot, choose, suited, weight};
pub use poison::{Onset, Poison, Severity, Trigger};
pub use poisoning::{
    DEADLY_TIMES, EASED_TICKS, Eater, GAME_MINUTE, SICK_TICKS, deadly_drain, onset_ticks,
    poisoning, recognised,
};
pub use table::Fungi;
pub use tag::FungusTag;
pub use traits::{Group, Niche, Where};
pub use underground::{DIM_REACH, PATCH_ONE_IN, Underground, WATER_REACH, grow_underground};

#[cfg(test)]
mod naming_tests;
#[cfg(test)]
mod place_tests;
#[cfg(test)]
mod poison_tests;
#[cfg(test)]
mod tests;
