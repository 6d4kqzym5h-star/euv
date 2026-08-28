use super::*;

/// One node of the [`euv_sidebar`] navigation tree.
///
/// A node with empty `children` is a leaf link; a node with children is a
/// collapsible group whose `link` (when set) navigates to the group index
/// page without toggling the group.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvSidebarItem {
    /// The display text.
    #[get(type(copy))]
    pub text: &'static str,
    /// The leaf route; groups may link to their index page.
    #[get(type(copy))]
    pub link: Option<&'static str>,
    /// Nested children (non-empty renders a collapsible group).
    #[get(type(copy))]
    pub children: &'static [EuvSidebarItem],
}

/// Props for the [`euv_sidebar`] component.
///
/// The `collapsed` signal is caller-owned so several sidebars (desktop aside
/// and mobile drawer) can share the collapse state; `prefix` is internal and
/// used by the recursion to key groups.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarProps {
    /// The current route signal (drives the active link highlight).
    pub route_signal: Signal<String>,
    /// The collapsed group keys.
    pub collapsed: Signal<Vec<String>>,
    /// The tree items.
    pub items: &'static [EuvSidebarItem],
    /// The group key prefix (internal recursion state, leave default).
    pub prefix: String,
}

/// Props of the [`euv_sidebar_item`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarItemProps {
    /// The current route signal.
    pub route_signal: Signal<String>,
    /// The collapsed group keys.
    pub collapsed: Signal<Vec<String>>,
    /// The item to render.
    pub item: EuvSidebarItem,
    /// The group key prefix of the parent.
    pub prefix: String,
}
