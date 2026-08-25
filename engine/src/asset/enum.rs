use super::*;

/// Categorizes the type of a game asset for appropriate loading and storage.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AssetType {
    /// An image asset (PNG, JPEG, SVG, etc.) loaded as `HtmlImageElement`.
    #[default]
    Image,
    /// An audio asset loaded as an `AudioBuffer` for Web Audio playback.
    Audio,
    /// A generic binary data asset loaded as a `Vec<u8>`.
    Data,
}

/// Represents the loading state of an asset in the cache.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AssetState {
    /// The asset has been requested but is still loading.
    #[default]
    Loading,
    /// The asset has been successfully loaded and is ready for use.
    Loaded,
    /// The asset failed to load.
    Error,
}
