//! The panel's words, and the product as an item to show: what one run makes,
//! its chance, its effects as far as they are known, and why a process can't
//! be run.

use bevy::prelude::*;
use shared::alchemy::{CERTAIN, Form, OrderRefused, Planned, Product, Refused, Seen, new_mixture};
use shared::brewing::stat_row;
use shared::herb::Stat;
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Contents, Dose, Liquid};

use crate::widgets::row_rgb;

/// What one run makes, as an item to show at its least quality.
pub(super) fn product_item(planned: &Planned) -> Option<Item> {
    let quality = planned.quality.unwrap_or(Quality::Poor);
    match &planned.product {
        Product::Nothing => None,
        Product::Mixture { form, contents } => Some(new_mixture(
            *form,
            contents.clone(),
            quality,
            1,
            Id::stand_in(0),
        )),
        Product::Potion { contents, .. } => {
            let mut bottle = Item::new(Id::stand_in(0), ItemKind::Bottle);
            let liquid = Some(Liquid::Potion(contents.clone()));
            bottle.form = Some(ItemForm::Bottle(BottleForm::new(
                liquid,
                look_seed(contents),
            )));
            bottle.quality = Some(quality);
            Some(bottle)
        }
    }
}

/// A potion's look seed from its doses, so the same potion looks the same.
fn look_seed(contents: &Contents) -> u64 {
    contents.doses().iter().fold(0u64, |seed, dose| {
        seed.wrapping_mul(31)
            .wrapping_add((dose.stat as u64) * 97)
            .wrapping_add(dose.step as u64)
    })
}

/// "Common Ointment", "2 bottles of Rare potion", "Nothing worth keeping".
pub(super) fn product_name(planned: &Planned) -> String {
    let quality = planned.quality.unwrap_or(Quality::Poor);
    match &planned.product {
        Product::Nothing => "Nothing worth keeping".to_owned(),
        Product::Mixture { form, .. } => {
            let form = match form {
                Form::Paste => "Paste",
                Form::Ointment => "Ointment",
                Form::Powder => "Powder",
            };
            format!("{quality:?} {form}, at least")
        }
        Product::Potion { bottles: 1, .. } => format!("A {quality:?} potion, at least"),
        Product::Potion { bottles, .. } => format!("{bottles} {quality:?} potions, at least"),
    }
}

/// "Chance 100%", "Chance 12.5%".
pub(super) fn chance_words(chance: u32) -> String {
    let whole = chance / (CERTAIN / 100);
    let tenths = chance % (CERTAIN / 100) / (CERTAIN / 1000);
    match tenths {
        0 => format!("Chance {whole}%"),
        tenths => format!("Chance {whole}.{tenths}%"),
    }
}

/// Each effect on a line: "Strength +4", or "Unknown".
pub(super) fn effect_words(seen: &[Seen]) -> String {
    let lines: Vec<String> = seen
        .iter()
        .map(|seen| match seen {
            Seen::Known(dose) => dose_words(*dose),
            Seen::Unknown => "Unknown".to_owned(),
        })
        .collect();
    lines.join("\n")
}

/// "Strength +4", "Health −2".
fn dose_words(dose: Dose) -> String {
    let name = match (dose.stat.attribute(), dose.stat) {
        (Some(attribute), _) => attribute.name(),
        (None, Stat::Health) => "Health",
        (None, _) => "Mana",
    };
    let sign = if dose.step < 0 { "−" } else { "+" };
    format!("{name} {sign}{}", dose.step.unsigned_abs())
}

/// The product's own colour: its strongest dose's row's (Body red, Mind
/// blue, Spirit green), or ink's for nothing.
pub(super) fn product_colour(planned: &Planned) -> Color {
    let doses = planned.product.contents().doses();
    let strongest = doses.iter().max_by_key(|dose| dose.step.unsigned_abs());
    match strongest {
        Some(dose) => {
            let [r, g, b] = row_rgb(stat_row(dose.stat));
            Color::srgb_u8(r, g, b)
        }
        None => Color::srgb(0.3, 0.25, 0.2),
    }
}

/// Why a process can't be run, in words.
pub(super) fn refused_words(why: OrderRefused) -> &'static str {
    match why {
        OrderRefused::Count => "Choose how many times.",
        OrderRefused::Apparatus(_) => "An apparatus in the process isn't carried.",
        OrderRefused::Source => "The source isn't a carried Paste or Ointment.",
        OrderRefused::Herb | OrderRefused::UnknownHerb => "A herb isn't carried.",
        OrderRefused::Oil => "The oil isn't carried.",
        OrderRefused::Liquid => "A liquid isn't carried.",
        OrderRefused::TooFew => "Not enough is carried for one run.",
        OrderRefused::Rules(why) => match why {
            Refused::Empty => "Add a step.",
            Refused::MortarNotFirst => "A Mortar can only be the first step, with no source.",
            Refused::NoHerbs => "Put herbs in the Mortar.",
            Refused::SameHerb => "Only one of each herb counts.",
            Refused::NoLiquid => "Put a liquid in the Retort.",
            Refused::TooMuchLiquid => "A Retort holds 500 ml at most.",
            Refused::Liquid => "A Retort takes only waters and oils so far.",
            Refused::NotBuilt(_) => "That apparatus can't be used yet.",
        },
    }
}
