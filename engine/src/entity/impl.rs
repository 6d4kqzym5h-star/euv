use crate::*;

/// Implements static factory and ID generation methods for `Entity`.
impl Entity {
    /// Generates the next unique entity ID using a global atomic counter.
    ///
    /// # Returns
    ///
    /// - `u64` - The next unique ID.
    pub fn generate_id() -> u64 {
        NEXT_ENTITY_ID.fetch_add(1, Ordering::Relaxed)
    }

    /// Creates a new entity with the given name and a default identity transform.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name of the entity.
    ///
    /// # Returns
    ///
    /// - `Entity` - The newly created entity.
    pub fn create(name: &str) -> Entity {
        Entity::new(
            Self::generate_id(),
            name.to_string(),
            Transform2D::identity(),
            true,
            Vec::new(),
            Vec::new(),
        )
    }

    /// Creates a new entity at the specified position with a default name.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The initial position.
    ///
    /// # Returns
    ///
    /// - `Entity` - The newly created entity.
    pub fn create_at(position: Vector2D) -> Entity {
        let mut entity: Entity = Self::create(DEFAULT_ENTITY_NAME);
        entity.get_mut_transform().set_position(position);
        entity
    }
}

/// Implements lifecycle and component management methods for `Entity`.
impl Entity {
    /// Adds a component to this entity and calls its `on_start` lifecycle method.
    ///
    /// # Arguments
    ///
    /// - `ComponentRc` - The component to add.
    pub fn add_component(&mut self, component: ComponentRc) {
        component.borrow_mut().on_start();
        self.get_mut_components().push(component);
    }

    /// Removes the first component matching the given name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The component name to match.
    ///
    /// # Returns
    ///
    /// - `Option<ComponentRc>` - The removed component, if found.
    pub fn remove_component_by_name(&mut self, name: &str) -> Option<ComponentRc> {
        let position: Option<usize> = self
            .get_components()
            .iter()
            .position(|component: &ComponentRc| component.borrow().name() == name);
        let index: usize = position?;
        let removed: ComponentRc = self.get_mut_components().remove(index);
        removed.borrow_mut().on_destroy();
        Some(removed)
    }

    /// Returns the first component matching the given name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The component name to match.
    ///
    /// # Returns
    ///
    /// - `Option<ComponentRc>` - The matching component, if found.
    pub fn get_component_by_name(&self, name: &str) -> Option<ComponentRc> {
        self.get_components()
            .iter()
            .find(|component: &&ComponentRc| component.borrow().name() == name)
            .cloned()
    }

    /// Calls `on_update` on all active components.
    ///
    /// # Arguments
    ///
    /// - `f64` - The delta time in seconds.
    pub fn update(&mut self, delta_time: f64) {
        if !self.get_active() {
            return;
        }
        for component in self.get_components() {
            component.borrow_mut().on_update(delta_time);
        }
    }

    /// Calls `on_render` on all active components.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas rendering context.
    pub fn render(&self, context: &CanvasRenderingContext2d) {
        if !self.get_active() {
            return;
        }
        let transform: Transform2D = self.get_transform();
        for component in self.get_components() {
            component.borrow_mut().on_render(context, &transform);
        }
    }

    /// Calls `on_destroy` on all components and clears the component list.
    pub fn destroy(&mut self) {
        for component in self.get_components() {
            component.borrow_mut().on_destroy();
        }
        self.get_mut_components().clear();
    }

    /// Adds a tag string to this entity.
    ///
    /// # Arguments
    ///
    /// - `String` - The tag to add.
    pub fn add_tag(&mut self, tag: String) {
        if !self.get_tags().contains(&tag) {
            self.get_mut_tags().push(tag);
        }
    }

    /// Tests whether this entity has the given tag.
    ///
    /// # Arguments
    ///
    /// - `&str` - The tag to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the tag is present.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.get_tags().iter().any(|t: &String| t == tag)
    }
}
