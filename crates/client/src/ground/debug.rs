//! Looking at the bags (`dev_tools`).

use bevy::prelude::*;
use shared::cask::{CaskLook, CaskSize, WOODS};
use shared::ground::Load;
use shared::material::Material;
use shared::spot::Spot;

use super::{BagAssets, Bags, set_bag};
use crate::common::MapPosition;
use crate::player::Player;

/// For looking at the bags (`dev_tools`).
#[derive(Message, Clone, Copy, Debug)]
pub enum GroundDebug {
    /// A row of every look (a cask of each size, and piles of 1, 2 and 3
    /// logs), a cell apart, starting from the player's cell's middle and
    /// going east.
    ShowLooks,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn run_debug(
    mut asked: MessageReader<GroundDebug>,
    mut assets: ResMut<BagAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    player: Single<&MapPosition, With<Player>>,
    mut bags: ResMut<Bags>,
    mut commands: Commands,
) {
    for GroundDebug::ShowLooks in asked.read() {
        let casks = CaskSize::ALL.map(|size| {
            Load::Cask(CaskLook {
                size,
                wood: WOODS[size as usize % WOODS.len()],
            })
        });
        let logs = [
            (Material::Oak, 1),
            (Material::Birch, 2),
            (Material::Pine, 3),
        ]
        .map(|(wood, count)| Load::Logs { wood, count });
        let looks = Load::ALL.into_iter().chain(casks).chain(logs);
        for (index, load) in looks.enumerate() {
            let at = Spot::middle_of((player.x + index as i64, player.y));
            super::assets::ensure_meshes(&mut assets, &mut meshes, load);
            set_bag(&mut bags, &mut commands, &assets, at, load);
        }
    }
}
