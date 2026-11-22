//! `LanternForm`: the oil in a lantern's fount, kept on the item in whole
//! numbers.

use serde::{Deserialize, Serialize};

use crate::item::{Item, ItemKind};
use crate::item_form::ItemForm;

/// What a lantern's fount holds, in millilitres: two bottles of oil (owner).
pub const FOUNT_ML: u32 = 200;

/// Seconds of playtime a millilitre of oil burns for: 100 ml an hour (owner),
/// so a bottle lasts about an hour and a full fount two.
pub const SECONDS_PER_ML: u32 = 36;

/// The oil in a lantern. Any oil burns the same (owner), so which is not kept.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LanternForm {
    /// Oil left, in millilitres, at most `FOUNT_ML`.
    pub ml: u32,
    /// Seconds burnt into the next millilitre, under `SECONDS_PER_ML`.
    #[serde(default)]
    pub burnt: u32,
}

impl LanternForm {
    /// Whether there is oil to burn.
    pub fn has_oil(&self) -> bool {
        self.ml > 0
    }

    /// How many more millilitres the fount takes.
    pub fn room(&self) -> u32 {
        FOUNT_ML.saturating_sub(self.ml)
    }

    /// Seconds of burning left, held or tied.
    pub fn seconds_left(&self) -> u32 {
        (self.ml * SECONDS_PER_ML).saturating_sub(self.burnt)
    }

    /// What the oil weighs, a gram a millilitre as a cask's liquids.
    pub fn grams(&self) -> u32 {
        self.ml
    }
}

impl Item {
    /// Its fount, if it is a lantern: empty for one with no form (made before
    /// lanterns burnt).
    pub fn lantern(&self) -> Option<LanternForm> {
        if self.kind != ItemKind::Lantern {
            return None;
        }
        match &self.form {
            Some(ItemForm::Lantern(lantern)) => Some(*lantern),
            _ => Some(LanternForm::default()),
        }
    }

    /// Its fount to change, if it is a lantern, given one first if it has none.
    pub(super) fn lantern_mut(&mut self) -> Option<&mut LanternForm> {
        if self.kind != ItemKind::Lantern {
            return None;
        }
        if !matches!(self.form, Some(ItemForm::Lantern(_))) {
            self.form = Some(ItemForm::Lantern(LanternForm::default()));
        }
        match &mut self.form {
            Some(ItemForm::Lantern(lantern)) => Some(lantern),
            _ => None,
        }
    }
}
