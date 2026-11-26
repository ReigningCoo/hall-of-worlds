//! Combat music's cue: each hostile action involving the player
//! (`common::Attacked`) is passed to the sequencer, which brings in its
//! combat parts on the next beat, or keeps them going (`sequencer::combat`).

use bevy::prelude::*;

use super::start::Music;
use super::stream::Command;
use crate::common::Attacked;

/// Tells the sequencer of each hostile action, once music plays.
pub(super) fn follow_attacks(mut attacked: MessageReader<Attacked>, music: Res<Music>) {
    if attacked.read().count() == 0 {
        return;
    }
    if let Some(shared) = music.shared() {
        shared.command(Command::Attacked);
    }
}
