//! The trade window (T, or its button at the bottom left): one window for
//! trading with shops, players and NPCs near enough (trading plan). For now
//! shops (PR 4): who to trade with, the own side, the middle, the shop's
//! stock, and Complete trade; players and NPCs come in PR 6. See
//! `docs/context/client/trade.md`.

use bevy::prelude::*;

use crate::common::Trading;
use crate::player::PlayerSystems;

// Split into files (`cargo xtask lint` keeps files small).
mod button;
#[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
mod debug_api;
mod grids;
mod icon;
mod markup;
mod open;
mod server;
mod shops;
mod show;
mod state;
mod truesight;
mod words;

pub struct TradePlugin;

impl Plugin for TradePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<state::TradeWindow>()
            .init_resource::<Trading>()
            .add_systems(Startup, button::spawn_button)
            .add_systems(Update, (button::bind_button, button::show_button))
            .add_systems(
                Update,
                (
                    open::notice_closed,
                    open::toggle_trade,
                    server::read_server,
                    server::follow_carried,
                    truesight::ask_again_with_truesight,
                    shops::show_shops,
                    shops::choose_shop,
                    grids::note_gifts,
                    grids::bind_grids,
                    grids::draw_grids,
                    grids::show_aim,
                    show::press_complete,
                    show::show_middle,
                )
                    .chain()
                    .after(PlayerSystems::Travel),
            );
        #[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
        debug_api::add(app);
    }
}

#[cfg(test)]
mod tests;
