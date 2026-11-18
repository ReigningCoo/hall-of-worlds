//! How a mob's feeling about a player is shown to them, and blocks their
//! steps: what it really feels, but unfriendly while it is after them.

use shared::creature::CreatureKind;
use shared::protocol::CharacterId;
use shared::sheet::Feeling;

use super::Mob;
use crate::combat::Act;

impl Mob {
    /// Its feeling (`Mob::feeling`) as the player sees it: `Unfriendly` while it
    /// has them in its sights (`Mob::target`, a player's entity id being its
    /// `CharacterId`'s) or is winding up a blow at them (a cornered timid one),
    /// so they can fight back and can't walk through it. Back to the real
    /// feeling once it gives up. Mob AI keeps to `feeling`.
    pub(crate) fn feeling_shown(
        &self,
        kind: CreatureKind,
        character: Option<&CharacterId>,
    ) -> Feeling {
        let feeling = self.feeling(kind, character);
        if self.escort.is_some() {
            return feeling;
        }
        let Some(&CharacterId(id)) = character else {
            return feeling;
        };
        let striking = (self.fight.pending).is_some_and(|pending| {
            pending.target == id && matches!(pending.act, Act::Strike(_) | Act::Kick(_))
        });
        if self.target == Some(id) || striking {
            Feeling::Unfriendly
        } else {
            feeling
        }
    }
}
