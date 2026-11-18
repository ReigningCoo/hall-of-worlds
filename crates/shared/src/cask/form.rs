//! `CaskForm`: what one cask is (size, wood) and what is in it, by the litre.

use serde::{Deserialize, Serialize};

use super::CaskSize;
use crate::item::Item;
use crate::item_form::ItemForm;
use crate::material::Material;
use crate::potion::Liquid;

/// The woods casks are made of (owner): oak (improves what ages in it),
/// chestnut and acacia (neutral), cherry (a little flavour), pine (taints).
/// What each does to a drink is for the fermentation rules (drinks PR 5).
pub const WOODS: [Material; 5] = [
    Material::Oak,
    Material::Chestnut,
    Material::Acacia,
    Material::Cherry,
    Material::Pine,
];

/// What can be seen of a cask from afar: its size and wood. A pile on the
/// ground holding one shows it (`ground::Load::Cask`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CaskLook {
    pub size: CaskSize,
    pub wood: Material,
}

/// A cask (`ItemKind::Cask`): its size and wood, and the one liquid in it,
/// by the litre.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CaskForm {
    pub size: CaskSize,
    pub wood: Material,
    /// What is in it, none when empty.
    pub liquid: Option<Liquid>,
    /// How many litres of it: 0 when empty, never more than it holds.
    pub litres: u32,
}

impl CaskForm {
    /// An empty one.
    pub fn new(size: CaskSize, wood: Material) -> Self {
        Self {
            size,
            wood,
            liquid: None,
            litres: 0,
        }
    }

    /// The same holding `litres` of `liquid`, no more than it holds; nothing
    /// (or 0 litres) leaves it empty.
    pub fn holding(self, liquid: Option<Liquid>, litres: u32) -> Self {
        let litres = litres.min(self.size.litres());
        match liquid {
            Some(liquid) if litres > 0 => Self {
                liquid: Some(liquid),
                litres,
                ..self
            },
            _ => Self::new(self.size, self.wood),
        }
    }

    /// Full of `liquid`.
    pub fn full_of(self, liquid: Liquid) -> Self {
        let litres = self.size.litres();
        self.holding(Some(liquid), litres)
    }

    pub fn is_empty(&self) -> bool {
        self.liquid.is_none()
    }

    /// How many more litres it takes.
    pub fn room(&self) -> u32 {
        self.size.litres() - self.litres
    }

    /// Whether `liquid` can be poured in: it has room, and is empty or holds
    /// the same.
    pub fn takes(&self, liquid: &Liquid) -> bool {
        self.room() > 0 && self.liquid.as_ref().is_none_or(|held| held == liquid)
    }

    /// The same with up to `litres` more of `liquid` (`takes` it), and how many
    /// went in.
    pub fn poured(self, liquid: Liquid, litres: u32) -> (Self, u32) {
        if !self.takes(&liquid) {
            return (self, 0);
        }
        let added = litres.min(self.room());
        let total = self.litres + added;
        (self.holding(Some(liquid), total), added)
    }

    /// "Oak Firkin", or with what is in it: "Oak Firkin of Moon Water".
    pub fn name(&self) -> String {
        let cask = format!("{} {}", self.wood.name(), self.size.name());
        match &self.liquid {
            None => cask,
            Some(Liquid::Potion(_)) => format!("{cask} of Potion"),
            Some(liquid) => format!("{cask} of {}", liquid.name()),
        }
    }

    /// What it weighs empty, in grams: oak's weight for its size, lighter or
    /// heavier by its wood's density.
    pub fn empty_grams(&self) -> u32 {
        let oak = Material::Oak.density();
        let grams = u64::from(self.size.oak_grams()) * u64::from(self.wood.density());
        (grams / u64::from(oak)) as u32
    }

    /// What it weighs with what is in it: about a kilogram a litre.
    pub fn grams(&self) -> u32 {
        self.empty_grams() + self.litres * 1000
    }

    pub fn look(&self) -> CaskLook {
        CaskLook {
            size: self.size,
            wood: self.wood,
        }
    }
}

impl Item {
    /// Its cask, if it is one.
    pub fn cask(&self) -> Option<&CaskForm> {
        match &self.form {
            Some(ItemForm::Cask(cask)) => Some(cask),
            _ => None,
        }
    }
}
