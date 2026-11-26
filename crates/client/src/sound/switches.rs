//! `HOW_SOUND` and `HOW_MUSIC`, read once at the start.

use crate::common::SoundSwitches;

/// Sound is off by default in debug builds (development copies, tests) and on
/// in releases. A browser has no variables, so it has the default.
const SOUND_BY_DEFAULT: bool = !cfg!(debug_assertions);

/// The switches as the variables set them.
pub(super) fn from_environment() -> SoundSwitches {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let sound = server::option_on("HOW_SOUND", SOUND_BY_DEFAULT);
        let music = server::option_on("HOW_MUSIC", true);
        SoundSwitches { sound, music }
    }
    #[cfg(target_arch = "wasm32")]
    SoundSwitches {
        sound: SOUND_BY_DEFAULT,
        music: true,
    }
}
