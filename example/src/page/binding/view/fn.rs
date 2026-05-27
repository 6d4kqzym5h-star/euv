use crate::*;

/// A child display component that receives strongly-typed props from parent.
///
/// Demonstrates one-way data flow: parent passes data down via props,
/// child communicates back up via callback.
///
/// # Arguments
///
/// - `VirtualNode` - The props node containing message and on_respond callback.
///
/// # Returns
///
/// - `VirtualNode` - A styled child display element.
#[component]
pub(crate) fn child_display(props: VirtualNode) -> VirtualNode {
    let ChildDisplayProps {
        message,
        on_respond,
    }: ChildDisplayProps = props.into();
    html! {
        div {
            class: c_binding_child_box()
            p {
                class: c_binding_child_label()
                "Child received:"
            }
            p {
                class: c_binding_child_message()
                message
            }
            primary_button {
                label: "Respond"
                onclick: on_respond
                "Respond to Parent"
            }
        }
    }
}

/// A limited counter component that receives strongly-typed (non-String) props.
///
/// Demonstrates passing `bool` and `i32` props through `html!` macro
/// and extracting them with `try_get_typed_prop` via `From<&VirtualNode>`.
///
/// # Arguments
///
/// - `VirtualNode` - The props node containing disabled (bool) and max_count (i32).
///
/// # Returns
///
/// - `VirtualNode` - A styled limited counter element.
#[component]
pub(crate) fn limited_counter(props: VirtualNode) -> VirtualNode {
    let LimitedCounterProps {
        disabled,
        max_count,
        on_increment,
        on_reset,
    }: LimitedCounterProps = props.into();
    html! {
        div {
            class: c_binding_child_box()
            p {
                class: c_binding_child_label()
                "Limited Counter"
            }
            p {
                class: c_demo_text()
                "Props received: disabled="
                span {
                    class: c_binding_typed_prop_value()
                    { disabled.to_string() }
                }
                ", max_count="
                span {
                    class: c_binding_typed_prop_value()
                    { max_count.to_string() }
                }
            }
            div {
                class: c_counter_row()
                primary_button {
                    label: "+1"
                    onclick: on_increment
                    disabled: disabled
                    "+1"
                }
                primary_button {
                    label: "Reset"
                    onclick: on_reset
                    disabled: disabled
                    "Reset"
                }
            }
            if { disabled } {
                p {
                    class: c_binding_typed_warning()
                    "Counter is disabled!"
                }
            } else {
                p {}
            }
        }
    }
}

/// A callback input component that receives custom callback functions as props.
///
/// Demonstrates passing `on_change` and `on_submit` callbacks through
/// `html!` macro and extracting them with `try_get_callback` via `From<&VirtualNode>`.
///
/// # Arguments
///
/// - `VirtualNode` - The props node containing on_change and on_submit callbacks.
///
/// # Returns
///
/// - `VirtualNode` - A styled input with callback element.
#[component]
pub(crate) fn callback_input(props: VirtualNode) -> VirtualNode {
    let CallbackInputProps {
        value,
        on_change,
        on_submit,
        on_reset,
    }: CallbackInputProps = props.into();
    html! {
        div {
            class: c_binding_child_box()
            p {
                class: c_binding_child_label()
                "Callback Input"
            }
            p {
                class: c_demo_text()
                "Custom callbacks: on_change, on_submit, on_reset"
            }
            div {
                class: c_binding_callback_form()
                input {
                    id: "callback-input"
                    name: "callback_input"
                    r#type: "text"
                    autocomplete: "off"
                    placeholder: "Type something..."
                    value: value
                    class: c_form_input_no_transition()
                    oninput: on_change
                }
                div {
                    class: c_counter_row()
                    primary_button {
                        label: "Submit"
                        onclick: on_submit
                        "Submit"
                    }
                    primary_button {
                        label: "Reset"
                        onclick: on_reset
                        "Reset"
                    }
                }
            }
        }
    }
}

