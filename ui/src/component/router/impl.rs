use super::*;

/// Inherent implementation of [`NestedRouteConfig`].
impl NestedRouteConfig {
    /// Creates a new nested route configuration.
    ///
    /// # Arguments
    ///
    /// - `impl Into<String>` - The route path.
    /// - `F: Fn() -> VirtualNode + 'static` - The component closure.
    /// - `Vec<NestedRouteConfig>` - The child
    ///   routes.
    pub fn new<P, F>(path: P, component: F, children: Vec<NestedRouteConfig>) -> Self
    where
        P: Into<String>,
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
