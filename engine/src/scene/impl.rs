use crate::*;

/// Implements static scene creation for `SceneManager`.
impl SceneManager {
    /// Creates a new `SceneRc` wrapping the given scene in a reference-counted cell.
    ///
    /// # Arguments
    ///
    /// - `T` - The concrete scene type implementing `Scene`.
    ///
    /// # Returns
    ///
    /// - `SceneRc` - The wrapped scene.
    pub fn create_scene<T>(scene: T) -> SceneRc
    where
        T: Scene + 'static,
    {
        Rc::new(RefCell::new(scene))
    }
}

/// Implements scene registration and lifecycle management for `SceneManager`.
impl SceneManager {
    /// Registers a scene under the given name.
    ///
    /// # Arguments
    ///
    /// - `String` - The name to register the scene under.
    /// - `SceneRc` - The scene to register.
    pub fn register(&mut self, name: String, scene: SceneRc) {
        self.get_mut_scenes().insert(name, scene);
    }

    /// Unregisters and removes the scene with the given name.
    ///
    /// # Arguments
    ///
    /// - `N: AsRef<str>` - The name of the scene to remove.
    pub fn unregister<N>(&mut self, name: N)
    where
        N: AsRef<str>,
    {
        self.get_mut_scenes().remove(name.as_ref());
    }

    /// Transitions to the scene with the given name.
    ///
    /// Calls `on_exit` on the current scene and `on_enter` on the new scene.
    /// Returns `false` if no scene with the given name is registered.
    ///
    /// # Arguments
    ///
    /// - `N: AsRef<str>` - The name of the scene to switch to.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the transition was successful.
    pub fn switch_to<N>(&mut self, name: N) -> bool
    where
        N: AsRef<str>,
    {
        let name_ref: &str = name.as_ref();
        if !self.get_scenes().contains_key(name_ref) {
            return false;
        }
        let current_name: Option<String> = self.get_mut_current_scene_name().clone();
        if let Some(name) = current_name.as_ref()
            && let Some(current_scene) = self.get_scenes().get(name)
        {
            current_scene.borrow_mut().on_exit();
        }
        self.set_current_scene_name(Some(name_ref.to_string()));
        if let Some(new_scene) = self.get_scenes().get(name_ref) {
            new_scene.borrow_mut().on_enter();
        }
        true
    }

    /// Requests a deferred scene transition to be applied on the next update.
    ///
    /// # Arguments
    ///
    /// - `String` - The name of the scene to switch to.
    pub fn request_transition(&mut self, name: String) {
        self.set_pending_scene_name(Some(name));
    }

    /// Processes a pending scene transition if one was requested.
    pub fn process_pending_transition(&mut self) {
        let Some(name) = self.get_mut_pending_scene_name().take() else {
            return;
        };
        self.switch_to(&name);
    }

    /// Calls `on_update` on the current scene.
    ///
    /// # Arguments
    ///
    /// - `f64` - The delta time in seconds.
    pub fn update(&mut self, delta_time: f64) {
        self.process_pending_transition();
        let current_name: Option<String> = self.get_mut_current_scene_name().clone();
        let Some(current_name) = current_name.as_ref() else {
            return;
        };
        let Some(scene) = self.get_scenes().get(current_name) else {
            return;
        };
        scene.borrow_mut().on_update(delta_time);
    }

    /// Calls `on_render` on the current scene.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas rendering context.
    pub fn render(&self, context: &CanvasRenderingContext2d) {
        let Some(current_name) = self.current_scene_name.as_ref() else {
            return;
        };
        let Some(scene) = self.get_scenes().get(current_name) else {
            return;
        };
        scene.borrow().on_render(context);
    }

    /// Returns whether a scene with the given name is registered.
    ///
    /// # Arguments
    ///
    /// - `N: AsRef<str>` - The scene name to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the scene is registered.
    pub fn has_scene<N>(&self, name: N) -> bool
    where
        N: AsRef<str>,
    {
        self.get_scenes().contains_key(name.as_ref())
    }

    /// Returns the name of the currently active scene.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - The current scene name, or `None`.
    pub fn current_name(&self) -> Option<&str> {
        self.current_scene_name.as_deref()
    }
}

/// Implements `Default` for `SceneManager` as a new empty manager.
impl Default for SceneManager {
    fn default() -> SceneManager {
        SceneManager::new()
    }
}