/// A child input component that shares a Signal with parent for two-way binding.
///
/// Demonstrates two-way binding: the same Signal is used by both parent
/// and child components, so changes in either are reflected everywhere.
///
/// # Arguments
///
/// - `Signal<String>` - The shared text signal.
/// - `Signal<i32>` - The shared count signal.
///
/// # Returns
///
/// - `VirtualNode` - A styled child input element.
pub(crate) fn child_input(text_signal: Signal<String>, count_signal: Signal<i32>) -> VirtualNode {
    let text_value: String = text_signal.get();
    let count_value: i32 = count_signal.get();
    html! {
        div {
            class: c_binding_child_box()
            p {
                class: c_binding_child_label()
                "Child Component"
            }
            div {
                class: c_form_input_wrapper()
                label {
                    r#for: "child-input-text"
                    class: c_form_label()
                    "Edit shared text:"
                }
                input {
                    id: "child-input-text"
                    name: "shared_text"
                    r#type: "text"
                    autocomplete: "off"
                    value: text_value
                    class: c_form_input_no_transition()
                    oninput: on_input_value(text_signal)
                }
            }
            div {
                class: c_counter_row()
                span {
                    class: c_demo_text()
                    "Shared count: "
                    span {
                        class: c_counter_value()
                        count_value
                    }
                }
                primary_button {
                    label: "Decrement"
                    onclick: two_way_on_decrement(count_signal)
                    "-"
                }
            }
        }
    }
}

/// A temperature converter component that reactively syncs Celsius and Fahrenheit.
///
/// Demonstrates cross-component reactive binding using `watch!`:
/// changing either field automatically updates the other.
///
/// # Arguments
///
/// - `Signal<f64>` - The celsius signal.
/// - `Signal<f64>` - The fahrenheit signal.
///
/// # Returns
///
/// - `VirtualNode` - A temperature converter element.
pub(crate) fn temperature_converter(
    celsius_signal: Signal<f64>,
    fahrenheit_signal: Signal<f64>,
) -> VirtualNode {
    let celsius_value: f64 = celsius_signal.get();
    let fahrenheit_value: f64 = fahrenheit_signal.get();
    html! {
        div {
            class: c_binding_temp_converter()
            div {
                class: c_binding_temp_field()
                label {
                    r#for: "temperature-celsius"
                    class: c_form_label()
                    "Celsius"
                }
                input {
                    id: "temperature-celsius"
                    name: "celsius"
                    r#type: "number"
                    autocomplete: "off"
                    value: format!("{:.1}", celsius_value)
                    class: c_form_input_no_transition()
                    oninput: cross_on_input_celsius(celsius_signal)
                }
            }
            span {
                class: c_binding_temp_arrow()
                "="
            }
            div {
                class: c_binding_temp_field()
                label {
                    r#for: "temperature-fahrenheit"
                    class: c_form_label()
                    "Fahrenheit"
                }
                input {
                    id: "temperature-fahrenheit"
                    name: "fahrenheit"
                    r#type: "number"
                    autocomplete: "off"
                    value: format!("{:.1}", fahrenheit_value)
                    class: c_form_input_no_transition()
                    oninput: cross_on_input_fahrenheit(fahrenheit_signal)
                }
            }
        }
    }
}

