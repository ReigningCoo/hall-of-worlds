//! Truesight changes what the server tells of a shop's stock: asked again,
//! while nothing is in the trade (asking again starts it afresh).

use bevy::prelude::*;
use shared::protocol::ClientMessage;

use crate::common::{Trading, Truesight};
use crate::connection::ToServer;

pub(super) fn ask_again_with_truesight(
    truesight: Res<Truesight>,
    trading: Res<Trading>,
    mut to_server: MessageWriter<ToServer>,
) {
    if truesight.is_changed()
        && !truesight.is_added()
        && let Some(with) = trading.with.filter(|_| !trading.is_busy())
    {
        to_server.write(ToServer(ClientMessage::OpenTrade { with }));
    }
}
