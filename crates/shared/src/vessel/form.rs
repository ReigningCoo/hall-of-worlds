//! `VesselForm`: which vessel one is and what is in it, by the millilitre.

use serde::{Deserialize, Serialize};

use super::VesselKind;
use crate::item::Item;
use crate::item_form::ItemForm;
use crate::potion::Liquid;

/// A vessel (`ItemKind::Vessel`): which one, the one liquid in it by the
/// millilitre, and whether it is sealed (only a wine bottle can be).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VesselForm {
    pub vessel: VesselKind,
    /// What is in it, none when empty. Never a potion: those stay in bottles.
    pub liquid: Option<Liquid>,
    /// How many millilitres of it: 0 when empty, never more than it holds.
    pub ml: u32,
    /// Corked since it was filled from a cask: never spoils until opened
    /// (drinks decisions 13 and 15).
    pub sealed: bool,
}

impl VesselForm {
    /// An empty one.
    pub fn new(vessel: VesselKind) -> Self {
        Self {
            vessel,
            liquid: None,
            ml: 0,
            sealed: false,
        }
    }

    /// The same holding `ml` of `liquid`, no more than it holds, unsealed;
    /// nothing (or 0 ml) leaves it empty.
    pub fn holding(self, liquid: Option<Liquid>, ml: u32) -> Self {
        let ml = ml.min(self.vessel.most_ml());
        match liquid {
            Some(liquid) if ml > 0 => Self {
                liquid: Some(liquid),
                ml,
                sealed: false,
                ..self
            },
            _ => Self::new(self.vessel),
        }
    }

    /// Full of `liquid`.
    pub fn full_of(self, liquid: Liquid) -> Self {
        let ml = self.vessel.most_ml();
        self.holding(Some(liquid), ml)
    }

    /// The same, sealed if it can be and holds something.
    pub fn sealed(self) -> Self {
        Self {
            sealed: self.vessel.seals() && self.liquid.is_some(),
            ..self
        }
    }

    pub fn is_empty(&self) -> bool {
        self.liquid.is_none()
    }

    /// How many more millilitres it takes.
    pub fn room(&self) -> u32 {
        self.vessel.most_ml() - self.ml
    }

    /// "Tankard", or with what is in it: "Tankard of Water".
    pub fn name(&self) -> String {
        let vessel = self.vessel.name();
        match &self.liquid {
            None => vessel.to_owned(),
            Some(liquid) => format!("{vessel} of {}", liquid.name()),
        }
    }

    /// What it weighs with what is in it: about a gram a millilitre.
    pub fn grams(&self) -> u32 {
        self.vessel.empty_grams() + self.ml
    }
}

impl Item {
    /// Its vessel, if it is one.
    pub fn vessel(&self) -> Option<&VesselForm> {
        match &self.form {
            Some(ItemForm::Vessel(vessel)) => Some(vessel),
            _ => None,
        }
    }
}
