use crate::*;

/// Props for the `child_display` component.
///
/// Demonstrates strongly-typed props with `String` message and optional callback.
#[derive(Clone, Default)]
pub(crate) struct ChildDisplayProps {
    /// The message text passed from parent to child.
    pub(crate) message: Signal<String>,
    /// Optional click handler for child-to-parent communication.
    pub(crate) on_respond: Option<Rc<dyn Fn(Event)>>,
}

/// Props for the `limited_counter` component.
///
/// Demonstrates strongly-typed non-String props (`bool`, `i32`) and custom callbacks.
#[derive(Clone, Default)]
pub(crate) struct LimitedCounterProps {
    /// Whether the increment button is disabled.
    pub(crate) disabled: Signal<bool>,
    /// The maximum count value allowed.
    pub(crate) max_count: Signal<i32>,
    /// Optional callback when increment is requested.
    pub(crate) on_increment: Option<Rc<dyn Fn(Event)>>,
    /// Optional callback when reset is requested.
    pub(crate) on_reset: Option<Rc<dyn Fn(Event)>>,
}

/// Props for the `callback_input` component.
///
/// Demonstrates strongly-typed custom callback props with reactive value binding.
#[derive(Clone, Default)]
pub(crate) struct CallbackInputProps {
    /// The reactive value signal bound to the input element.
    pub(crate) value: Signal<String>,
    /// Optional callback when the input value changes.
    pub(crate) on_change: Option<Rc<dyn Fn(Event)>>,
    /// Optional callback when submit is requested.
    pub(crate) on_submit: Option<Rc<dyn Fn(Event)>>,
    /// Optional callback when reset is requested.
    pub(crate) on_reset: Option<Rc<dyn Fn(Event)>>,
}

/// Props for the `signal_child_display` component.
///
/// Demonstrates Signal-based parent-child communication where the child
/// reads from and writes to shared signals passed down from the parent.
#[derive(Clone, Default)]
pub(crate) struct SignalChildDisplayProps {
    /// The shared message signal for parent-to-child and child-to-parent text communication.
    pub(crate) message: Signal<String>,
    /// The shared counter signal for parent-child counter synchronization.
    pub(crate) counter: Signal<i32>,
}

/// Props for the `page_component_binding` component.
#[derive(Clone, Default)]
pub(crate) struct PageComponentBindingProps;
