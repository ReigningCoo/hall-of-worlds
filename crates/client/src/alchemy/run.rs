//! A process running: the panel locks and its steps wash out, then colour
//! fills along them like a progress bar as each run winds up (the server's
//! `AbilityStarted`, kept in `common::OwnAbility`), each step its share of
//! the run's time, through black, white and gold to the product's own
//! colour. What a run made is shown in the box after the last step. It ends
//! when the last run lands, or is called off (moving, harm, Stop or Esc), or
//! if the server never starts it (what is carried changed).

use bevy::prelude::*;
use shared::ability::Ability;
use shared::alchemy::{AlchemySkills, Planned, run_seconds};
use shared::protocol::{Alchemised, Landed, ServerMessage};

use super::bench::Bench;
use super::boxes::{Fill, GOLD, WASH, Wash};
use super::words::{product_colour, product_item};
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
                ability: Ability::Alchemy,
                ..
            } => running.started = true,
            ServerMessage::AbilityLanded {
                who: None,
                landed: Landed::Alchemy { made, left, .. },
                ..
            } => {
                if let (Alchemised::Made { quality, .. }, Some(planned)) = (made, &running.planned)
                {
                    let planned = Planned {
                        quality: Some(*quality),
                        ..planned.clone()
                    };
                    bench.made = product_item(&planned);
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
    let skills = AlchemySkills::of(&sheet.0.skills);
    let seconds: Vec<f32> = (bench.steps.iter())
        .map(|station| run_seconds(&[station.apparatus], skills).max(1) as f32)
        .collect();
    let total: f32 = seconds.iter().sum();
    let product = running
        .and_then(|running| running.planned.as_ref())
        .map_or(GOLD, product_colour);
    let colour = stage_colour(done, product);
    for (Fill(step), mut node, mut background) in &mut fills {
        let before: f32 = seconds.iter().take(*step).sum();
        let length = seconds.get(*step).copied().unwrap_or(1.0);
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
