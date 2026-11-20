//! Kept birds' feathers and markings (livestock plan PR 5), from the research's
//! colours (`docs/context/research-livestock.md`): a duck's mallard colours by sex,
//! a cock's hackles, turkeys' and guineafowl's bare heads, a quail cock's rufous
//! breast. A marking has one colour, so a mallard drake is grey with a green head
//! (its chestnut breast left out). Saved by place, so new ones go at the end; mobs
//! roll them with the weights in `mob::looks`.

use super::looks::{Choice, colour};
use super::markings::{CREAM, Face, Marks, WHITE, marked, marks, patched};
use crate::creature::CreatureKind;
use crate::gender::Gender;

/// A kept bird's feathers, or `None` for a kind that isn't one (or a chicken,
/// whose are `looks.rs`'s).
pub(super) fn bird_feathers(kind: CreatureKind) -> Option<&'static [Choice]> {
    use CreatureKind::*;
    Some(match kind {
        Duck => DUCK,
        MuscovyDuck => MUSCOVY,
        Goose => GOOSE,
        Turkey => TURKEY,
        Guineafowl => GUINEAFOWL,
        Pigeon => PIGEON,
        Quail => QUAIL,
        _ => return None,
    })
}

/// A kept bird's markings; none for a kind without.
pub(super) fn bird_markings(kind: CreatureKind) -> &'static [Choice] {
    use CreatureKind::*;
    match kind {
        Chicken => CHICKEN_MARKS,
        Duck => DUCK_MARKS,
        MuscovyDuck => MUSCOVY_MARKS,
        Goose => GOOSE_MARKS,
        Turkey => TURKEY_MARKS,
        Guineafowl => GUINEAFOWL_MARKS,
        Pigeon => PIGEON_MARKS,
        Quail => QUAIL_MARKS,
        _ => &[],
    }
}

/// White (Pekin), a mallard drake's grey or a duck's speckled brown, black
/// (Cayuga) and buff.
const DUCK: &[Choice] = &[
    colour("White", [240, 238, 230]),
    colour("Mallard", [150, 146, 138]).only(Gender::Male),
    colour("Mallard", [138, 102, 66]).only(Gender::Female),
    colour("Black", [36, 38, 40]),
    colour("Buff", [214, 178, 122]),
];

const MUSCOVY: &[Choice] = &[
    colour("Black", [34, 34, 38]),
    colour("White", [240, 238, 232]),
    colour("Chocolate", [96, 66, 46]),
    colour("Blue", [112, 120, 132]),
];

/// White (Embden), grey (greylag, Toulouse), brown (Chinese) and buff.
const GOOSE: &[Choice] = &[
    colour("White", [240, 240, 234]),
    colour("Grey", [140, 136, 128]),
    colour("Brown", [166, 134, 100]),
    colour("Buff", [206, 170, 120]),
];

/// Bronze, black, white, Bourbon red and slate.
const TURKEY: &[Choice] = &[
    colour("Bronze", [92, 66, 46]),
    colour("Black", [34, 32, 34]),
    colour("White", [238, 236, 230]),
    colour("Red", [140, 72, 46]),
    colour("Slate", [110, 114, 122]),
];

/// Pearl grey (the helmeted guineafowl's), lavender and white.
const GUINEAFOWL: &[Choice] = &[
    colour("Pearl grey", [78, 80, 90]),
    colour("Lavender", [150, 146, 164]),
    colour("White", [238, 236, 232]),
];

/// Blue-grey (the rock dove's), white, black, red and brown.
const PIGEON: &[Choice] = &[
    colour("Blue-grey", [128, 134, 146]),
    colour("White", [240, 240, 236]),
    colour("Black", [40, 40, 44]),
    colour("Red", [140, 80, 60]),
    colour("Brown", [116, 86, 62]),
];

/// The wild streaked brown, white, golden and dark.
const QUAIL: &[Choice] = &[
    colour("Wild brown", [140, 108, 74]),
    colour("White", [236, 232, 222]),
    colour("Golden", [196, 152, 92]),
    colour("Dark", [78, 62, 48]),
];

/// The neck's top and sides from the head to the shoulders.
const fn hackles(colour: [u8; 3]) -> Marks {
    Marks {
        hackles: true,
        ..marks(colour, false, Face::Bare)
    }
}

/// The whole head and the neck's top half: a drake's green, a turkey's or a
/// guineafowl's bare skin.
const fn hood(colour: [u8; 3]) -> Marks {
    Marks {
        hood: true,
        ..marks(colour, false, Face::Bare)
    }
}

/// Hens plain; cocks mostly with golden or pale hackles.
const CHICKEN_MARKS: &[Choice] = &[
    super::looks::word("Plain"),
    marked("Golden hackles", hackles([212, 150, 62])).only(Gender::Male),
    marked("Pale hackles", hackles([226, 214, 180])).only(Gender::Male),
];

const DUCK_MARKS: &[Choice] = &[
    super::looks::word("Plain"),
    marked("Green head", hood([38, 92, 54])).only(Gender::Male),
];

/// Plain, or pied with white.
const MUSCOVY_MARKS: &[Choice] = &[
    super::looks::word("Plain"),
    patched("White patches", WHITE, 22),
];

/// Plain, or a greylag's pale belly.
const GOOSE_MARKS: &[Choice] = &[
    super::looks::word("Plain"),
    marked("Pale belly", marks(CREAM, true, Face::Bare)),
];

/// The bare head, red or pale blue.
const TURKEY_MARKS: &[Choice] = &[
    marked("Bare red head", hood([200, 70, 60])),
    marked("Bare blue head", hood([150, 176, 204])),
];

/// The bare head, blue or white.
const GUINEAFOWL_MARKS: &[Choice] = &[
    marked("Bare blue head", hood([120, 150, 196])),
    marked("Bare white head", hood([224, 222, 214])),
];

/// Plain, a sheen of green and purple on the neck, or pied with white.
const PIGEON_MARKS: &[Choice] = &[
    super::looks::word("Plain"),
    marked("Shining neck", hackles([88, 118, 108])),
    patched("White patches", WHITE, 23),
];

/// Plain; a cock's rufous breast and cheeks, a hen's pale spotted breast.
const QUAIL_MARKS: &[Choice] = &[
    super::looks::word("Plain"),
    marked("Rufous breast", marks([150, 82, 48], true, Face::Bare)).only(Gender::Male),
    marked("Pale breast", marks(CREAM, true, Face::Bare)).only(Gender::Female),
];
