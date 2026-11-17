//! A page per built-in herb: its picture, names, rarity, where it grows, the
//! part used, its weight, its plant, and how it grows and is picked. Never its
//! effects: in the game those are learnt by playing.

use shared::climate::Biome;
use shared::herb::{EARLY_STAGE, Growth, HerbDef, Herbs, MARSH_STAND_INS, Regrow, STAGES};

/// Every herb's page, by file name, in name order (their order in the menu).
pub(super) fn pages(herbs: &Herbs) -> Vec<(String, String)> {
    sorted(herbs)
        .into_iter()
        .enumerate()
        .map(|(at, herb)| (format!("{}.md", file_name(&herb.key)), page(herb, at + 1)))
        .collect()
}

/// The herbs in name order, as the menu and the Herbs page's table list them.
pub(super) fn sorted(herbs: &Herbs) -> Vec<&HerbDef> {
    let mut sorted: Vec<_> = herbs.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));
    sorted
}

/// A herb's page's name, without `.md`: its key with hyphens (`labrador-tea`).
pub(super) fn file_name(key: &str) -> String {
    key.replace('_', "-")
}

fn page(herb: &HerbDef, order: usize) -> String {
    let name = &herb.name;
    let key = &herb.key;
    let plant = &herb.plant;
    let rows = [
        ("Rarity", format!("{:?}", herb.abundance)),
        ("Grows in", lands(herb)),
        ("Warmth", warmth(herb)),
        ("Part used", herb.part_name.clone()),
        ("Weight", format!("{} g each", herb.grams)),
        (
            "Plant",
            format!(
                "{}, about {} cm tall and {} cm across",
                plant.form, plant.height, plant.spread
            ),
        ),
        ("Growth", growth(herb.growth)),
        ("Picking", picking(herb.regrow)),
        ("Picked early", early(herb)),
    ];
    let table: String = rows
        .iter()
        .map(|(what, says)| format!("| {what} | {says} |\n"))
        .collect();
    format!(
        "---
title: {name}
nav_order: {order}
parent: Herbs
grand_parent: Items
---

<!-- Written by `cargo xtask wiki` from the game's herbs: change them, not this page. -->

# {name}

![{name}]({{{{ '/assets/herbs/{key}.png' | relative_url }}}})

*{latin}*. {description} Back to [Herbs](../herbs).

| | |
| - | - |
{table}",
        latin = herb.latin,
        description = plant.description,
    )
}

/// The lands it grows in wild, as the Herbs page names them.
fn lands(herb: &HerbDef) -> String {
    let named = |biomes: &[Biome]| {
        biomes
            .iter()
            .map(|&biome| land_name(biome))
            .collect::<Vec<_>>()
            .join(", ")
    };
    match (herb.biomes.is_empty(), herb.marsh) {
        (true, _) => format!(
            "Marshes; until the world has marshes, the wettest lands: {}",
            named(&MARSH_STAND_INS)
        ),
        (false, true) => format!(
            "{}; marshes too, once the world has them",
            named(&herb.biomes)
        ),
        (false, false) => named(&herb.biomes),
    }
}

/// A land's name, as the Herbs page has it.
pub(super) fn land_name(biome: Biome) -> &'static str {
    match biome {
        Biome::Tundra => "Tundra",
        Biome::BorealForest => "Boreal forest",
        Biome::Grassland => "Grassland",
        Biome::Woodland => "Woodland",
        Biome::SeasonalForest => "Seasonal forest",
        Biome::TemperateRainforest => "Temperate rainforest",
        Biome::Desert => "Desert",
        Biome::Savanna => "Savanna",
        Biome::TropicalRainforest => "Tropical rainforest",
    }
}

fn warmth(herb: &HerbDef) -> String {
    let degrees = |celsius: i16| {
        // A real minus sign reads better than a hyphen.
        if celsius < 0 {
            format!("−{}", -celsius)
        } else {
            celsius.to_string()
        }
    };
    format!(
        "Lands whose year averages {} to {} °C",
        degrees(herb.temperature.min),
        degrees(herb.temperature.max)
    )
}

fn growth(growth: Growth) -> String {
    let speed = match growth {
        Growth::Fast => "Fast",
        Growth::Medium => "Medium",
        Growth::Slow => "Slow",
        Growth::VerySlow => "Very slow",
    };
    format!("{speed}: full grown in {} days of game time", growth.days())
}

fn picking(regrow: Regrow) -> String {
    match regrow {
        Regrow::Cut => "Cut: it grows again from its second stage".into(),
        Regrow::Pick => "Picked: it grows again from its third stage".into(),
        Regrow::Dig => {
            "Dug up: nothing grows there for a whole growth, then it starts again from a shoot"
                .into()
        }
        Regrow::Coppice => "Coppiced: cut to the ground, it grows again from a shoot".into(),
    }
}

fn early(herb: &HerbDef) -> String {
    match herb.early.taken {
        0 => "No".into(),
        100 => format!("Yes, from stage {EARLY_STAGE} of {STAGES}, for a full picking"),
        taken => {
            format!("Yes, from stage {EARLY_STAGE} of {STAGES}, for {taken}% of a full picking")
        }
    }
}
