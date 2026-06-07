use crate::*;

/// Reactive state for the native bridge system info feature.
///
/// Aggregates all signals needed for the bridge permissions display.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseNativeBridge {
    /// Whether the native bridge is available on this platform.
    #[get(type(copy))]
    pub(crate) available: Signal<bool>,
    /// Whether data is currently being loaded from the native bridge.
    #[get(type(copy))]
    pub(crate) loading: Signal<bool>,
    /// The resolved bridge group permissions.
    #[get(type(copy))]
    pub(crate) permissions: Signal<String>,
}
