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

/// A nested route configuration entry.
///
/// Routes form a tree: each route can have its own
/// component (the "layout" or "page") and zero or more
/// child routes. The path of a child route is resolved
/// against the parent path during matching.
#[derive(Clone, CustomDebug)]
pub struct NestedRouteConfig {
    /// The route path (e.g. `/settings/profile`).
    pub path: String,
    /// The component to render for this route.
    #[debug(skip)]
    pub component: Rc<dyn Fn() -> VirtualNode>,
    /// The child routes, if any.
    pub children: Vec<NestedRouteConfig>,
}

impl NestedRouteConfig {
    /// Creates a new nested route configuration.
    ///
    /// # Arguments
    ///
    /// - `path: impl Into<String>` - The route path.
    /// - `component: F` - The component closure.
    /// - `children: Vec<NestedRouteConfig>` - The child
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

/// Normalizes a path by stripping a trailing `/` (except
/// for the root path `/`, which stays as `/`).
pub fn normalize_path(path: &str) -> String {
    if path == "/" {
        return "/".to_string();
    }
    path.trim_end_matches('/').to_string()
}

/// Returns `true` if `path` matches the route's pattern.
///
/// Matching rules:
/// - Exact match after normalization wins.
/// - For a parent route with children, a path that
///   starts with `route.path + "/"` matches the parent
///   (so `/settings` matches `/settings/profile`).
/// - The root route `/` matches everything except the
///   empty string.
pub fn route_matches(route_path: &str, request_path: &str) -> bool {
    let normalized_route: String = normalize_path(route_path);
    let normalized_request: String = normalize_path(request_path);
    if normalized_route == normalized_request {
        return true;
    }
    // Parent-route match: request starts with
    // route_path + "/".
    if normalized_request.starts_with(&normalized_route)
        && normalized_request.chars().nth(normalized_route.len()) == Some('/')
    {
        return true;
    }
    // Root catch-all.
    if normalized_route == "/" && !normalized_request.is_empty() {
        return true;
    }
    false
}

/// Recursively finds the deepest matching route in the
/// configuration tree.
///
/// Exact matches on children win over parent-prefix
/// matches. If multiple children match, the first one
/// in declaration order wins (matching `euv_routes`'s
/// existing "first match wins" semantics).
///
/// Returns `None` if no route matches.
pub fn find_active_route<'a>(
    path: &str,
    routes: &'a [NestedRouteConfig],
) -> Option<&'a NestedRouteConfig> {
    // First pass: exact match wins.
    for route in routes.iter() {
        let normalized: String = normalize_path(&route.path);
        let normalized_request: String = normalize_path(path);
        if normalized == normalized_request {
            // Try to find a deeper match in the
            // children of this route.
            if !route.children.is_empty() {
                if let Some(child_match) = find_active_route(path, &route.children) {
                    return Some(child_match);
                }
            }
            return Some(route);
        }
    }
    // Second pass: parent-prefix match. Walk all
    // routes; for each, if it could be a parent of the
    // request, recurse into its children. If a child
    // matches, return the child. If no child matches,
    // return the parent.
    for route in routes.iter() {
        if route_matches(&route.path, path) && route.path != normalize_path(path) {
            if !route.children.is_empty() {
                if let Some(child_match) = find_active_route(path, &route.children) {
                    return Some(child_match);
                }
            }
            return Some(route);
        }
    }
    None
}

/// Returns the chain of routes from the root to the
/// matched route, in render order (root first).
///
/// This is the "breadcrumb" chain that the layout
/// component walks to render `<Outlet>` for each parent.
/// Returns an empty vec if no route matches.
pub fn route_chain<'a>(path: &str, routes: &'a [NestedRouteConfig]) -> Vec<&'a NestedRouteConfig> {
    let mut chain: Vec<&'a NestedRouteConfig> = Vec::new();
    build_chain(path, routes, &mut chain);
    chain
}

fn build_chain<'a>(
    path: &str,
    routes: &'a [NestedRouteConfig],
    chain: &mut Vec<&'a NestedRouteConfig>,
) -> bool {
    // First pass: exact match.
    for route in routes.iter() {
        let normalized: String = normalize_path(&route.path);
        let normalized_request: String = normalize_path(path);
        if normalized == normalized_request {
            chain.push(route);
            // Recurse into children for deeper matches.
            if !route.children.is_empty() {
                let _ = build_chain(path, &route.children, chain);
            }
            return true;
        }
    }
    // Second pass: parent-prefix match.
    for route in routes.iter() {
        if route_matches(&route.path, path) && route.path != normalize_path(path) {
            chain.push(route);
            if !route.children.is_empty() {
                let _ = build_chain(path, &route.children, chain);
            }
            return true;
        }
    }
    false
}
