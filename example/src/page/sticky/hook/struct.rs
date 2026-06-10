use crate::*;

/// Reactive state for the sticky and CSS effects demo feature.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseSticky {
    /// Whether the sticky header is enabled.
    #[get(type(copy))]
    pub(crate) sticky_enabled: Signal<bool>,
    /// Whether the glassmorphism effect is enabled.
    #[get(type(copy))]
    pub(crate) glass_enabled: Signal<bool>,
    /// Whether the text shadow effect is enabled.
    #[get(type(copy))]
    pub(crate) text_shadow_enabled: Signal<bool>,
    /// Whether the clip path effect is enabled.
    #[get(type(copy))]
    pub(crate) clip_path_enabled: Signal<bool>,
    /// Whether the overflow scroll shadow is visible.
    #[get(type(copy))]
    pub(crate) scroll_shadow_visible: Signal<bool>,
}
