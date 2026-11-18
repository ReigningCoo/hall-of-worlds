//! The wood Give Item's logs and kindling are made of: as asked, or rolled
//! from the seed, the same on both sides.

use crate::item_form::{Field, Refused};
use crate::material::{Material, WOODS};
use crate::rng::Seed;

/// The wood asked for, if it is a tree's (`material::WOODS`), or with none
/// asked, one rolled from `seed`.
pub fn dev_wood(asked: Option<Material>, seed: Seed) -> Result<Material, Refused> {
    match asked {
        Some(wood) if wood.wood().is_some() => Ok(wood),
        Some(_) => Err(Refused(Field::Material)),
        None => {
            let at = seed.derive("dev wood").value() % WOODS.len() as u64;
            Ok(WOODS[at as usize].material)
        }
    }
}
