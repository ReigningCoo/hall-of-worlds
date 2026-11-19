//! `CaskForm`: what one cask is (size, wood) and what is in it, by the
//! millilitre, and whether it has been opened.

use serde::{Deserialize, Serialize};

mod saved;

use super::{CaskSize, CaskTime};
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

/// A cask (`ItemKind::Cask`): its size and wood, the one liquid in it, by
/// the millilitre, and whether it is still sealed (drinks decision 13).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct CaskForm {
    pub size: CaskSize,
    pub wood: Material,
    /// What is in it, none when empty.
    pub liquid: Option<Liquid>,
    /// How many millilitres of it: 0 when empty, never more than it holds.
    pub ml: u32,
    /// Whether it has been opened: tapped, poured from or added to. A sealed
    /// one never spoils (its best-before is drinks PR 6). Saves from before
    /// this (2026-11-18) load sealed.
    pub opened: bool,
    /// Its clocks: fermenting, ageing, opened (drinks plan PR 6). Emptied
    /// casks start again. Boxed, so every item doesn't grow by them.
    pub time: Box<CaskTime>,
}

impl CaskForm {
    /// An empty one.
    pub fn new(size: CaskSize, wood: Material) -> Self {
        Self {
            size,
            wood,
            liquid: None,
            ml: 0,
            opened: false,
            time: Box::default(),
        }
    }

    /// How many millilitres it holds.
    pub fn most_ml(&self) -> u32 {
        self.size.litres() * 1000
    }

    /// The same holding `ml` of `liquid`, no more than it holds; nothing (or
    /// 0 ml) leaves it empty, its clocks starting again. Whether it was opened
    /// stays, and when it was put on the ground.
    pub fn holding(self, liquid: Option<Liquid>, ml: u32) -> Self {
        let ml = ml.min(self.most_ml());
        match liquid {
            Some(liquid) if ml > 0 => Self {
                liquid: Some(liquid),
                ml,
                ..self
            },
            _ => Self {
                opened: self.opened,
                time: Box::new(CaskTime {
                    lying_since: self.time.lying_since,
                    ..CaskTime::default()
                }),
                ..Self::new(self.size, self.wood)
            },
        }
    }

    /// Full of `liquid`.
    pub fn full_of(self, liquid: Liquid) -> Self {
        let ml = self.most_ml();
        self.holding(Some(liquid), ml)
    }

    /// The same, opened.
    pub fn opened(self) -> Self {
        Self {
            opened: true,
            ..self
        }
    }

    /// Whole litres in it, rounded down.
    pub fn litres(&self) -> u32 {
        self.ml / 1000
    }

    pub fn is_empty(&self) -> bool {
        self.liquid.is_none()
    }

    /// How many more millilitres it takes.
    pub fn room(&self) -> u32 {
        self.most_ml() - self.ml
    }

    /// Whether `liquid` can be poured in: it has room, and is empty or holds
    /// the same.
    pub fn takes(&self, liquid: &Liquid) -> bool {
        self.room() > 0 && self.liquid.as_ref().is_none_or(|held| held == liquid)
    }

    /// The same with up to `ml` more of `liquid` (`takes` it), opened, and how
    /// many millilitres went in.
    pub fn poured(self, liquid: Liquid, ml: u32) -> (Self, u32) {
        if !self.takes(&liquid) {
            return (self, 0);
        }
        let added = ml.min(self.room());
        let total = self.ml + added;
        (self.holding(Some(liquid), total).opened(), added)
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
        self.empty_grams() + self.ml
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
