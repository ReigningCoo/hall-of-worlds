//! `KeyLook`: what can be seen of a key, rolled from its lock's id, so every key
//! to one lock looks alike (owner, 2026-11-17). Old mortice and skeleton keys.

use shared::id::Id;
use shared::item::Item;
use shared::rng::{Rng, Seed};

/// How many columns a key's bit is cut in.
pub(super) const COLUMNS: usize = 7;

/// A key's look: discrete choices only, so looks that match share a picture.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyLook {
    pub(super) bow: Bow,
    /// How big the bow is, 0 to 3.
    pub(super) bow_size: u8,
    pub(super) fill: Fill,
    pub(super) collar: Collar,
    /// A square shank rather than a round one.
    pub(super) square: bool,
    /// A skeleton key's hollow pipe end, or a mortice key's solid end with a stop.
    pub(super) pipe: bool,
    /// How long the shank is, 0 to 4, and how thick, 0 to 2.
    pub(super) length: u8,
    pub(super) thickness: u8,
    /// How wide the bit is along the shank, and how far it reaches out, 0 to 3.
    pub(super) bit_width: u8,
    pub(super) bit_depth: u8,
    /// Each column of the bit: 0 whole, 1 to 3 a notch that deep from its edge,
    /// 4 a slot through its middle.
    pub(super) cuts: [u8; COLUMNS],
    /// A bit on both sides.
    pub(super) double: bool,
    pub(super) metal: KeyMetal,
    /// How tarnished, 0 to 2.
    pub(super) tarnish: u8,
}

/// The bow's shape: what the key is held by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Bow {
    Ring,
    Oval,
    Trefoil,
    Quatrefoil,
    Heart,
    Diamond,
    Scroll,
}

/// What is inside the bow.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Fill {
    Open,
    /// A bar across it.
    Bar,
    /// A small ring in the middle.
    Boss,
}

/// What is between the bow and the shank.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Collar {
    None,
    Bead,
    Rings,
    Baluster,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum KeyMetal {
    Iron,
    DarkIron,
    Brass,
    Bronze,
}

impl KeyLook {
    /// A key item's look: by its lock, or by its own id if it has none.
    pub fn of(item: &Item) -> Self {
        let id = item.form.as_ref().and_then(|form| form.lock());
        Self::of_lock(id.unwrap_or(item.id))
    }

    /// The look of every key to `lock`.
    pub fn of_lock(lock: Id) -> Self {
        Self::roll(lock.seed())
    }

    /// The look rolled from `seed` (a lock's, or the Keyring's fixed ones).
    pub(in super::super) fn roll(seed: Seed) -> Self {
        let mut rng = seed.derive("key look").rng();
        let rng = &mut rng;
        use Bow::*;
        let bow = rng.pick_weighted(&[
            (Ring, 4),
            (Oval, 3),
            (Trefoil, 2),
            (Quatrefoil, 2),
            (Heart, 1),
            (Diamond, 2),
            (Scroll, 1),
        ]);
        let bow_size = small(rng, 4);
        let fill = rng.pick_weighted(&[(Fill::Open, 5), (Fill::Bar, 2), (Fill::Boss, 1)]);
        let collar = rng.pick_weighted(&[
            (Collar::None, 2),
            (Collar::Bead, 3),
            (Collar::Rings, 2),
            (Collar::Baluster, 2),
        ]);
        let square = rng.range_u64(0, 4) == 0;
        let pipe = rng.range_u64(0, 2) == 0;
        let (length, thickness) = (small(rng, 5), small(rng, 3));
        let (bit_width, bit_depth) = (small(rng, 4), small(rng, 4));
        let cuts = cuts(rng);
        let double = rng.range_u64(0, 5) == 0;
        let metal = rng.pick_weighted(&[
            (KeyMetal::Iron, 6),
            (KeyMetal::DarkIron, 3),
            (KeyMetal::Brass, 3),
            (KeyMetal::Bronze, 2),
        ]);
        let tarnish = small(rng, 3);
        Self {
            bow,
            bow_size,
            fill,
            collar,
            square,
            pipe,
            length,
            thickness,
            bit_width,
            bit_depth,
            cuts,
            double,
            metal,
            tarnish,
        }
    }
}

/// A number below `below`.
fn small(rng: &mut Rng, below: u64) -> u8 {
    rng.range_u64(0, below) as u8
}

/// The bit's wards: 2 to 5 of its columns notched or slotted.
fn cuts(rng: &mut Rng) -> [u8; COLUMNS] {
    let mut cuts = [0; COLUMNS];
    let wanted = rng.range_u64(2, 6) as usize;
    while cuts.iter().filter(|&&cut| cut > 0).count() < wanted {
        let column = rng.range_u64(0, COLUMNS as u64) as usize;
        cuts[column] = rng.range_u64(1, 5) as u8;
    }
    cuts
}
