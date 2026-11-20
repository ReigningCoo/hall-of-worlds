//! A young animal's body (livestock plan PR 6): the neutral body reshaped as
//! the research's young look (`docs/context/research-livestock.md`): smaller,
//! its head bigger and its legs longer for its size, its back shorter, horns
//! and antlers only buds, no tusks, beard, udder, comb or skirt yet, and down or
//! a fluffy coat. A juvenile is halfway to the grown body (`stage.rs`).

use super::super::Layout;
use super::super::parts::{Coat, Parts, Tail};
use super::{Beast, Plan};
use crate::creature::CreatureKind;

/// How a kind's young differ from the grown body, in thousandths of it (or
/// millimetres of fluff, at the grown size).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Young {
    /// Its height against the grown neutral body's.
    pub size: i32,
    pub legs: i32,
    pub girth: i32,
    pub back: i32,
    pub neck: i32,
    pub head: i32,
    pub ears: i32,
    /// Horn and antler buds, against the grown ones; 0 for none at all.
    pub buds: i32,
    pub hump: i32,
    pub fluff: i32,
}

/// A newborn on four legs: leggy, short-backed, big-headed, horn buds.
const FOURFOOT: Young = Young {
    size: 560,
    legs: 1250,
    girth: 820,
    back: 800,
    neck: 800,
    head: 1350,
    ears: 1300,
    buds: 120,
    hump: 400,
    fluff: 0,
};

/// A chick: small and round, a big head on a short neck, downy.
const BIRD: Young = Young {
    size: 420,
    legs: 900,
    girth: 1050,
    back: 750,
    neck: 650,
    head: 1450,
    ears: 1000,
    buds: 0,
    hump: 0,
    fluff: 8,
};

impl Young {
    /// A kind's young, from the research's notes on each.
    pub(super) fn of(kind: CreatureKind, beast: &Beast) -> Young {
        use CreatureKind::*;
        let base = match beast.plan {
            Plan::Fourfoot(_) => FOURFOOT,
            Plan::Bird(_) => BIRD,
        };
        match kind {
            // Leggy calf, big ears, horn buds.
            Cattle => Young { ears: 1450, ..base },
            // Fluffy, no horns yet.
            Coo => Young {
                buds: 0,
                fluff: 25,
                ..base
            },
            WaterBuffalo => Young { fluff: 10, ..base },
            // Woolly, no hump or skirt.
            Yak => Young {
                hump: 0,
                fluff: 25,
                ..base
            },
            // Curly lambs; big-eared kids.
            Sheep => Young { fluff: 10, ..base },
            Goat => Young { ears: 1500, ..base },
            // Piglets: short legs, long heads anyway.
            Pig => Young {
                legs: 1000,
                head: 1200,
                ..base
            },
            // Gangly foals with a fuzzy coat.
            // A horse's head grows out of its neck: less of it to grow.
            Horse | Mule => Young {
                legs: 1350,
                head: 1200,
                fluff: 10,
                ..base
            },
            Donkey => Young {
                legs: 1300,
                head: 1200,
                ears: 1500,
                fluff: 20,
                ..base
            },
            // Leggy pale calves with small, floppy humps.
            Dromedary => Young { legs: 1350, ..base },
            BactrianCamel => Young {
                legs: 1300,
                fluff: 25,
                ..base
            },
            Llama | Alpaca => Young {
                legs: 1300,
                fluff: 20,
                ..base
            },
            // Fluffy, round-faced young.
            Rabbit => Young {
                legs: 1000,
                head: 1500,
                fluff: 15,
                ..base
            },
            _ => base,
        }
    }

    /// The grown neutral body as this young one.
    pub(super) fn body(&self, grown: &Layout) -> Layout {
        let mut young = grown
            .with_legs(self.legs)
            .with_girth(self.girth)
            .with_back(self.back)
            .with_neck(self.neck)
            .with_head(self.head)
            .with_fluff(self.fluff);
        young.parts = self.parts(&grown.parts);
        young.scaled(self.size)
    }

    /// What it has yet of the grown body's parts.
    fn parts(&self, grown: &Parts) -> Parts {
        let share = |value: i32, thousandths: i32| value * thousandths / 1000;
        let tail = match grown.tail_shape {
            // A chick's stub of a tail.
            Tail::Fan | Tail::Sickle => (Tail::Fan, share(grown.tail, 300)),
            shape => (shape, grown.tail),
        };
        Parts {
            ears: share(grown.ears, self.ears),
            horn_length: share(grown.horn_length, self.buds),
            antlers: share(grown.antlers, self.buds),
            tines: 0,
            tusks: 0,
            beard: 0,
            comb: share(grown.comb, 200),
            wattles: share(grown.wattles, 150),
            snood: 0,
            caruncles: 0,
            knob: 0,
            spurs: 0,
            tail_shape: tail.0,
            tail: tail.1,
            tail_hair: share(grown.tail_hair, 500),
            hump: share(grown.hump, self.hump),
            humps: if self.hump == 0 { 0 } else { grown.humps },
            udder: 0,
            mane: share(grown.mane, 400),
            dewlap: share(grown.dewlap, 300),
            fringe: share(grown.fringe, 500),
            coat_shape: match grown.coat_shape {
                Coat::Skirt => Coat::Smooth,
                shape => shape,
            },
            coat_length: match grown.coat_shape {
                Coat::Skirt => 0,
                _ => share(grown.coat_length, 600),
            },
            ..*grown
        }
    }
}
