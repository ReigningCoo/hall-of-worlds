//! Item Pictures' Bottles page: every shape in clear glass with a cork, every
//! shade of glass, every stopper on two shapes, then bottles as rolled.

use bevy::prelude::*;
use shared::potion::{BottleForm, Contents};

use super::Shown;
use super::armour::spawn_row;
use crate::item_art::{BottleGlass, BottleLook, BottleShape, BottleStopper};

/// A bottle of this shape, glass and stopper.
fn bottle(shape: BottleShape, glass: BottleGlass, stopper: BottleStopper) -> Shown {
    Shown::Bottle(BottleLook {
        shape,
        glass,
        stopper,
    })
}

/// The rows of the Bottles page.
pub(super) fn spawn_bottle_rows(grid: &mut ChildSpawnerCommands) {
    let plain = |shape| bottle(shape, BottleGlass::Clear, BottleStopper::Cork);
    let (first, second) = BottleShape::ALL.split_at(BottleShape::ALL.len() / 2);
    spawn_row(grid, "Shapes", first.iter().map(|&shape| plain(shape)));
    spawn_row(grid, "", second.iter().map(|&shape| plain(shape)));
    let glass = BottleGlass::ALL;
    let shades = glass.map(|glass| bottle(BottleShape::RoundFlask, glass, BottleStopper::Cork));
    spawn_row(grid, "Glass", shades.into_iter());
    let stoppers = [BottleShape::Vial, BottleShape::Jar]
        .into_iter()
        .flat_map(|shape| {
            BottleStopper::ALL.map(|stopper| bottle(shape, BottleGlass::PaleGreen, stopper))
        });
    spawn_row(grid, "Stoppers", stoppers);
    for row in 0..2u64 {
        let rolled = (0..8).map(move |n| {
            Shown::Bottle(BottleLook::of(&BottleForm {
                contents: Contents::EMPTY,
                look: row * 8 + n,
            }))
        });
        spawn_row(grid, if row == 0 { "Rolled" } else { "" }, rolled);
    }
}
