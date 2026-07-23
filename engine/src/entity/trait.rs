use super::*;

/// The common lifecycle stage shared by `Component` and `Scene`.
///
/// Both abstractions have a per-tick update callback (`on_update`), and both
/// share the same `f64` delta-time argument. Lifting that single method into
/// a supertrait lets generic code iterate over mixed collections (e.g. a
/// registry of components and scenes) and call `on_update` uniformly.
///
/// `Component` adds `on_start`, `on_render` with a `Transform2D`, `on_destroy`,
/// and `name`. `Scene` adds `on_enter`, `on_render` without a transform,
/// `on_exit`, and `name`. `Lifecycle` intentionally exposes only the shared
/// surface — the domain-specific entry/exit and render hooks stay on the
/// sub-traits where their semantics differ.
pub trait Lifecycle {
    /// Advances the object's internal state by the given delta time.
    ///
    /// # Arguments
    ///
    /// - `f64` - The delta time in seconds since the last update.
    fn on_update(&mut self, delta_time: f64);
}

/// The base trait for all entity components.
///
/// Components encapsulate behavior that can be attached to an `Entity`.
/// The engine calls lifecycle methods at appropriate times during the scheduler loop.
pub trait Component: Lifecycle {
    /// Called once when the component is first added to an active entity
    /// in a scene.
    fn on_start(&mut self);

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
