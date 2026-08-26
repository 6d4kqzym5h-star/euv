use super::*;

impl NestedRouteConfig {
    /// Creates a new nested route configuration.
    ///
    /// # Arguments
    ///
    /// - `impl Into<String>` - The route path.
    /// - `F: Fn() -> VirtualNode + 'static` - The component closure.
    /// - `Vec<NestedRouteConfig>` - The child
    ///   routes.
    pub fn new<F>(path: impl Into<String>, component: F, children: Vec<NestedRouteConfig>) -> Self
    where
        F: Fn() -> VirtualNode + 'static,
    {
        Self {
            path: path.into(),
            component: Rc::new(component),
            children,
        }
    }

    /// Returns the component closure.
    pub fn component(&self) -> Rc<dyn Fn() -> VirtualNode> {
        self.get_component().clone()
    }

    /// Returns the child routes.
    pub fn children(&self) -> &[NestedRouteConfig] {
        self.get_children()
    }
}
