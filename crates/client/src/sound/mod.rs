//! Sound: whether this run plays any (`HOW_SOUND`, `HOW_MUSIC`, into
//! `common::SoundSwitches`), and the volumes chosen in Settings, Audio, put
//! into play: General as Bevy's `GlobalVolume`, Music as `MusicVolume` for
//! whatever plays music. Nothing plays yet.

mod switches;
mod volume;

pub use volume::MusicVolume;

use bevy::prelude::*;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(switches::from_environment())
            .init_resource::<MusicVolume>()
            .add_systems(Update, volume::apply_volumes);
    }
}

#[cfg(test)]
mod tests;
