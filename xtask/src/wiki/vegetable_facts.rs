//! The rows of a vegetable's, pulse's or grain's facts table, in plain words:
//! how it comes, what eating it raw does, its plant and how it grows and is
//! picked. Few fine numbers, as the game may change them soon. Effects only
//! eaten raw are told, as the item's popup tells them; what cooking would do
//! stays out, as no one can cook yet.

use shared::fruit::HarvestTier;
use shared::herb::Growth;
use shared::vegetable::{
    Abundance, Part, PlantShape, Unit, VegetableDef, VegetableForm, VegetableSet,
};

use super::fruit_facts::effects;
use super::herb_page::{land_name, picking as regrowing};

/// Every row, as (what, says), in the table's order.
pub(super) fn rows(def: &VegetableDef) -> Vec<(&'static str, String)> {
    vec![
        ("Kind", kind(def.set).into()),
        ("Rarity", rarity(def)),
        ("Grows in", lands(def)),
        ("Part picked", part(def.part).into()),
        ("Comes as", comes_as(def)),
        ("Weight", weight(def)),
        ("Eating", eating(def)),
        ("Effects", raw_effects(def)),
        ("Plant", plant(def)),
        ("Growth", growth(def.growth)),
        ("Picking", picking(def)),
        ("After picking", regrowing(def.regrow)),
    ]
}

/// "Vegetable", "Pulse" or "Grain".
pub(super) fn kind(set: VegetableSet) -> &'static str {
    match set {
        VegetableSet::Vegetable => "Vegetable",
        VegetableSet::Pulse => "Pulse",
        VegetableSet::Grain => "Grain",
    }
}

fn rarity(def: &VegetableDef) -> String {
    let wild = match def.abundance {
        Abundance::Common => "Common",
        Abundance::Uncommon => "Uncommon",
        Abundance::Rare => "Rare",
        Abundance::VeryRare => "Very rare",
    };
    match def.wild_only {
        true => format!("{wild} wild; never farmed"),
        false => format!("{wild} wild; farmed in fields"),
    }
}

fn lands(def: &VegetableDef) -> String {
    let lands: Vec<&str> = def.biomes.iter().map(|&biome| land_name(biome)).collect();
    match def.wild_only {
        true => lands.join(", "),
        false => format!("{}; farms' fields there too", lands.join(", ")),
    }
}

fn part(part: Part) -> &'static str {
    match part {
        Part::Root => "The root",
        Part::Tuber => "The tuber",
        Part::Bulb => "The bulb",
        Part::LeafBunch => "The leaves",
        Part::StalkBunch => "The stalks",
        Part::Shoots => "The young shoots",
        Part::FlowerHead => "The flower head",
        Part::Fruit => "The fruit",
        Part::Pods => "The pods",
        Part::Seeds => "The seeds",
        Part::EarOfGrain => "The ears of grain",
    }
}

fn comes_as(def: &VegetableDef) -> String {
    let servings = def.servings;
    match def.unit {
        Unit::One => "One at a time".into(),
        Unit::Handful => "A handful".into(),
        Unit::Sheaf => "A sheaf of grain".into(),
        Unit::Cluster => "A cluster of roots".into(),
        Unit::Whole if servings > 1 => {
            format!("A whole one, cut into {servings} servings when you eat it")
        }
        Unit::Piece if servings > 1 => {
            format!(
                "A piece cut from a big one, cut again into {servings} servings when you eat it"
            )
        }
        Unit::Whole => "A whole one".into(),
        Unit::Piece => "A piece cut from a big one".into(),
    }
}

fn weight(def: &VegetableDef) -> String {
    let each = match def.unit {
        Unit::One | Unit::Cluster => "each",
        Unit::Handful => "a handful",
        Unit::Sheaf => "a sheaf",
        Unit::Whole => "whole",
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

fn eating(def: &VegetableDef) -> String {
    if def.set == VegetableSet::Grain {
        return "A sheaf of grain gives back no Health as it is".into();
    }
    let serving = match has_slice(def) {
        true => "A serving",
        false => "It",
    };
    let amount = match shared::vegetable::healing(def) {
        0 => {
            return "Seldom eaten raw: eaten as it is, it gives back no Health".into();
        }
        1..5 => "a little Health",
        5..15 => "some Health",
        _ => "plenty of Health",
    };
    format!("{serving} gives back {amount} eaten raw, until you move or are hurt")
}

/// Its effects eaten raw, as the item's popup shows them, in words.
fn raw_effects(def: &VegetableDef) -> String {
    let raw: Vec<_> = (def.effects.iter())
        .filter(|effect| effect.when.raw())
        .map(|effect| effect.effect())
        .collect();
    effects(&raw)
}

fn plant(def: &VegetableDef) -> String {
    let look = &def.plant_look;
    let size = |cm: u16| match cm {
        cm if cm >= 200 => format!("{} m", (cm + 50) / 100),
        cm => format!("{cm} cm"),
    };
    format!(
        "{}, about {} tall and {} across",
        shape(def.plant),
        size(look.height),
        size(look.spread)
    )
}

fn shape(shape: PlantShape) -> &'static str {
    match shape {
        PlantShape::LeavesOverARoot => "Leaves over a swollen root",
        PlantShape::UprightLeafyHerb => "An upright leafy plant",
        PlantShape::LeafyRosette => "A rosette of leaves",
        PlantShape::Bush => "A bush",
        PlantShape::GroundVine => "A vine along the ground",
        PlantShape::ClimberOnAPole => "A climber up a pole",
        PlantShape::ClimberOnSticks => "A climber on sticks",
        PlantShape::BulbClump => "A clump of bulbs",
        PlantShape::WaterPlant => "A water plant",
        PlantShape::GrainGrass => "A grass with heads of grain",
        PlantShape::TallGrass => "A tall grass",
        PlantShape::WaterGrass => "A grass standing in water",
        PlantShape::PaddyGrass => "A grass in a flooded field",
        PlantShape::Fern => "A fern",
        PlantShape::Succulent => "A succulent",
        PlantShape::AgaveRosette => "A rosette of thick, spiny leaves",
        PlantShape::BambooGrove => "A grove of bamboo",
        PlantShape::ThornyTree => "A thorny tree",
    }
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

fn picking(def: &VegetableDef) -> String {
    let many = match def.harvest {
        HarvestTier::One => "One",
        HarvestTier::UpToTwo => "One or two",
        HarvestTier::UpToThree => "One to three",
        HarvestTier::TwoToFour => "Two to four",
        HarvestTier::ThreeToSix => "Three to six",
    };
    format!("{many} when full grown, more with Foraging and Crops")
}

/// A picture's pixels along its longest side: 128 a cell.
pub(super) fn pixels(def: &VegetableDef, serving: bool) -> u32 {
    let seed = shared::rng::Seed::new(0);
    let form = match serving {
        true => VegetableForm::serving(def, seed),
        false => VegetableForm::new(def, seed),
    };
    let shape = form.shape();
    128 * u32::from(shape.width.max(shape.height))
}

/// Whether its page shows a slice beside it: a whole one or a piece cut into
/// servings.
pub(super) fn has_slice(def: &VegetableDef) -> bool {
    VegetableForm::new(def, shared::rng::Seed::new(0)).is_whole(def)
}
