use super::*;

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
    /// Whether a native (browser) fullscreen is currently active.
    ///
    /// Set to `true` when `fullscreenchange` fires and a fullscreen element
    /// exists; set to `false` when the exit is fully handled (either by the
    /// `popstate` handler or by the `fullscreenchange` handler). The
    /// `popstate` handler checks this flag so that the system back button
    /// exits native fullscreen instead of navigating to the previous route.
    pub(crate) static NATIVE_FULLSCREEN_ACTIVE: Cell<bool> = const { Cell::new(false) };
    /// Indicates that the native fullscreen exit was triggered by a
    /// `popstate` event (system back gesture), so the `fullscreenchange`
    /// handler should NOT call `history.back()` — the `popstate` itself
    /// already consumed the `pushState` entry.
    pub(crate) static NATIVE_FULLSCREEN_EXIT_BY_POPSTATE: Cell<bool> = const { Cell::new(false) };
}
