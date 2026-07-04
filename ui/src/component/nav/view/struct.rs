use crate::*;

/// Props for the `euv_nav_item` component.
///
/// Defines the strongly-typed interface for a navigation item link.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavItemProps {
    pub route_signal: Signal<String>,
    pub icon: &'static str,
    pub label: &'static str,
    pub target: &'static str,
    #[debug(skip)]
    pub on_click: Option<ClickEventHandler>,
    pub class: Option<Css>,
}

/// Props for the `euv_mobile_nav_item` component.
///
/// Defines the strongly-typed interface for a mobile navigation item link
/// that closes the drawer on navigation.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMobileNavItemProps {
    pub route_signal: Signal<String>,
    pub drawer_open: Signal<bool>,
    pub icon: &'static str,
    pub label: &'static str,
    pub target: &'static str,
    #[debug(skip)]
    pub on_navigate: Option<NavEventCallback>,
}

/// Configuration for a navigation item.
///
/// Used with `EuvNavItemsProps` for declarative navigation configuration.
#[derive(Clone, Debug, Default)]
pub struct EuvNavItemConfig {
    /// The emoji icon for the navigation item.
    pub icon: &'static str,
    /// The display label for the navigation item.
    pub label: &'static str,
    /// The target route path.
    pub target: &'static str,
}

/// Props for the `euv_nav_items` component.
///
/// Defines the strongly-typed interface for a configurable navigation items list.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavItemsProps {
    pub route_signal: Signal<String>,
    pub items: Vec<EuvNavItemConfig>,
    pub drawer_open: Option<Signal<bool>>,
    #[debug(skip)]
    pub on_item_click: Option<NavItemClickCallback>,
}
