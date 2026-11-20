//! How a grown Neutral animal looks where the research gives a castrated male
//! a look of its own (`docs/context/research-livestock.md`): an ox tall and
//! rangy, a wether near a ewe, a gelding like a mare, a capon plump with a
//! small comb. Never why: only the look (owner). Kinds with none keep the
//! neutral body.

use super::super::Layout;
use super::super::blend::WHOLE;
use crate::creature::CreatureKind;
use crate::gender::Gender;

/// A castrate's build: between the male and the female (thousandths of the way
/// to her), then its legs, girth and size in thousandths of that.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Castrate {
    toward_female: i32,
    legs: i32,
    girth: i32,
    size: i32,
}

const BETWEEN: Castrate = Castrate {
    toward_female: 500,
    legs: WHOLE,
    girth: WHOLE,
    size: WHOLE,
};

fn castrate(kind: CreatureKind) -> Option<Castrate> {
    use CreatureKind::*;
    Some(match kind {
        // An ox: tall and rangy.
        Cattle => Castrate {
            legs: 1120,
            girth: 920,
            size: 1040,
            ..BETWEEN
        },
        // A steer between the two.
        Coo => BETWEEN,
        // A bullock like a cow in build.
        WaterBuffalo => Castrate {
            toward_female: 850,
            ..BETWEEN
        },
        // A steer near the bull's frame.
        Yak => Castrate {
            toward_female: 250,
            ..BETWEEN
        },
        // A wether near the ewe.
        Sheep => Castrate {
            toward_female: 800,
            ..BETWEEN
        },
        // A goat wether tall and lean.
        Goat => Castrate {
            legs: 1080,
            girth: 900,
            ..BETWEEN
        },
        // A barrow like a sow.
        Pig => Castrate {
            toward_female: 850,
            ..BETWEEN
        },
        // A gelding like a mare.
        Horse => Castrate {
            toward_female: WHOLE,
            ..BETWEEN
        },
        // A large gelding.
        BactrianCamel => Castrate {
            toward_female: 300,
            size: 1030,
            ..BETWEEN
        },
        // A härk: large, keeping its antlers.
        Reindeer => Castrate {
            toward_female: 300,
            size: 1050,
            ..BETWEEN
        },
        // A capon: plump, a smaller comb, the hen's tail.
        Chicken => Castrate {
            toward_female: 700,
            girth: 1150,
            ..BETWEEN
        },
        _ => return None,
    })
}

/// The grown body of a Neutral animal of this kind that has a castrate's look.
pub(super) fn castrate_body(kind: CreatureKind) -> Option<Layout> {
    let build = castrate(kind)?;
    let male = Layout::of_gender(kind, Gender::Male)?;
    let female = Layout::of_gender(kind, Gender::Female)?;
    let mut body = Layout::blend(&male, &female, build.toward_female)
        .with_legs(build.legs)
        .with_girth(build.girth);
    body.parts.udder = 0;
    Some(match build.size {
        WHOLE => body,
        size => body.scaled(size),
    })
}

/// Kinds with a castrate's look, for tests.
#[cfg(test)]
pub(super) fn has_castrate(kind: CreatureKind) -> bool {
    castrate(kind).is_some()
}
