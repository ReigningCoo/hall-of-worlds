//! Where Bench Mode can be used, and why not elsewhere: benches are kept in a
//! site level's save, so only inside sites (the server refuses the rest).

use shared::protocol::LevelKind;

use crate::common::{CurrentDungeon, GameView};

/// Why Bench Mode can't be used here, shown in the panel; none when it can.
/// `allowed`: the account's role gives Create Crafting Bench.
pub(super) fn why_not(
    view: &GameView,
    dungeon: &CurrentDungeon,
    allowed: bool,
) -> Option<&'static str> {
    if !allowed {
        return Some("Your role doesn't allow creating benches");
    }
    let kind = dungeon.info.as_ref().map(|info| info.kind);
    match (view, kind) {
        (GameView::Dungeon, Some(LevelKind::Poi(_))) => None,
        (GameView::Dungeon, Some(LevelKind::Wilderness)) => {
            Some("Only inside a site, not in the wilderness")
        }
        (GameView::Dungeon, Some(LevelKind::Hall { .. })) => {
            Some("Only inside a site, not in the Hall of Worlds")
        }
        _ => Some("Only inside a site"),
    }
}
