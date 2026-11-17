//! `BottleForm`: what one bottle is, empty or a potion, and how it looks.

use serde::{Deserialize, Serialize};

use super::Contents;
use crate::item::Item;
use crate::item_form::ItemForm;

/// What an empty bottle weighs, in grams: small, thick glass and a stopper.
pub const EMPTY_GRAMS: u32 = 120;
/// What a full one weighs, in grams: about 100 ml more.
pub const POTION_GRAMS: u32 = 220;

/// A bottle (`ItemKind::Bottle`): what it holds and a seed for how it looks.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BottleForm {
    /// What is in it; the liquid's look comes from this alone.
    pub contents: Contents,
    /// A seed the client picks the bottle's shape, glass and stopper from. A
    /// stack shows its bottom one's; stacks don't compare it.
    pub look: u64,
}

impl BottleForm {
    /// Whether nothing is in it.
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    /// "Potion", or "Empty Bottle".
    pub fn name(&self) -> &'static str {
        match self.is_empty() {
            true => "Empty Bottle",
            false => "Potion",
        }
    }

    /// What one weighs.
    pub fn grams(&self) -> u32 {
        match self.is_empty() {
            true => EMPTY_GRAMS,
            false => POTION_GRAMS,
        }
    }
}

impl Item {
    /// Its bottle, if it is one.
    pub fn bottle(&self) -> Option<&BottleForm> {
        match &self.form {
            Some(ItemForm::Bottle(bottle)) => Some(bottle),
            _ => None,
        }
    }
}
