use crate::*;

thread_local! {
    /// Cached safe-area inset pixel values read at initialisation.
    ///
    /// On Android, after exiting video fullscreen, `env(safe-area-inset-*)`
    /// permanently returns `0px`. These cached values are set once from the
    /// sentinel element during `cache_safe_area_insets()` and reused by
    /// `force_safe_area_reflow()` to override the CSS custom properties.
    pub(crate) static SAFE_AREA_INSET_TOP: RefCell<String> = const { RefCell::new(String::new()) };
    pub(crate) static SAFE_AREA_INSET_RIGHT: RefCell<String> = const { RefCell::new(String::new()) };
    pub(crate) static SAFE_AREA_INSET_BOTTOM: RefCell<String> = const { RefCell::new(String::new()) };
    pub(crate) static SAFE_AREA_INSET_LEFT: RefCell<String> = const { RefCell::new(String::new()) };
}
