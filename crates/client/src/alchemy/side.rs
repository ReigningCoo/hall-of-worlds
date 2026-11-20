//! The right of the panel: Start (Stop while it runs), the count with `|<`
//! `<` `>` `>|` (once, one fewer, one more, as many as the ingredients allow),
//! and what one run would make: the product's frame (it can't be picked up)
//! at its least quality, its name, the chance, and each effect, Unknown until
//! it is known on every herb that gives to it (`shared::alchemy::preview`).

use bevy::prelude::*;
use shared::alchemy::{AlchemySkills, Known, OrderRefused, preview, steps_allowed};
use shared::protocol::ClientMessage;

use super::bench::{Bench, Running};
use super::boxes::space_shown;
use super::markup::{NONE_KNOWN, WORDS};
use super::rows::known_apparatus;
use super::words::{chance_words, effect_words, product_item, product_name, refused_words};
use crate::character::CharacterSheet;
use crate::common::HerbTable;
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
    (carried, sheet, table): (Res<Carried>, Res<CharacterSheet>, Res<HerbTable>),
    time: Res<Time>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("alchemy", name) = event.id.parts() else {
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
                let skills = AlchemySkills::of(&sheet.0.skills);
                let Ok(checked) = bench.check(&carried.0, &table.0, skills) else {
                    continue;
                };
                let order = bench.order(bench.count.min(most));
                to_server.write(ToServer(ClientMessage::Alchemy(order)));
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

/// Says what the process would make, whenever it, what is carried or the
/// skills change.
pub(super) fn show_side(
    bench: Res<Bench>,
    (carried, sheet, table): (Res<Carried>, Res<CharacterSheet>, Res<HerbTable>),
    mut product: Query<&mut SlotShown, With<ProductSlot>>,
    added: Query<(), Added<ProductSlot>>,
    mut dom: Dom,
) {
    let changed = bench.is_changed() || carried.is_changed() || sheet.is_changed();
    if !bench.is_open() || !(changed || table.is_changed() || !added.is_empty()) {
        return;
    }
    let skills = AlchemySkills::of(&sheet.0.skills);
    let known = known_apparatus(&sheet.0.skills);
    dom.set_text(
        "alchemy/words",
        if known.is_empty() { NONE_KNOWN } else { WORDS },
    );
    let allowed = steps_allowed(skills.alchemy_percent());
    let steps = bench.steps.len();
    dom.set_text(
        "alchemy/steps",
        format!("Steps: {steps} ({allowed} allowed; each past that halves the chance)"),
    );
    let most = bench.most_runs(&carried.0);
    let count = bench.count.min(most);
    dom.set_text("alchemy/count", count.to_string());
    let checked = bench.check(&carried.0, &table.0, skills);
    let running = bench.is_running();
    dom.set_text("alchemy/start", if running { "Stop" } else { "Start" });
    dom.set_flag("alchemy/start", "disabled", !running && checked.is_err());
    for (button, off) in [
        ("alchemy/fewest", count <= 1),
        ("alchemy/fewer", count <= 1),
        ("alchemy/more", count >= most),
        ("alchemy/most", count >= most),
    ] {
        dom.set_flag(button, "disabled", running || off);
    }
    let item = checked.as_ref().ok().and_then(|c| product_item(&c.planned));
    for mut shown in &mut product {
        let new = space_shown(item.as_ref(), Handle::default(), None);
        let new = SlotShown { icon: None, ..new };
        shown.set_if_neq(new);
    }
    match &checked {
        Ok(checked) => {
            let known = Known {
                herbs: &sheet.0.herbs,
                fungi: &sheet.0.fungi,
            };
            let seen = preview(&checked.planned, known);
            dom.set_text("alchemy/product_name", product_name(&checked.planned));
            dom.set_text("alchemy/chance", chance_words(checked.planned.chance));
            dom.set_text("alchemy/effects", effect_words(&seen));
        }
        Err(why) => {
            let empty = matches!(why, OrderRefused::Rules(shared::alchemy::Refused::Empty));
            let words = if empty {
                "Add a step."
            } else {
                refused_words(*why)
            };
            dom.set_text("alchemy/product_name", words);
            dom.set_text("alchemy/chance", "");
            dom.set_text("alchemy/effects", "");
        }
    }
}
