//! How the kept birds of livestock plan PR 5, and the chicken, differ by sex
//! (`Sexes`), from the research's dimorphism notes
//! (`docs/context/research-livestock.md`). Guineafowl and pigeons have one model
//! for both, so none here; colours by sex are the looks' (`sheet::looks`).

use super::super::parts::{NONE, Tail};
use super::sexes::{Sex, Sexes};

/// Chickens: cocks bigger, with a big comb and wattles, spurs and arching
/// sickles (their hackles are a marking); hens with a small comb and a short
/// upright tail.
pub(super) const CHICKEN: Sexes = Sexes {
    male: Sex {
        comb: 55,
        wattles: 48,
        spurs: 22,
        tail: Some((Tail::Sickle, 260)),
        size: 1120,
        ..Sex::of(NONE)
    },
    female: Sex {
        comb: 16,
        wattles: 12,
        tail: Some((Tail::Fan, 90)),
        size: 920,
        ..Sex::of(NONE)
    },
};

/// Ducks: drakes a little bigger (their green head is a marking).
pub(super) const DUCK: Sexes = Sexes {
    male: Sex {
        size: 1040,
        ..Sex::of(NONE)
    },
    female: Sex {
        size: 960,
        ..Sex::of(NONE)
    },
};

/// Muscovy ducks: drakes nearly twice the ducks' weight, with red caruncles over
/// the face, a knob on the bill and a crest; ducks with a feathered face.
pub(super) const MUSCOVY_DUCK: Sexes = Sexes {
    male: Sex {
        caruncles: 9,
        knob: 14,
        size: 1120,
        ..Sex::of(NONE)
    },
    female: Sex {
        size: 880,
        ..Sex::of(NONE)
    },
};

/// Geese: ganders taller, with a bigger knob on the bill.
pub(super) const GOOSE: Sexes = Sexes {
    male: Sex {
        knob: 26,
        size: 1060,
        ..Sex::of(NONE)
    },
    female: Sex {
        knob: 14,
        size: 950,
        ..Sex::of(NONE)
    },
};

/// Turkeys: toms twice the hens' weight, with a long snood, big wattles and
/// caruncles, a breast beard, spurs and a fanned tail; hens smaller, with small
/// ones and a small tail.
pub(super) const TURKEY: Sexes = Sexes {
    male: Sex {
        snood: 90,
        wattles: 70,
        caruncles: 10,
        beard: 130,
        spurs: 26,
        tail: Some((Tail::Fan, 420)),
        size: 1120,
        ..Sex::of(NONE)
    },
    female: Sex {
        snood: 18,
        wattles: 24,
        caruncles: 5,
        tail: Some((Tail::Fan, 130)),
        size: 880,
        ..Sex::of(NONE)
    },
};

/// Quail: hens a little bigger (the cock's rufous breast is its colour).
pub(super) const QUAIL: Sexes = Sexes {
    male: Sex {
        size: 970,
        ..Sex::of(NONE)
    },
    female: Sex {
        size: 1030,
        ..Sex::of(NONE)
    },
};

#[cfg(test)]
mod tests;
