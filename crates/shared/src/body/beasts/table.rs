//! The animals drawn so far, each with its body's measurements, parts and sexes.

use super::super::measured::{COW, DOG, HORSE};
use super::super::measured_heads::{COW_HEAD, HORSE_HEAD, WOLF_HEAD};
use super::super::parts::{Ears, Feet, Horns, NONE, Parts, Snout, Tail, Wings};
use super::livestock::{GOAT, PIG, SHEEP, WATER_BUFFALO, YAK};
use super::{Beast, Bird, CHICKEN_HEAD, Fourfoot, Plan, livestock_sexes, sexes};

/// The animals drawn, measured from the research's heights and lengths (the
/// middle of each range); the kept animals after the first five are in
/// `livestock.rs`.
pub const BEASTS: [Beast; 10] = [
    Beast {
        name: "Horse",
        plan: Plan::Fourfoot(Fourfoot {
            withers: 1580,
            body: 1100,
            barrel: (270, 340),
            fore: [100, 79, 50, 45],
            hind: [145, 125, 70, 50],
            foot: 100,
            neck: (650, 45, 130),
            head: (860, 45, &HORSE_HEAD),
            coat: 0,
            measured: &HORSE,
        }),
        parts: Parts {
            snout: Snout::Muzzle,
            snout_length: 420,
            ears: 120,
            ear_shape: Ears::Upright,
            tail: 900,
            feet: Feet::Hooves,
            mane: 140,
            tail_hair: 700,
            ..NONE
        },
        sexes: None,
    },
    Beast {
        name: "Wolf",
        plan: Plan::Fourfoot(Fourfoot {
            withers: 740,
            body: 720,
            barrel: (110, 160),
            fore: [55, 42, 28, 30],
            hind: [70, 51, 38, 36],
            foot: 110,
            neck: (230, 65, 90),
            head: (290, 8, &WOLF_HEAD),
            coat: 25,
            measured: &DOG,
        }),
        parts: Parts {
            snout: Snout::Long,
            snout_length: 170,
            ears: 90,
            ear_shape: Ears::Upright,
            tail: 450,
            tail_shape: Tail::Bushy,
            feet: Feet::Digitigrade,
            ..NONE
        },
        sexes: None,
    },
    Beast {
        name: "Cattle",
        plan: Plan::Fourfoot(Fourfoot {
            withers: 1350,
            body: 1250,
            barrel: (320, 380),
            fore: [105, 78, 46, 42],
            hind: [130, 103, 58, 51],
            foot: 104,
            neck: (420, 75, 170),
            head: (520, 40, &COW_HEAD),
            coat: 0,
            measured: &COW,
        }),
        parts: Parts {
            snout: Snout::Muzzle,
            snout_length: 330,
            ears: 130,
            ear_shape: Ears::Sideways,
            horns: Horns::Bull,
            horn_length: 350,
            tail: 800,
            tail_shape: Tail::Thin,
            feet: Feet::Hooves,
            tail_hair: 220,
            dewlap: 130,
            udder: 160,
            ..NONE
        },
        sexes: Some(livestock_sexes::CATTLE),
    },
    Beast {
        name: "Coo",
        // Highland cattle: lower than other cattle, with a thick double coat, long
        // horns, a fringe over the eyes and a full switch.
        plan: Plan::Fourfoot(Fourfoot {
            withers: 1150,
            body: 1150,
            barrel: (300, 350),
            fore: [100, 76, 46, 42],
            hind: [124, 98, 56, 50],
            foot: 100,
            neck: (360, 75, 170),
            head: (480, 40, &COW_HEAD),
            coat: 70,
            measured: &COW,
        }),
        parts: Parts {
            snout: Snout::Muzzle,
            snout_length: 290,
            ears: 130,
            ear_shape: Ears::Sideways,
            horns: Horns::Upturned,
            horn_length: 500,
            tail: 750,
            tail_shape: Tail::Thin,
            feet: Feet::Hooves,
            tail_hair: 330,
            dewlap: 110,
            udder: 140,
            fringe: 150,
            ..NONE
        },
        sexes: Some(sexes::COO),
    },
    Beast {
        name: "Chicken",
        plan: Plan::Bird(Bird {
            hips: 170,
            body: (240, 65),
            barrel: (75, 90),
            leg: 14,
            neck: (70, 20, 28),
            head: (80, 0, &CHICKEN_HEAD),
        }),
        parts: Parts {
            snout: Snout::Beak,
            snout_length: 30,
            comb: 28,
            wattles: 28,
            feet: Feet::Talons,
            wings: Wings::Folded,
            ..NONE
        },
        sexes: None,
    },
    SHEEP,
    GOAT,
    PIG,
    WATER_BUFFALO,
    YAK,
];
