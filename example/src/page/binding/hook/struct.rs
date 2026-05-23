use crate::*;

/// Reactive state for the parent-child props passing demo.
#[derive(Clone, Copy, Data, New)]
pub struct UsePropsDemo {
    /// The parent message to pass down to child.
    #[get(pub, type(copy))]
    pub parent_message: Signal<String>,
    /// The message received from child via callback.
    #[get(pub, type(copy))]
    pub child_response: Signal<String>,
}

/// Reactive state for the two-way binding demo.
#[derive(Clone, Copy, Data, New)]
pub struct UseTwoWayDemo {
    /// The shared text value bound between parent and child.
    #[get(pub, type(copy))]
    pub shared_text: Signal<String>,
    /// The shared counter value bound between parent and child.
    #[get(pub, type(copy))]
    pub shared_count: Signal<i32>,
}

/// Reactive state for the cross-component reactive binding demo.
#[derive(Clone, Copy, Data, New)]
pub struct UseCrossComponentDemo {
    /// The temperature in Celsius.
    #[get(pub, type(copy))]
    pub celsius: Signal<f64>,
    /// The temperature in Fahrenheit (derived).
    #[get(pub, type(copy))]
    pub fahrenheit: Signal<f64>,
    /// The RGB red channel value.
    #[get(pub, type(copy))]
    pub red: Signal<i32>,
    /// The RGB green channel value.
    #[get(pub, type(copy))]
    pub green: Signal<i32>,
    /// The RGB blue channel value.
    #[get(pub, type(copy))]
    pub blue: Signal<i32>,
    /// The hex color string (derived).
    #[get(pub, type(copy))]
    pub hex_color: Signal<String>,
}

/// Reactive state for the typed props demo.
///
/// Demonstrates passing non-String props (bool, i32) to child components.
#[derive(Clone, Copy, Data, New)]
pub struct UseTypedPropsDemo {
    /// Whether the child input is disabled.
    #[get(pub, type(copy))]
    pub disabled: Signal<bool>,
    /// The maximum count allowed.
    #[get(pub, type(copy))]
    pub max_count: Signal<i32>,
    /// The current count value.
    #[get(pub, type(copy))]
    pub current_count: Signal<i32>,
}

/// Reactive state for the custom callback demo.
///
/// Demonstrates passing custom callback functions as component props.
#[derive(Clone, Copy, Data, New)]
pub struct UseCustomCallbackDemo {
    /// The text input value.
    #[get(pub, type(copy))]
    pub text_value: Signal<String>,
    /// The last callback event name received.
    #[get(pub, type(copy))]
    pub last_event: Signal<String>,
}
