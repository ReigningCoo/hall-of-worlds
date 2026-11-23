//! Chests and wardrobes (tavern furniture plan PR 7): inn rooms' storage
//! furniture, a grid inside like a bag's (`shared::item`). A chest takes
//! anything that fits; a wardrobe only clothes (worn clothing, underclothes,
//! footwear and armour). The server keeps what lies in one by the piece's
//! cell and saves it; it opens in the same panel as every piece with places
//! (`furniture::Holder::Store`, `Held::Store`), laid out by the server as the
//! ground is. `Stored` is whole in itself, so it can be kept elsewhere later
//! (The Way Inn's private rooms, saved with the character).

mod kind;
mod stored;

pub use kind::{Chest, Storage, Wardrobe};
pub use stored::Stored;

#[cfg(test)]
mod tests;
