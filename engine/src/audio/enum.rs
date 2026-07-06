/// Represents the playback state of an audio clip.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AudioPlayState {
    /// The audio is currently playing.
    #[default]
    Playing,
    /// The audio is paused and can be resumed.
    Paused,
    /// The audio has stopped or finished playing.
    Stopped,
}
