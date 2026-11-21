//! The right of the panel: Start (Stop while it runs), the count with `|<`
//! `<` `>` `>|`, and what one run would make: the first output's frame (it
//! can't be picked up) at its least quality, every output's name, the
//! chance, diet labels and Well Fed (`shared::cooking::plan`, by the
//! server's own check); and, under the grid, the steps allowed and the
//! kitchen.

use bevy::prelude::*;
use shared::alchemy::steps_allowed;
use shared::cooking::{CookingSkills, OrderRefused, Refused};
use shared::protocol::ClientMessage;

use super::bench::{Bench, Running};
use super::boxes::space_shown;
use super::kitchen::Kitchens;
use super::markup::WORDS;
use super::words::{
    chance_words, kitchen_words, output_item, output_lines, output_name, refused_words,
};
use crate::character::CharacterSheet;
use crate::common::Wording;
use crate::connection::ToServer;
use crate::dom::{Dom, SlotShown, UiEvent, UiValue};
use crate::inventory::Carried;

/// The product's frame.
#[derive(Component)]
pub(super) struct ProductSlot;

/// Start, Stop and the count's buttons.
pub(super) fn answer_panel(
    mut events: MessageReader<UiEvent>,
    mut bench: ResMut<Bench>,
    (carried, sheet, kitchens): (Res<Carried>, Res<CharacterSheet>, Res<Kitchens>),
    time: Res<Time>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("cooking", name) = event.id.parts() else {
            continue;
        };
        if event.value != UiValue::Pressed {
            continue;
        }
        let most = bench.most_runs(&carried.0);
        match name {
            "start" if bench.is_running() => {
                to_server.write(ToServer(ClientMessage::CancelAbility));
            }
            "start" => {
                let skills = CookingSkills::of(&sheet.0.skills);
                let Ok(checked) = bench.check(&carried.0, &kitchens.0, skills) else {
                    continue;
                };
                let order = bench.order(bench.count.min(most), &kitchens.0);
                to_server.write(ToServer(ClientMessage::Cook(order)));
                bench.made = None;
                bench.running = Some(Running {
                    asked: time.elapsed_secs_f64(),
                    started: false,
                    planned: Some(checked.planned),
                });
            }
            "fewest" => bench.count = 1,
            "fewer" => bench.count = bench.count.saturating_sub(1).max(1),
            "more" => bench.count = (bench.count + 1).min(most),
            "most" => bench.count = most,
            _ => {}
        }
    }
}

/// Says what the process would make, whenever it, what is carried, the
/// skills or the kitchen change.
pub(super) fn show_side(
    bench: Res<Bench>,
    (carried, sheet, kitchens, wording): (
        Res<Carried>,
        Res<CharacterSheet>,
        Res<Kitchens>,
        Res<Wording>,
    ),
    mut product: Query<&mut SlotShown, With<ProductSlot>>,
    added: Query<(), Added<ProductSlot>>,
    mut dom: Dom,
) {
    let changed = bench.is_changed() || carried.is_changed() || sheet.is_changed();
    if !bench.is_open() || !(changed || kitchens.is_changed() || !added.is_empty()) {
        return;
    }
    let kitchen = &kitchens.0;
    let skills = CookingSkills::of(&sheet.0.skills);
    dom.set_text("cooking/words", WORDS);
    dom.set_text("cooking/kitchen", kitchen_words(kitchen));
    let allowed = steps_allowed(skills.cooking_percent());
    let steps = bench.steps.len();
    dom.set_text(
        "cooking/steps",
        format!(
            "Steps: {steps} ({allowed} allowed; each past that halves the chance, a join counts 1½)"
        ),
    );
    let most = bench.most_runs(&carried.0);
    let count = bench.count.min(most);
    dom.set_text("cooking/count", count.to_string());
    let checked = bench.check(&carried.0, kitchen, skills);
    let running = bench.is_running();
    dom.set_text("cooking/start", if running { "Stop" } else { "Start" });
    dom.set_flag("cooking/start", "disabled", !running && checked.is_err());
    for (button, off) in [
        ("cooking/fewest", count <= 1),
        ("cooking/fewer", count <= 1),
        ("cooking/more", count >= most),
        ("cooking/most", count >= most),
    ] {
        dom.set_flag(button, "disabled", running || off);
    }
    let first = checked
        .as_ref()
        .ok()
        .and_then(|c| c.planned.outputs.first());
    let item = (bench.made.clone()).or_else(|| first.map(|o| output_item(o, o.quality)));
    for mut shown in &mut product {
        let new = space_shown(
            item.as_ref().map(|i| (i, i.count())),
            Handle::default(),
            None,
        );
        let new = SlotShown { icon: None, ..new };
        shown.set_if_neq(new);
    }
    match &checked {
        Ok(checked) => {
            let outputs = &checked.planned.outputs;
            let names: Vec<String> = outputs.iter().map(output_name).collect();
            dom.set_text("cooking/product_name", names.join("\n"));
            dom.set_text("cooking/chance", chance_words(checked.planned.chance));
            let mut about = Vec::new();
            for output in outputs {
                if outputs.len() > 1 {
                    about.push(output_name(output));
                }
                about.extend(output_lines(output, &wording.0));
            }
            dom.set_text("cooking/about", about.join("\n"));
        }
        Err(why) => {
            let empty = matches!(why, OrderRefused::Rules(Refused::NoSteps));
            let words = if empty {
                "Add a step.".to_owned()
            } else {
                refused_words(why, kitchen.heat)
            };
            dom.set_text("cooking/product_name", words);
            dom.set_text("cooking/chance", "");
            dom.set_text("cooking/about", "");
        }
    }
}
