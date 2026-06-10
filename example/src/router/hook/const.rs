/// The debounce interval in milliseconds for the resize event handler.
pub(crate) const RESIZE_DEBOUNCE_MILLIS: i32 = 16;

/// The `window.open` target name that forces the URL to open in the
/// system browser rather than a web-view tab.
pub(crate) const SYSTEM_BROWSER_TARGET: &str = "_system";

/// CSS selector for the mobile navigation drawer element.
pub(crate) const DRAWER_NAV_SELECTOR: &str = "nav.c_mobile_nav_drawer";

/// CSS selector for the currently active navigation item link inside the mobile drawer.
pub(crate) const ACTIVE_NAV_ITEM_SELECTOR: &str = ".c_nav_item_active";

/// CSS selector for the scrollable navigation items container inside the nav drawer.
pub(crate) const NAV_ITEMS_SCROLL_SELECTOR: &str = ".c_nav_items_scroll";
