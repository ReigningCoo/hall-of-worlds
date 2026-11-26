//! The SoundFont as an asset: `assets/music/music.sf2`, read by rustysynth.
//! Loaded only when music first plays (`start`), so a browser with music off
//! never downloads it.

use std::io;
use std::sync::Arc;

use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use rustysynth::SoundFont;

/// Where the SoundFont is, under the assets folder.
pub(super) const PATH: &str = "music/music.sf2";

/// A SoundFont's instruments, shared with each synthesizer made from it.
#[derive(Asset, TypePath)]
pub(super) struct SoundFontAsset(pub Arc<SoundFont>);

/// Reads `.sf2` files.
#[derive(Default, TypePath)]
pub(super) struct SoundFontLoader;

impl AssetLoader for SoundFontLoader {
    type Asset = SoundFontAsset;
    type Settings = ();
    type Error = io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<SoundFontAsset, io::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let font = SoundFont::new(&mut bytes.as_slice())
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, format!("{error:?}")))?;
        Ok(SoundFontAsset(Arc::new(font)))
    }

    fn extensions(&self) -> &[&str] {
        &["sf2"]
    }
}