/// A color mixer component that reactively syncs RGB sliders with a hex color display.
///
/// Demonstrates cross-component reactive binding: changing any RGB slider
/// automatically updates the hex color string and the color preview.
///
/// # Arguments
///
/// - `Signal<i32>` - The red channel signal.
/// - `Signal<i32>` - The green channel signal.
/// - `Signal<i32>` - The blue channel signal.
/// - `Signal<String>` - The hex color signal.
///
/// # Returns
///
/// - `VirtualNode` - A color mixer element.
pub(crate) fn color_mixer(
    red_signal: Signal<i32>,
    green_signal: Signal<i32>,
    blue_signal: Signal<i32>,
    hex_color_signal: Signal<String>,
) -> VirtualNode {
    let red_value: i32 = red_signal.get();
    let green_value: i32 = green_signal.get();
    let blue_value: i32 = blue_signal.get();
    let hex_value: String = hex_color_signal.get();
    html! {
        div {
            class: c_binding_color_mixer()
            div {
                class: c_binding_color_preview()
                style: { background: { &hex_value }; }
                span {
                    class: c_binding_color_hex()
                    hex_value
                }
            }
            div {
                class: c_binding_slider_row()
                label {
                    r#for: "color-mixer-red"
                    class: c_binding_slider_label()
                    style: { color: "#ef4444"; }
                    "R"
                }
                input {
                    id: "color-mixer-red"
                    name: "red"
                    r#type: "range"
                    autocomplete: "off"
                    min: "0"
                    max: "255"
                    value: red_value.to_string()
                    class: c_binding_slider()
                    oninput: cross_on_input_i32(red_signal)
                }
                span {
                    class: c_binding_slider_value()
                    red_value
                }
            }
            div {
                class: c_binding_slider_row()
                label {
                    r#for: "color-mixer-green"
                    class: c_binding_slider_label()
                    style: { color: "#22c55e"; }
                    "G"
                }
                input {
                    id: "color-mixer-green"
                    name: "green"
                    r#type: "range"
                    autocomplete: "off"
                    min: "0"
                    max: "255"
                    value: green_value.to_string()
                    class: c_binding_slider()
                    oninput: cross_on_input_i32(green_signal)
                }
                span {
                    class: c_binding_slider_value()
                    green_value
                }
            }
            div {
                class: c_binding_slider_row()
                label {
                    r#for: "color-mixer-blue"
                    class: c_binding_slider_label()
                    style: { color: "#3b82f6"; }
                    "B"
                }
                input {
                    id: "color-mixer-blue"
                    name: "blue"
                    r#type: "range"
                    autocomplete: "off"
                    min: "0"
                    max: "255"
                    value: blue_value.to_string()
                    class: c_binding_slider()
                    oninput: cross_on_input_i32(blue_signal)
                }
                span {
                    class: c_binding_slider_value()
                    blue_value
                }
            }
        }
    }
}

