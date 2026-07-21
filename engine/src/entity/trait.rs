use super::*;

/// The base trait for all entity components.
///
/// Components encapsulate behavior that can be attached to an `Entity`.
/// The engine calls lifecycle methods at appropriate times during the scheduler loop.
pub trait Component {
    /// Called once when the component is first added to an active entity
    /// in a scene.
    fn on_start(&mut self);

    /// Called every fixed timestep with the elapsed time since the last update.
    ///
    /// # Arguments
    ///
    /// - `f64` - The delta time in seconds.
    fn on_update(&mut self, delta_time: f64);

    /// Called every render frame to record the component's draw commands.
    ///
    /// # Arguments
    ///
    /// - `&mut DrawList` - The draw list to record commands into.
    /// - `&Transform2D` - The world-space transform of the owning entity.
    fn on_render(&self, draw_list: &mut DrawList, transform: &Transform2D);

    /// Called once when the component is being removed or the entity is destroyed.
    fn on_destroy(&mut self);

    /// Returns the name of this component type for debugging and identification.
    ///
    /// # Returns
    ///
    /// - `&str` - The component name.
    fn name(&self) -> &str;
}
