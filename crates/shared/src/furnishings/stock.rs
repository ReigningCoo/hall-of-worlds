//! What generated racks start with (Claude's first guesses for the owner to
//! tune): the server fills them once, with its own ids, as it stocks cellar
//! casks. Cask racks' casks are rolled as cellar casks (`cellar::stock`).

use crate::fermentation::{GAME_DAY_SECS, rolled_drink};
use crate::item::Quality;
use crate::potion::Liquid;
use crate::rng::Rng;
use crate::vessel::{VesselForm, VesselKind};

/// In percent: how many of a generated wine rack's bottles are empty.
const EMPTY: u64 = 15;
/// The longest a rack's bottle has aged in its glass.
const MOST_AGED_DAYS: u64 = 120;

/// How many of a generated rack's `capacity` places start filled: half to
/// all of them.
pub fn rack_fill(capacity: usize, rng: &mut Rng) -> usize {
    let half = capacity.div_ceil(2) as u64;
    rng.range_u64(half, capacity as u64 + 1) as usize
}

/// A bottle as found on a generated wine or spirit rack: a wine bottle,
/// sometimes empty, else sealed and full of a rolled drink made at a rolled
/// quality and aged in its glass up to 120 game days.
pub fn rack_bottle(rng: &mut Rng) -> VesselForm {
    let bottle = VesselForm::new(VesselKind::WineBottle);
    if rng.range_u64(0, 100) < EMPTY {
        return bottle;
    }
    let made = rng.pick_weighted(&[
        (Quality::Poor, 10),
        (Quality::Common, 55),
        (Quality::Uncommon, 25),
        (Quality::Rare, 10),
    ]);
    let mut liquid = rolled_drink(rng);
    if let Liquid::Drink(drink) = &mut liquid {
        drink.quality = made;
    }
    let mut form = bottle.full_of(liquid).sealed();
    let secs = rng.range_u64(0, MOST_AGED_DAYS * GAME_DAY_SECS + 1);
    let VesselForm {
        liquid,
        glass,
        sealed,
        ..
    } = &mut form;
    glass.pass(liquid, *sealed, secs, 100);
    form
}
