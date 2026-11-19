//! Showing Ferment's panel: its dropdowns filled from what is carried and
//! lying in reach, and what the choice would make.

use bevy::prelude::*;
use shared::ferment::{FermentPlan, FermentRefused, days_left, fermentable_cask, plan};
use shared::item::Item;

use super::Fermenting;
use super::markup::PICK;
use crate::dom::Dom;
use crate::inventory::{Carried, GroundInReach};

/// Up while choosing, its choices from what is carried and in reach now.
pub(in crate::abilities) fn show_panel(
    mut fermenting: ResMut<Fermenting>,
    carried: Res<Carried>,
    reach: Res<GroundInReach>,
    mut dom: Dom,
) {
    let opened = fermenting.is_changed() && fermenting.open;
    if opened || (fermenting.open && (carried.is_changed() || reach.is_changed())) {
        let choices = carried.0.ferment_choices();
        let found = |ids: &[shared::id::Id]| -> Vec<Item> {
            (ids.iter())
                .filter_map(|id| carried.0.find(*id).cloned())
                .collect()
        };
        let mut casks = found(&choices.casks);
        let lying = reach
            .0
            .iter()
            .filter(|item| item.count() == 1 && fermentable_cask(item));
        casks.extend(lying.cloned());
        let adds = found(&choices.adds);
        let same = casks == fermenting.casks
            && adds == fermenting.adds
            && choices.yeasts == fermenting.yeasts;
        if opened || !same {
            fill(
                &mut dom,
                "ferment/cask",
                casks.iter().map(Item::name).collect(),
                "No cask",
            );
            let mut rows = vec!["Nothing".to_owned()];
            rows.extend(
                adds.iter()
                    .map(|add| format!("{} ({})", add.name(), add.count())),
            );
            fill(&mut dom, "ferment/add", rows, "Nothing");
            let rows = (choices.yeasts.iter())
                .map(|yeast| format!("{:?} yeast ({} usable)", yeast.quality, yeast.usable));
            fill(&mut dom, "ferment/yeast", rows.collect(), "No yeast");
            dom.set_text("ferment/words", PICK);
            fermenting.casks = casks;
            fermenting.adds = adds;
            fermenting.yeasts = choices.yeasts;
            (fermenting.cask, fermenting.add, fermenting.yeast) = (0, 0, 0);
            fermenting.count = 1;
            fermenting.dirty = true;
        }
    }
    if fermenting.is_changed() {
        dom.set_flag("ferment/asking", "hidden", !fermenting.open);
    }
    if fermenting.open && fermenting.dirty {
        fermenting.dirty = false;
        let usable = fermenting
            .yeasts
            .get(fermenting.yeast)
            .map_or(1, |y| y.usable);
        let counts = (1..=usable.max(1)).map(|n| n.to_string()).collect();
        fill(&mut dom, "ferment/count", counts, "1");
        let count = fermenting.count.min(usable.max(1));
        dom.set("ferment/count", "value", &(count - 1).to_string());
        let made = made(&fermenting, &carried);
        dom.set_text("ferment/makes", makes(&made));
        dom.set_flag("ferment/ferment", "disabled", made.is_err());
    }
}

/// What the choice would make, by the server's own rule.
fn made(fermenting: &Fermenting, carried: &Carried) -> Result<FermentPlan, FermentRefused> {
    let cask = (fermenting.casks.get(fermenting.cask)).ok_or(FermentRefused::Nothing)?;
    let yeasts = match fermenting.yeasts.get(fermenting.yeast) {
        Some(yeast) => carried
            .0
            .usable_yeasts(yeast.stack, fermenting.count.max(1)),
        None => Vec::new(),
    };
    plan(cask, fermenting.added(), &yeasts)
}

/// "Makes 20 L of Cider (5.8%, dry, Common) in 36 days.", or why not.
fn makes(made: &Result<FermentPlan, FermentRefused>) -> String {
    match made {
        Ok(plan) => format!(
            "Makes {} of {} ({}, {}, {:?}) in {} days.",
            litres(plan.ml),
            plan.drink.name(),
            plan.drink.strength_words(),
            plan.drink.sweetness.words(),
            plan.drink.quality,
            days_left(plan.secs),
        ),
        Err(FermentRefused::Nothing) => {
            "No cask carried or in reach holds juice, cordial or water.".to_owned()
        }
        Err(why) => why.words().to_owned(),
    }
}

/// Millilitres as litres to a tenth: "19.5 L".
fn litres(ml: u32) -> String {
    format!("{}.{} L", ml / 1000, ml % 1000 / 100)
}

/// A dropdown's options, or one saying there is none.
fn fill(dom: &mut Dom, id: &str, rows: Vec<String>, none: &str) {
    let rows = match rows.is_empty() {
        true => vec![none.to_owned()],
        false => rows,
    };
    let options = rows.into_iter().enumerate();
    dom.set_options(id, options.map(|(at, row)| (at.to_string(), row)));
    dom.set(id, "value", "0");
}
