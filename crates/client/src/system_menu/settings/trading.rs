//! Settings' Gameplay page's "Trading" row (trading plan PR 6): how this
//! character takes trades with other players. Allow opens the trade window
//! by itself, Ask (the default) puts each to them first, Never greys them out
//! in others' lists. Kept on the character by the server, so only while
//! playing: a choice is asked of it (`SetTrading`) and shown as it answers
//! (`common::TradeSetting`).

use bevy::prelude::*;
use shared::protocol::ClientMessage;
use shared::trade::TradingSetting;

use super::rows::tip_words;
use crate::common::{Session, TradeSetting};
use crate::connection::ToServer;
use crate::dom::{Dom, UiEvent, UiValue};
use crate::widgets::UiId;

const ID: &str = "settings.gameplay/trading";

const WORDS: &str = "How other players' asks to trade are taken: Allow opens the trade window \
                     straight away, Ask puts each to you first, Never lets nobody trade with you.";

/// Each setting's value in the dropdown.
pub(in crate::system_menu) fn value(setting: TradingSetting) -> &'static str {
    match setting {
        TradingSetting::Allow => "allow",
        TradingSetting::Ask => "ask",
        TradingSetting::Never => "never",
    }
}

/// The setting a value stands for.
pub(in crate::system_menu) fn setting(value: &str) -> Option<TradingSetting> {
    match value {
        "allow" => Some(TradingSetting::Allow),
        "ask" => Some(TradingSetting::Ask),
        "never" => Some(TradingSetting::Never),
        _ => None,
    }
}

/// Shows the row, only while playing, with the setting the server keeps.
pub(in crate::system_menu) fn show_trading_row(
    setting: Res<TradeSetting>,
    session: Res<Session>,
    mut dom: Dom,
    mut shown: Local<Option<(bool, TradeSetting)>>,
) {
    let now = (session.is_playing(), *setting);
    if *shown == Some(now) {
        return;
    }
    dom.set_flag("settings.gameplay/trading_row", "hidden", !now.0);
    dom.set(ID, "value", value(setting.0));
    dom.set(
        "settings.gameplay/trading_row",
        "title",
        &tip_words(WORDS, None),
    );
    *shown = Some(now);
}

/// A choice is asked of the server, which answers with the setting kept.
pub(in crate::system_menu) fn use_trading_row(
    mut events: MessageReader<UiEvent>,
    current: Res<TradeSetting>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        if event.id != UiId::new("settings.gameplay", "trading") {
            continue;
        }
        let UiValue::Choice { value, .. } = &event.value else {
            continue;
        };
        if let Some(chosen) = setting(value).filter(|&chosen| chosen != current.0) {
            to_server.write(ToServer(ClientMessage::SetTrading(chosen)));
        }
    }
}
