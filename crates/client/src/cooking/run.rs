//! A process running: the panel locks and its steps wash out, then colour
//! fills along them like a progress bar as each run winds up (the server's
//! `AbilityStarted`, kept in `common::OwnAbility`), each step its share of
//! the run's time in the order they run, through black, white and gold to
//! the first output's own tint. What a run made is shown in the product's
//! frame. It ends when the last run lands, or is called off (moving, harm,
//! Stop or Esc), or if the server never starts it (what is carried changed).

use bevy::prelude::*;
use shared::ability::Ability;
use shared::cooking::CookingSkills;
use shared::protocol::{Cooked, Landed, ServerMessage};

use super::bench::Bench;
use super::boxes::{Fill, GOLD, WASH, Wash};
use super::words::{output_colour, output_item};
use crate::character::CharacterSheet;
use crate::common::OwnAbility;
use crate::connection::FromServer;

/// How long a run asked for may go unstarted before the panel gives up, in
/// seconds: the server says nothing when what is carried no longer allows it.
const START_WAIT: f64 = 3.0;

/// Following the server's answers to the order.
pub(super) fn follow_run(
    mut from_server: MessageReader<FromServer>,
    mut bench: ResMut<Bench>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs_f64();
    for FromServer(message) in from_server.read() {
        let Some(mut running) = bench.running.clone() else {
            continue;
        };
        match message {
            ServerMessage::AbilityStarted {
                who: None,
                ability: Ability::Cook,
                ..
            } => running.started = true,
            ServerMessage::AbilityLanded {
                who: None,
                landed: Landed::Cooked { made, left, .. },
                ..
            } => {
                let first = running.planned.as_ref().and_then(|p| p.outputs.first());
                if let (Cooked::Made { quality, count, .. }, Some(output)) = (made, first) {
                    let mut item = output_item(output, *quality);
                    item.count = *count;
                    bench.made = Some(item);
                }
                if *left == 0 {
                    bench.running = None;
                    continue;
                }
                // The next run is asked for by the server itself.
                (running.started, running.asked) = (false, now);
            }
            ServerMessage::AbilityLanded {
                who: None,
                landed: Landed::Cancelled,
                ..
            } => {
                bench.running = None;
                continue;
            }
            _ => continue,
        }
        bench.running = Some(running);
    }
    let gave_up =
        (bench.running.as_ref()).is_some_and(|r| !r.started && now - r.asked > START_WAIT);
    if gave_up {
        bench.running = None;
    }
}

/// The wash and the colour fill on each step.
pub(super) fn show_fill(
    bench: Res<Bench>,
    (own, sheet, time): (Res<OwnAbility>, Res<CharacterSheet>, Res<Time>),
    mut fills: Query<(&Fill, &mut Node, &mut BackgroundColor), Without<Wash>>,
    mut washes: Query<&mut BackgroundColor, (With<Wash>, Without<Fill>)>,
    added: Query<(), Added<Fill>>,
    mut was_running: Local<bool>,
) {
    let running = bench.running.as_ref();
    if running.is_none() && !*was_running && added.is_empty() {
        return;
    }
    *was_running = running.is_some();
    let wash = if running.is_some() { WASH } else { Color::NONE };
    for mut colour in &mut washes {
        colour.set_if_neq(BackgroundColor(wash));
    }
    let now = time.elapsed_secs_f64();
    let done = match running {
        Some(running) if running.started && own.lands > own.started => {
            ((now - own.started) / (own.lands - own.started)).clamp(0.0, 1.0) as f32
        }
        _ => 0.0,
    };
    let skills = CookingSkills::of(&sheet.0.skills);
    // Each step's seconds, in the order they run (as `run_seconds` counts).
    let order = bench.in_order();
    let seconds = |at: usize| {
        let step = &bench.steps[at];
        let halves = if step.height > 1 { 3 } else { 2 };
        (step.tool.seconds(skills) * halves / 2).max(1) as f32
    };
    let total: f32 = order.iter().map(|&at| seconds(at)).sum();
    let product = running
        .and_then(|running| running.planned.as_ref())
        .and_then(|planned| planned.outputs.first())
        .map_or(GOLD, output_colour);
    let colour = stage_colour(done, product);
    for (Fill(step), mut node, mut background) in &mut fills {
        let place = order.iter().position(|at| at == step).unwrap_or(0);
        let before: f32 = order.iter().take(place).map(|&at| seconds(at)).sum();
        let length = if *step < bench.steps.len() {
            seconds(*step)
        } else {
            1.0
        };
        let share = ((done * total - before) / length).clamp(0.0, 1.0);
        let shown = if running.is_some() { share } else { 0.0 };
        if node.width != percent(shown * 100.0) {
            node.width = percent(shown * 100.0);
        }
        background.set_if_neq(BackgroundColor(colour.with_alpha(0.55)));
    }
}

/// The fill's colour this far along a run (0 to 1): black, then white, then
/// gold, then the product's own colour.
pub(super) fn stage_colour(done: f32, product: Color) -> Color {
    let stages = [Color::BLACK, Color::WHITE, GOLD, product];
    let along = done.clamp(0.0, 1.0) * (stages.len() - 1) as f32;
    let at = (along.floor() as usize).min(stages.len() - 2);
    let (from, to) = (stages[at].to_srgba(), stages[at + 1].to_srgba());
    let t = along - at as f32;
    let mix = |a: f32, b: f32| a + (b - a) * t;
    Color::srgb(
        mix(from.red, to.red),
        mix(from.green, to.green),
        mix(from.blue, to.blue),
    )
}
