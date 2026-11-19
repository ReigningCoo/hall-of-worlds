//! Fruit plants on a level (`shared::fruit`), by cell: what grows there and
//! when it was last picked. One with a trunk (`PlantFamily::has_trunk`) is in
//! the way as a wild tree is: its cell can't be walked on, only stood in
//! beside the trunk (`clearance`).

use super::Level;
use crate::fruit::{FruitPlant, Fruits};

impl Level {
    /// The fruit plant at `(x, y)`, if any.
    pub fn fruit_at(&self, x: i64, y: i64) -> Option<&FruitPlant> {
        self.fruit.get(&(x, y))
    }

    /// Every fruit plant, by cell.
    pub fn fruit_plants(&self) -> impl Iterator<Item = ((i64, i64), &FruitPlant)> + '_ {
        self.fruit.iter().map(|(&at, plant)| (at, plant))
    }

    /// Puts a fruit plant at `(x, y)`, replacing any there.
    pub fn set_fruit(&mut self, x: i64, y: i64, plant: FruitPlant) {
        self.fruit.insert((x, y), plant);
    }

    /// Takes away the fruit plant at `(x, y)`, if any.
    pub fn remove_fruit(&mut self, x: i64, y: i64) {
        self.fruit.remove(&(x, y));
    }

    /// Notes when the plant at `(x, y)` was last picked, if there is one there.
    pub fn set_fruit_picked(&mut self, x: i64, y: i64, picked: Option<u64>) {
        if let Some(plant) = self.fruit.get_mut(&(x, y)) {
            plant.picked = picked;
        }
    }

    /// Whether a fruit plant with a trunk stands at `(x, y)`: not while it is
    /// felled and growing back (`felled.rs`).
    pub fn has_trunk(&self, x: i64, y: i64) -> bool {
        self.felled_at(x, y).is_none()
            && self
                .fruit_at(x, y)
                .and_then(|plant| Fruits::built_in().get(&plant.fruit))
                .is_some_and(|def| def.plant.has_trunk())
    }
}
