//! The oil counted down here between the server's word (it tells the fount
//! again every 10 ml and when a lantern goes out): lanterns held or tied burn
//! by playtime, which stops 5 minutes after the player last did anything, as
//! the server counts it. Burnt a millilitre at a time into `inventory::Carried`,
//! so whatever draws a lantern (its fuel bar, the panel) follows.

use bevy::prelude::*;
use shared::lantern::SECONDS_PER_ML;
use shared::protocol::ClientMessage;

use crate::connection::ToServer;
use crate::inventory::Carried;

/// Playtime stops counting this long after the player last did anything, in
/// seconds: the server's `ACTIVE_FOR`, 5 minutes.
const ACTIVE_FOR: f64 = 5.0 * 60.0;

/// When the player last did something, by the real clock, and the seconds
/// burnt not yet taken from the oil.
#[derive(Resource, Default)]
pub(super) struct Activity {
    last_done: f64,
    owed: f64,
}

/// Notes the player doing something: anything sent but what happens by itself.
pub(super) fn note_activity(
    mut sent: MessageReader<ToServer>,
    time: Res<Time<Real>>,
    mut activity: ResMut<Activity>,
) {
    if sent.read().any(|ToServer(message)| shows_activity(message)) {
        activity.last_done = time.elapsed_secs_f64();
    }
}

/// Whether sending this is the player doing something, as the server's
/// `playtime::shows_activity` reads it, near enough: it puts the oil right.
fn shows_activity(message: &ClientMessage) -> bool {
    !matches!(
        message,
        ClientMessage::Ping { .. } | ClientMessage::LookUpItems(_)
    )
}

/// Burns the lanterns held or tied by the playtime passing, a millilitre at a
/// time.
pub(super) fn burn_locally(
    time: Res<Time<Real>>,
    mut activity: ResMut<Activity>,
    mut carried: ResMut<Carried>,
) {
    let now = time.elapsed_secs_f64();
    let active = now - activity.last_done <= ACTIVE_FOR;
    if !active || !carried.0.lantern_lit() {
        activity.owed = 0.0;
        return;
    }
    activity.owed += time.delta_secs_f64();
    if activity.owed >= f64::from(SECONDS_PER_ML) {
        let secs = activity.owed.floor();
        activity.owed -= secs;
        carried.0.burn_lanterns(secs as u32);
    }
}
