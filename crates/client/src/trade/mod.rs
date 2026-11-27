//! The trade window (T, or its button at the bottom left): one window for
//! trading with shops, players and NPCs near enough (trading plan). For now it
//! opens and closes an empty panel; the next PRs fill it. See
//! `docs/context/client/trade.md`.

use bevy::prelude::*;

// Split into files (`cargo xtask lint` keeps files small).
mod button;
mod icon;
mod markup;
mod open;

pub struct TradePlugin;

impl Plugin for TradePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<open::TradePanel>()
            .add_systems(Startup, button::spawn_button)
            .add_systems(Update, (button::bind_button, button::show_button))
            .add_systems(Update, (open::notice_closed, open::toggle_trade).chain());
    }
}

#[cfg(test)]
mod tests;
