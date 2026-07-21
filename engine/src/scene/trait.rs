use super::*;

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

    /// Called every render frame to record the scene's draw commands.
    ///
    /// Instead of drawing immediately, the scene pushes `DrawCommand`s into the
    /// provided `DrawList`; the engine replays the whole list once per frame in
    /// a single batched pass.
    ///
    /// # Arguments
    ///
    /// - `&mut DrawList` - The draw list to record commands into.
    fn on_render(&self, draw_list: &mut DrawList);

    /// Returns the name of this scene for identification.
    ///
    /// # Returns
    ///
    /// - `&str` - The scene name.
    fn name(&self) -> &str;
}
