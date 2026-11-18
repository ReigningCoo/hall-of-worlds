//! Vegetable plants on a level (`shared::vegetable`), by cell: what grows there
//! and when it was last picked. They never block anything.

use super::Level;
use crate::vegetable::VegetablePlant;

impl Level {
    /// The vegetable plant at `(x, y)`, if any.
    pub fn vegetable_at(&self, x: i64, y: i64) -> Option<&VegetablePlant> {
        self.vegetables.get(&(x, y))
    }

    /// Every vegetable plant, by cell.
    pub fn vegetable_plants(&self) -> impl Iterator<Item = ((i64, i64), &VegetablePlant)> + '_ {
        self.vegetables.iter().map(|(&at, plant)| (at, plant))
    }

    /// Puts a vegetable plant at `(x, y)`, replacing any there.
    pub fn set_vegetable(&mut self, x: i64, y: i64, plant: VegetablePlant) {
        self.vegetables.insert((x, y), plant);
    }

    /// Takes away the vegetable plant at `(x, y)`, if any.
    pub fn remove_vegetable(&mut self, x: i64, y: i64) {
        self.vegetables.remove(&(x, y));
    }

    /// Notes when the plant at `(x, y)` was last picked, if there is one there.
    pub fn set_vegetable_picked(&mut self, x: i64, y: i64, picked: Option<u64>) {
        if let Some(plant) = self.vegetables.get_mut(&(x, y)) {
            plant.picked = picked;
        }
    }
}
