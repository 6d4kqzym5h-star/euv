use crate::*;

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

/// Props for the `page_component_binding` component.
#[derive(Clone, Default)]
pub(crate) struct PageComponentBindingProps;
