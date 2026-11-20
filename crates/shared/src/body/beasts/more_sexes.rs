//! How the kept animals of livestock plan PR 4 differ by sex (`Sexes`), from the
//! research's dimorphism notes (`docs/context/research-livestock.md`). Mules,
//! llamas and alpacas have one model for both, so none here.

use super::super::parts::NONE;
use super::sexes::{Sex, Sexes};

/// Donkeys: jacks a little bigger; jennies with an udder.
pub(super) const DONKEY: Sexes = Sexes {
    male: Sex {
        size: 1040,
        ..Sex::of(NONE)
    },
    female: Sex {
        udder: 70,
        size: 960,
        ..Sex::of(NONE)
    },
};

/// Dromedaries (bulls 1.8 to 2.4 m at the shoulder, cows 1.7 to 1.9): bulls
/// bigger; cows with an udder.
pub(super) const DROMEDARY: Sexes = Sexes {
    male: Sex {
        hump: 340,
        size: 1080,
        ..Sex::of(NONE)
    },
    female: Sex {
        hump: 300,
        udder: 90,
        size: 930,
        ..Sex::of(NONE)
    },
};

/// Bactrian camels (bulls about 600 kg, cows 480): bulls bigger, with a heavier
/// mane and beard; cows with lighter ones and an udder.
pub(super) const BACTRIAN_CAMEL: Sexes = Sexes {
    male: Sex {
        hump: 240,
        mane: 70,
        beard: 180,
        size: 1060,
        ..Sex::of(NONE)
    },
    female: Sex {
        hump: 200,
        mane: -30,
        beard: 60,
        udder: 80,
        size: 940,
        ..Sex::of(NONE)
    },
};

/// Reindeer: both sexes antlered, unlike any other deer; bulls much bigger, with
/// big branching antlers; cows with small, thin ones and an udder.
pub(super) const REINDEER: Sexes = Sexes {
    male: Sex {
        antlers: 820,
        tines: 6,
        size: 1120,
        ..Sex::of(NONE)
    },
    female: Sex {
        antlers: 380,
        tines: 2,
        udder: 40,
        size: 900,
        ..Sex::of(NONE)
    },
};

/// Rabbits: does a little bigger, with a dewlap under the chin.
pub(super) const RABBIT: Sexes = Sexes {
    male: Sex {
        size: 980,
        ..Sex::of(NONE)
    },
    female: Sex {
        dewlap: 18,
        size: 1030,
        ..Sex::of(NONE)
    },
};

#[cfg(test)]
mod tests;
