//! No fighting in The Way Inn: what harms (`shared::ability::Ability::harms`:
//! Attack, Kick, Trample) is greyed out there and never asked for, as the
//! server refuses it.

use bevy::prelude::*;
use shared::ability::Ability;
use shared::protocol::LevelKind;

use crate::common::{CurrentDungeon, GameView};

/// Whether the player is in The Way Inn, where nothing may harm anyone.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Peace(pub(super) bool);

impl Peace {
    /// Whether `ability` is refused here.
    pub(super) fn refuses(self, ability: Ability) -> bool {
        self.0 && ability.harms()
    }
}

/// Keeps `Peace` from the level the player is on.
pub(super) fn keep_peace(
    view: Res<State<GameView>>,
    dungeon: Res<CurrentDungeon>,
    mut peace: ResMut<Peace>,
) {
    let in_inn = *view.get() == GameView::Dungeon
        && (dungeon.info.as_ref()).is_some_and(|info| info.kind == LevelKind::Inn);
    peace.set_if_neq(Peace(in_inn));
}
