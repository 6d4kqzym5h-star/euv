use super::*;

/// Represents the facing direction of the camera device.
///
/// Used to switch between the front (user-facing) and rear
/// (environment-facing) cameras.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvCameraFacing {
    /// The user-facing (front) camera.
    User,
    /// The environment-facing (rear) camera.
    #[default]
    Environment,
}
