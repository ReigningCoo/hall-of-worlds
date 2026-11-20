//! How a beast's males and females differ: their horns or antlers, tusks, a
//! beard, a comb, wattles, a snood, caruncles, a bill knob, spurs, a tail of
//! their own, a hump, an udder, a mane, a dewlap, and how big they are. Only for looks: a creature's `Size`, weight and footprint are its
//! kind's whatever its gender. Colours that differ by gender are looks' choices
//! (`shared::sheet::Choice::gender`).

use super::super::Layout;
use super::super::blend::WHOLE;
use super::super::parts::{Horns, NONE, Parts, Tail};
use super::Beast;
use crate::creature::CreatureKind;
use crate::gender::Gender;

/// What one sex has in place of the neutral body's: each of these but the mane
/// and dewlap replaces the neutral body's own (`Parts`), so a sex states all of
/// them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sex {
    pub horns: Horns,
    pub horn_length: i32,
    /// Antlers' length and tines on each.
    pub antlers: i32,
    pub tines: i32,
    pub tusks: i32,
    /// A goat's beard.
    pub beard: i32,
    pub comb: i32,
    pub wattles: i32,
    pub snood: i32,
    pub caruncles: i32,
    pub knob: i32,
    pub spurs: i32,
    /// Its own tail's shape and length (a cock's sickles, a tom's fan), or
    /// `None` to keep the body's.
    pub tail: Option<(Tail, i32)>,
    /// Each hump's height (as many as the neutral body has).
    pub hump: i32,
    pub udder: i32,
    /// How much more mane than the neutral body's (less if negative): a
    /// Bactrian bull's heavier one. Added, unlike the rest, so a sex that leaves
    /// it out keeps the body's.
    pub mane: i32,
    /// How much more dewlap than the neutral body's, as a rabbit doe's; added as
    /// the mane is.
    pub dewlap: i32,
    /// How big it is, in thousandths of the neutral body.
    pub size: i32,
}

impl Sex {
    /// A sex with what this body has, at its size: to start one from with
    /// `..`.
    pub const fn of(parts: Parts) -> Sex {
        Sex {
            horns: parts.horns,
            horn_length: parts.horn_length,
            antlers: parts.antlers,
            tines: parts.tines,
            tusks: parts.tusks,
            beard: parts.beard,
            comb: parts.comb,
            wattles: parts.wattles,
            snood: parts.snood,
            caruncles: parts.caruncles,
            knob: parts.knob,
            spurs: parts.spurs,
            tail: None,
            hump: parts.hump,
            udder: parts.udder,
            mane: 0,
            dewlap: 0,
            size: WHOLE,
        }
    }

    /// These parts with this sex's in place of their own.
    pub fn on(&self, parts: Parts) -> Parts {
        Parts {
            horns: self.horns,
            horn_length: self.horn_length,
            antlers: self.antlers,
            tines: self.tines,
            tusks: self.tusks,
            beard: self.beard,
            comb: self.comb,
            wattles: self.wattles,
            snood: self.snood,
            caruncles: self.caruncles,
            knob: self.knob,
            spurs: self.spurs,
            tail_shape: self.tail.map_or(parts.tail_shape, |(shape, _)| shape),
            tail: self.tail.map_or(parts.tail, |(_, long)| long),
            hump: self.hump,
            udder: self.udder,
            mane: (parts.mane + self.mane).max(0),
            dewlap: (parts.dewlap + self.dewlap).max(0),
            ..parts
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sexes {
    pub male: Sex,
    pub female: Sex,
}

/// Highland cattle (owner): bulls about 125 cm at the withers and cows about 105
/// (the neutral body 115); cows' horns longer and sweeping up at the tips,
/// bulls' thicker and pointing forwards.
pub(super) const COO: Sexes = Sexes {
    male: Sex {
        horns: Horns::Forward,
        horn_length: 450,
        udder: 0,
        size: 1087,
        ..Sex::of(NONE)
    },
    female: Sex {
        horns: Horns::Upturned,
        horn_length: 580,
        udder: 140,
        size: 913,
        ..Sex::of(NONE)
    },
};

impl Layout {
    /// A kind's body as `Layout::of`, for a creature of this gender: a beast
    /// whose sexes differ has its sex's parts (`Sex`) and size.
    pub fn of_gender(kind: CreatureKind, gender: Gender) -> Option<Layout> {
        let layout = Layout::of(kind)?;
        let Some(sexes) = Beast::of(kind).and_then(|beast| beast.sexes) else {
            return Some(layout);
        };
        let sex = match gender {
            Gender::Male => sexes.male,
            Gender::Female => sexes.female,
            Gender::Neutral => return Some(layout),
        };
        let mut sexed = layout;
        sexed.parts = sex.on(layout.parts);
        Some(match sex.size {
            WHOLE => sexed,
            size => sexed.scaled(size),
        })
    }
}

#[cfg(test)]
mod tests;
