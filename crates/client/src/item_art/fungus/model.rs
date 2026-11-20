//! One fungus as it grows (`fungus_one`): its pieces built by its look's form,
//! about a unit across, its foot at the middle and up along +y. The item
//! pictures lay these out; the maps' patches can stand them up as they are.

use bevy::prelude::*;
use shared::fungus::{Form, FungusDef};

use super::paint::Colours;
use super::variety::Variety;
use super::{ball, bracket, cap, club, coral, cup, funnel, jelly, lichen, lump, morel};

/// One piece of a fungus's model: its mesh (with vertex colours, for a white
/// material), where it is, and whether it is glossy (jelly).
pub struct FungusPiece {
    pub mesh: Mesh,
    pub transform: Transform,
    pub shiny: bool,
}

impl FungusPiece {
    pub(super) fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
            transform: Transform::IDENTITY,
            shiny: false,
        }
    }

    pub(super) fn at(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub(super) fn shiny(mut self) -> Self {
        self.shiny = true;
        self
    }

    /// The piece moved by `transform` after its own.
    pub(super) fn moved(mut self, transform: Transform) -> Self {
        self.transform = transform * self.transform;
        self
    }
}

/// What every form's builder is handed.
pub(super) struct Build<'a> {
    pub(super) def: &'a FungusDef,
    pub(super) colours: Colours,
    pub(super) variety: &'a Variety,
}

/// One of `def` as it grows, in variant `variant`: a mushroom, a bracket, a
/// clump of lichen.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the maps' fungus patches (fungi plan PR 4) will stand it up"
    )
)]
pub fn fungus_one(def: &FungusDef, variant: u8) -> Vec<FungusPiece> {
    let variety = Variety::of(variant);
    let build = Build {
        def,
        colours: Colours::of(&def.look, def.group, variety.shade),
        variety: &variety,
    };
    one(&build, 0)
}

/// The `n`th of a handful, a little different from the others.
pub(super) fn one(build: &Build, n: usize) -> Vec<FungusPiece> {
    match build.def.look.form {
        Form::CapAndStem => cap::cap_and_stem(build, n),
        Form::Funnel => funnel::funnel(build, n),
        Form::Bracket => bracket::shelf(build, n, 1.0),
        Form::Ball => ball::ball(build, n),
        Form::Coral => coral::coral(build, n),
        Form::Club => club::clubs(build, n),
        Form::Cup => cup::cup(build, n),
        Form::Jelly => jelly::jelly(build, n),
        Form::Honeycomb => morel::morel(build, n),
        Form::Truffle => lump::lump(build, n),
        Form::Crust => lump::crust(build, n),
        Form::LichenTufts => lichen::tufts(build, n),
        Form::LichenBeard => lichen::beard(build, n),
        Form::LichenPlates => lichen::plates(build, n),
    }
}
