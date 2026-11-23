//! Notes for new players, when the server says one is wanted
//! (`ServerMessage::Tutorial`): for now The Way Inn's, on a new character's
//! first arrival there, saying what it is and how to come back to it. See
//! `docs/context/client/tutorial.md`.

use bevy::prelude::*;
use shared::protocol::{ServerMessage, Tutorial};

use crate::common::LoadingStep;
use crate::connection::FromServer;
use crate::dom::{Dom, Template, UiEvent, UiValue, document, top_strip};
use crate::html;

/// Notes for new players.
pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Waiting>()
            .add_systems(Startup, spawn_notes)
            .add_systems(Update, (hear_notes, show_notes, close_notes).chain());
    }
}

/// The Way Inn's note: a requester over the game, closed by its button or a
/// press outside.
static NOTES: Template = html!(
    "tutorial",
    r#"
<form id="tutorial">
  <dialog id="way_inn" title="The Way Inn" depth="1" hidden>
    <p class="medium" width="420">The Way Inn has always been here, between the worlds: a meeting place for those who travel the Hall of Worlds. Its magic stops all fighting inside its walls.</p>
    <p class="medium" width="420">Its door opens onto the Hall, a marble walkway with gates to countless worlds along both sides.</p>
    <p class="medium" width="420">To come back, step off the walkway's edge halfway between two gates, or click there.</p>
    <footer>
      <button id="close" class="main">Onward</button>
    </footer>
  </dialog>
</form>
"#
);

/// A note the server asked for, until the game takes input to show it.
#[derive(Resource, Default)]
struct Waiting(Option<Tutorial>);

fn spawn_notes(mut commands: Commands) {
    commands.spawn(document(&NOTES)).insert(top_strip());
}

/// Keeps the note the server asks for.
fn hear_notes(mut from_server: MessageReader<FromServer>, mut waiting: ResMut<Waiting>) {
    for FromServer(message) in from_server.read() {
        if let ServerMessage::Tutorial(note) = message {
            waiting.0 = Some(*note);
        }
    }
}

/// Shows a waiting note once the loading screen has gone.
fn show_notes(step: Res<LoadingStep>, mut waiting: ResMut<Waiting>, mut dom: Dom) {
    if *step != LoadingStep::Ready {
        return;
    }
    if let Some(Tutorial::WayInn) = waiting.0.take() {
        dom.set_flag("tutorial/way_inn", "hidden", false);
    }
}

/// Onward, or a press outside, closes it.
fn close_notes(mut events: MessageReader<UiEvent>, mut dom: Dom) {
    for event in events.read() {
        let ("tutorial", name) = event.id.parts() else {
            continue;
        };
        if matches!(
            (&event.value, name),
            (UiValue::Pressed, "close") | (UiValue::Dismissed, _)
        ) {
            dom.set_flag("tutorial/way_inn", "hidden", true);
        }
    }
}
