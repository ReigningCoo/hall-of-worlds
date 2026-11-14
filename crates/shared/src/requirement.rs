//! Requirements: what an item asks of whoever uses it (an attribute, a skill,
//! being of some kinds). Armour's come from how heavy it is for what it covers
//! (`armour`). Nothing is ever refused for an unmet attribute: it only works
//! less well (`effectiveness`). See `docs/context/shared/requirement.md`.

mod armour;
mod effectiveness;
mod kind;

pub use armour::{
    AGILITY_AND_STRENGTH_ABOVE, NON_METAL_PLATE_MG, REFERENCE_MG, mg_at_reference,
    milligrams_per_cm2, needs_agility, requirement_for,
};
pub use effectiveness::{FULL, effective_bonuses, effectiveness, factor_for};
pub use kind::{Requirement, item_requirements};

#[cfg(test)]
mod tests;
