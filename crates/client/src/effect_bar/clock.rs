//! The server's action clock as the client works it out: the tick it last
//! said (`ServerMessage::Clock`, after every `Sheet`), counted on by the
//! game's own `Time`, which stops while single player is paused, as the
//! server's clock does.

use bevy::prelude::*;
use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::protocol::ServerMessage;

use crate::connection::FromServer;

/// The tick the server last said, and when (`Time::elapsed_secs_f64`).
/// `None` until it has.
#[derive(Resource, Default)]
pub(super) struct ServerTick(Option<(u64, f64)>);

impl ServerTick {
    /// The action tick now, with its fraction, if the server has said one.
    pub(super) fn now(&self, elapsed: f64) -> Option<f64> {
        let (tick, at) = self.0?;
        let since = (elapsed - at).max(0.0);
        Some(tick as f64 + since * ACTION_TICKS_PER_SECOND)
    }

    /// Takes the server's word for the tick at `elapsed`.
    pub(super) fn set(&mut self, tick: u64, elapsed: f64) {
        self.0 = Some((tick, elapsed));
    }
}

/// Keeps the tick each `Clock` says.
pub(super) fn take_clock(
    mut from_server: MessageReader<FromServer>,
    time: Res<Time>,
    mut clock: ResMut<ServerTick>,
) {
    for FromServer(message) in from_server.read() {
        if let &ServerMessage::Clock { tick } = message {
            clock.set(tick, time.elapsed_secs_f64());
        }
    }
}
