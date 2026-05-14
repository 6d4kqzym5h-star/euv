use crate::*;

/// Builds the error display node if validation errors exist.
fn build_error_node(errors_read: Signal<String>) -> VirtualNode {
    let error_text: String = errors_read.get();
    if !error_text.is_empty() {
        rsx! {
            div {
                class: c_error_box()
                error_text
            }
        }
    } else {
        VirtualNode::Empty
    }
}

/// Builds the success display node if form was submitted.
fn build_submitted_node(submitted_read: Signal<String>) -> VirtualNode {
    let submitted_text: String = submitted_read.get();
    if !submitted_text.is_empty() {
        rsx! {
            div {
                class: c_success_box()
                submitted_text
            }
        }
    } else {
        VirtualNode::Empty
    }
}

/// A form demo page with two-way binding and validation.
///
/// # Returns
///
/// - `VirtualNode`: The form demo page virtual DOM tree.
pub fn page_form() -> VirtualNode {
    let username: Signal<String> = use_signal(|| "".to_string());
    let username_updater_input: Signal<String> = username;
    let username_updater_submit: Signal<String> = username;
    let username_read: Signal<String> = username;
    let email: Signal<String> = use_signal(|| "".to_string());
    let email_updater_input: Signal<String> = email;
    let email_updater_submit: Signal<String> = email;
    let email_read: Signal<String> = email;
    let password: Signal<String> = use_signal(|| "".to_string());
    let password_updater_input: Signal<String> = password;
    let password_updater_submit: Signal<String> = password;
    let password_read: Signal<String> = password;
    let agree: Signal<bool> = use_signal(|| true);
    let agree_updater_change: Signal<bool> = agree;
    let agree_updater_submit: Signal<bool> = agree;
    let agree_read: Signal<bool> = agree;
    let submitted: Signal<String> = use_signal(|| "".to_string());
    let submitted_updater: Signal<String> = submitted;
    let submitted_read: Signal<String> = submitted;
    let errors: Signal<String> = use_signal(|| "".to_string());
    let errors_updater: Signal<String> = errors;
    let errors_read: Signal<String> = errors;
    rsx! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Form Demo"
                }
                p {
                    class: c_page_subtitle()
                    "Two-way binding and validation example."
                }
            }
            my_card {
                title: "Registration Form"
                div {
                    class: c_form_input_wrapper()
                    label {
                        class: c_form_label()
                        "Username"
                    }
                    input {
                        r#type: "text"
                        placeholder: "Enter username"
                        value: username_read
                        class: c_form_input_no_transition()
                        oninput: move |event: NativeEvent| {
                            if let NativeEvent::Input(input_event) = event {
                                username_updater_input.set(input_event.get_value().clone());
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
                        placeholder: "Enter email"
                        value: email_read
                        class: c_form_input_no_transition()
                        oninput: move |event: NativeEvent| {
                            if let NativeEvent::Input(input_event) = event {
                                email_updater_input.set(input_event.get_value().clone());
                            }
                        }
                    }
                }
                div {
                    class: c_form_input_wrapper()
                    label {
                        class: c_form_label()
                        "Password"
                    }
                    input {
                        r#type: "password"
                        placeholder: "Enter password"
                        value: password_read
                        class: c_form_input_no_transition()
                        oninput: move |event: NativeEvent| {
                            if let NativeEvent::Input(input_event) = event {
                                password_updater_input.set(input_event.get_value().clone());
                            }
                        }
                    }
                }
                div {
                    class: c_form_checkbox_row()
                    input {
                        r#type: "checkbox"
                        checked: agree_read
                        class: c_form_checkbox()
                        onchange: move |event: NativeEvent| {
                            if let NativeEvent::Change(change_event) = event {
                                agree_updater_change.set(*change_event.get_checked());
                            }
                        }
                    }
                    label {
                        class: c_form_checkbox_label()
                        "I agree to the terms and conditions"
                    }
                }
                build_error_node(errors_read)
                primary_button {
                    label: "Submit"
                    onclick: move |_event: NativeEvent| {
                        let mut validation_errors: Vec<String> = Vec::new();
                        if username_updater_submit.get().trim().is_empty() {
                            validation_errors.push("Username is required".to_string());
                        }
                        if email_updater_submit.get().trim().is_empty() {
                            validation_errors.push("Email is required".to_string());
                        }
                        if password_updater_submit.get().len() < 6 {
                            validation_errors.push("Password must be at least 6 characters".to_string());
                        }
                        if !agree_updater_submit.get() {
                            validation_errors.push("You must agree to the terms".to_string());
                        }
                        if validation_errors.is_empty() {
                            errors_updater.set("".to_string());
                            submitted_updater.set(format!("Submitted: username={}, email={}", username_updater_submit.get(), email_updater_submit.get()));
                        } else {
                            errors_updater.set(validation_errors.join("; "));
                        }
                    }
                    "Submit"
                }
                build_submitted_node(submitted_read)
            }
        }
    }
}
