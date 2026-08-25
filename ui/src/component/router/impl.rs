//! Nested route matching utility.
//!
//! Provides a pure-data route matcher that supports
//! nested route configurations (parent routes with
//! child layouts). The router UI component (in
//! `ui/src/component/router/`) can use this utility
//! to resolve the active route against a tree of
//! route configs and render the matched chain.
//!
//! # Why pure data?
//!
//! The matcher is just a recursive walk over a tree of
//! route configs. No DOM access, no signal subscription,
//! no reactive dependency. This lets us:
//!
//! - Cover every match branch with native unit tests.
//! - Reuse the matcher in non-browser contexts (SSR,
//!   tests, snapshot tests in CI).
//!
//! # API
//!
//! ```ignore
//! use euv_ui::component::router::nested::{NestedRouteConfig, find_active_route};
//!
//! let routes: Vec<NestedRouteConfig> = vec![
//!     NestedRouteConfig::new(
//!         "/",
//!         || html! { div { "Home" } },
//!         vec![],
//!     ),
//!     NestedRouteConfig::new(
//!         "/settings",
//!         || html! { div { "Settings layout" } },
//!         vec![
//!             NestedRouteConfig::new(
//!                 "/settings/profile",
//!                 || html! { div { "Profile" } },
//!                 vec![],
//!             ),
//!         ],
//!     ),
//! ];
//!
//! let active: Option<&NestedRouteConfig> =
//!     find_active_route("/settings/profile", &routes);
//! // `active` points to the /settings/profile route.
//! ```
//!
//! # Path matching rules
//!
//! - An exact path match wins over a prefix match.
//! - Trailing slashes are normalized away before
//!   matching: `/settings/` and `/settings` are
//!   treated as the same path.
//! - Empty paths and `/` are treated as the root
//!   route.
use super::*;

impl NestedRouteConfig {
    /// Creates a new nested route configuration.
    ///
    /// # Arguments
    ///
    /// - `impl Into<String>` - The route path.
    /// - `F` - The component closure.
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
        self.component.clone()
    }

    /// Returns the child routes.
    pub fn children(&self) -> &[NestedRouteConfig] {
        &self.children
    }
}