/// A component binding demo page showcasing parent-child data passing,
/// two-way binding, cross-component reactive binding, typed props,
/// and custom callback functions.
///
/// # Returns
///
/// - `VirtualNode` - The component binding demo page virtual DOM tree.
pub(crate) fn page_component_binding() -> VirtualNode {
    let props_state: UsePropsDemo = use_props_demo();
    let two_way_state: UseTwoWayDemo = use_two_way_demo();
    let cross_state: UseCrossComponentDemo = use_cross_component_demo();
    let typed_state: UseTypedPropsDemo = use_typed_props_demo();
    let callback_state: UseCustomCallbackDemo = use_custom_callback_demo();
    let callback_text: Signal<String> = callback_state.get_text_value();
    let last_event: Signal<String> = callback_state.get_last_event();
    html! {
        div {
            class: c_page_container()
            page_header("Component Binding", "Parent-child data passing, two-way binding, and cross-component reactive binding.")
            my_card {
                title: "Props Down, Callback Up"
                p {
                    class: c_demo_text()
                    "Parent passes data to child via props. Child communicates back via callbacks."
                }
                div {
                    class: c_form_input_wrapper()
                    label {
                        r#for: "binding-parent-message"
                        class: c_form_label()
                        "Parent message:"
                    }
                    input {
                        id: "binding-parent-message"
                        name: "parent_message"
                        r#type: "text"
                        autocomplete: "off"
                        value: props_state.get_parent_message()
                        class: c_form_input_no_transition()
                        oninput: on_input_value(props_state.get_parent_message())
                    }
                }
                child_display {
                    message: props_state.get_parent_message()
                    onclick: props_on_child_respond(
                        props_state.get_child_response(),
                        format!("Child says: I got \"{}\"!", props_state.get_parent_message().get()),
                    )
                }
                if { !props_state.get_child_response().get().is_empty() } {
                    div {
                        class: c_success_box()
                        props_state.get_child_response()
                    }
                } else {
                    ""
                }
            }
            my_card {
                title: "Typed Props (bool, i32)"
                p {
                    class: c_demo_text()
                    "Pass non-String props (bool, i32) through html! and extract with try_get_typed_prop."
                }
                div {
                    class: c_binding_parent_box()
                    p {
                        class: c_binding_child_label()
                        "Parent Controls"
                    }
                    div {
                        class: c_counter_row()
                        primary_button {
                            label: "Toggle Disabled"
                            onclick: typed_props_on_toggle_disabled(typed_state.get_disabled())
                            { if typed_state.get_disabled().get() { "Enable" } else { "Disable" } }
                        }
                        div {
                            class: c_binding_typed_prop_group()
                            label {
                                class: c_form_label()
                                "Max: "
                                span {
                                    class: c_binding_typed_prop_value()
                                    typed_state.get_max_count()
                                }
                            }
                        }
                    }
                    p {
                        class: c_demo_text()
                        "Count: "
                        span {
                            class: c_counter_value()
                            typed_state.get_current_count()
                        }
                        " / "
                        span {
                            class: c_binding_typed_prop_value()
                            typed_state.get_max_count()
                        }
                    }
                }
                limited_counter {
                    disabled: typed_state.get_disabled().get()
                    max_count: typed_state.get_max_count().get()
                    on_increment: typed_props_on_increment(typed_state.get_current_count(), typed_state.get_max_count(), typed_state.get_disabled())
                    on_reset: typed_props_on_reset_count(typed_state.get_current_count(), typed_state.get_disabled())
                }
            }
            my_card {
                title: "Custom Callbacks"
                p {
                    class: c_demo_text()
                    "Pass custom callback functions as component props (on_change, on_submit, on_reset)."
                }
                div {
                    class: c_binding_parent_box()
                    p {
                        class: c_binding_child_label()
                        "Parent State"
                    }
                    p {
                        class: c_demo_text()
                        "Text: "
                        span {
                            class: c_event_highlight()
                            if { callback_text.get().is_empty() } {
                                "(empty)"
                            } else {
                                callback_text
                            }
                        }
                    }
                    p {
                        class: c_demo_text()
                        "Last event: "
                        span {
                            class: c_binding_typed_prop_value()
                            { last_event.get() }
                        }
                    }
                }
                callback_input {
                    value: callback_text
                    on_change: custom_on_change(callback_state.get_text_value(), callback_state.get_last_event(), "on_change".to_string())
                    on_submit: custom_on_submit(callback_state.get_last_event(), "on_submit".to_string())
                    on_reset: custom_on_reset(callback_state.get_text_value(), callback_state.get_last_event(), "on_reset".to_string())
                }
            }
            my_card {
                title: "Two-Way Binding (Shared Signal)"
                p {
                    class: c_demo_text()
                    "Parent and child share the same Signal. Changes in either component are reflected everywhere."
                }
                div {
                    class: c_binding_parent_box()
                    p {
                        class: c_binding_child_label()
                        "Parent Component"
                    }
                    p {
                        class: c_demo_text()
                        "Text: "
                        span {
                            class: c_event_highlight()
                            two_way_state.get_shared_text()
                        }
                    }
                    p {
                        class: c_demo_text()
                        "Count: "
                        span {
                            class: c_counter_value()
                            two_way_state.get_shared_count()
                        }
                    }
                    primary_button {
                        label: "Increment"
                        onclick: two_way_on_increment(two_way_state.get_shared_count())
                        "+"
                    }
                }
                { child_input(two_way_state.get_shared_text(), two_way_state.get_shared_count()) }
            }
            my_card {
                title: "Cross-Component Reactive Binding (watch!)"
                p {
                    class: c_demo_text()
                    "Signals are linked across components using watch! Changing one automatically updates the other."
                }
                h4 {
                    class: c_binding_section_title()
                    "Temperature Converter"
                }
                p {
                    class: c_demo_text_muted()
                    "Edit either field — the other updates reactively via watch!"
                }
                { temperature_converter(cross_state.get_celsius(), cross_state.get_fahrenheit()) }
                h4 {
                    class: c_binding_section_title()
                    "Color Mixer"
                }
                p {
                    class: c_demo_text_muted()
                    "Adjust RGB sliders — the hex color updates reactively via watch!"
                }
                { color_mixer(cross_state.get_red(), cross_state.get_green(), cross_state.get_blue(), cross_state.get_hex_color()) }
            }
        }
    }
}
