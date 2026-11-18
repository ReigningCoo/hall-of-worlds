//! `HerbTable`: every herb on the server being played on (`shared::herb`), as
//! it last said (`ServerMessage::Herbs`, after `Welcome`): the game's own until
//! then. Set by `connection`; for whatever shows herbs.

use bevy::prelude::*;
use shared::fruit::{FruitStage, PlantFamily};
use shared::herb::Herbs;
use shared::vegetable::PlantShape;

/// The herbs of the server being played on: its mods may add to or change the
/// game's own.
#[derive(Resource, Clone, Debug, Deref)]
pub struct HerbTable(pub Herbs);

impl Default for HerbTable {
    fn default() -> Self {
        Self(Herbs::built_in())
    }
}

/// The herb showcase (debug API `client.herb_showcase`): every herb drawn at
/// this stage in rows round the player, in place of the patches there; `None`
/// off.
#[derive(Resource, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct HerbShowcase(pub Option<u8>);

/// The fruit showcase (debug API `client.fruit_showcase`): every fruit
/// plant (of one family, if given) drawn at this stage in rows round the
/// player, in place of the plants there; `None` off.
#[derive(Resource, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct FruitShowcase(pub Option<(FruitStage, Option<PlantFamily>)>);

/// The vegetable showcase (debug API `client.vegetable_showcase`): a cell of
/// every vegetable (of one plant shape, if given) drawn at this stage (1 to
/// 4) in rows round the player, in place of the plants there; `None` off.
#[derive(Resource, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct VegetableShowcase(pub Option<(u8, Option<PlantShape>)>);
