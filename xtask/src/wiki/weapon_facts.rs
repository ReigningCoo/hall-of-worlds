//! What a named weapon is like in the game, found by making it many times the
//! way a shop would (`commission_blade`): ordinary size, Common, a steel blade,
//! fixed seeds. Only those that come out with its own name count, so the facts
//! are of weapons called that. The first one is the one pictured.

use shared::blade::{BladeForm, Curve, Edges, Guard, Hands, WeaponName, commission_blade};
use shared::creature::Size;
use shared::id::Id;
use shared::item::{Item, Quality};
use shared::item_form::{ItemForm, Limits};
use shared::material::Material;
use shared::rng::Seed;

/// How many are made of each weapon to find what it is like.
const MADE: u64 = 200;
/// Where the seeds start, so the pictures stay the same from run to run.
const FIRST_SEED: u64 = 0x5EED_B1AD;

/// What a weapon called this turns out like, most often first.
pub(super) struct Facts {
    pub hands: Vec<Hands>,
    pub edges: Vec<Edges>,
    pub curves: Vec<Curve>,
    pub guards: Vec<Guard>,
    /// The middle weight, in grams.
    pub grams: u32,
    /// Its cells, across by along, each once, smallest first.
    pub shapes: Vec<(u8, u8)>,
    /// The one pictured.
    pub example: Item,
}

/// Makes the weapon `MADE` times and gathers what those named it are like.
pub(super) fn facts(weapon: &WeaponName) -> Facts {
    let limits = Limits {
        metal: Some(Material::Steel),
        ..Limits::NONE
    };
    let made: Vec<BladeForm> = (0..MADE)
        .filter_map(|n| {
            let mut rng = Seed::new(FIRST_SEED + n).rng();
            commission_blade(
                &mut rng,
                weapon.name,
                Quality::Common,
                Size::Medium,
                &limits,
            )
        })
        .map(|rolled| rolled.form)
        .filter(|form| form.name() == weapon.name)
        .collect();
    let first = *made
        .first()
        .unwrap_or_else(|| panic!("no {} made comes out with its name", weapon.name));
    let mut grams: Vec<u32> = made.iter().map(BladeForm::grams).collect();
    grams.sort_unstable();
    let mut shapes: Vec<(u8, u8)> = made
        .iter()
        .map(|form| {
            let shape = form.shape();
            (shape.width, shape.height)
        })
        .collect();
    shapes.sort_unstable();
    shapes.dedup();
    let mut example = Item::new(Id::for_test(1), weapon.family.kind());
    example.quality = Some(Quality::Common);
    example.form = Some(ItemForm::Blade(Box::new(first)));
    Facts {
        hands: most_often(made.iter().map(|form| form.hands)),
        edges: most_often(made.iter().map(|form| form.edges)),
        curves: most_often(made.iter().map(|form| form.curve)),
        guards: most_often(made.iter().map(|form| form.guard)),
        grams: grams[grams.len() / 2],
        shapes,
        example,
    }
}

/// Each value once, the most often seen first; ties in the order first seen.
fn most_often<T: PartialEq + Copy>(values: impl Iterator<Item = T>) -> Vec<T> {
    let mut counted: Vec<(T, usize)> = Vec::new();
    for value in values {
        match counted.iter_mut().find(|(seen, _)| *seen == value) {
            Some((_, count)) => *count += 1,
            None => counted.push((value, 1)),
        }
    }
    counted.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    counted.into_iter().map(|(value, _)| value).collect()
}
