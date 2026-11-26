//! Sound settings: the player's volumes (Settings, Audio), kept in
//! `Preferences`, and whether this run plays sound at all (`HOW_SOUND`,
//! `HOW_MUSIC`, read by `sound`).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The volumes chosen in Settings, Audio, as percentages, each with its mute.
/// General is everything the game plays; Music is a share of that.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(default)]
pub struct AudioPrefs {
    pub general: u8,
    pub general_muted: bool,
    pub music: u8,
    pub music_muted: bool,
}

impl Default for AudioPrefs {
    fn default() -> Self {
        Self {
            general: 100,
            general_muted: false,
            music: 60,
            music_muted: false,
        }
    }
}

/// Whether this run may play sound: `sound` for anything (`HOW_SOUND`),
/// `music` for music only (`HOW_MUSIC`). Read once at the start; both on
/// unless `sound` says otherwise.
#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug)]
pub struct SoundSwitches {
    pub sound: bool,
    pub music: bool,
}

impl Default for SoundSwitches {
    fn default() -> Self {
        Self {
            sound: true,
            music: true,
        }
    }
}

impl SoundSwitches {
    /// Whether music may play: sound on, and music on.
    pub fn music_on(self) -> bool {
        self.sound && self.music
    }
}
