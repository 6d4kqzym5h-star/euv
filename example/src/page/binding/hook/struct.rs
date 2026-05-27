use crate::*;

/// Reactive state for the parent-child props passing demo.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UsePropsDemo {
    /// The parent message to pass down to child.
    #[get(type(copy))]
    pub(crate) parent_message: Signal<String>,
    /// The message received from child via callback.
    #[get(type(copy))]
    pub(crate) child_response: Signal<String>,
}

/// Reactive state for the two-way binding demo.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseTwoWayDemo {
    /// The shared text value bound between parent and child.
    #[get(type(copy))]
    pub(crate) shared_text: Signal<String>,
    /// The shared counter value bound between parent and child.
    #[get(type(copy))]
    pub(crate) shared_count: Signal<i32>,
}

/// Reactive state for the cross-component reactive binding demo.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseCrossComponentDemo {
    /// The temperature in Celsius.
    #[get(type(copy))]
    pub(crate) celsius: Signal<f64>,
    /// The temperature in Fahrenheit (derived).
    #[get(type(copy))]
    pub(crate) fahrenheit: Signal<f64>,
    /// The RGB red channel value.
    #[get(type(copy))]
    pub(crate) red: Signal<i32>,
    /// The RGB green channel value.
    #[get(type(copy))]
    pub(crate) green: Signal<i32>,
    /// The RGB blue channel value.
    #[get(type(copy))]
    pub(crate) blue: Signal<i32>,
    /// The hex color string (derived).
    #[get(type(copy))]
    pub(crate) hex_color: Signal<String>,
}

/// Reactive state for the typed props demo.
///
/// Demonstrates passing non-String props (bool, i32) to child components.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseTypedPropsDemo {
    /// Whether the child input is disabled.
    #[get(type(copy))]
    pub(crate) disabled: Signal<bool>,
    /// The maximum count allowed.
    #[get(type(copy))]
    pub(crate) max_count: Signal<i32>,
    /// The current count value.
    #[get(type(copy))]
    pub(crate) current_count: Signal<i32>,
}

/// Reactive state for the custom callback demo.
///
/// Demonstrates passing custom callback functions as component props.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseCustomCallbackDemo {
    /// The text input value.
    #[get(type(copy))]
    pub(crate) text_value: Signal<String>,
    /// The last callback event name received.
    #[get(type(copy))]
    pub(crate) last_event: Signal<String>,
}
