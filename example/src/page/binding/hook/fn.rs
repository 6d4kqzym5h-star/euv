use crate::*;

/// Creates props demo state signals.
///
/// # Returns
///
/// - `UsePropsDemo` - The props demo state.
pub fn use_props_demo() -> UsePropsDemo {
    UsePropsDemo::new(
        use_signal(|| "Hello from Parent!".to_string()),
        use_signal(String::new),
    )
}

/// Creates a click event handler that simulates a child responding to parent.
///
/// # Arguments
///
/// - `Signal<String>` - The child response signal.
/// - `String` - The child response message.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler.
pub fn props_on_child_respond(
    response_signal: Signal<String>,
    message: String,
) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        response_signal.set(message.clone());
    })
}

/// Creates two-way binding demo state signals.
///
/// # Returns
///
/// - `UseTwoWayDemo` - The two-way binding demo state.
pub fn use_two_way_demo() -> UseTwoWayDemo {
    UseTwoWayDemo::new(use_signal(|| "Type here...".to_string()), use_signal(|| 0))
}

/// Creates a click event handler that increments the shared counter.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler.
pub fn two_way_on_increment(counter: Signal<i32>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let current: i32 = counter.get();
        counter.set(current + 1);
    })
}

/// Creates a click event handler that decrements the shared counter.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler.
pub fn two_way_on_decrement(counter: Signal<i32>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let current: i32 = counter.get();
        counter.set(current - 1);
    })
}

/// Creates cross-component demo state signals with watch! bindings.
///
/// # Returns
///
/// - `UseCrossComponentDemo` - The cross-component demo state.
pub fn use_cross_component_demo() -> UseCrossComponentDemo {
    let state: UseCrossComponentDemo = UseCrossComponentDemo::new(
        use_signal(|| 0.0),
        use_signal(|| 32.0),
        use_signal(|| 79),
        use_signal(|| 70),
        use_signal(|| 229),
        use_signal(|| "#4f46e5".to_string()),
    );
    let celsius: Signal<f64> = state.get_celsius();
    let fahrenheit: Signal<f64> = state.get_fahrenheit();
    let red: Signal<i32> = state.get_red();
    let green: Signal<i32> = state.get_green();
    let blue: Signal<i32> = state.get_blue();
    let hex_color: Signal<String> = state.get_hex_color();
    watch!(celsius, |celsius_value| {
        fahrenheit.set(celsius_value * 9.0 / 5.0 + 32.0);
    });
    watch!(fahrenheit, |fahrenheit_value| {
        celsius.set((fahrenheit_value - 32.0) * 5.0 / 9.0);
    });
    watch!(red, green, blue, |red_value, green_value, blue_value| {
        let clamped_red: i32 = red_value.clamp(0, 255);
        let clamped_green: i32 = green_value.clamp(0, 255);
        let clamped_blue: i32 = blue_value.clamp(0, 255);
        hex_color.set(format!(
            "#{:02x}{:02x}{:02x}",
            clamped_red, clamped_green, clamped_blue
        ));
    });
    state
}

/// Creates an input event handler that updates the celsius value.
///
/// # Arguments
///
/// - `Signal<f64>` - The celsius signal.
///
/// # Returns
///
/// - `NativeEventHandler` - An input handler.
pub fn cross_on_input_celsius(signal: Signal<f64>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Input, move |event: NativeEvent| {
        if let NativeEvent::Input(input_event) = event {
            let parsed: f64 = input_event.get_value().parse().unwrap_or(0.0);
            signal.set(parsed);
        }
    })
}

/// Creates an input event handler that updates the fahrenheit value.
///
/// # Arguments
///
/// - `Signal<f64>` - The fahrenheit signal.
///
/// # Returns
///
/// - `NativeEventHandler` - An input handler.
pub fn cross_on_input_fahrenheit(signal: Signal<f64>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Input, move |event: NativeEvent| {
        if let NativeEvent::Input(input_event) = event {
            let parsed: f64 = input_event.get_value().parse().unwrap_or(0.0);
            signal.set(parsed);
        }
    })
}

