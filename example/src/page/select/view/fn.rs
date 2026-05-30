use crate::*;

/// A select demo page showcasing dropdown and cascading selections.
///
/// # Returns
///
/// - `VirtualNode` - The select demo page virtual DOM tree.
#[component]
pub(crate) fn page_select(mut node: VirtualNode<PageSelectProps>) -> VirtualNode {
    let PageSelectProps = node.try_take_props().unwrap_or_default();
    let state: UseSelect = use_select();
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Select & Textarea"
                subtitle: "Dropdown selection, cascading selects, and textarea binding."
            }
            my_card {
                title: "Simple Select"
                div {
                    class: c_form_input_wrapper()
                    label {
                        r#for: SELECT_FRUIT_ID
                        class: c_form_label()
                        "Choose a fruit"
                    }
                    select {
                        id: SELECT_FRUIT_ID
                        name: SELECT_FRUIT_NAME
                        autocomplete: SELECT_AUTOCOMPLETE_OFF
                        class: c_select_input()
                        value: state.get_selected_fruit()
                        onchange: on_change_value(state.get_selected_fruit())
                        option {
                            value: "apple"
                            "Apple"
                        }
                        option {
                            value: "banana"
                            "Banana"
                        }
                        option {
                            value: "cherry"
                            "Cherry"
                        }
                        option {
                            value: "durian"
                            "Durian"
                        }
                    }
                }
                p {
                    class: c_event_result()
                    "Selected: "
                    span {
                        class: c_event_highlight()
                        { state.get_selected_fruit().get() }
                    }
                }
            }
            my_card {
                title: "Cascading Select"
                div {
                    class: c_form_input_wrapper()
                    label {
                        r#for: SELECT_COUNTRY_ID
                        class: c_form_label()
                        "Country"
                    }
                    select {
                        id: SELECT_COUNTRY_ID
                        name: SELECT_COUNTRY_NAME
                        autocomplete: SELECT_AUTOCOMPLETE_COUNTRY
                        class: c_select_input()
                        onchange: select_on_country_change(state)
                        option {
                            value: ""
                            "-- Select Country --"
                        }
                        option {
                            value: "china"
                            "China"
                        }
                        option {
                            value: "japan"
                            "Japan"
                        }
                        option {
                            value: "usa"
                            "USA"
                        }
                    }
                }
                if { state.get_selected_country().get() == "china" } {
                    div {
                        class: c_form_input_wrapper()
                        label {
                            r#for: SELECT_CITY_ID
                            class: c_form_label()
                            "City"
                        }
                        select {
                            id: SELECT_CITY_ID
                            name: SELECT_CITY_NAME
                            autocomplete: SELECT_AUTOCOMPLETE_OFF
                            class: c_select_input()
                            onchange: on_change_value(state.get_selected_city())
                            option { value: "none" "-- Select City --" }
                            option { value: "beijing" "Beijing" }
                            option { value: "shanghai" "Shanghai" }
                            option { value: "guangzhou" "Guangzhou" }
                        }
                    }
                } else if { state.get_selected_country().get() == "japan" } {
                    div {
                        class: c_form_input_wrapper()
                        label {
                            r#for: SELECT_CITY_ID
                            class: c_form_label()
                            "City"
                        }
                        select {
                            id: SELECT_CITY_ID
                            name: SELECT_CITY_NAME
                            autocomplete: SELECT_AUTOCOMPLETE_OFF
                            class: c_select_input()
                            onchange: on_change_value(state.get_selected_city())
                            option { value: "none" "-- Select City --" }
                            option { value: "tokyo" "Tokyo" }
                            option { value: "osaka" "Osaka" }
                            option { value: "kyoto" "Kyoto" }
                        }
                    }
                } else if { state.get_selected_country().get() == "usa" } {
                    div {
                        class: c_form_input_wrapper()
                        label {
                            r#for: SELECT_CITY_ID
                            class: c_form_label()
                            "City"
                        }
                        select {
                            id: SELECT_CITY_ID
                            name: SELECT_CITY_NAME
                            autocomplete: SELECT_AUTOCOMPLETE_OFF
                            class: c_select_input()
                            onchange: on_change_value(state.get_selected_city())
                            option { value: "none" "-- Select City --" }
                            option { value: "new-york" "New York" }
                            option { value: "los-angeles" "Los Angeles" }
                            option { value: "chicago" "Chicago" }
                        }
                    }
                }
                if { !state.get_selected_city().get().is_empty() } {
                    div {
                        class: c_success_box()
                        "You selected: "
                        span {
                            class: c_event_highlight()
                            { state.get_selected_city().get() }
                        }
                    }
                }
            }
            my_card {
                title: "Textarea with Feedback"
                div {
                    class: c_form_input_wrapper()
                    label {
                        r#for: SELECT_FEEDBACK_ID
                        class: c_form_label()
                        "Your feedback"
                    }
                    textarea {
                        id: SELECT_FEEDBACK_ID
                        name: SELECT_FEEDBACK_NAME
                        autocomplete: SELECT_AUTOCOMPLETE_OFF
                        class: if { state.get_textarea_error().get().is_empty() } { c_textarea_input() } else { c_form_input_error() }
                        placeholder: SELECT_FEEDBACK_PLACEHOLDER
                        value: state.get_textarea_content()
                        oninput: select_on_input_textarea(state)
                        rows: SELECT_FEEDBACK_ROWS
                    }
                    if { !state.get_textarea_error().get().is_empty() } {
                        p {
                            class: c_field_error_text()
                            state.get_textarea_error()
                        }
                    } else {
                        ""
                    }
                }
                div {
                    class: c_textarea_counter()
                    span {
                        class: c_textarea_counter_text()
                        { format!("{} / 200 characters", state.get_textarea_content().get().len()) }
                    }
                }
                primary_button {
                    label: "Submit"
                    onclick: select_on_submit_feedback(state)
                    "Submit"
                }
                if { !state.get_feedback().get().is_empty() } {
                    div {
                        class: c_success_box()
                        state.get_feedback()
                    }
                }
            }
        }
    }
}
