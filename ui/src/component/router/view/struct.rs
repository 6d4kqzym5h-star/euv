use crate::*;

/// Props for the `euv_page_router` component.
///
/// Defines the strongly-typed interface for a page router that
/// renders different content based on the current route.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvPageRouterProps {
    /// The reactive signal holding the current route.
    pub route_signal: Signal<String>,
    /// The default/fallback view when no route matches.
    #[debug(skip)]
    pub fallback: Option<Rc<dyn Fn() -> VirtualNode>>,
}

/// Configuration for a route entry in the router.
#[derive(Clone, CustomDebug)]
pub struct EuvRouteConfig {
    /// The route path to match.
    pub path: &'static str,
    /// The component to render for this route.
    #[debug(skip)]
    pub component: Rc<dyn Fn() -> VirtualNode>,
}

/// Props for declarative route-based rendering.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvRoutesProps {
    /// The reactive signal holding the current route.
    pub route_signal: Signal<String>,
    /// The list of route configurations.
    pub routes: Vec<EuvRouteConfig>,
    /// Optional fallback component when no route matches.
    #[debug(skip)]
    pub fallback: Option<Rc<dyn Fn() -> VirtualNode>>,
}
