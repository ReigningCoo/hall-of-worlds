//! Markings: a second colour over parts of an animal's coat, chosen when it is made as
//! its other looks are (`Trait::Markings`), so everyone sees the same animal. Kept
//! simple (owner, 2026-11-12): a paler belly, a face mask, a blaze or a white face,
//! and patches laid out by a number. Saved by place in the list, so new ones go at the
//! end; the first is what a creature has if nothing was chosen.

use crate::creature::CreatureKind;
use crate::gender::Gender;

use super::looks::word;
use super::{Choice, Pattern};

/// Where on the body a second colour is, and the colour: the client draws it by
/// region of the body (`item_art`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Marks {
    /// Red, green and blue, 0 to 255.
    pub colour: [u8; 3],
    /// Paler underneath: the belly, the throat and under the jaw.
    pub belly: bool,
    pub face: Face,
    /// Patches over the trunk, neck, head and the tops of the legs, laid out by this
    /// number, so animals with the same markings have different patches.
    pub patches: Option<u8>,
    /// Brindle: wavy stripes across the trunk, neck and legs, from a brindled coat
    /// (`Pattern::Brindle`) rather than a choice of markings.
    pub stripes: bool,
    /// The whole head and the top half of the neck, as a mallard drake's green
    /// head.
    pub hood: bool,
    /// The neck's top and sides from the head to the shoulders, as a cock's
    /// hackles.
    pub hackles: bool,
    /// A narrow stripe along the top of the back, as an aurochs bull's.
    pub eel: bool,
    /// A stripe down over each shoulder, which with an eel stripe makes a
    /// donkey's cross.
    pub cross: bool,
}

impl Marks {
    /// A brindled coat's stripes, in this colour.
    pub const fn brindle(colour: [u8; 3]) -> Self {
        Marks {
            colour,
            belly: false,
            face: Face::Bare,
            patches: None,
            stripes: true,
            hood: false,
            hackles: false,
            eel: false,
            cross: false,
        }
    }
}

/// What of the face is marked.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Face {
    Bare,
    /// The cheeks, the muzzle's sides and under the jaw, as a wolf's.
    Mask,
    /// A stripe down the front of the face, as a horse's.
    Blaze,
    /// The whole face, as a Hereford's.
    Whole,
}

/// Off-white, as a wolf's or a horse's markings.
pub(super) const CREAM: [u8; 3] = [226, 218, 200];
pub(super) const WHITE: [u8; 3] = [236, 234, 228];
const DARK: [u8; 3] = [40, 34, 32];

pub(super) const fn marked(name: &'static str, marks: Marks) -> Choice {
    Choice {
        name,
        colour: None,
        pattern: Pattern::Marked(marks),
        gender: None,
    }
}

pub(super) const fn marks(colour: [u8; 3], belly: bool, face: Face) -> Marks {
    Marks {
        colour,
        belly,
        face,
        patches: None,
        stripes: false,
        hood: false,
        hackles: false,
        eel: false,
        cross: false,
    }
}

pub(super) const fn patched(name: &'static str, colour: [u8; 3], layout: u8) -> Choice {
    marked(
        name,
        Marks {
            colour,
            belly: false,
            face: Face::Bare,
            patches: Some(layout),
            stripes: false,
            hood: false,
            hackles: false,
            eel: false,
            cross: false,
        },
    )
}

const WOLF: &[Choice] = &[
    marked("Pale belly and mask", marks(CREAM, true, Face::Mask)),
    marked("Pale belly", marks(CREAM, true, Face::Bare)),
];

/// Mostly patched (black and white, red and white), as farm cattle are.
const CATTLE: &[Choice] = &[
    word("Plain"),
    marked("White face", marks(WHITE, true, Face::Whole)),
    patched("White patches", WHITE, 0),
    patched("White patches", WHITE, 1),
    patched("White patches", WHITE, 2),
    patched("White patches", WHITE, 3),
    patched("White patches", WHITE, 4),
    patched("White patches", WHITE, 5),
    patched("Dark patches", DARK, 6),
    patched("Dark patches", DARK, 7),
    marked("Eel stripe", EEL_STRIPE).only(Gender::Male),
];

const HORSE: &[Choice] = &[
    word("Plain"),
    marked("Blaze", marks(WHITE, false, Face::Blaze)),
    marked("Blaze", marks(WHITE, false, Face::Blaze)),
    patched("Pinto", WHITE, 8),
    patched("Pinto", WHITE, 9),
];

/// An aurochs bull's pale stripe along the back (wild cattle only: mobs don't
/// roll it for kept cattle).
const EEL_STRIPE: Marks = Marks {
    eel: true,
    ..marks([214, 196, 160], false, Face::Bare)
};

