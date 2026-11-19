//! Personal map marks (`shared::map_mark`): each character's own, per planet,
//! kept in `CharacterWorldSave::marks`. The client asks; this checks each ask
//! against the same rules and tells it what changed.

use bevy::log::warn;
use shared::map_mark::{MapMark, MarkColour, MarkIcon, may_add_by_hand, place};
use shared::protocol::{MarkAsk, MarkNews, ServerMessage};

use super::Client;
use crate::ServerWorld;

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
        };
        self.character.marks.push(mark);
        self.unsaved = true;
        self.link.send(&ServerMessage::Marks(MarkNews::Added(mark)))
    }
}
