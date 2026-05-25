use crate::*;

/// Props for the `child_display` component.
///
/// Demonstrates strongly-typed props with `String` message and optional callback.
pub(crate) struct ChildDisplayProps {
    /// The message text passed from parent to child.
    pub(crate) message: String,
    /// Optional click handler for child-to-parent communication.
    pub(crate) on_respond: Option<NativeEventHandler>,
}

/// Props for the `limited_counter` component.
///
/// Demonstrates strongly-typed non-String props (`bool`, `i32`) and custom callbacks.
pub(crate) struct LimitedCounterProps {
    /// Whether the increment button is disabled.
    pub(crate) disabled: bool,
    /// The maximum count value allowed.
    pub(crate) max_count: i32,
    /// Optional callback when increment is requested.
    pub(crate) on_increment: Option<NativeEventHandler>,
    /// Optional callback when reset is requested.
    pub(crate) on_reset: Option<NativeEventHandler>,
}

/// Props for the `callback_input` component.
///
/// Demonstrates strongly-typed custom callback props.
pub(crate) struct CallbackInputProps {
    /// Optional callback when the input value changes.
    pub(crate) on_change: Option<NativeEventHandler>,
    /// Optional callback when submit is requested.
    pub(crate) on_submit: Option<NativeEventHandler>,
    /// Optional callback when reset is requested.
    pub(crate) on_reset: Option<NativeEventHandler>,
}
