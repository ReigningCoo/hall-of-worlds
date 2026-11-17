//! Item Pictures' Bottles page: every shape in clear glass with a cork, every
//! shade of glass, every stopper on two shapes, bottles as rolled, then potions:
//! each stat alone helping and harming, stronger and weaker, mixes, and liquids
//! in every shape and glass.

use bevy::prelude::*;
use shared::herb::Stat;
use shared::potion::{BottleForm, Contents, Dose};

use super::Shown;
use super::armour::spawn_row;
use crate::item_art::{BottleGlass, BottleLook, BottleShape, BottleStopper, LiquidLook};

/// A bottle of this shape, glass and stopper.
fn look(shape: BottleShape, glass: BottleGlass, stopper: BottleStopper) -> BottleLook {
    BottleLook {
        shape,
        glass,
        stopper,
    }
}

fn bottle(shape: BottleShape, glass: BottleGlass, stopper: BottleStopper) -> Shown {
    Shown::Bottle(look(shape, glass, stopper), None)
}

/// A round flask in clear glass holding these.
fn potion(doses: &[(Stat, i16)]) -> Shown {
    potion_in(
        look(
            BottleShape::RoundFlask,
            BottleGlass::Clear,
            BottleStopper::Cork,
        ),
        doses,
    )
}

fn potion_in(bottle: BottleLook, doses: &[(Stat, i16)]) -> Shown {
    let contents = Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }));
    Shown::Bottle(bottle, LiquidLook::of(&contents))
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
            let form = BottleForm {
                contents: Contents::EMPTY,
                look: row * 8 + n,
            };
            Shown::Bottle(BottleLook::of(&form), None)
        });
        spawn_row(grid, if row == 0 { "Rolled" } else { "" }, rolled);
    }
    spawn_potion_rows(grid);
}

/// Potions: what the liquid shows of what is in it.
fn spawn_potion_rows(grid: &mut ChildSpawnerCommands) {
    let (first, second) = Stat::ALL.split_at(6);
    for (label, step) in [("Helping", 6), ("Harming", -6)] {
        spawn_row(
            grid,
            label,
            first.iter().map(|&stat| potion(&[(stat, step)])),
        );
        spawn_row(grid, "", second.iter().map(|&stat| potion(&[(stat, step)])));
    }
    let steps = [-16, -8, -4, -2, 2, 4, 8, 16];
    spawn_row(
        grid,
        "Steps",
        steps.iter().map(|&step| potion(&[(Stat::Health, step)])),
    );
    use Stat::*;
    let mixes: [&[(Stat, i16)]; 8] = [
        &[(Health, 6), (Mana, 4)],
        &[(Intellect, 6), (Presence, 4)],
        &[(Toughness, 6), (Health, 4)],
        &[(Health, 6), (Will, -4)],
        &[(Strength, -6), (Mana, 4)],
        &[(Agility, 5), (Aura, 3)],
        &[(Perception, 4), (Empathy, -6)],
        &[(Toughness, -5), (Intellect, 4)],
    ];
    spawn_row(grid, "Mixes", mixes.iter().map(|doses| potion(doses)));
    let strongest: [&[(Stat, i16)]; 4] = [
        &[(Strength, 8), (Agility, 8), (Mana, 8), (Intellect, 8)],
        &[(Health, -8), (Will, -8), (Toughness, -8), (Aura, -8)],
        &[(Health, 8), (Strength, -8), (Intellect, 8), (Mana, -8)],
        &[(Empathy, 2), (Presence, 2), (Aura, 2), (Will, 2)],
    ];
    spawn_row(
        grid,
        "Strongest",
        strongest.iter().map(|doses| potion(doses)),
    );
    let mixed: &[(Stat, i16)] = &[(Mana, 6), (Intellect, 4), (Agility, 2)];
    let (first, second) = BottleShape::ALL.split_at(BottleShape::ALL.len() / 2);
    let in_shape = |&shape: &BottleShape| {
        potion_in(look(shape, BottleGlass::Clear, BottleStopper::Cork), mixed)
    };
    spawn_row(grid, "In shapes", first.iter().map(in_shape));
    spawn_row(grid, "", second.iter().map(in_shape));
    let in_glass = BottleGlass::ALL.map(|glass| {
        potion_in(
            look(BottleShape::Tall, glass, BottleStopper::Waxed),
            &[(Health, 6)],
        )
    });
    spawn_row(grid, "In glass", in_glass.into_iter());
}
