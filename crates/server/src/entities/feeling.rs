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
        self.feeling_from(character, self.kind.feeling_about(kind))
    }

    /// How it feels about someone it starts out feeling `start` about, with
    /// what it remembers of them (players by their character; other mobs by
    /// their own id, `foes.rs`).
    pub(super) fn feeling_from(&self, who: Option<&CharacterId>, start: Feeling) -> Feeling {
        if self.escort.is_some() {
            return Feeling::Friendly;
        }
        let feeling = who.map_or(start, |who| self.memory.standing(who, start));
        // A Calm leaves it unfriendly to nobody (magic plan PR 13).
        match feeling {
            Feeling::Unfriendly if self.spells.has(shared::spell::CALMED) => Feeling::Neutral,
            feeling => feeling,
        }
    }

    /// Whether Fear lies on it: it keeps far from whoever it flees.
    pub(super) fn frightened(&self) -> bool {
        self.spells.has(shared::spell::FRIGHTENED)
    }
}
