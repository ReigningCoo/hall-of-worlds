//! The web page's own loading screen (`index.html`, filled by `web/loader.js`)
//! until this one takes over: saying "Starting..." as `main` begins, marking
//! when each stage is reached for the page's figures, and removing it; and
//! the game's stage on the page for its memory log (`show_stage`).

use bevy::prelude::*;
use wasm_bindgen::JsCast;

use super::stage::{Where, stage_word};
use crate::common::{
    CharacterSelect, CurrentDungeon, GameView, LoadingStep, LoggingIn, MakingCharacter, Session,
};

/// Marks a moment in the browser's timeline (`performance.mark`), which the
/// page reads to log and keep how long each stage took.
pub fn mark(name: &str) {
    if let Some(performance) = web_sys::window().and_then(|window| window.performance()) {
        let _ = performance.mark(name);
    }
}

/// As `main` begins, before the heavy setup: the page says "Starting..."
/// (`window.howLoading.stage`). It only shows once the page's thread is free
/// again, but marks when the game began.
pub fn starting() {
    mark("how-main");
    let window = js_sys::global();
    let Ok(loading) = js_sys::Reflect::get(&window, &"howLoading".into()) else {
        return;
    };
    let stage = js_sys::Reflect::get(&loading, &"stage".into());
    if let Ok(stage) = stage.and_then(|stage| stage.dyn_into::<js_sys::Function>()) {
        let _ = stage.call1(&loading, &"starting".into());
    }
}

/// Marks the first frame the game runs.
pub(super) fn mark_first_frame(mut marked: Local<bool>) {
    if !*marked {
        *marked = true;
        mark("how-first-frame");
    }
}

/// Removes the screen in `index.html`, shown during the download, once this one
/// has certainly been drawn to take its place.
pub(super) fn remove_page_screen(step: Res<LoadingStep>, mut removed: Local<bool>) {
    if *removed || *step == LoadingStep::Starting {
        return;
    }
    *removed = true;
    mark("how-page-screen-removed");
    let screen = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("loading"));
    if let Some(screen) = screen {
        screen.remove();
    }
}

/// Writes where the game is on the page (`<html data-how-stage>`), for the
/// page's memory log (`web/memlog.js`), only when it changes.
#[allow(clippy::too_many_arguments)]
pub(super) fn show_stage(
    session: Res<Session>,
    logging_in: Res<LoggingIn>,
    select: Res<CharacterSelect>,
    making: Res<MakingCharacter>,
    step: Res<LoadingStep>,
    view: Res<State<GameView>>,
    dungeon: Res<CurrentDungeon>,
    mut shown: Local<String>,
) {
    let word = stage_word(Where {
        playing: session.is_playing(),
        logging_in: logging_in.shows(),
        selecting: select.open,
        making: making.0,
        step: *step,
        view: *view.get(),
        level: dungeon.info.as_ref().map(|info| info.kind),
    });
    if *shown == word {
        return;
    }
    let page = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element());
    if let Some(page) = page {
        let _ = page.set_attribute("data-how-stage", &word);
    }
    *shown = word;
}
