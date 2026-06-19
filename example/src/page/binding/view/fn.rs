use crate::*;

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
pub(crate) fn limited_counter(node: VirtualNode<LimitedCounterProps>) -> VirtualNode {
    let LimitedCounterProps {
        disabled,
        max_count,
        on_increment,
        on_reset,
    }: LimitedCounterProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_binding_child_box()
            p {
                class: c_binding_child_label()
                "Limited Counter"
            }
            p {
                class: c_binding_demo_text()
                "Props received: disabled="
                span {
                    class: c_binding_typed_prop_value()
                    { disabled.get().to_string() }
                }
                ", max_count="
                span {
                    class: c_binding_typed_prop_value()
                    { max_count.get().to_string() }
                }
            }
            div {
                class: c_counter_row()
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: "+1"
                    onclick: on_increment
                    disabled: disabled
                    "+1"
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: "Reset"
                    onclick: on_reset
                    disabled: disabled
                    "Reset"
                }
            }
            if { disabled.get() } {
                p {
                    class: c_binding_typed_warning()
                    "Counter is disabled!"
                }
            }
        }
    }
}

/// A signal-based child display component that reads from and writes to shared signals.
///
/// Demonstrates Signal-based parent-child communication: both parent and child
/// share the same Signal instances, so changes in either component are
/// immediately reflected in the other.
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
                class: c_element_stack()
                label {
                    for: CHILD_INPUT_TEXT_ID
                    class: c_binding_form_label()
                    "Edit shared text:"
                }
                input {
                    id: CHILD_INPUT_TEXT_ID
                    name: CHILD_INPUT_TEXT_NAME
                    type: BINDING_TEXT_TYPE
                    autocomplete: BINDING_AUTOCOMPLETE_OFF
                    value: text_value
                    class: c_euv_input_no_transition()
                    oninput: on_input_value(text_signal)
                }
            }
            div {
                class: c_counter_text()
                "Shared count: "
                span {
                    class: c_counter_value()
                    count_value
                }
            }
            button {
                class: c_binding_compact_button()
                onclick: two_way_on_decrement(count_signal)
                "-"
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
                    for: TEMPERATURE_CELSIUS_ID
                    class: c_form_label()
                    "Celsius"
                }
                input {
                    id: TEMPERATURE_CELSIUS_ID
                    name: TEMPERATURE_CELSIUS_NAME
                    type: BINDING_NUMBER_TYPE
                    autocomplete: BINDING_AUTOCOMPLETE_OFF
                    value: format!("{:.1}", celsius_value)
                    class: c_euv_input_no_transition()
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
                    for: TEMPERATURE_FAHRENHEIT_ID
                    class: c_form_label()
                    "Fahrenheit"
                }
                input {
                    id: TEMPERATURE_FAHRENHEIT_ID
                    name: TEMPERATURE_FAHRENHEIT_NAME
                    type: BINDING_NUMBER_TYPE
                    autocomplete: BINDING_AUTOCOMPLETE_OFF
                    value: format!("{:.1}", fahrenheit_value)
                    class: c_euv_input_no_transition()
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
                        for: COLOR_MIXER_RED_ID
                        class: c_binding_slider_label()
                        style: { color: var!(color-red-channel); }
                        "R"
                    }
                input {
                    id: COLOR_MIXER_RED_ID
                    name: COLOR_MIXER_RED_NAME
                    type: BINDING_RANGE_TYPE
                    autocomplete: BINDING_AUTOCOMPLETE_OFF
                    min: COLOR_MIXER_MIN
                    max: COLOR_MIXER_MAX
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
                        for: COLOR_MIXER_GREEN_ID
                        class: c_binding_slider_label()
                        style: { color: var!(color-green-channel); }
                        "G"
                    }
                input {
                    id: COLOR_MIXER_GREEN_ID
                    name: COLOR_MIXER_GREEN_NAME
                    type: BINDING_RANGE_TYPE
                    autocomplete: BINDING_AUTOCOMPLETE_OFF
                    min: COLOR_MIXER_MIN
                    max: COLOR_MIXER_MAX
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
                        for: COLOR_MIXER_BLUE_ID
                        class: c_binding_slider_label()
                        style: { color: var!(color-blue-channel); }
                        "B"
                    }
                input {
                    id: COLOR_MIXER_BLUE_ID
                    name: COLOR_MIXER_BLUE_NAME
                    type: BINDING_RANGE_TYPE
                    autocomplete: BINDING_AUTOCOMPLETE_OFF
                    min: COLOR_MIXER_MIN
                    max: COLOR_MIXER_MAX
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

/// A component binding demo page showcasing props passing with callbacks,
/// two-way binding, and cross-component reactive binding.
///
/// # Returns
///
/// - `VirtualNode` - The component binding demo page virtual DOM tree.
#[component]
pub(crate) fn page_component_binding(node: VirtualNode<PageComponentBindingProps>) -> VirtualNode {
    let PageComponentBindingProps: PageComponentBindingProps =
        node.try_get_props().unwrap_or_default();
    let props_state: UsePropsDemo = use_props_demo();
    let two_way_state: UseTwoWayDemo = use_two_way_demo();
    let cross_state: UseCrossComponentDemo = use_cross_component_demo();
    let typed_state: UseTypedPropsDemo = use_typed_props_demo();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🔗"
                title: "Component Binding"
                subtitle: "Props passing, two-way binding, and cross-component reactive binding."
            }
            euv_card {
                title: "Props & Callbacks"
                p {
                    class: c_demo_text()
                    "Parent passes data to child via props. Child communicates back via callbacks."
                }
                div {
                    class: c_euv_input_wrapper()
                    label {
                        for: BINDING_PARENT_MESSAGE_ID
                        class: c_form_label()
                        "Parent message: "
                    }
                    input {
                        id: BINDING_PARENT_MESSAGE_ID
                        name: BINDING_PARENT_MESSAGE_NAME
                        type: BINDING_TEXT_TYPE
                        autocomplete: BINDING_AUTOCOMPLETE_OFF
                        value: props_state.get_parent_message()
                        class: c_euv_input_no_transition()
                        oninput: on_input_value(props_state.get_parent_message())
                    }
                }
                p {
                    class: c_binding_demo_text()
                    "Message: "
                    span {
                        class: c_event_highlight()
                        props_state.get_parent_message()
                    }
                }
                div {
                    class: c_binding_parent_box()
                    p {
                        class: c_binding_child_label()
                        "Typed Props Controls"
                    }
                    div {
                        class: c_button_controls()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Toggle"
                            onclick: typed_props_on_toggle_disabled(typed_state.get_disabled())
                            if { typed_state.get_disabled().get() } {
                                "Enable"
                            } else {
                                "Disable"
                            }
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
                        class: c_binding_demo_text()
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
                    disabled: typed_state.get_disabled()
                    max_count: typed_state.get_max_count()
                    on_increment: typed_props_on_increment(typed_state.get_current_count(), typed_state.get_max_count(), typed_state.get_disabled())
                    on_reset: typed_props_on_reset_count(typed_state.get_current_count(), typed_state.get_disabled())
                }
            }
            euv_card {
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
                        class: c_binding_demo_text()
                        "Text: "
                        span {
                            class: c_event_highlight()
                            two_way_state.get_shared_text()
                        }
                    }
                    p {
                        class: c_binding_demo_text()
                        "Count: "
                        span {
                            class: c_counter_value()
                            two_way_state.get_shared_count()
                        }
                    }
            button {
                class: c_binding_compact_button()
                onclick: two_way_on_increment(two_way_state.get_shared_count())
                "+"
            }
                }
                { child_input(two_way_state.get_shared_text(), two_way_state.get_shared_count()) }
            }
            euv_card {
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
                    class: c_hint()
                    "Edit either field — the other updates reactively via watch!"
                }
                { temperature_converter(cross_state.get_celsius(), cross_state.get_fahrenheit()) }
                h4 {
                    class: c_binding_section_title()
                    "Color Mixer"
                }
                p {
                    class: c_hint()
                    "Adjust RGB sliders — the hex color updates reactively via watch!"
                }
                { color_mixer(cross_state.get_red(), cross_state.get_green(), cross_state.get_blue(), cross_state.get_hex_color()) }
            }
        }
    }
}
