//! What the debug panel reads and sets while music plays: the tune playing,
//! as the stream reports it, and whether combat music is forced on.

use bevy::prelude::*;

use super::start::Music;

/// The tune playing, by its place in `TuneList` (none while no music plays).
#[derive(Resource, Default, Debug, PartialEq, Eq)]
pub struct PlayingTune(pub Option<usize>);

/// While on, combat music plays as if the player were attacked all the time;
/// off, it eases back after the usual quiet hold. Real attacks work alongside.
#[derive(Resource, Default, Debug)]
pub struct ForcedCombat(pub bool);

/// Copies the stream's tune out and the forcing in.
pub(super) fn follow_now(
    music: Res<Music>,
    forced: Res<ForcedCombat>,
    mut playing: ResMut<PlayingTune>,
) {
    let shared = music.shared();
    if let Some(shared) = shared {
        shared.set_forced(forced.0);
    }
    let now = shared.and_then(|shared| shared.tune());
    if playing.0 != now {
        playing.0 = now;
    }
}
