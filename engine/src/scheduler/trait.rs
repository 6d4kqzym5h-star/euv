/// The trait that game implementations must fulfill to receive
/// fixed-timestep update and interpolated render callbacks from the scheduler.
pub trait TickHandler {
    /// Called at a fixed interval defined by `SchedulerConfig::fixed_timestep`.
    ///
    /// All game logic, physics, and state mutations should happen here.
    ///
    /// # Arguments
    ///
    /// - `f64` - The fixed delta time in seconds.
    fn on_update(&mut self, delta_time: f64);

    /// Called once per animation frame after all accumulated updates have been processed.
    ///
    /// The interpolation factor represents how far between fixed updates the current
    /// frame is, allowing smooth rendering at variable frame rates.
    ///
    /// # Arguments
    ///
    /// - `f64` - The interpolation factor in the range 0.0 to 1.0.
    fn on_render(&mut self, interpolation: f64);
}
