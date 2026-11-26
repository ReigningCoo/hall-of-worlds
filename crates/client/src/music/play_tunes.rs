//! Choosing what the sequencer plays. Until music by place (PR 4), every
//! tune in the playlist's order, linked one into the next; the debug menu's
//! Play Tune... plays one alone, round and round.

use bevy::prelude::*;

use super::library::Library;
use super::sequencer::Program;
use super::start::Music;
use super::stream::Command;

/// What to play: every tune, or one by its place in `TuneList`.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayTunes {
    All,
    One(usize),
}

/// What was asked last, and whether the sequencer has been told.
#[derive(Resource)]
pub(super) struct Asked {
    pub tunes: PlayTunes,
    pub told: bool,
}

impl Default for Asked {
    fn default() -> Self {
        Self {
            tunes: PlayTunes::All,
            told: false,
        }
    }
}

/// Tells the sequencer what to play once music plays and the tunes are in,
/// and again whenever something else is asked for.
pub(super) fn play_tunes(
    mut asking: MessageReader<PlayTunes>,
    mut asked: ResMut<Asked>,
    music: Res<Music>,
    library: Res<Library>,
) {
    if let Some(&tunes) = asking.read().last() {
        *asked = Asked { tunes, told: false };
    }
    let (Some(shared), Library::Ready(ready)) = (music.shared(), &*library) else {
        return;
    };
    if asked.told {
        return;
    }
    asked.told = true;
    let tunes = match asked.tunes {
        PlayTunes::All => ready.tunes.clone(),
        PlayTunes::One(at) => ready.tunes.get(at).cloned().into_iter().collect(),
    };
    if tunes.is_empty() {
        return;
    }
    let players = ready.players.clone();
    shared.command(Command::Play(Program { tunes, players }));
}
