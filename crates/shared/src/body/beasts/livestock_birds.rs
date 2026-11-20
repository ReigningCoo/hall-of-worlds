//! Kept birds' bodies (livestock plan PR 5): ducks, Muscovy ducks, geese, turkeys,
//! guineafowl, pigeons and quail, from the research's sizes
//! (`docs/context/research-creatures.md`, livestock): a neutral adult's (`parts`
//! and size), each sex's in `bird_sexes.rs`. Bare heads and coloured necks are
//! markings (`sheet::markings`), as they are colours.

use super::super::parts::{Feet, NONE, Parts, Snout, Tail, Wings};
use super::drawn_heads::DUCK_HEAD;
use super::{Beast, Bird, CHICKEN_HEAD, Plan, bird_sexes};

/// A water bird's: a flat bill, webbed feet and a short tail.
const WATER_BIRD: Parts = Parts {
    snout: Snout::Bill,
    feet: Feet::Webbed,
    wings: Wings::Folded,
    tail_shape: Tail::Fan,
    ..NONE
};

/// A land bird's: a short beak and toes. Birds' tails are small fans: a plain one
/// hangs down from a leaning back.
const LAND_BIRD: Parts = Parts {
    snout: Snout::Beak,
    feet: Feet::Talons,
    wings: Wings::Folded,
    ..NONE
};

/// A duck: long and low, nearly level, on short legs.
pub(super) const DUCK: Beast = Beast {
    name: "Duck",
    plan: Plan::Bird(Bird {
        hips: 120,
        body: (300, 78),
        barrel: (95, 95),
        leg: 13,
        neck: (120, 20, 28),
        head: (90, 10, &DUCK_HEAD),
    }),
    parts: Parts {
        snout_length: 55,
        tail: 45,
        ..WATER_BIRD
    },
    sexes: Some(bird_sexes::DUCK),
};

/// A Muscovy duck: bigger and broader than a duck, with a wide flat tail.
pub(super) const MUSCOVY_DUCK: Beast = Beast {
    name: "Muscovy duck",
    plan: Plan::Bird(Bird {
        hips: 130,
        body: (330, 78),
        barrel: (105, 100),
        leg: 15,
        neck: (110, 25, 32),
        head: (100, 10, &DUCK_HEAD),
    }),
    parts: Parts {
        snout_length: 55,
        tail: 70,
        ..WATER_BIRD
    },
    sexes: Some(bird_sexes::MUSCOVY_DUCK),
};

/// A goose: big and upright on its legs, with a long neck held up.
pub(super) const GOOSE: Beast = Beast {
    name: "Goose",
    plan: Plan::Bird(Bird {
        hips: 210,
        body: (380, 70),
        barrel: (120, 125),
        leg: 18,
        neck: (260, 10, 30),
        head: (110, 15, &DUCK_HEAD),
    }),
    parts: Parts {
        snout_length: 70,
        knob: 18,
        tail: 60,
        ..WATER_BIRD
    },
    sexes: Some(bird_sexes::GOOSE),
};

/// A turkey: a big deep body on long legs, a bare head and neck with a snood,
/// wattles and caruncles.
pub(super) const TURKEY: Beast = Beast {
    name: "Turkey",
    plan: Plan::Bird(Bird {
        hips: 360,
        body: (440, 58),
        barrel: (160, 185),
        leg: 26,
        neck: (200, 20, 34),
        head: (100, 0, &CHICKEN_HEAD),
    }),
    parts: Parts {
        snout_length: 32,
        snood: 45,
        wattles: 40,
        caruncles: 7,
        tail: 160,
        tail_shape: Tail::Fan,
        ..LAND_BIRD
    },
    sexes: Some(bird_sexes::TURKEY),
};

/// A guineafowl: a round body and a small bare head with a casque (a small comb)
/// and wattles; one model for both sexes.
pub(super) const GUINEAFOWL: Beast = Beast {
    name: "Guineafowl",
    plan: Plan::Bird(Bird {
        hips: 170,
        body: (290, 72),
        barrel: (100, 115),
        leg: 14,
        neck: (100, 25, 22),
        head: (60, 5, &CHICKEN_HEAD),
    }),
    parts: Parts {
        snout_length: 22,
        comb: 22,
        wattles: 20,
        tail: 70,
        tail_shape: Tail::Fan,
        ..LAND_BIRD
    },
    sexes: None,
};

/// A pigeon: small and plump, with a short neck and a spread tail.
pub(super) const PIGEON: Beast = Beast {
    name: "Pigeon",
    plan: Plan::Bird(Bird {
        hips: 85,
        body: (200, 70),
        barrel: (60, 72),
        leg: 9,
        neck: (55, 20, 20),
        head: (50, 5, &CHICKEN_HEAD),
    }),
    parts: Parts {
        snout_length: 16,
        tail: 90,
        tail_shape: Tail::Fan,
        ..LAND_BIRD
    },
    sexes: None,
};

/// A quail: small, round and nearly tailless, low on short legs.
pub(super) const QUAIL: Beast = Beast {
    name: "Quail",
    plan: Plan::Bird(Bird {
        hips: 50,
        body: (130, 78),
        barrel: (52, 55),
        leg: 6,
        neck: (35, 25, 13),
        head: (52, 5, &CHICKEN_HEAD),
    }),
    parts: Parts {
        snout_length: 10,
        tail: 30,
        tail_shape: Tail::Fan,
        ..LAND_BIRD
    },
    sexes: Some(bird_sexes::QUAIL),
};
