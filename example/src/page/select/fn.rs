use crate::*;

/// A select demo page showcasing dropdown and cascading selections.
///
/// # Returns
///
/// - `VirtualNode`: The select demo page virtual DOM tree.
pub fn page_select() -> VirtualNode {
    let selected_fruit: Signal<String> = use_signal(|| "apple".to_string());
    let selected_country: Signal<String> = use_signal(|| "".to_string());
    let selected_city: Signal<String> = use_signal(|| "".to_string());
    let feedback: Signal<String> = use_signal(|| "".to_string());
    let textarea_content: Signal<String> = use_signal(|| "".to_string());
    html! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Select & Textarea"
                }
                p {
                    class: c_page_subtitle()
                    "Dropdown selection, cascading selects, and textarea binding."
                }
            }
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
                        value: selected_fruit
                        onchange: move |event: NativeEvent| {
                            if let NativeEvent::Change(change_event) = event {
                                selected_fruit.set(change_event.get_value().clone());
                            }
                        }
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
                        selected_fruit
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
                        value: selected_country
                        onchange: move |event: NativeEvent| {
                            if let NativeEvent::Change(change_event) = event {
                                selected_country.set(change_event.get_value().clone());
                                selected_city.set("".to_string());
                            }
                        }
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
                if {!selected_country.get().is_empty()} {
                    div {
                        class: c_form_input_wrapper()
                        label {
                            class: c_form_label()
                            "City"
                        }
                        select {
                            class: c_select_input()
                            value: selected_city
                            onchange: move |event: NativeEvent| {
                                if let NativeEvent::Change(change_event) = event {
                                    selected_city.set(change_event.get_value().clone());
                                }
                            }
                            option {
                                value: ""
                                "-- Select City --"
                            }
                            for (city_value, city_label) in {match selected_country.get().as_str() { "china" => vec![("beijing".to_string(), "Beijing".to_string()), ("shanghai".to_string(), "Shanghai".to_string()), ("guangzhou".to_string(), "Guangzhou".to_string())], "japan" => vec![("tokyo".to_string(), "Tokyo".to_string()), ("osaka".to_string(), "Osaka".to_string()), ("kyoto".to_string(), "Kyoto".to_string())], "usa" => vec![("new-york".to_string(), "New York".to_string()), ("los-angeles".to_string(), "Los Angeles".to_string()), ("chicago".to_string(), "Chicago".to_string())], _ => vec![] }} {
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
                if {!selected_city.get().is_empty()} {
                    div {
                        class: c_success_box()
                        "You selected: "
                        span {
                            class: c_event_highlight()
                            selected_city
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
                        value: textarea_content
                        oninput: move |event: NativeEvent| {
                            if let NativeEvent::Input(input_event) = event {
                                textarea_content.set(input_event.get_value().clone());
                            }
                        }
                        rows: "4"
                        cols: "50"
                    }
                }
                div {
                    class: c_textarea_counter()
                    span {
                        class: c_textarea_counter_text()
                        {format!("{} / 200 characters", textarea_content.get().len())}
                    }
                }
                primary_button {
                    label: "Submit"
                    onclick: move |_event: NativeEvent| {
                        let content: String = textarea_content.get();
                        if content.trim().is_empty() {
                            feedback.set("Please enter some feedback.".to_string());
                        } else if content.len() > 200 {
                            feedback.set("Feedback is too long (max 200 chars).".to_string());
                        } else {
                            feedback.set(format!("Thank you for your feedback: \"{}\"", content));
                            textarea_content.set("".to_string());
                        }
                    }
                    "Submit"
                }
                if {!feedback.get().is_empty()} {
                    div {
                        class: c_success_box()
                        feedback
                    }
                } else {
                    ""
                }
            }
        }
    }
}
