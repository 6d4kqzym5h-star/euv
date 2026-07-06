use crate::*;

/// The base trait for all game scenes.
///
/// A scene encapsulates a self-contained portion of the game world,
/// managing its own game objects, resources, and update logic.
pub trait Scene {
    /// Called once when this scene becomes the active scene.
    fn on_enter(&mut self);

    /// Called once when this scene is being replaced or removed.
    fn on_exit(&mut self);

    /// Called every fixed timestep to update game logic.
    ///
    /// # Arguments
    ///
    /// - `f64` - The delta time in seconds.
    fn on_update(&mut self, delta_time: f64);

    /// Called every render frame to draw the scene.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas rendering context.
    fn on_render(&self, context: &CanvasRenderingContext2d);

    /// Returns the name of this scene for identification.
    ///
    /// # Returns
    ///
    /// - `&str` - The scene name.
    fn name(&self) -> &str;
}
