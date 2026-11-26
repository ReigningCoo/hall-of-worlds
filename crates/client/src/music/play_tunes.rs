//! Choosing what the sequencer plays: by default the tunes and players of
//! the place the player is in (`PlaceNow`), changing at the next bar line
//! when it changes; every tune before the game shows a place. The debug
//! menu can instead play every tune in turn, or one alone, round and round,
//! and go back to music by place.

use std::collections::HashMap;

use bevy::prelude::*;

use super::library::{Library, Ready};
use super::place::Place;
use super::place_now::PlaceNow;
use super::sequencer::Program;
use super::sets::{players_for, tunes_for};
use super::start::Music;
use super::stream::Command;

/// What to play: by place, every tune, or one by its place in `TuneList`.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayTunes {
    ByPlace,
    All,
    One(usize),
}

/// What the sequencer was last told to play.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Told {
    All,
    One(usize),
    Place(Place),
}

/// What was asked last, what the sequencer has been told, and how far round
/// each place's tunes have been started.
#[derive(Resource)]
pub(super) struct Asked {
    pub tunes: PlayTunes,
    told: Option<Told>,
    /// A place's tunes start one further on each time it is come back to.
    turns: HashMap<Place, usize>,
}

impl Default for Asked {
    fn default() -> Self {
        Self {
            tunes: PlayTunes::ByPlace,
            told: None,
            turns: HashMap::new(),
        }
    }
}

/// Tells the sequencer what to play once music plays and the tunes are in,
/// at once when asked from the debug menu, and at the next bar line when
/// the place changes.
pub(super) fn play_tunes(
    mut asking: MessageReader<PlayTunes>,
    mut asked: ResMut<Asked>,
    music: Res<Music>,
    library: Res<Library>,
    place: Res<PlaceNow>,
) {
    let mut at_once = false;
    if let Some(&tunes) = asking.read().last() {
        (asked.tunes, asked.told, at_once) = (tunes, None, true);
    }
    let (Some(shared), Library::Ready(ready)) = (music.shared(), &*library) else {
        return;
    };
    let wanted = match (asked.tunes, place.place) {
        (PlayTunes::ByPlace, Some(place)) => Told::Place(place),
        (PlayTunes::ByPlace | PlayTunes::All, _) => Told::All,
        (PlayTunes::One(at), _) => Told::One(at),
    };
    if asked.told == Some(wanted) {
        return;
    }
    let first = asked.told.is_none();
    asked.told = Some(wanted);
    let Some(program) = program(wanted, ready, &mut asked.turns) else {
        return;
    };
    shared.command(match first || at_once {
        true => Command::Play(program),
        false => Command::Change(program),
    });
}

/// The tunes and players for what is wanted (none if there are no tunes).
fn program(wanted: Told, ready: &Ready, turns: &mut HashMap<Place, usize>) -> Option<Program> {
    let (tunes, players) = match wanted {
        Told::All => (ready.tunes.clone(), ready.players.clone()),
        Told::One(at) => (
            ready.tunes.get(at).cloned().into_iter().collect(),
            ready.players.clone(),
        ),
        Told::Place(place) => {
            let mut chosen: Vec<_> = tunes_for(place, &ready.places)
                .into_iter()
                .map(|at| ready.tunes[at].clone())
                .collect();
            let turn = turns.entry(place).or_default();
            if !chosen.is_empty() {
                let start = *turn % chosen.len();
                chosen.rotate_left(start);
            }
            *turn += 1;
            let players = players_for(place, &ready.ensembles, &ready.players);
            (chosen, players)
        }
    };
    (!tunes.is_empty()).then_some(Program { tunes, players })
}
