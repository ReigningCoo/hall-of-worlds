//! The rows of a mushroom's, fungus's or lichen's facts table, in plain words:
//! what sort it is, where and on what it grows, how it comes and what eating it
//! raw does. Only what its popup would tell: never a poison or its uses like a
//! herb, which a player learns by playing.

use shared::fungus::{FungusDef, FungusForm, Group, Niche, Unit, Where};
use shared::herb::Growth;
use shared::vegetable::Abundance;

use super::fruit_facts::effects;
use super::herb_page::{land_name, picking as regrowing};

/// Every row, as (what, says), in the table's order.
pub(super) fn rows(def: &FungusDef) -> Vec<(&'static str, String)> {
    vec![
        ("Kind", kind(def.group).into()),
        ("Rarity", rarity(def.abundance).into()),
        ("Grows", grows(def)),
        ("Lands", lands(def)),
        ("Warmth", warmth(def)),
        ("Size", size(def)),
        ("Comes as", comes_as(def)),
        ("Weight", weight(def)),
        ("Eating", eating(def)),
        ("Effects", raw_effects(def)),
        ("Growth", growth(def.growth)),
        ("After picking", regrowing(def.regrow)),
    ]
}

/// What sort of fungus it is.
pub(super) fn kind(group: Group) -> &'static str {
    match group {
        Group::Mushroom => "Mushroom",
        Group::Bracket => "Bracket fungus: a shelf or hoof on wood",
        Group::Jelly => "Jelly fungus",
        Group::Truffle => "Truffle: a lump grown under the ground",
        Group::Sclerotium => "Sclerotium: a hard buried lump",
        Group::Cup => "Cup fungus",
        Group::Crust => "Crust fungus: a skin spreading over wood",
        Group::Lichen => "Lichen: a fungus living with algae",
    }
}

fn rarity(abundance: Abundance) -> &'static str {
    match abundance {
        Abundance::Common => "Common",
        Abundance::Uncommon => "Uncommon",
        Abundance::Rare => "Rare",
        Abundance::VeryRare => "Very rare",
    }
}

fn grows(def: &FungusDef) -> String {
    let place = match def.grows {
        Where::Deep => "Underground",
        Where::Dim => "Underground, near daylight",
        Where::Dug => "Buried in the earth: dug up",
        Where::Surface => "Above ground",
    };
    let on: Vec<&str> = def.niches.iter().map(|&niche| niche_name(niche)).collect();
    format!("{place}; {}", on.join(", "))
}

fn niche_name(niche: Niche) -> &'static str {
    match niche {
        Niche::Wood => "on old wood",
        Niche::Dung => "on dung",
        Niche::Soil => "on damp earth",
        Niche::Remains => "on bones and remains",
        Niche::Water => "by water",
        Niche::Lava => "beside lava",
        Niche::Trees => "on living trees",
        Niche::Roots => "by tree roots",
        Niche::Rock => "on rock",
        Niche::Insects => "on buried insects",
        Niche::Termites => "on termites' nests",
    }
}

fn lands(def: &FungusDef) -> String {
    let names: Vec<&str> = def.biomes.iter().map(|&biome| land_name(biome)).collect();
    match (def.grows, names.is_empty()) {
        (_, true) => "Under any land".into(),
        (Where::Surface, false) => names.join(", "),
        (_, false) => format!("Under {}", names.join(", ")),
    }
}

fn warmth(def: &FungusDef) -> String {
    let degrees = |celsius: i16| match celsius < 0 {
        true => format!("−{}", -celsius),
        false => celsius.to_string(),
    };
    let (min, max) = (def.temperature.min, def.temperature.max);
    format!(
        "Lands whose year averages {} to {} °C",
        degrees(min),
        degrees(max)
    )
}

fn size(def: &FungusDef) -> String {
    let look = &def.look;
    let cm = |mm: u16| (mm + 5) / 10;
    match (look.smallest < 10, cm(look.smallest) == cm(look.largest)) {
        (true, _) => format!(
            "About {} mm to {} cm across",
            look.smallest,
            cm(look.largest)
        ),
        (false, true) => format!("About {} cm across", cm(look.largest)),
        (false, false) => format!(
            "About {} to {} cm across",
            cm(look.smallest),
            cm(look.largest)
        ),
    }
}

fn comes_as(def: &FungusDef) -> String {
    match (def.unit, def.servings) {
        (Unit::One, _) => "One at a time".into(),
        (Unit::Handful, _) => "A handful".into(),
        (Unit::Piece, servings) if servings > 1 => {
            format!("A piece of a big one, cut into {servings} servings when you eat it")
        }
        (Unit::Piece, _) => "A piece of a big one".into(),
    }
}

fn weight(def: &FungusDef) -> String {
    let each = match def.unit {
        Unit::One => "each",
        Unit::Handful => "a handful",
        Unit::Piece => "a piece",
    };
    match def.grams {
        grams if grams >= 1000 => {
            let kg = format!("{:.1}", f64::from(grams) / 1000.0);
            format!("About {} kg {each}", kg.trim_end_matches(".0"))
        }
        grams => format!("{grams} g {each}"),
    }
}

fn eating(def: &FungusDef) -> String {
    if def.kcal == 0 {
        return "Not food: it gives back no Health".into();
    }
    let serving = match has_slice(def) {
        true => "A serving",
        false => "It",
    };
    let amount = match shared::fungus::healing(def) {
        0 => return "Seldom eaten raw: eaten as it is, it gives back no Health".into(),
        1..5 => "a little Health",
        5..15 => "some Health",
        _ => "plenty of Health",
    };
    format!("{serving} gives back {amount} eaten raw, until you move or are hurt")
}

/// Its food effects eaten raw, as the item's popup shows them, in words.
fn raw_effects(def: &FungusDef) -> String {
    let raw: Vec<_> = (def.food.iter())
        .filter(|effect| effect.when.raw())
        .map(|effect| effect.effect())
        .collect();
    effects(&raw)
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

/// A picture's pixels along its longest side: 128 a cell.
pub(super) fn pixels(def: &FungusDef, serving: bool) -> u32 {
    let shape = form(def, serving).shape();
    128 * u32::from(shape.width.max(shape.height))
}

/// One of it as the pictures show it: variant 0, whole or a serving.
pub(super) fn form(def: &FungusDef, serving: bool) -> FungusForm {
    let seed = shared::rng::Seed::new(0);
    let form = match serving {
        true => FungusForm::serving(def, seed),
        false => FungusForm::new(def, seed),
    };
    FungusForm { look: 0, ..form }
}

/// Whether its page shows a slice beside it: a piece cut into servings.
pub(super) fn has_slice(def: &FungusDef) -> bool {
    form(def, false).is_whole(def)
}
