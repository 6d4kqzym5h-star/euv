/// Represents the facing direction of the camera device.
///
/// Used to switch between the front (user-facing) and rear
/// (environment-facing) cameras.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum CameraFacing {
    /// The user-facing (front) camera.
    User,
    /// The environment-facing (rear) camera.
    Environment,
}
