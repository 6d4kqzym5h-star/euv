use crate::*;

/// Props for the `page_router` component.
///
/// Defines the strongly-typed interface for the route-based page resolver.
#[derive(Default)]
pub(crate) struct PageRouterProps {
    /// The reactive signal holding the current route path.
    pub(crate) route_signal: Signal<String>,
}
