//! Trees flash red as blows land on them (woodcutting plan PR 4, owner): every
//! blow that lands is told to everyone who sees the tree
//! (`ServerMessage::TreeChopped`), the feller too, and its coloured copy
//! (`trees::TreeFill`) is drawn red for a moment, as tokens flash when harmed.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::protocol::ServerMessage;

use super::herbs::HerbMaterials;
use super::sight_lines::SurfaceMaterial;
use super::trees::{DrawnTrees, TreeFill, TreeMaterials};
use crate::connection::FromServer;

/// How long a flash lasts, in seconds: a little longer than a token's, as a
/// blow comes only every two seconds.
const FLASH_SECONDS: f32 = 0.15;

/// Cells of trees.
type Cells = Vec<(i64, i64)>;

/// The trees flashing, by cell, with the seconds left.
#[derive(Resource, Default, Debug)]
pub(super) struct TreeFlashes(HashMap<(i64, i64), f32>);

impl TreeFlashes {
    /// A blow landed on the tree at `cell`: its flash starts again.
    fn chopped(&mut self, cell: (i64, i64)) {
        self.0.insert(cell, FLASH_SECONDS);
    }

    /// Counts `seconds` off: the cells to draw red this frame, each at least
    /// once however long the frame, and those whose flash has ended.
    fn tick(&mut self, seconds: f32) -> (Cells, Cells) {
        let (mut red, mut ended) = (Vec::new(), Vec::new());
        for (&cell, left) in &mut self.0 {
            match *left > 0.0 {
                true => red.push(cell),
                false => ended.push(cell),
            }
            *left -= seconds;
        }
        for cell in &ended {
            self.0.remove(cell);
        }
        (red, ended)
    }
}

/// Starts a flash for each blow the server tells of; a new level forgets them.
pub(super) fn take_chops(
    mut from_server: MessageReader<FromServer>,
    mut flashes: ResMut<TreeFlashes>,
) {
    for FromServer(message) in from_server.read() {
        match message {
            ServerMessage::TreeChopped { x, y, .. } => flashes.chopped((*x, *y)),
            ServerMessage::EnterLevel(_) | ServerMessage::Welcome { .. } => flashes.0.clear(),
            _ => {}
        }
    }
}

/// Draws each flashing tree's coloured copy red, and back as usual once over.
pub(super) fn flash_trees(
    time: Res<Time>,
    mut flashes: ResMut<TreeFlashes>,
    drawn: Res<DrawnTrees>,
    (lit, trees): (Res<HerbMaterials>, Res<TreeMaterials>),
    children: Query<&Children>,
    mut fills: Query<&mut MeshMaterial3d<SurfaceMaterial>, With<TreeFill>>,
) {
    let (red, ended) = flashes.tick(time.delta_secs());
    let mut paint = |cell: (i64, i64), material: &Handle<SurfaceMaterial>| {
        let Some(tree) = drawn.at(cell) else {
            return;
        };
        for child in children.get(tree).into_iter().flatten() {
            if let Ok(mut now) = fills.get_mut(*child)
                && now.0 != *material
            {
                now.0 = material.clone();
            }
        }
    };
    for cell in red {
        paint(cell, &trees.flash);
    }
    for cell in ended {
        paint(cell, &lit.fill);
    }
}

#[cfg(test)]
mod tests;
