//! Kept animals' bodies (livestock plan PR 4): the horse's kin, camels, llamas,
//! reindeer and rabbits, from the research's heights and lengths
//! (`docs/context/research-creatures.md`, livestock), the middle of each range: a
//! neutral adult's (`parts` and size), each sex's in `more_sexes.rs`.

use super::super::measured::HORSE;
use super::super::measured_heads::HORSE_HEAD;
use super::super::parts::{Coat, Ears, Feet, NONE, Parts, Snout, Tail};
use super::drawn_heads::{CAMEL_HEAD, RABBIT_HEAD, SHEEP_HEAD};
use super::drawn_trunks::RABBIT as RABBIT_TRUNK;
use super::{Beast, Fourfoot, Plan, more_sexes};

/// A donkey: smaller than a horse, with a big head, long ears, an upright mane
/// and a cow's tail with a tuft.
pub(super) const DONKEY: Beast = Beast {
    name: "Donkey",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 1150,
        body: 850,
        barrel: (200, 260),
        fore: [74, 58, 36, 32],
        hind: [104, 88, 48, 36],
        foot: 75,
        neck: (430, 40, 105),
        head: (640, 42, &HORSE_HEAD),
        coat: 10,
        measured: &HORSE,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 300,
        ears: 260,
        ear_shape: Ears::Long,
        tail: 650,
        tail_shape: Tail::Thin,
        feet: Feet::Hooves,
        mane: 70,
        tail_hair: 200,
        ..NONE
    },
    sexes: Some(more_sexes::DONKEY),
};

/// A mule: a horse's frame and tail with a donkey's long ears and short mane;
/// one model for both sexes.
pub(super) const MULE: Beast = Beast {
    name: "Mule",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 1450,
        body: 1000,
        barrel: (240, 310),
        fore: [90, 72, 46, 40],
        hind: [130, 110, 62, 46],
        foot: 90,
        neck: (580, 45, 120),
        head: (780, 45, &HORSE_HEAD),
        coat: 0,
        measured: &HORSE,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 380,
        ears: 210,
        ear_shape: Ears::Long,
        tail: 850,
        feet: Feet::Hooves,
        mane: 80,
        tail_hair: 550,
        ..NONE
    },
    sexes: None,
};

/// A dromedary: one hump in the middle of a short back on long legs, a long
/// neck reaching forwards and a small head carried level.
pub(super) const DROMEDARY: Beast = Beast {
    name: "Dromedary",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 1900,
        body: 1050,
        barrel: (250, 330),
        fore: [95, 70, 42, 36],
        hind: [120, 90, 48, 38],
        foot: 170,
        neck: (850, 62, 125),
        head: (560, 12, &CAMEL_HEAD),
        coat: 0,
        measured: &HORSE,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 260,
        ears: 80,
        ear_shape: Ears::Upright,
        tail: 500,
        tail_shape: Tail::Thin,
        feet: Feet::Hooves,
        tail_hair: 150,
        humps: 1,
        hump: 320,
        ..NONE
    },
    sexes: Some(more_sexes::DROMEDARY),
};

/// A Bactrian camel: two humps, shorter legs than a dromedary's, a shaggy coat
/// with a mane down the neck and a beard at the throat.
pub(super) const BACTRIAN_CAMEL: Beast = Beast {
    name: "Bactrian camel",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 1750,
        body: 1150,
        barrel: (280, 350),
        fore: [100, 76, 44, 38],
        hind: [126, 96, 50, 40],
        foot: 170,
        neck: (780, 60, 130),
        head: (560, 12, &CAMEL_HEAD),
        coat: 40,
        measured: &HORSE,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 260,
        ears: 80,
        ear_shape: Ears::Upright,
        tail: 500,
        tail_shape: Tail::Thin,
        feet: Feet::Hooves,
        mane: 120,
        tail_hair: 200,
        beard: 110,
        humps: 2,
        hump: 220,
        ..NONE
    },
    sexes: Some(more_sexes::BACTRIAN_CAMEL),
};

/// A llama: a woolly body on slim legs, a long neck held upright, a small head
/// and long ears curving in; one model for both sexes.
pub(super) const LLAMA: Beast = Beast {
    name: "Llama",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 1100,
        body: 850,
        barrel: (190, 250),
        fore: [62, 46, 30, 26],
        hind: [82, 62, 36, 28],
        foot: 110,
        neck: (720, 10, 80),
        head: (360, 25, &CAMEL_HEAD),
        coat: 20,
        measured: &HORSE,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 170,
        ears: 170,
        ear_shape: Ears::Long,
        tail: 250,
        feet: Feet::Hooves,
        coat_shape: Coat::Fleece,
        coat_length: 30,
        ..NONE
    },
    sexes: None,
};

/// An alpaca: smaller than a llama, in a deep, even fleece, with short spear
/// ears; one model for both sexes.
pub(super) const ALPACA: Beast = Beast {
    name: "Alpaca",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 900,
        body: 700,
        barrel: (170, 220),
        fore: [50, 38, 24, 21],
        hind: [66, 50, 28, 23],
        foot: 90,
        neck: (540, 10, 75),
        head: (290, 25, &CAMEL_HEAD),
        coat: 30,
        measured: &HORSE,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 130,
        ears: 100,
        ear_shape: Ears::Upright,
        tail: 180,
        feet: Feet::Hooves,
        coat_shape: Coat::Fleece,
        coat_length: 70,
        ..NONE
    },
    sexes: None,
};

/// A reindeer: a deer with a thick coat, a hanging fringe of hair at the throat,
/// big hooves and antlers (a neutral draught reindeer's, kept all year).
pub(super) const REINDEER: Beast = Beast {
    name: "Reindeer",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 1100,
        body: 900,
        barrel: (190, 250),
        fore: [62, 48, 26, 22],
        hind: [82, 62, 32, 24],
        foot: 110,
        neck: (420, 45, 80),
        head: (400, 35, &SHEEP_HEAD),
        coat: 30,
        measured: &HORSE,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 180,
        ears: 100,
        ear_shape: Ears::Upright,
        tail: 130,
        feet: Feet::Hooves,
        dewlap: 70,
        antlers: 650,
        tines: 4,
        ..NONE
    },
    sexes: Some(more_sexes::REINDEER),
};

/// A rabbit, crouched as it sits and hops: a round rump high over long hind
/// feet, a short chest, a round head, long ears and a white scut.
pub(super) const RABBIT: Beast = Beast {
    name: "Rabbit",
    plan: Plan::Fourfoot(Fourfoot {
        withers: 170,
        body: 170,
        barrel: (62, 95),
        fore: [10, 8, 6, 5],
        hind: [22, 16, 9, 7],
        foot: 60,
        neck: (40, 50, 30),
        head: (95, 20, &RABBIT_HEAD),
        coat: 5,
        measured: &RABBIT_TRUNK,
    }),
    parts: Parts {
        snout: Snout::Muzzle,
        snout_length: 20,
        ears: 95,
        ear_shape: Ears::Long,
        tail: 45,
        tail_shape: Tail::Bushy,
        feet: Feet::Digitigrade,
        ..NONE
    },
    sexes: Some(more_sexes::RABBIT),
};
