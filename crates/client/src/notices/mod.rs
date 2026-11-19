//! Short notices at the top middle of the window that go by themselves (a
//! snackbar): for now "Found" when the player finds a poi in the wilderness.
//! See `docs/context/client/notices.md`.

use bevy::prelude::*;
use shared::item::ItemKind;
use shared::protocol::{Alchemised, Landed, PickNews, ServerMessage};

use crate::common::{HerbTable, words};
use crate::connection::FromServer;
use crate::dom::{Dom, Template, document, top_strip};
use crate::html;

/// How long a notice shows, in seconds.
const SHOWN_FOR: f32 = 4.0;

/// Notices.
pub struct NoticesPlugin;

impl Plugin for NoticesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Shown>()
            .add_systems(Startup, spawn_notice)
            .add_systems(Update, show_notices);
    }
}

/// The notice: a panel in the middle of a row across the window, under the
/// bar of timed effects (`effect_bar`). The pointer is never on it (`inert`).
static NOTICES: Template = html!(
    "notices",
    r#"
<form id="notices">
  <div id="row" top="72" left="0" right="0" align="center" inert hidden>
    <panel id="notice"><p id="words" class="medium"/></panel>
  </div>
</form>
"#
);

/// How long the notice showing has left, in seconds (0: none).
#[derive(Resource, Default)]
struct Shown(f32);

fn spawn_notice(mut commands: Commands) {
    // Along the top of the window, for the row to be placed from.
    commands.spawn(document(&NOTICES)).insert(top_strip());
}

/// What a found poi's notice says.
fn found(kind: shared::poi::PoiKind) -> String {
    format!("Found: {}", words(&format!("{kind:?}")))
}

/// What a harvest says: "Picked 3 Roseroot" (picking teaches the name).
fn picked(count: u32, herb: &str, table: &HerbTable) -> String {
    let name = table.0.get(herb).map_or(herb, |def| &def.name);
    format!("Picked {count} {name}")
}

/// What a door found locked says.
const LOCKED: &str = "Locked";

/// Shows a notice when the server says something worth one, the newest in
/// place of any showing, and hides it after `SHOWN_FOR`.
fn show_notices(
    mut from_server: MessageReader<FromServer>,
    time: Res<Time>,
    mut shown: ResMut<Shown>,
    mut dom: Dom,
    mut showing: Local<bool>,
    table: Res<HerbTable>,
) {
    for FromServer(message) in from_server.read() {
        let words = match message {
            &ServerMessage::PoiFound { kind } => found(kind),
            ServerMessage::DoorRefused { locked: true, .. } => LOCKED.to_owned(),
            ServerMessage::Pick(PickNews::Picked { herb, count, .. }) => {
                picked(*count, herb, &table)
            }
            ServerMessage::Pick(PickNews::PickedFruit { fruit, count, .. }) => {
                let fruits = shared::fruit::Fruits::built_in();
                let name = fruits.get(fruit).map_or(fruit.as_str(), |def| &def.name);
                format!("Picked {count} {name}")
            }
            ServerMessage::Pick(PickNews::PickedVegetable {
                vegetable, count, ..
            }) => {
                let vegetables = shared::vegetable::Vegetables::built_in();
                let name = (vegetables.get(vegetable)).map_or(vegetable.as_str(), |def| &def.name);
                format!("Picked {count} {name}")
            }
            ServerMessage::AbilityLanded {
                who: None,
                landed: Landed::Alchemy { made, left },
                ..
            } => alchemised(*made, *left),
            ServerMessage::AbilityLanded {
                who: None,
                landed: Landed::Juiced(ml),
                ..
            } => format!("Pressed {ml} ml of juice"),
            ServerMessage::AbilityLanded {
                who: None,
                landed: Landed::Fermented { days },
                ..
            } => fermenting(*days),
            ServerMessage::VinegarSoon(cask) => format!("{cask} turns to vinegar soon"),
            _ => continue,
        };
        dom.set_text("notices/words", words);
        shown.0 = SHOWN_FOR;
    }
    if shown.0 > 0.0 {
        shown.0 -= time.delta_secs();
    }
    let now = shown.0 > 0.0;
    if *showing != now {
        *showing = now;
        dom.set_flag("notices/row", "hidden", !now);
    }
}

/// What a run of Alchemy made: "Made 2 Rare potions", that it failed or came to
/// nothing (the ingredients are gone all the same), and the runs left.
fn alchemised(made: Alchemised, left: u32) -> String {
    let made = match made {
        Alchemised::Failed => "The process failed".to_owned(),
        Alchemised::Nothing => "The mix came to nothing".to_owned(),
        Alchemised::Made {
            kind: ItemKind::Bottle,
            count,
            quality,
        } => match count {
            1 => format!("Made a {quality:?} potion"),
            count => format!("Made {count} {quality:?} potions"),
        },
        Alchemised::Made { kind, quality, .. } => format!("Made {quality:?} {}", kind.def().name),
    };
    match left {
        0 => made,
        1 => format!("{made}: 1 more to go"),
        left => format!("{made}: {left} more to go"),
    }
}

/// A cask set fermenting: "Fermenting: ready in 36 days".
fn fermenting(days: u32) -> String {
    match days {
        1 => "Fermenting: ready in a day".to_owned(),
        days => format!("Fermenting: ready in {days} days"),
    }
}

#[cfg(test)]
mod tests;
