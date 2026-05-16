use crate::*;

/// A modal demo page showcasing different modal variations.
///
/// # Returns
///
/// - `VirtualNode`: The modal demo page virtual DOM tree.
pub fn page_modal() -> VirtualNode {
    let show_basic: Signal<bool> = use_signal(|| false);
    let show_confirm: Signal<bool> = use_signal(|| false);
    let show_form: Signal<bool> = use_signal(|| false);
    let confirm_result: Signal<String> = use_signal(|| "".to_string());
    let modal_name: Signal<String> = use_signal(|| "".to_string());
    let modal_email: Signal<String> = use_signal(|| "".to_string());
    let modal_submitted: Signal<String> = use_signal(|| "".to_string());
    html! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Modal Dialog"
                }
                p {
                    class: c_page_subtitle()
                    "Overlay dialogs with different content patterns."
                }
            }
            my_card {
                title: "Basic Modal"
                p {
                    class: c_demo_text()
                    "A simple modal with text content."
                }
                primary_button {
                    label: "Open"
                    onclick: move |_event: NativeEvent| {
                        show_basic.set(true);
                    }
                    "Open"
                }
            }
            my_card {
                title: "Confirm Modal"
                p {
                    class: c_demo_text()
                    "A modal requiring user confirmation."
                }
                primary_button {
                    label: "Open"
                    onclick: move |_event: NativeEvent| {
                        show_confirm.set(true);
                        confirm_result.set("".to_string());
                    }
                    "Ask Confirm"
                }
                if { !confirm_result.get().is_empty() } {
                    div {
                        class: c_success_box()
                        confirm_result
                    }
                } else {
                    ""
                }
            }
            my_card {
                title: "Form Modal"
                p {
                    class: c_demo_text()
                    "A modal containing a form with inputs."
                }
                primary_button {
                    label: "Open"
                    onclick: move |_event: NativeEvent| {
                        show_form.set(true);
                        modal_name.set("".to_string());
                        modal_email.set("".to_string());
                        modal_submitted.set("".to_string());
                    }
                    "Open Form"
                }
                if { !modal_submitted.get().is_empty() } {
                    div {
                        class: c_success_box()
                        modal_submitted
                    }
                } else {
                    ""
                }
            }
            if { show_basic.get() } {
                my_modal {
                    title: "Basic Modal"
                    onclick: move |_event: NativeEvent| {
                        show_basic.set(false);
                    }
                    p {
                        class: c_demo_text()
                        "This is a basic modal dialog. Click the close button or the overlay to dismiss."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Modals are useful for displaying focused content that requires user attention."
                    }
                }
            } else {
                ""
            }
            if { show_confirm.get() } {
                my_modal {
                    title: "Confirm Action"
                    onclick: move |_event: NativeEvent| {
                        show_confirm.set(false);
                    }
                    p {
                        class: c_demo_text()
                        "Are you sure you want to proceed with this action?"
                    }
                    div {
                        class: c_modal_actions()
                        primary_button {
                            label: "Confirm"
                            onclick: move |_event: NativeEvent| {
                                confirm_result.set("Action confirmed!".to_string());
                                show_confirm.set(false);
                            }
                            "Confirm"
                        }
                        primary_button {
                            label: "Cancel"
                            onclick: move |_event: NativeEvent| {
                                show_confirm.set(false);
                            }
                            "Cancel"
                        }
                    }
                }
            } else {
                ""
            }
            if { show_form.get() } {
                my_modal {
                    title: "Quick Sign Up"
                    onclick: move |_event: NativeEvent| {
                        show_form.set(false);
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            class: c_form_label()
                            "Name"
                        }
                        input {
                            r#type: "text"
                            placeholder: "Enter your name"
                            value: modal_name
                            class: c_form_input_no_transition()
                            oninput: move |event: NativeEvent| {
                                if let NativeEvent::Input(input_event) = event {
                                    modal_name.set(input_event.get_value().clone());
                                }
                            }
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            class: c_form_label()
                            "Email"
                        }
                        input {
                            r#type: "email"
                            placeholder: "Enter your email"
                            value: modal_email
                            class: c_form_input_no_transition()
                            oninput: move |event: NativeEvent| {
                                if let NativeEvent::Input(input_event) = event {
                                    modal_email.set(input_event.get_value().clone());
                                }
                            }
                        }
                    }
                    div {
                        class: c_modal_actions()
                        primary_button {
                            label: "Submit"
                            onclick: move |_event: NativeEvent| {
                                let name: String = modal_name.get();
                                let email: String = modal_email.get();
                                if !name.trim().is_empty() && !email.trim().is_empty() {
                                    modal_submitted.set(format!("Signed up: {} ({})", name, email));
                                    show_form.set(false);
                                }
                            }
                            "Submit"
                        }
                        primary_button {
                            label: "Cancel"
                            onclick: move |_event: NativeEvent| {
                                show_form.set(false);
                            }
                            "Cancel"
                        }
                    }
                }
            } else {
                ""
            }
        }
    }
}
