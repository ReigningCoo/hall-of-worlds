//! Harm from handling a fungus (fungi plan PR 9, decision 13): a poison
//! brought on by touch (`Trigger::Touched`: oakmoss) harms whoever picks or
//! handles it with bare hands, and one brought on by breath
//! (`Trigger::Breathed`: puffballs' spores, the cellar cup, split gill) whoever
//! does so with the mouth uncovered. What protects is `Guard`.

use super::{Eater, FungusDef, Poison, Trigger, poisoning};
use crate::bonus::Effect;
use crate::item::Inventory;

/// What keeps a handled fungus's poison off.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Guard {
    /// Anything worn on the hands (`Inventory::hands_covered`).
    Hands,
    /// Something over the mouth (`Inventory::mouth_covered`).
    Mouth,
}

/// Whether the hands and the mouth are covered now.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Guarded {
    pub hands: bool,
    pub mouth: bool,
}

impl Guarded {
    /// What `inventory`'s worn things cover.
    pub fn of(inventory: &Inventory) -> Self {
        Self {
            hands: inventory.hands_covered(),
            mouth: inventory.mouth_covered(),
        }
    }

    fn keeps_off(self, guard: Guard) -> bool {
        match guard {
            Guard::Hands => self.hands,
            Guard::Mouth => self.mouth,
        }
    }
}

impl Poison {
    /// What keeps it off when handled: `None` for one only eating brings on.
    pub fn guard(&self) -> Option<Guard> {
        match self.trigger {
            Trigger::Touched => Some(Guard::Hands),
            Trigger::Breathed => Some(Guard::Mouth),
            _ => None,
        }
    }
}

/// What handling `def` (picking it, or using it in Alchemy) at `now` brings
/// on: its poison if handling brings it on and nothing worn keeps it off
/// (`poisoning`, from its onset), else nothing.
pub fn handling(def: &FungusDef, guarded: Guarded, now: u64, eater: Eater) -> Vec<Effect> {
    let Some(poison) = &def.poison else {
        return Vec::new();
    };
    match poison.guard() {
        Some(guard) if !guarded.keeps_off(guard) => poisoning(def, poison, now, eater),
        _ => Vec::new(),
    }
}
