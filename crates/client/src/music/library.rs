//! The tunes, loaded once music starts: the playlists (natively every
//! `.playlist.ron` in `music/playlists/`, the game's first and mods' after
//! in name order; in a browser only the game's), merged and ordered, then
//! each tune's MIDI file. A tune that can't be read is left out with a
//! warning.

use std::sync::Arc;

use bevy::asset::LoadState;
use bevy::prelude::*;

use super::loaders::{PlaylistAsset, TuneAsset};
use super::playlist::{Players, PlaylistFile, TuneEntry, in_order, merge};
use super::start::Music;
use super::tune::Tune;

/// The game's own playlist.
const GAME_PLAYLIST: &str = "music/playlists/game.playlist.ron";
/// Where mods put theirs, beside it.
#[cfg(not(target_arch = "wasm32"))]
const PLAYLISTS: &str = "music/playlists";

/// Every tune's title, in the order they play (the debug menu lists them).
#[derive(Resource, Default, Debug)]
pub struct TuneList(pub Vec<String>);

/// The tunes, ready to play.
pub(super) struct Ready {
    pub tunes: Vec<Arc<Tune>>,
    pub players: Players,
}

#[derive(Resource, Default)]
pub(super) enum Library {
    /// Music hasn't started.
    #[default]
    Idle,
    #[cfg(not(target_arch = "wasm32"))]
    Folder(Handle<bevy::asset::LoadedFolder>),
    Playlists(Vec<Handle<PlaylistAsset>>),
    Tunes(PlaylistFile, Vec<Handle<TuneAsset>>),
    Ready(Ready),
}

/// Moves the loading on, a step whenever what it waits for is in.
pub(super) fn load_library(
    mut library: ResMut<Library>,
    music: Res<Music>,
    asset_server: Res<AssetServer>,
    #[cfg(not(target_arch = "wasm32"))] folders: Res<Assets<bevy::asset::LoadedFolder>>,
    playlists: Res<Assets<PlaylistAsset>>,
    tunes: Res<Assets<TuneAsset>>,
    mut list: ResMut<TuneList>,
) {
    let next = match &*library {
        Library::Idle if music.shared().is_some() => Some(start(&asset_server)),
        #[cfg(not(target_arch = "wasm32"))]
        Library::Folder(folder) => match folders.get(folder) {
            Some(folder) => Some(Library::Playlists(playlists_in(folder))),
            None if failed(&asset_server, folder.id().untyped()) => {
                warn!("Music: couldn't list {PLAYLISTS}; playing the game's playlist only");
                Some(Library::Playlists(vec![asset_server.load(GAME_PLAYLIST)]))
            }
            None => None,
        },
        Library::Playlists(handles) if all_done(&asset_server, handles) => {
            let read = handles.iter().filter_map(|handle| playlists.get(handle));
            let mut merged = merge(read.map(|asset| asset.0.clone()));
            merged.tunes = in_order(&merged.tunes);
            let files = merged.tunes.iter().map(|t| asset_server.load(&t.file));
            let handles = files.collect();
            Some(Library::Tunes(merged, handles))
        }
        Library::Tunes(playlist, handles) if all_done(&asset_server, handles) => {
            let ready = ready(playlist, handles, &tunes);
            list.0 = ready.1;
            info!("Music: {} tunes", list.0.len());
            Some(Library::Ready(ready.0))
        }
        _ => None,
    };
    if let Some(next) = next {
        *library = next;
    }
}

fn start(asset_server: &AssetServer) -> Library {
    #[cfg(not(target_arch = "wasm32"))]
    return Library::Folder(asset_server.load_folder(PLAYLISTS));
    #[cfg(target_arch = "wasm32")]
    return Library::Playlists(vec![asset_server.load(GAME_PLAYLIST)]);
}

/// The playlists in the folder: the game's first, the rest by name.
#[cfg(not(target_arch = "wasm32"))]
fn playlists_in(folder: &bevy::asset::LoadedFolder) -> Vec<Handle<PlaylistAsset>> {
    let mut found: Vec<(bool, String, Handle<PlaylistAsset>)> = folder
        .handles
        .iter()
        .filter_map(|handle| {
            let path = handle.path()?.to_string();
            let typed = handle.clone().try_typed::<PlaylistAsset>().ok()?;
            Some((!path.ends_with("game.playlist.ron"), path, typed))
        })
        .collect();
    found.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    found.into_iter().map(|(.., handle)| handle).collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn failed(asset_server: &AssetServer, id: bevy::asset::UntypedAssetId) -> bool {
    matches!(asset_server.load_state(id), LoadState::Failed(_))
}

/// Whether every handle has loaded or failed.
fn all_done<A: Asset>(asset_server: &AssetServer, handles: &[Handle<A>]) -> bool {
    handles.iter().all(|handle| {
        let state = asset_server.load_state(handle);
        state.is_loaded() || matches!(state, LoadState::Failed(_))
    })
}

/// The tunes that loaded, and their titles; the rest left out.
fn ready(
    playlist: &PlaylistFile,
    handles: &[Handle<TuneAsset>],
    tunes: &Assets<TuneAsset>,
) -> (Ready, Vec<String>) {
    let mut loaded = Vec::new();
    let mut titles = Vec::new();
    for (entry, handle) in playlist.tunes.iter().zip(handles) {
        match tunes.get(handle) {
            Some(tune) => {
                loaded.push(tune.0.clone());
                titles.push(title_of(entry));
            }
            None => warn!(
                "Music: left out {}: {} couldn't be read",
                entry.id, entry.file
            ),
        }
    }
    let players = playlist.players.clone().unwrap_or_default();
    let ready = Ready {
        tunes: loaded,
        players,
    };
    (ready, titles)
}

fn title_of(entry: &TuneEntry) -> String {
    match entry.title.is_empty() {
        true => entry.id.clone(),
        false => entry.title.clone(),
    }
}
