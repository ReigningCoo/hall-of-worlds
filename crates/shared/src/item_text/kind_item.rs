//! What a kind of item in text (`KindToken`) is called, and an item standing in
//! for it, made the same every time, to draw its picture from. English names for
//! now, as other items' are.

use uuid::Uuid;

use super::KindToken;
use crate::cooking::{Ingredients, Source, Tool, Wanted, food_name, usual_food};
use crate::creature::CreatureKind;
use crate::fruit::Fruits;
use crate::fungus::Fungi;
use crate::herb::Herbs;
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};
use crate::item_form::{ItemForm, ItemOrder, make_item};
use crate::livestock::Livestock;
use crate::potion::{BottleForm, Liquid, Water};
use crate::rng::Seed;
use crate::vegetable::Vegetables;

/// What every stand-in is rolled from, so one looks the same each time.
const SEED: u64 = 0x6b69_6e64;

impl KindToken {
    /// "Any egg", "Carrot", "Chanterelle", "Knife", "Water", "Rain Water". A
    /// fungus asked for by key is named, as the recipe naming it knows it.
    pub fn name(&self) -> String {
        match self {
            Self::Food(Wanted::Any(source)) => format!("Any {}", any_word(*source)),
            Self::Food(Wanted::One(Source::Fungus, key)) => (Fungi::built_in().get(key))
                .map_or_else(|| food_name(Source::Fungus, key), |def| def.name.clone()),
            Self::Food(Wanted::One(source, key)) => food_name(*source, key),
            Self::Tool(tool) => tool.name().to_owned(),
            Self::Water(water) => water.unwrap_or(Water::Plain).name().to_owned(),
        }
    }

    /// An item to draw it by: the usual food of a source asked for whole (a
    /// hen's egg for any egg), a liquid food or water in a bottle, a tool's own
    /// item (any knife a plain one). `None` for what has no item: the Oven, a
    /// butchered part, a food no table has.
    pub fn stand_in(&self) -> Option<Item> {
        let id = Id(Uuid::nil());
        match self {
            Self::Food(wanted) => {
                let (source, key) = usual_food(wanted, Ingredients::built_in())?;
                food_item(source, &key, id)
            }
            Self::Tool(tool) => made(tool_item(*tool)?, id),
            Self::Water(water) => Some(bottle(Liquid::Water(water.unwrap_or(Water::Plain)), id)),
        }
    }
}

/// The word after "Any" for a source.
fn any_word(source: Source) -> &'static str {
    match source {
        Source::Herb => "herb",
        Source::Fruit => "fruit",
        Source::Vegetable => "vegetable",
        Source::Fungus => "mushroom",
        Source::Meat => "meat",
        Source::Milk => "milk",
        Source::Egg => "egg",
        Source::Honey => "honey",
        Source::Part => "offal",
        Source::Pantry => "staple",
    }
}

/// The item one of a food is.
fn food_item(source: Source, key: &str, id: Id) -> Option<Item> {
    match source {
        Source::Herb => Some(crate::herb::new_item(
            Herbs::built_in_ref().get(key)?,
            1,
            id,
            None,
        )),
        Source::Fruit => Some(crate::fruit::new_item(
            Fruits::built_in().get(key)?,
            1,
            id,
            None,
        )),
        Source::Vegetable => {
            let def = Vegetables::built_in().get(key)?;
            Some(crate::vegetable::new_item(def, 1, id, None))
        }
        Source::Fungus => Some(crate::fungus::new_item(
            Fungi::built_in().get(key)?,
            1,
            id,
            None,
        )),
        Source::Egg => crate::farmyard::new_egg(Livestock::built_in().get(key)?, 1, id, None),
        Source::Milk => Some(bottle(Liquid::Milk(key.to_owned()), id)),
        Source::Honey => Some(bottle(Liquid::Honey(key.to_owned()), id)),
        Source::Meat => made(ItemKind::RawMeat, id),
        Source::Pantry if key == "salt" => made(ItemKind::Salt, id),
        Source::Pantry if key == "soy_sauce" => Some(bottle(Liquid::SoySauce, id)),
        Source::Pantry | Source::Part => None,
    }
}

/// The item a tool is; the Oven has none.
fn tool_item(tool: Tool) -> Option<ItemKind> {
    match tool {
        Tool::Knife => Some(ItemKind::Knife),
        Tool::Pan => Some(ItemKind::Pan),
        Tool::Cauldron => Some(ItemKind::Cauldron),
        Tool::Bowl => Some(ItemKind::Bowl),
        Tool::Sieve => Some(ItemKind::Sieve),
        Tool::Mortar => Some(ItemKind::MortarAndPestle),
        Tool::Oven => None,
    }
}

/// An ordinary item of this kind, rolled from `SEED`.
fn made(kind: ItemKind, id: Id) -> Option<Item> {
    let order = ItemOrder::plain(kind, Quality::Common);
    let human = CreatureKind::Human;
    Some(make_item(
        &order,
        human,
        id,
        Seed::new(SEED),
        None,
        &mut || id,
    ))
}

/// A bottle of `liquid`.
fn bottle(liquid: Liquid, id: Id) -> Item {
    let mut item = Item::new(id, ItemKind::Bottle);
    item.form = Some(ItemForm::Bottle(BottleForm::new(Some(liquid), SEED)));
    item
}
