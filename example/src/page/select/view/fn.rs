use crate::*;

/// A select demo page showcasing dropdown and cascading selections.
///
/// # Returns
///
/// - `VirtualNode` - The select demo page virtual DOM tree.
pub fn page_select() -> VirtualNode {
    let state: UseSelect = use_select();
    html! {
        div {
            class: c_page_container()
            { page_header("Select & Textarea", "Dropdown selection, cascading selects, and textarea binding.") }
            my_card {
                title: "Simple Select"
                div {
                    class: c_form_input_wrapper()
                    label {
                        class: c_form_label()
                        "Choose a fruit"
                    }
                    select {
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
                        state.get_selected_fruit()
                    }
                }
            }
            my_card {
                title: "Cascading Select"
                div {
                    class: c_form_input_wrapper()
                    label {
                        class: c_form_label()
                        "Country"
                    }
                    select {
                        class: c_select_input()
                        value: state.get_selected_country()
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
                if { !state.get_selected_country().get().is_empty() } {
                    div {
                        class: c_form_input_wrapper()
                        label {
                            class: c_form_label()
                            "City"
                        }
                        select {
                            class: c_select_input()
                            value: state.get_selected_city()
                            onchange: on_change_value(state.get_selected_city())
                            option {
                                value: ""
                                "-- Select City --"
                            }
                            for (city_value, city_label) in { match state.get_selected_country().get().as_str() { "china" => vec![("beijing".to_string(), "Beijing".to_string()), ("shanghai".to_string(), "Shanghai".to_string()), ("guangzhou".to_string(), "Guangzhou".to_string())], "japan" => vec![("tokyo".to_string(), "Tokyo".to_string()), ("osaka".to_string(), "Osaka".to_string()), ("kyoto".to_string(), "Kyoto".to_string())], "usa" => vec![("new-york".to_string(), "New York".to_string()), ("los-angeles".to_string(), "Los Angeles".to_string()), ("chicago".to_string(), "Chicago".to_string())], _ => vec![] } } {
                                option {
                                    value: city_value
                                    city_label
                                }
                            }
                        }
                    }
                } else {
                    ""
                }
                if { !state.get_selected_city().get().is_empty() } {
                    div {
                        class: c_success_box()
                        "You selected: "
                        span {
                            class: c_event_highlight()
                            state.get_selected_city()
                        }
                    }
                } else {
                    ""
                }
            }
            my_card {
                title: "Textarea with Feedback"
                div {
                    class: c_form_input_wrapper()
                    label {
                        class: c_form_label()
                        "Your feedback"
                    }
                    textarea {
                        class: c_textarea_input()
                        placeholder: "Share your thoughts..."
                        value: state.get_textarea_content()
                        oninput: on_input_value(state.get_textarea_content())
                        rows: "4"
                        cols: "50"
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
                } else {
                    ""
                }
            }
        }
    }
}