/// Creates an input event handler that updates an i32 signal.
///
/// # Arguments
///
/// - `Signal<i32>` - The signal to update.
///
/// # Returns
///
/// - `NativeEventHandler` - An input handler.
pub fn cross_on_input_i32(signal: Signal<i32>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Input, move |event: NativeEvent| {
        if let NativeEvent::Input(input_event) = event {
            let parsed: i32 = input_event.get_value().parse().unwrap_or(0);
            signal.set(parsed.clamp(0, 255));
        }
    })
}

/// Creates typed props demo state signals.
///
/// # Returns
///
/// - `UseTypedPropsDemo` - The typed props demo state.
pub fn use_typed_props_demo() -> UseTypedPropsDemo {
    UseTypedPropsDemo::new(use_signal(|| false), use_signal(|| 5), use_signal(|| 0))
}

/// Creates a click event handler that toggles the disabled signal.
///
/// # Arguments
///
/// - `Signal<bool>` - The disabled signal to toggle.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler.
pub fn typed_props_on_toggle_disabled(disabled: Signal<bool>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let current: bool = disabled.get();
        disabled.set(!current);
    })
}

/// Creates a click event handler that increments the count within the max limit.
///
/// # Arguments
///
/// - `Signal<i32>` - The current count signal.
/// - `Signal<i32>` - The max count signal.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler.
pub fn typed_props_on_increment(count: Signal<i32>, max_count: Signal<i32>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let current: i32 = count.get();
        let max: i32 = max_count.get();
        if current < max {
            count.set(current + 1);
        }
    })
}

/// Creates a click event handler that resets the count to zero.
///
/// # Arguments
///
/// - `Signal<i32>` - The current count signal.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler.
pub fn typed_props_on_reset_count(count: Signal<i32>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        count.set(0);
    })
}

/// Creates custom callback demo state signals.
///
/// # Returns
///
/// - `UseCustomCallbackDemo` - The custom callback demo state.
pub fn use_custom_callback_demo() -> UseCustomCallbackDemo {
    UseCustomCallbackDemo::new(use_signal(String::new), use_signal(|| "none".to_string()))
}

/// Creates a custom callback handler that records the event name when the value changes.
///
/// # Arguments
///
/// - `Signal<String>` - The text value signal to update.
/// - `String` - The event name to record.
///
/// # Returns
///
/// - `NativeEventHandler` - A callback handler.
pub fn custom_on_change(text_signal: Signal<String>, event_name: String) -> NativeEventHandler {
    NativeEventHandler::new(
        NativeEventName::Other("on_change".to_string()),
        move |event: NativeEvent| {
            if let NativeEvent::Input(input_event) = event {
                text_signal.set(input_event.get_value().clone());
            }
            let _event_name: String = event_name.clone();
        },
    )
}

/// Creates a custom callback handler that records a submit event.
///
/// # Arguments
///
/// - `Signal<String>` - The last event signal to update.
/// - `String` - The event name to record.
///
/// # Returns
///
/// - `NativeEventHandler` - A callback handler.
pub fn custom_on_submit(last_event: Signal<String>, event_name: String) -> NativeEventHandler {
    NativeEventHandler::new(
        NativeEventName::Other("on_submit".to_string()),
        move |_event: NativeEvent| {
            last_event.set(event_name.clone());
        },
    )
}

/// Creates a custom callback handler that records a reset event.
///
/// # Arguments
///
/// - `Signal<String>` - The text value signal to clear.
/// - `Signal<String>` - The last event signal to update.
/// - `String` - The event name to record.
///
/// # Returns
///
/// - `NativeEventHandler` - A callback handler.
pub fn custom_on_reset(
    text_signal: Signal<String>,
    last_event: Signal<String>,
    event_name: String,
) -> NativeEventHandler {
    NativeEventHandler::new(
        NativeEventName::Other("on_reset".to_string()),
        move |_event: NativeEvent| {
            text_signal.set(String::new());
            last_event.set(event_name.clone());
        },
    )
}
