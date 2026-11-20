//! How kept animals' males and females differ (`Sexes`), from the research's
//! dimorphism notes (`docs/context/research-livestock.md`): sizes as shares of the
//! neutral body, horns, tusks, beards, humps and udders.

use super::super::parts::{Horns, NONE};
use super::sexes::{Sex, Sexes};

/// Cattle: bulls bigger, with short thick horns; cows finer, their horns longer
/// and lyre-shaped, with an udder. An aurochs's colours are its coat's.
pub(super) const CATTLE: Sexes = Sexes {
    male: Sex {
        horns: Horns::Bull,
        horn_length: 300,
        size: 1080,
        ..Sex::of(NONE)
    },
    female: Sex {
        horns: Horns::Upturned,
        horn_length: 420,
        udder: 160,
        size: 930,
        ..Sex::of(NONE)
    },
};

/// Sheep: rams bigger with coiled horns; ewes hornless, with an udder.
pub(super) const SHEEP: Sexes = Sexes {
    male: Sex {
        horns: Horns::Spiral,
        horn_length: 420,
        size: 1100,
        ..Sex::of(NONE)
    },
    female: Sex {
        udder: 60,
        size: 920,
        ..Sex::of(NONE)
    },
};

/// Goats: bucks bigger, with longer horns and a fuller beard; does with small
/// horns, a wisp of a beard and an udder.
pub(super) const GOAT: Sexes = Sexes {
    male: Sex {
        horns: Horns::Back,
        horn_length: 300,
        beard: 150,
        size: 1100,
        ..Sex::of(NONE)
    },
    female: Sex {
        horns: Horns::Back,
        horn_length: 140,
        beard: 50,
        udder: 80,
        size: 920,
        ..Sex::of(NONE)
    },
};

/// Pigs: boars bigger, with tusks; sows with their teats (as an udder).
pub(super) const PIG: Sexes = Sexes {
    male: Sex {
        tusks: 60,
        size: 1080,
        ..Sex::of(NONE)
    },
    female: Sex {
        udder: 70,
        size: 940,
        ..Sex::of(NONE)
    },
};

/// Water buffalo (bulls about 131 cm at the withers, cows 124): bulls' horns
/// thicker, cows' a little longer, with an udder.
pub(super) const WATER_BUFFALO: Sexes = Sexes {
    male: Sex {
        horns: Horns::Sweep,
        horn_length: 650,
        size: 1030,
        ..Sex::of(NONE)
    },
    female: Sex {
        horns: Horns::Sweep,
        horn_length: 720,
        udder: 160,
        size: 970,
        ..Sex::of(NONE)
    },
};

/// Yaks (bulls about 125 cm at the withers, cows 111, and twice as heavy): bulls
/// with wide horns and a big hump; cows with small upright horns, a low hump and a
/// small udder.
pub(super) const YAK: Sexes = Sexes {
    male: Sex {
        horns: Horns::Bull,
        horn_length: 650,
        hump: 130,
        size: 1120,
        ..Sex::of(NONE)
    },
    female: Sex {
        horns: Horns::Upturned,
        horn_length: 420,
        hump: 40,
        udder: 100,
        size: 890,
        ..Sex::of(NONE)
    },
};

#[cfg(test)]
mod tests;
