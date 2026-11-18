//! Vessels (drinks plan PR 2, `docs/context/drinks.md`): tankards, cups,
//! goblets, drinking horns, wine bottles, jugs, flasks and waterskins, each
//! holding one liquid by the millilitre. One item kind, `ItemKind::Vessel`,
//! not counted; its form says which vessel it is and what is in it. Filled and
//! emptied by pouring (`shared::pour`); water drunk from one does nothing yet.
//! See `docs/context/shared/vessel.md`.

mod drink;
mod form;
mod kind;
mod roll;

pub use drink::SERVING_ML;
pub use form::VesselForm;
pub use kind::VesselKind;
pub use roll::{vessel_for_shop, vessel_to};

#[cfg(test)]
mod tests;
