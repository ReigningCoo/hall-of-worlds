//! The rows of a fruit's or nut's facts table, in plain words: how it comes,
//! what eating it does, its plant and how it grows and is picked. Few fine
//! numbers, as the game may change them soon.

use shared::fruit::{Abundance, FruitDef, FruitForm, FruitSet, HarvestTier, PlantFamily, Unit};
use shared::herb::{Effect, Growth, Stat};

use super::herb_page::land_name;

/// Every row, as (what, says), in the table's order.
pub(super) fn rows(def: &FruitDef) -> Vec<(&'static str, String)> {
    let mut rows = vec![
        ("Kind", kind(def).into()),
        ("Rarity", rarity(def.abundance).into()),
        ("Grows in", lands(def)),
        ("Warmth", warmth(def)),
        ("Comes as", comes_as(def)),
        ("Weight", weight(def)),
        ("Eating", eating(def)),
        ("Effects", effects(&def.effects)),
        ("Plant", plant(def)),
        ("Growth", growth(def.growth())),
        ("Picking", picking(def)),
    ];
    if def.plant.has_trunk() {
        rows.push(("Trunk", "In the way, as a tree's is".into()));
    }
    rows
}

/// "Fruit" or "Nut".
pub(super) fn kind(def: &FruitDef) -> &'static str {
    match def.set {
        FruitSet::Fruit => "Fruit",
        FruitSet::Nut => "Nut",
    }
}

fn rarity(abundance: Abundance) -> &'static str {
    match abundance {
        Abundance::Common => "Common",
        Abundance::Uncommon => "Uncommon",
        Abundance::Rare => "Rare",
        Abundance::OrchardOnly => "Grown only in orchards",
    }
}

fn lands(def: &FruitDef) -> String {
    let lands: Vec<&str> = def.biomes.iter().map(|&biome| land_name(biome)).collect();
    match def.abundance {
        Abundance::OrchardOnly => format!(
            "Orchards, where it is warm enough; it would suit {}",
            lands.join(", ")
        ),
        _ => format!(
            "{}; orchards too, where it is warm enough",
            lands.join(", ")
        ),
    }
}

fn warmth(def: &FruitDef) -> String {
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
        degrees(def.temperature.min),
        degrees(def.temperature.max)
    )
}

fn comes_as(def: &FruitDef) -> String {
    match def.unit {
        Unit::One => format!("One {}", kind(def).to_lowercase()),
        Unit::Handful => "A handful".into(),
        Unit::Whole => format!(
            "A whole fruit, cut into {} servings when you eat it",
            def.servings
        ),
    }
}

fn weight(def: &FruitDef) -> String {
    let each = match def.unit {
        Unit::One => "each",
        Unit::Handful => "a handful",
        Unit::Whole => "whole",
    };
    match def.grams {
        grams if grams >= 1000 => {
            let kg = format!("{:.1}", f64::from(grams) / 1000.0);
            format!("About {} kg {each}", kg.trim_end_matches(".0"))
        }
        grams => format!("{grams} g {each}"),
    }
}

fn eating(def: &FruitDef) -> String {
    let serving = match def.unit {
        Unit::Whole => "A serving",
        _ => "It",
    };
    if def.harmful_raw {
        return "Harmful raw: eaten as it is, it gives back no Health".into();
    }
    let amount = match shared::fruit::healing(def) {
        0..10 => "a little Health",
        10..25 => "some Health",
        _ => "plenty of Health",
    };
    let longer = match def.exceptional {
        true => ", over longer than most",
        false => "",
    };
    format!("{serving} gives back {amount}{longer}, until you move or are hurt")
}

/// Its effects as the item's popup shows them, in words.
pub(super) fn effects(effects: &[Effect]) -> String {
    if effects.is_empty() {
        return "None".into();
    }
    let said: Vec<String> = effects
        .iter()
        .map(|effect| {
            let stat = format!("{:?}", effect.stat);
            match (effect.stat, effect.raises(), effect.strength()) {
                (Stat::Health | Stat::Mana, true, _) => format!("Restores {stat} for a while"),
                (Stat::Health | Stat::Mana, false, _) => format!("Drains {stat} for a while"),
                (_, true, 1) => format!("Raises {stat} a little for a while"),
                (_, true, _) => format!("Raises {stat} for a while"),
                (_, false, 1) => format!("Lowers {stat} a little for a while"),
                (_, false, _) => format!("Lowers {stat} for a while"),
            }
        })
        .collect();
    said.join("; ")
}

fn plant(def: &FruitDef) -> String {
    let look = &def.plant_look;
    let size = |cm: u16| match cm {
        cm if cm >= 200 => format!("{} m", (cm + 50) / 100),
        cm => format!("{cm} cm"),
    };
    format!(
        "{}, about {} tall and {} across",
        family(def.plant),
        size(look.height),
        size(look.spread)
    )
}

fn family(family: PlantFamily) -> &'static str {
    match family {
        PlantFamily::BroadleafTree => "A tree",
        PlantFamily::Conifer => "A conifer",
        PlantFamily::Palm => "A palm",
        PlantFamily::Shrub => "A shrub",
        PlantFamily::Canes => "Canes",
        PlantFamily::Vine => "A climbing vine",
        PlantFamily::GroundVine => "A vine along the ground",
        PlantFamily::LowMat => "A low mat",
        PlantFamily::LowHerb => "A low plant",
        PlantFamily::GiantHerb => "A giant herb",
        PlantFamily::Rosette => "A rosette of leaves",
        PlantFamily::PadCactus => "A cactus of flat pads",
        PlantFamily::ColumnCactus => "A column cactus",
    }
}

fn growth(growth: Growth) -> String {
    let speed = match growth {
        Growth::Fast => "Fast",
        Growth::Medium => "Medium",
        Growth::Slow => "Slow",
        Growth::VerySlow => "Very slow",
    };
    format!(
        "{speed}: leaves, flowers and ripe fruit come round every {} days of game time",
        growth.days()
    )
}

fn picking(def: &FruitDef) -> String {
    let many = match def.harvest {
        HarvestTier::One => "One",
        HarvestTier::UpToTwo => "One or two",
        HarvestTier::UpToThree => "One to three",
        HarvestTier::TwoToFour => "Two to four",
        HarvestTier::ThreeToSix => "Three to six",
    };
    let whole = match def.unit {
        Unit::Whole => " (each a whole fruit)",
        _ => "",
    };
    format!("{many} when ripe{whole}, more with Foraging and Orchards")
}

/// A picture's pixels along its longest side: 128 a cell.
pub(super) fn pixels(def: &FruitDef, serving: bool) -> u32 {
    let servings = u32::from(def.servings.max(1));
    let form = FruitForm {
        fruit: def.key.clone(),
        serving,
        look: 0,
        grams: match serving {
            true => def.grams.div_ceil(servings),
            false => def.grams,
        },
    };
    let shape = form.shape();
    128 * u32::from(shape.width.max(shape.height))
}
