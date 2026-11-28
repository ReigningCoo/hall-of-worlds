//! A short word for where the game is, written on the web page
//! (`<html data-how-stage>`, by `page::show_stage`) for the page's memory log
//! (`web/memlog.js`, with `?memlog`): `menu`, `login`, `select`, `create`,
//! `connecting`, `worldmap`, `hall`, `inn`, `wild` or `site:<kind>`. Never a
//! name, an account or anything else about the player.

use shared::poi::PoiKind;
use shared::protocol::LevelKind;

use crate::common::{GameView, LoadingStep};

/// What the stage word is made from, gathered by the system that writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Where {
    pub playing: bool,
    pub logging_in: bool,
    pub selecting: bool,
    pub making: bool,
    pub step: LoadingStep,
    pub view: GameView,
    /// The level's kind, inside a site.
    pub level: Option<LevelKind>,
}

/// The stage word for `at`.
pub(super) fn stage_word(at: Where) -> String {
    if at.making {
        return "create".into();
    }
    if at.logging_in {
        return "login".into();
    }
    if at.selecting {
        return "select".into();
    }
    if !at.playing {
        return "menu".into();
    }
    if at.step != LoadingStep::Ready || at.view == GameView::Connecting {
        return "connecting".into();
    }
    match (at.view, at.level) {
        (GameView::Dungeon, Some(LevelKind::Inn)) => "inn".into(),
        (GameView::Dungeon, Some(LevelKind::Hall { .. })) => "hall".into(),
        (GameView::Dungeon, Some(LevelKind::Wilderness)) => "wild".into(),
        (GameView::Dungeon, Some(LevelKind::Poi(kind))) => format!("site:{}", poi_word(kind)),
        (GameView::Dungeon, None) => "site".into(),
        _ => "worldmap".into(),
    }
}

/// A poi kind in lower case ("RockCave" as "rockcave").
fn poi_word(kind: PoiKind) -> String {
    format!("{kind:?}").to_lowercase()
}

#[cfg(test)]
mod tests;
