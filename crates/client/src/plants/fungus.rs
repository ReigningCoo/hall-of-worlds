//! Fungus patches' models (fungi plan PR 4): a cell's worth of one fungus by
//! its look's form (`patch.rs`), at each of its four stages, in a few variants,
//! from `item_art`'s fungus meshes stood up as they are; with black outlines
//! and a grey copy for memory. Built once per fungus, stage and variant.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::fungus::Fungi;

mod ground;
mod patch;
mod shell;

/// A patch model's meshes: its colours, its black outline (drawn inside out),
/// and grey for where it is remembered.
#[derive(Clone, Debug)]
pub struct FungusMeshes {
    pub fill: Handle<Mesh>,
    pub outline: Handle<Mesh>,
    pub memory: Handle<Mesh>,
}

/// Fungus patch models built so far, by fungus key, stage and variant; `None`
/// for a fungus the table lacks.
#[derive(Resource, Default)]
pub struct FungusModels {
    built: HashMap<(String, u8, u8), Option<FungusMeshes>>,
}

impl FungusModels {
    /// The patch of fungus `key` at `stage` (1 to 4) in `variant` (any: taken
    /// modulo `VARIANTS`), built if it hasn't been.
    pub fn get(
        &mut self,
        key: &str,
        stage: u8,
        variant: u8,
        meshes: &mut Assets<Mesh>,
    ) -> Option<FungusMeshes> {
        let variant = variant % super::models::VARIANTS;
        self.built
            .entry((key.to_owned(), stage, variant))
            .or_insert_with(|| {
                let def = Fungi::built_in().get(key)?;
                let shell = patch::build(def, stage, variant);
                (!shell.is_empty()).then(|| FungusMeshes {
                    fill: meshes.add(shell.fill()),
                    outline: meshes.add(shell.outline(patch::outline_share(def))),
                    memory: meshes.add(shell.memory()),
                })
            })
            .clone()
    }
}

#[cfg(test)]
mod tests;
