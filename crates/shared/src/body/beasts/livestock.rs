//! Kept animals' bodies (livestock plan PR 3), from the research's heights and
//! lengths (`docs/context/research-creatures.md`, livestock), the middle of each
//! range: a neutral adult's (`parts` and size), each sex's in `livestock_sexes.rs`.

use super::super::measured::COW;
use super::super::measured_heads::COW_HEAD;
use super::super::parts::{Coat, Ears, Feet, Horns, NONE, Parts, Snout, Tail};
use super::drawn_heads::{PIG_HEAD, SHEEP_HEAD};
use super::{Beast, Fourfoot, Plan, livestock_sexes};

/// A sheep: small and boxy under a thick fleece, with a long undocked tail.
pub(super) const SHEEP: Beast = Beast {
    name: "Sheep",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 750,
        body: 650,
        barrel: (170, 210),
        fore: [52, 38, 20, 18],
        hind: [66, 50, 26, 22],
        foot: 55,
        neck: (280, 50, 70),
        head: (250, 45, &SHEEP_HEAD),
        coat: 0,
        measured: &COW,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 120,
        ears: 80,
        ear_shape: Ears::Sideways,
        tail: 280,
        tail_shape: Tail::Thin,
        feet: Feet::Hooves,
        udder: 50,
        coat_shape: Coat::Fleece,
        coat_length: 50,
        ..NONE
    },
    sexes: Some(livestock_sexes::SHEEP),
};

/// A goat: leaner than a sheep, its neck upright, horns swept back, a beard and a
/// short tail held up.
pub(super) const GOAT: Beast = Beast {
    name: "Goat",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 700,
        body: 600,
        barrel: (140, 190),
        fore: [48, 34, 18, 16],
        hind: [60, 46, 22, 18],
        foot: 50,
        neck: (300, 30, 55),
        head: (230, 35, &SHEEP_HEAD),
        coat: 0,
        measured: &COW,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 110,
        ears: 100,
        ear_shape: Ears::Sideways,
        horns: Horns::Back,
        horn_length: 200,
        tail: 130,
        feet: Feet::Hooves,
        beard: 80,
        udder: 60,
        ..NONE
    },
    sexes: Some(livestock_sexes::GOAT),
};

/// A pig: a long, deep body on short legs, a short thick neck and a wedge of a head
/// ending in a flat snout.
pub(super) const PIG: Beast = Beast {
    name: "Pig",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 750,
        body: 950,
        barrel: (240, 330),
        fore: [62, 46, 24, 20],
        hind: [82, 60, 28, 22],
        foot: 55,
        neck: (160, 75, 150),
        head: (360, 25, &PIG_HEAD),
        coat: 0,
        measured: &COW,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 160,
        ears: 110,
        ear_shape: Ears::Upright,
        tail: 220,
        tail_shape: Tail::Thin,
        feet: Feet::Hooves,
        udder: 50,
        ..NONE
    },
    sexes: Some(livestock_sexes::PIG),
};

/// A water buffalo: bigger and heavier than cattle, with great horns sweeping out
/// and back, its head carried low and level.
pub(super) const WATER_BUFFALO: Beast = Beast {
    name: "Water buffalo",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 1400,
        body: 1350,
        barrel: (350, 410),
        fore: [112, 82, 48, 44],
        hind: [138, 108, 60, 52],
        foot: 110,
        neck: (380, 85, 180),
        head: (560, 35, &COW_HEAD),
        coat: 0,
        measured: &COW,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 330,
        ears: 140,
        ear_shape: Ears::Sideways,
        horns: Horns::Sweep,
        horn_length: 700,
        tail: 800,
        tail_shape: Tail::Thin,
        feet: Feet::Hooves,
        tail_hair: 180,
        dewlap: 40,
        udder: 140,
        ..NONE
    },
    sexes: Some(livestock_sexes::WATER_BUFFALO),
};

/// A yak: a hump over the shoulders, a long shaggy skirt and a bushy tail.
pub(super) const YAK: Beast = Beast {
    name: "Yak",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 1250,
        body: 1150,
        barrel: (320, 390),
        fore: [100, 75, 44, 40],
        hind: [118, 94, 52, 46],
        foot: 100,
        neck: (320, 80, 165),
        head: (480, 50, &COW_HEAD),
        coat: 50,
        measured: &COW,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 270,
        ears: 100,
        ear_shape: Ears::Sideways,
        horns: Horns::Bull,
        horn_length: 450,
        tail: 650,
        tail_shape: Tail::Bushy,
        feet: Feet::Hooves,
        tail_hair: 450,
        udder: 90,
        humps: 1,
        hump: 80,
        hump_along: 780,
        coat_shape: Coat::Skirt,
        coat_length: 300,
        ..NONE
    },
    sexes: Some(livestock_sexes::YAK),
};
