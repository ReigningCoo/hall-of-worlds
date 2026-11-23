//! A mob's speed and how it feels about someone.

use shared::creature::CreatureKind;
use shared::protocol::CharacterId;
use shared::sheet::Feeling;

use super::Mob;

impl Mob {
    /// How fast it acts: energy a tick.
    pub(super) fn speed(&self) -> u32 {
        self.speed
    }

    /// How it feels about a creature of this kind, played by this character, with
    /// what it remembers of them.
    pub(super) fn feeling(&self, kind: CreatureKind, character: Option<&CharacterId>) -> Feeling {
        if self.escort.is_some() {
            return Feeling::Friendly;
        }
        let start = self.kind.feeling_about(kind);
        character.map_or(start, |character| self.memory.standing(character, start))
    }
}
