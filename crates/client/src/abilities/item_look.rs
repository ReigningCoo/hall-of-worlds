//! How a kind of item on the action bar looks (owner, 2026-11-15): its picture
//! in an ability's frame, with how many are carried; greyed, with a dull red
//! frame, when none are carried or the player's kind won't eat it (pressing it
//! then does nothing). A potion (brewing decision 22) the same, greyed also
//! while one of its stats and signs is cooling down.

use bevy::prelude::*;
use shared::ability::BarThing;
use shared::bonus::may_eat;
use shared::brewing::Combination;
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality};
use shared::item_form::ItemForm;
use shared::lang::Lang;
use shared::potion::{BottleForm, Liquid, PotionKey};
use shared::sheet::Sheet;

use super::page::look;
use super::spells::spell_look;
use crate::widgets::{AbilityLook, GRID_COLOUR};

/// How a kind of item looks on the bar, with `count` carried.
pub(super) fn item_look(kind: ItemKind, count: u32, eater: CreatureKind) -> AbilityLook {
    AbilityLook {
        icon: Handle::default(),
        name: kind.def().name.to_string(),
        passive: false,
        usable: can_eat(kind, count, eater),
        colour: GRID_COLOUR,
        // Every one of a kind looks alike so far (food): a plain one stands for them.
        item: Some(Item::new(Id::stand_in(0), kind)),
    }
}

/// Whether pressing it eats one.
pub(super) fn can_eat(kind: ItemKind, count: u32, eater: CreatureKind) -> bool {
    count > 0 && may_eat(kind, eater)
}

/// How a potion looks on the bar; greyed unless `usable` (`can_drink`).
pub(super) fn potion_look(potion: PotionKey, usable: bool) -> AbilityLook {
    AbilityLook {
        icon: Handle::default(),
        name: "Potion".to_owned(),
        passive: false,
        usable,
        colour: GRID_COLOUR,
        item: Some(potion_item(potion)),
    }
}

/// A bottle of this potion standing for every one carried: its look seed made
/// from its doses, so each potion keeps one bottle.
fn potion_item(potion: PotionKey) -> Item {
    let contents = potion.contents();
    let look = contents.doses().iter().fold(0u64, |seed, dose| {
        let step = dose.step as u64;
        seed.wrapping_mul(31)
            .wrapping_add((dose.stat as u64) * 97)
            .wrapping_add(step)
    });
    let mut bottle = Item::new(Id::stand_in(0), ItemKind::Bottle);
    let liquid = Some(Liquid::Potion(contents));
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(liquid, look)));
    bottle.quality = Some(Quality::Common);
    bottle
}

/// Whether pressing it drinks one: one is carried, and its stats and signs
/// aren't cooling down at action tick `now` (taken as ready until the server
/// has said the tick; the server decides).
pub(super) fn can_drink(potion: PotionKey, count: u32, sheet: &Sheet, now: Option<f64>) -> bool {
    let combination = Combination::of(&potion.contents());
    count > 0 && now.is_none_or(|now| sheet.drinks.ready(combination, now as u64))
}

/// Its doses in words: "Strength +4, Health −2".
pub(super) fn potion_words(potion: PotionKey) -> String {
    let words: Vec<String> = (potion.contents().doses().iter())
        .map(|dose| {
            let name = dose.stat.name();
            let sign = if dose.step < 0 { "−" } else { "+" };
            format!("{name} {sign}{}", dose.step.unsigned_abs())
        })
        .collect();
    words.join(", ")
}

/// How an ability, a kind of item, a potion or a spell (from the sheet's
/// book, worded in `lang`) looks, with `count` carried of an item; an
/// ability, a potion or a spell greyed unless `usable` (`page::usable`,
/// `can_drink`, `spells::can_cast`).
pub(super) fn thing_look(
    thing: BarThing,
    (count, eater): (u32, CreatureKind),
    (sheet, lang): (&Sheet, &Lang),
    usable: bool,
    assets: &AssetServer,
) -> AbilityLook {
    match thing {
        BarThing::Ability(ability) => look(ability, usable, assets),
        BarThing::Item(kind) => item_look(kind, count, eater),
        BarThing::Potion(potion) => potion_look(potion, usable),
        BarThing::Spell(key) => spell_look(key, sheet, lang, usable, assets),
    }
}
