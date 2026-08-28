use super::*;

/// One link entry of the [`euv_navbar`] top navigation bar.
///
/// Internal links (hash routes) highlight when their path prefix matches the
/// current route; links starting with `http` open in a new tab.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvNavbarItem {
    /// The display text.
    #[get(type(copy))]
    pub text: &'static str,
    /// The link target (hash route path or external URL).
    #[get(type(copy))]
    pub link: &'static str,
}

/// Props for the [`euv_navbar`] component.
///
/// The trailing actions area (theme toggle, language menu, …) is supplied as
/// children so callers compose arbitrary controls.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavbarProps {
    /// The current route signal (drives the active link highlight).
    pub route_signal: Signal<String>,
    /// The brand logo text rendered inside the accent square.
    pub brand_logo: &'static str,
    /// The brand title rendered next to the logo.
    pub brand_title: &'static str,
    /// The brand home route.
    pub brand_href: &'static str,
    /// The navigation links.
    pub items: &'static [EuvNavbarItem],
    /// The mobile drawer signal; when `Some` a hamburger button is rendered
    /// on small viewports and toggles it.
    pub drawer_open: Option<Signal<bool>>,
}

/// Props of the [`euv_navbar_link`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvNavbarLinkProps {
    /// The current route signal.
    pub route_signal: Signal<String>,
    /// The navbar item to render.
    pub item: EuvNavbarItem,
}
