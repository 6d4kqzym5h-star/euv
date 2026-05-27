use crate::*;

/// Props for the `child_display` component.
///
/// Demonstrates strongly-typed props with `String` message and optional callback.
#[derive(Data, New)]
pub(crate) struct ChildDisplayProps {
    /// The message text passed from parent to child.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) message: String,
    /// Optional click handler for child-to-parent communication.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) on_respond: Option<NativeEventHandler>,
}

/// Props for the `limited_counter` component.
///
/// Demonstrates strongly-typed non-String props (`bool`, `i32`) and custom callbacks.
#[derive(Data, New)]
pub(crate) struct LimitedCounterProps {
    /// Whether the increment button is disabled.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) disabled: bool,
    /// The maximum count value allowed.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) max_count: i32,
    /// Optional callback when increment is requested.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) on_increment: Option<NativeEventHandler>,
    /// Optional callback when reset is requested.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) on_reset: Option<NativeEventHandler>,
}

/// Props for the `callback_input` component.
///
/// Demonstrates strongly-typed custom callback props with reactive value binding.
#[derive(Data, New)]
pub(crate) struct CallbackInputProps {
    /// The reactive value signal bound to the input element.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) value: Signal<String>,
    /// Optional callback when the input value changes.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) on_change: Option<NativeEventHandler>,
    /// Optional callback when submit is requested.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) on_submit: Option<NativeEventHandler>,
    /// Optional callback when reset is requested.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) on_reset: Option<NativeEventHandler>,
}
