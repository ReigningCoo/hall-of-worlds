//! Ferment and vinegar (drinks plan PR 6, `docs/context/drinks.md`): the
//! Ferment ability's order and what it starts in a cask (`order.rs`), the
//! yeast it uses (`yeasts.rs`), what the client's panel offers
//! (`choices.rs`), and time passing on casks (`clock.rs`): fermenting,
//! ageing sealed, an opened drink counting down to vinegar and opened juice
//! fermenting wild. The rules themselves are `shared::fermentation`'s. See
//! `docs/context/shared/ferment.md`.

mod choices;
mod clock;
mod order;
mod yeasts;

pub use choices::{FermentChoices, YeastStack, fermentable_cask};
pub use clock::{CaskSettled, days_left};
pub use order::{
    FERMENT_PRACTICE, FERMENTING, FermentOrder, FermentPlan, FermentRefused, ferment_ticks, plan,
};

#[cfg(test)]
mod tests;
