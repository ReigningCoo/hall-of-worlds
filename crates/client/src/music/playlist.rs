//! The playlist: which tunes there are (each a MIDI file), where each comes
//! from, whether its rhythm is the source's or ours, the places it suits,
//! and the players who play them, everywhere and by place (`ensembles`). The game's is
//! `assets/music/playlists/game.playlist.ron`; mods add their own files
//! beside it (`load`), each able to add tunes, replace one by its id, or
//! change the players.

use serde::Deserialize;

use super::place::Place;

/// One `.playlist.ron` file.
#[derive(Deserialize, Clone, Debug, Default, PartialEq)]
pub(super) struct PlaylistFile {
    #[serde(default)]
    pub players: Option<Players>,
    #[serde(default)]
    pub tunes: Vec<TuneEntry>,
    /// Players by place, instead of `players` there: the last naming a
    /// place wins, so a mod's come after the game's.
    #[serde(default)]
    pub ensembles: Vec<Ensemble>,
}

/// Who plays in some places.
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub(super) struct Ensemble {
    pub places: Vec<Place>,
    pub players: Players,
}

/// A tune in the playlist.
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub(super) struct TuneEntry {
    /// Its name in the playlist (other entries' `after` use it).
    pub id: String,
    /// The MIDI file, from the assets folder.
    pub file: String,
    pub title: String,
    /// The manuscript the notes come from.
    pub source: String,
    /// The image or edition they were read from, and its licence.
    #[serde(default)]
    pub read_from: String,
    #[serde(default)]
    pub licence: String,
    pub rhythm: Rhythm,
    /// The places it plays in (none: everywhere).
    #[serde(default)]
    pub places: Vec<Place>,
    /// A tune it is played straight after (La Rotta after the Lamento).
    #[serde(default)]
    pub after: Option<String>,
}

/// Whether the source fixes the rhythm, or we chose it.
#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Rhythm {
    Written,
    Ours,
}

/// Who plays: an instrument for each part, by General MIDI's bank and
/// program (`music.sf2` keeps their numbers).
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub(super) struct Players {
    pub melody: Instrument,
    /// The second player, playing the melody's beats an octave down.
    #[serde(default)]
    pub second: Option<Instrument>,
    /// Holding the final and its fifth.
    #[serde(default)]
    pub drone: Option<Instrument>,
    #[serde(default)]
    pub percussion: Option<Percussion>,
    /// Roughly how long a link between tunes lasts (1 to 3 bars).
    pub link_seconds: f32,
    /// What joins in when the player is attacked (none: no combat music,
    /// as in The Way Inn and on the world map).
    #[serde(default)]
    pub combat: Option<Combat>,
}

impl Default for Players {
    /// A recorder alone, for playlists that name no players.
    fn default() -> Self {
        Self {
            melody: Instrument {
                bank: 0,
                program: 74,
            },
            second: None,
            drone: None,
            percussion: None,
            link_seconds: 5.0,
            combat: None,
        }
    }
}

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Instrument {
    pub bank: u8,
    pub program: u8,
}

/// Drum keys on the standard kit: the downbeat's and the rest's.
#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Percussion {
    pub strong: u8,
    pub light: u8,
}

/// The combat parts: a drum's key on the standard kit for the roll and the
/// ostinato, and the stinger's instrument (none: the drum's accent).
#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Combat {
    pub drum: u8,
    #[serde(default)]
    pub stinger: Option<Stinger>,
}

/// The stinger's accent: an instrument (a horn, trumpet or bell) playing the
/// tune's final in the fourth octave, `above` semitones higher.
#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Stinger {
    pub instrument: Instrument,
    #[serde(default)]
    pub above: u8,
}

/// The files read together, in order: later players win, and a tune with a
/// known id takes its place.
pub(super) fn merge(files: impl IntoIterator<Item = PlaylistFile>) -> PlaylistFile {
    let mut merged = PlaylistFile::default();
    for file in files {
        if file.players.is_some() {
            merged.players = file.players;
        }
        merged.ensembles.extend(file.ensembles);
        for tune in file.tunes {
            match merged.tunes.iter_mut().find(|known| known.id == tune.id) {
                Some(known) => *known = tune,
                None => merged.tunes.push(tune),
            }
        }
    }
    merged
}

/// The order tunes play in: the playlist's, with each `after` tune moved to
/// straight after the one it follows (when that one is there).
pub(super) fn in_order(tunes: &[TuneEntry]) -> Vec<TuneEntry> {
    let mut ordered: Vec<TuneEntry> = Vec::new();
    for tune in tunes.iter().filter(|tune| follows(tune, tunes).is_none()) {
        push_with_followers(tune, tunes, &mut ordered);
    }
    // Tunes following each other round in a circle: in the playlist's order.
    for tune in tunes {
        push_with_followers(tune, tunes, &mut ordered);
    }
    ordered
}

/// The tune this one follows, if it is in the list.
fn follows<'a>(tune: &'a TuneEntry, tunes: &[TuneEntry]) -> Option<&'a String> {
    let after = tune.after.as_ref()?;
    let there = tunes.iter().any(|t| &t.id == after && t.id != tune.id);
    there.then_some(after)
}

fn push_with_followers(tune: &TuneEntry, tunes: &[TuneEntry], ordered: &mut Vec<TuneEntry>) {
    if ordered.iter().any(|known| known.id == tune.id) {
        return;
    }
    ordered.push(tune.clone());
    for follower in tunes.iter().filter(|t| follows(t, tunes) == Some(&tune.id)) {
        push_with_followers(follower, tunes, ordered);
    }
}

#[cfg(test)]
mod tests;
