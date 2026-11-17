//! A page per named weapon: its picture, what sets it apart, and a table of its
//! lengths, hands, edges, curve, guard, weight and size in a pack.

use shared::blade::{Build, Curve, Edges, Guard, Hands, WeaponName};

use super::weapon_facts::Facts;
use super::weapon_family::FamilyPage;

/// A weapon's page's file name, without `.md`, and its picture's: its name in
/// lower case, hyphens for spaces and plain letters for accented ones.
pub(super) fn file_name(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .replace('ä', "a")
        .replace('ö', "o")
        .replace('ü', "u")
}

pub(super) fn page(
    weapon: &WeaponName,
    family: &FamilyPage,
    facts: &Facts,
    line: &str,
    order: usize,
) -> String {
    let name = weapon.name;
    let picture = file_name(name);
    let mut rows = vec![
        (blade_word(weapon.build), centimetres(weapon.blade)),
        ("Handle", centimetres(weapon.handle)),
        ("Hands", either(&facts.hands, hands)),
        ("Edges", either(&facts.edges, edges)),
    ];
    // A head at the side has no curve or guard to speak of.
    if weapon.build == Build::Along {
        rows.push(("Curve", either(&facts.curves, curve)));
        rows.push(("Guard", either(&facts.guards, guard)));
    }
    rows.push(("Weight", weight(facts.grams)));
    rows.push(("Takes up", cells(&facts.shapes)));
    let table: String = rows
        .iter()
        .map(|(what, says)| format!("| {what} | {says} |\n"))
        .collect();
    format!(
        "---
title: {name}
nav_order: {order}
parent: {family_title}
grand_parent: Weapons
---

<!-- Written by `cargo xtask wiki` from the game's weapons: change them, not this page. -->

# {name}

![{name}]({{{{ '/assets/weapons/{picture}.png' | relative_url }}}})

{line} One of the [{family_lower}](./). No two are quite alike: this is what one usually is.

| | |
| - | - |
{table}",
        family_title = family.title,
        family_lower = family.title.to_lowercase(),
    )
}

/// What the length along the weapon's working part is called.
pub(super) fn blade_word(build: Build) -> &'static str {
    match build {
        Build::Along => "Blade",
        Build::Side => "Edge of the head",
    }
}

/// A span of millimetres as whole centimetres, rounded.
pub(super) fn centimetres((from, to): (u16, u16)) -> String {
    let from = (from + 5) / 10;
    let to = (to + 5) / 10;
    if from >= to {
        format!("About {from} cm")
    } else {
        format!("{from} to {to} cm")
    }
}

/// The values seen, most often first: "One edge", or "Mostly one edge; some
/// two edges".
fn either<T: Copy>(seen: &[T], word: fn(T) -> &'static str) -> String {
    let words: Vec<&str> = seen.iter().map(|&value| word(value)).collect();
    let text = match words.as_slice() {
        [] => "none".into(),
        [only] => (*only).into(),
        [first, rest @ ..] => format!("mostly {first}; some {}", rest.join("; some ")),
    };
    let mut letters = text.chars();
    letters
        .next()
        .map(|first| first.to_uppercase().chain(letters).collect())
        .unwrap_or_default()
}

fn hands(hands: Hands) -> &'static str {
    match hands {
        Hands::One => "one hand",
        Hands::OneOrTwo => "one hand or two",
        Hands::Two => "two hands",
    }
}

fn edges(edges: Edges) -> &'static str {
    match edges {
        Edges::Single => "one edge",
        Edges::Double => "two edges",
        Edges::PointOnly => "only a point",
    }
}

fn curve(curve: Curve) -> &'static str {
    match curve {
        Curve::Straight => "straight",
        Curve::Slight => "slightly curved",
        Curve::Strong => "strongly curved",
        Curve::Inward => "bent forward",
        Curve::Wavy => "wavy",
    }
}

fn guard(guard: Guard) -> &'static str {
    match guard {
        Guard::None => "no guard",
        Guard::Cross => "a cross guard",
        Guard::Disc => "a disc",
        Guard::Cup => "a cup",
        Guard::KnuckleBow => "a bow over the knuckles",
        Guard::Basket => "a basket",
    }
}

/// A weight rounded as the wiki gives them, saying what changes it.
pub(super) fn weight(grams: u32) -> String {
    let about = if grams < 1000 {
        format!("{} g", (grams + 25) / 50 * 50)
    } else if grams < 10_000 {
        let tenths = (grams + 50) / 100;
        format!("{}.{} kg", tenths / 10, tenths % 10)
    } else {
        format!("{} kg", (grams + 500) / 1000)
    };
    format!("About {about} with a steel blade; its size and what it is made of change it")
}

/// The cells it takes in a pack, across by along: the fewest to the most.
pub(super) fn cells(shapes: &[(u8, u8)]) -> String {
    let cell = |(w, h): &(u8, u8)| format!("{w} × {h}");
    match shapes {
        [only] => format!("{} cells", cell(only)),
        [first, .., last] => format!("{} to {} cells", cell(first), cell(last)),
        [] => "Nothing".into(),
    }
}
