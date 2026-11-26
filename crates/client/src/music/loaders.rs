//! Tunes and playlists as Bevy assets: `.mid` files read by `midi`, and
//! `.playlist.ron` files.

use std::io;
use std::sync::Arc;

use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;

use super::midi::read_tune;
use super::playlist::PlaylistFile;
use super::tune::Tune;

/// A tune read from a MIDI file.
#[derive(Asset, TypePath)]
pub(super) struct TuneAsset(pub Arc<Tune>);

/// A playlist file.
#[derive(Asset, TypePath)]
pub(super) struct PlaylistAsset(pub PlaylistFile);

#[derive(Default, TypePath)]
pub(super) struct TuneLoader;

#[derive(Default, TypePath)]
pub(super) struct PlaylistLoader;

fn invalid(error: impl ToString) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}

impl AssetLoader for TuneLoader {
    type Asset = TuneAsset;
    type Settings = ();
    type Error = io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<TuneAsset, io::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let tune = read_tune(&bytes).map_err(invalid)?;
        Ok(TuneAsset(Arc::new(tune)))
    }

    fn extensions(&self) -> &[&str] {
        &["mid", "midi"]
    }
}

impl AssetLoader for PlaylistLoader {
    type Asset = PlaylistAsset;
    type Settings = ();
    type Error = io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<PlaylistAsset, io::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let file = ron::de::from_bytes(&bytes).map_err(invalid)?;
        Ok(PlaylistAsset(file))
    }

    fn extensions(&self) -> &[&str] {
        &["playlist.ron"]
    }
}
