//! What a mob starts with: its kind's chances and its own rolls.

use super::Seed;
use crate::creature::CreatureKind;
use crate::item::{Drop, ItemKind, Quality, roll_drop};

/// Where the rolls for a mob's gear start, clear of those for its place and looks.
const GEAR_ROLLS: i64 = 1000;

/// What a kind of creature may start with: for each chance, in 100, of having
/// something there, the kinds it could be and how much each weighs. First guesses.
/// Fruit and vegetables are what it foraged there (`shared::produce`, by the server).
pub(super) fn gear_of(kind: CreatureKind) -> &'static [(u64, &'static [(ItemKind, u64)])] {
    use ItemKind::{
        Axe, Belt, Boomerang, Boots, Cloak, Dagger, Fruit, FullArmour, Gloves, Hat, Knife,
        Longsword, Pouch, Sandals, Shirt, Shoes, ShortSword, Socks, Spear, Stone, Sword, Trousers,
        Vegetable,
    };
    match kind {
        CreatureKind::Beggar => &[
            (100, &[(Shirt, 1)]),
            (100, &[(Trousers, 1)]),
            (50, &[(Shoes, 2), (Sandals, 1), (Socks, 1), (Hat, 1)]),
            (40, &[(Fruit, 1), (Vegetable, 1), (Stone, 1)]),
        ],
        CreatureKind::Giant => &[
            (100, &[(Trousers, 1)]),
            (60, &[(Belt, 1), (Boots, 2), (Cloak, 1)]),
            (20, &[(FullArmour, 1)]),
            (40, &[(Sword, 2), (Longsword, 1), (Axe, 2), (Spear, 1)]),
            (30, &[(Stone, 2), (Pouch, 1)]),
        ],
        CreatureKind::Kobold => &[
            (50, &[(Shirt, 1), (Trousers, 1)]),
            // On their toes, so no boots.
            (30, &[(Gloves, 1), (Hat, 1)]),
            (
                70,
                &[(Knife, 1), (Dagger, 2), (ShortSword, 1), (Boomerang, 2)],
            ),
            (30, &[(Stone, 2), (Fruit, 1), (Vegetable, 1), (Pouch, 1)]),
        ],
        CreatureKind::Goblin => &[
            (50, &[(Shirt, 1), (Trousers, 1)]),
            (30, &[(Gloves, 1), (Boots, 1), (Hat, 1)]),
            (70, &[(Knife, 2), (Dagger, 2), (ShortSword, 1)]),
            (30, &[(Stone, 2), (Fruit, 1), (Vegetable, 1), (Pouch, 1)]),
        ],
        CreatureKind::Orc => &[
            (100, &[(Trousers, 1)]),
            (60, &[(Belt, 1), (Boots, 2), (Cloak, 1)]),
            (70, &[(Axe, 2), (Sword, 2), (Spear, 1)]),
            (30, &[(Stone, 1), (Pouch, 1)]),
        ],
        _ => &[],
    }
}

/// Rolls a mob's gear, each place on its own so changing one moves no others.
pub(super) fn roll_gear(
    roll: &impl Fn(i64) -> u64,
    kind: CreatureKind,
) -> Vec<(ItemKind, Option<Quality>)> {
    let places = gear_of(kind).iter().zip(GEAR_ROLLS..);
    places
        .filter_map(|(&(chance, kinds), place)| {
            let mut rng = Seed::new(roll(place)).rng();
            if rng.range_u64(0, 100) >= chance {
                return None;
            }
            let table: Vec<Drop> = Quality::ALL
                .iter()
                .flat_map(|&quality| {
                    kinds.iter().map(move |&(kind, weight)| Drop {
                        quality,
                        kind,
                        weight,
                    })
                })
                .collect();
            roll_drop(&mut rng, &table)
        })
        .collect()
}
