//! Personal map marks (`shared::map_mark`): each character's own, per planet,
//! kept in `CharacterWorldSave::marks`. The client asks; this checks each ask
//! against the same rules and tells it what changed.

use bevy::log::warn;
use shared::id::Id;
use shared::map_mark::{MapMark, MarkColour, MarkIcon, may_add_by_hand, place};
use shared::protocol::{MarkAsk, MarkNews, ServerMessage};
use shared::spot::Spot;

use super::Client;
use crate::ServerWorld;
use crate::storage::SiteId;

impl Client {
    /// Tells the client every mark on this planet (after `Welcome`).
    pub(super) fn send_marks(&self) -> bool {
        let all = self.character.marks.clone();
        self.link.send(&ServerMessage::Marks(MarkNews::All(all)))
    }

    /// Answers one ask about marks; `false` if the client has gone. A refused
    /// one is not answered: the client only shows what it is told.
    pub(super) fn answer_mark(&mut self, ask: MarkAsk, world: &ServerWorld) -> bool {
        match ask {
            MarkAsk::Add { at, icon, colour } => self.add_mark(at, icon, colour, world.width),
            MarkAsk::Remove(id) => {
                let marks = &mut self.character.marks;
                let Some(index) = marks.iter().position(|mark| mark.id == id) else {
                    return true;
                };
                marks.remove(index);
                self.unsaved = true;
                self.link.send(&ServerMessage::Marks(MarkNews::Removed(id)))
            }
        }
    }

    fn add_mark(
        &mut self,
        at: (i64, i64),
        icon: MarkIcon,
        colour: MarkColour,
        points: i64,
    ) -> bool {
        if !may_add_by_hand(&self.character.marks) {
            warn!("{} asked for a mark past the limit", self.in_log());
            return true;
        }
        let Some(at) = place(at, points) else {
            warn!("{} asked for a mark where none can go", self.in_log());
            return true;
        };
        let mark = MapMark {
            id: crate::ids::new_id(),
            at,
            icon,
            colour,
            auto: false,
            pile: None,
        };
        self.character.marks.push(mark);
        self.unsaved = true;
        self.link.send(&ServerMessage::Marks(MarkNews::Added(mark)))
    }

    /// The player has just started a pile at `pile` on the wilderness's
    /// ground: marks it for them (white; Logs for a log pile, else Bag), unless
    /// they already have a mark on it. Not counted towards the limit.
    pub(crate) fn mark_pile(&mut self, pile: Spot, logs: bool, points: i64) {
        let marks = &mut self.character.marks;
        if marks.iter().any(|mark| mark.pile == Some(pile)) {
            return;
        }
        let Some(mark) = MapMark::for_pile(crate::ids::new_id(), pile, points, logs) else {
            return;
        };
        marks.push(mark);
        self.unsaved = true;
        self.link.send(&ServerMessage::Marks(MarkNews::Added(mark)));
    }

    /// Forgets the automatic marks whose piles are no longer on the
    /// wilderness's ground (emptied by anyone), returning their ids.
    pub(super) fn drop_gone_piles(&mut self, world: &mut ServerWorld) -> Vec<Id> {
        let marks = &mut self.character.marks;
        if !marks.iter().any(|mark| mark.pile.is_some()) {
            return Vec::new();
        }
        let level = world.site(SiteId::Wilderness).levels.get(&0);
        let lies = |pile: Spot| level.is_some_and(|level| level.ground.contains_key(&pile));
        let mut gone = Vec::new();
        marks.retain(|mark| match mark.pile {
            Some(pile) if !lies(pile) => {
                gone.push(mark.id);
                false
            }
            _ => true,
        });
        if !gone.is_empty() {
            self.unsaved = true;
        }
        gone
    }

    /// Every tick: tells the client of automatic marks whose piles have gone.
    /// `false` if the client has gone.
    pub(crate) fn forget_gone_piles(&mut self, world: &mut ServerWorld) -> bool {
        let gone = self.drop_gone_piles(world);
        (gone.into_iter()).all(|id| self.link.send(&ServerMessage::Marks(MarkNews::Removed(id))))
    }
}
