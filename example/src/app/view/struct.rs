use crate::*;

/// Props for the `nav_item` component.
///
/// Defines the strongly-typed interface for a desktop navigation item link.
#[derive(Clone, Default)]
pub(crate) struct NavItemProps {
    /// The reactive signal holding the current route.
    pub(crate) route_signal: Signal<String>,
    /// The emoji icon for the navigation item.
    pub(crate) icon: &'static str,
    /// The display label for the navigation item.
    pub(crate) label: &'static str,
    /// The target route path.
    pub(crate) target: &'static str,
}

/// Props for the `mobile_nav_item` component.
///
/// Defines the strongly-typed interface for a mobile navigation item link.
#[derive(Clone, Default)]
pub(crate) struct MobileNavItemProps {
    /// The reactive signal holding the current route.
    pub(crate) route_signal: Signal<String>,
    /// The reactive signal controlling the mobile nav drawer visibility.
    pub(crate) drawer_open: Signal<bool>,
    /// The emoji icon for the navigation item.
    pub(crate) icon: &'static str,
    /// The display label for the navigation item.
    pub(crate) label: &'static str,
    /// The target route path.
    pub(crate) target: &'static str,
}

/// Props for the `build_desktop_nav_items` component.
///
/// Defines the strongly-typed interface for the desktop navigation items list.
#[derive(Clone, Default)]
pub(crate) struct BuildDesktopNavItemsProps {
    /// The reactive signal holding the current route.
    pub(crate) route_signal: Signal<String>,
}

/// Props for the `build_mobile_nav_items` component.
///
/// Defines the strongly-typed interface for the mobile navigation items list.
#[derive(Clone, Default)]
pub(crate) struct BuildMobileNavItemsProps {
    /// The reactive signal holding the current route.
    pub(crate) route_signal: Signal<String>,
    /// The reactive signal controlling the mobile nav drawer visibility.
    pub(crate) drawer_open: Signal<bool>,
}

/// Props for the `desktop_layout` component.
///
/// Defines the strongly-typed interface for the desktop application shell layout.
#[derive(Clone, Default)]
pub(crate) struct DesktopLayoutProps {
    /// The reactive signal holding the current route.
    pub(crate) route_signal: Signal<String>,
    /// The reactive signal holding the current theme (kept for API consistency).
    #[allow(dead_code)]
    pub(crate) theme_signal: Signal<String>,
    /// The reactive signal holding the root class name.
    pub(crate) root_class_signal: Signal<String>,
    /// The reactive signal controlling vconsole panel visibility.
    pub(crate) panel_open: Signal<bool>,
}

/// Props for the `mobile_layout` component.
///
/// Defines the strongly-typed interface for the mobile application shell layout.
#[derive(Clone, Default)]
pub(crate) struct MobileLayoutProps {
    /// The reactive signal holding the current route.
    pub(crate) route_signal: Signal<String>,
    /// The reactive signal holding the current theme.
    pub(crate) theme_signal: Signal<String>,
    /// The reactive signal holding the root class name.
    pub(crate) root_class_signal: Signal<String>,
    /// The reactive signal controlling vconsole panel visibility.
    pub(crate) panel_open: Signal<bool>,
    /// The reactive signal controlling the mobile nav drawer visibility.
    pub(crate) drawer_open: Signal<bool>,
}