/// Sheep: plain, a black face, a pale belly (a mouflon's), or a Jacob's patches.
const SHEEP: &[Choice] = &[
    word("Plain"),
    marked("Black face", marks(DARK, false, Face::Whole)),
    marked("Pale belly", marks(CREAM, true, Face::Bare)),
    patched("Dark patches", DARK, 11),
];

/// Goats: plain, pale face stripes and belly (a Toggenburg's), or patches.
const GOAT: &[Choice] = &[
    word("Plain"),
    marked("Pale face and belly", marks(CREAM, true, Face::Blaze)),
    patched("White patches", WHITE, 12),
    patched("Dark patches", DARK, 13),
];

/// Pigs: plain, or spotted or patched.
const PIG: &[Choice] = &[
    word("Plain"),
    patched("Dark spots", DARK, 14),
    patched("White patches", WHITE, 15),
];

/// Water buffalo: plain, or a pale throat and chest (a swamp buffalo's chevrons).
const WATER_BUFFALO: &[Choice] = &[
    word("Plain"),
    marked("Pale throat", marks(CREAM, true, Face::Bare)),
];

/// Yaks: plain, a white face, or white patches.
const YAK: &[Choice] = &[
    word("Plain"),
    marked("White face", marks(WHITE, false, Face::Whole)),
    patched("White patches", WHITE, 17),
];

/// A minotaur's, to pick when making one.
const MINOTAUR: &[Choice] = &[
    word("Plain"),
    marked("White face", marks(WHITE, false, Face::Whole)),
    patched("Patched", WHITE, 10),
];

/// Donkeys: mostly a dark cross over the shoulders and along the back, or a
/// pale muzzle, eyes and belly; mobs roll these with the weights in `mob::looks`.
/// (One colour each, so a donkey has the cross or the pale points, not both.)
const DONKEY: &[Choice] = &[
    word("Plain"),
    marked("Dark cross", DONKEY_CROSS),
    marked("Pale points", marks(CREAM, true, Face::Mask)),
];

/// A donkey's dark stripe along the back and across the shoulders.
const DONKEY_CROSS: Marks = Marks {
    eel: true,
    cross: true,
    ..marks([58, 48, 42], false, Face::Bare)
};

/// Mules: mostly a donkey's mealy pale points, sometimes its cross, or a
/// horse's blaze.
const MULE: &[Choice] = &[
    word("Plain"),
    marked("Pale points", marks(CREAM, true, Face::Mask)),
    marked("Dark cross", DONKEY_CROSS),
    marked("Blaze", marks(WHITE, false, Face::Blaze)),
];

/// Camels: plain, or paler underneath.
const CAMEL: &[Choice] = &[
    word("Plain"),
    marked("Pale belly", marks(CREAM, true, Face::Bare)),
];

/// Llamas and alpacas: plain, a white face, or piebald.
const LLAMA: &[Choice] = &[
    word("Plain"),
    marked("White face", marks(WHITE, false, Face::Whole)),
    patched("White patches", WHITE, 18),
    patched("Dark patches", DARK, 19),
];

/// Reindeer: mostly a pale neck and belly; mobs roll these with the weights in
/// `mob::looks`.
const REINDEER: &[Choice] = &[
    word("Plain"),
    marked("Pale neck and belly", marks(CREAM, true, Face::Bare)),
];

/// Rabbits: plain, a wild rabbit's pale belly, or a spotted rabbit's patches.
const RABBIT: &[Choice] = &[
    word("Plain"),
    marked("Pale belly", marks(CREAM, true, Face::Bare)),
    patched("White patches", WHITE, 20),
    patched("Dark patches", DARK, 21),
];

/// The markings a creature of this kind may have; none for a kind without.
pub(super) fn markings_choices(kind: CreatureKind) -> &'static [Choice] {
    match kind {
        CreatureKind::Wolf => WOLF,
        CreatureKind::Cattle => CATTLE,
        CreatureKind::Horse => HORSE,
        CreatureKind::Minotaur => MINOTAUR,
        CreatureKind::Sheep => SHEEP,
        CreatureKind::Goat => GOAT,
        CreatureKind::Pig => PIG,
        CreatureKind::WaterBuffalo => WATER_BUFFALO,
        CreatureKind::Yak => YAK,
        CreatureKind::Donkey => DONKEY,
        CreatureKind::Mule => MULE,
        CreatureKind::Dromedary | CreatureKind::BactrianCamel => CAMEL,
        CreatureKind::Llama | CreatureKind::Alpaca => LLAMA,
        CreatureKind::Reindeer => REINDEER,
        CreatureKind::Rabbit => RABBIT,
        _ => super::bird_looks::bird_markings(kind),
    }
}
