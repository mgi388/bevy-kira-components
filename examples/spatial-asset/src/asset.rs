use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use bevy_kira_components::sources::audio_file::source::AudioFile;
use thiserror::Error;

#[derive(Asset, TypePath, Clone)]
pub struct CustomAsset {
    pub handle: Handle<AudioFile>,
}

#[derive(Default)]
pub struct CustomAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CustomAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

/// The contents of the asset just contains the path to the actual audio file.
impl AssetLoader for CustomAssetLoader {
    type Asset = CustomAsset;
    type Settings = ();
    type Error = CustomAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let file_name = String::from_utf8(bytes).unwrap().trim().to_string();
            let handle = load_context.load(file_name);
            Ok(CustomAsset { handle })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["custom"]
    }
}
