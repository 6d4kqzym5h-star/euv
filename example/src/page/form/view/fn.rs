use crate::*;

/// Builds the error display node if validation errors exist.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal holding validation error messages.
///
/// # Returns
///
/// - `VirtualNode` - An error box element if errors exist, or `VirtualNode::Empty`.
fn build_error_node(errors: Signal<String>) -> VirtualNode {
    let error_text: String = errors.get();
    if !error_text.is_empty() {
        html! {
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
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal holding the submission result message.
///
/// # Returns
///
/// - `VirtualNode` - A success box element if submitted, or `VirtualNode::Empty`.
fn build_submitted_node(submitted: Signal<String>) -> VirtualNode {
    let submitted_text: String = submitted.get();
    if !submitted_text.is_empty() {
        html! {
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
/// - `VirtualNode` - The form demo page virtual DOM tree.
pub fn page_form() -> VirtualNode {
    let form: UseForm = use_form();
    let username_touched: Signal<bool> = use_signal(|| false);
    let email_touched: Signal<bool> = use_signal(|| false);
    let password_touched: Signal<bool> = use_signal(|| false);
    let username_error: Signal<String> = use_signal(String::new);
    let email_error: Signal<String> = use_signal(String::new);
    let password_error: Signal<String> = use_signal(String::new);
    watch!(
        form.get_username(),
        form.get_email(),
        form.get_password(),
        |username_value, email_value, password_value| {
            let mut validation_warnings: Vec<String> = Vec::new();
            if username_value.trim().is_empty() {
                validation_warnings.push("username is empty".to_string());
            }
            if email_value.trim().is_empty() {
                validation_warnings.push("email is empty".to_string());
            }
            if password_value.len() < 6 {
                validation_warnings.push("password too short".to_string());
            }
            if validation_warnings.is_empty() {
                Console::log("watch! all fields valid");
            } else {
                Console::log(&format!(
                    "watch! validation: {}",
                    validation_warnings.join(", ")
                ));
            }
        }
    );
    watch!(
        form.get_username(),
        username_touched,
        |username_value, touched| {
            if touched {
                if username_value.trim().is_empty() {
                    username_error.set("Username is required".to_string());
                } else {
                    username_error.set(String::new());
                }
            }
        }
    );
    watch!(form.get_email(), email_touched, |email_value, touched| {
        if touched {
            if email_value.trim().is_empty() {
                email_error.set("Email is required".to_string());
            } else if !email_value.contains('@') || !email_value.contains('.') {
                email_error.set("Please enter a valid email".to_string());
            } else {
                email_error.set(String::new());
            }
        }
    });
    watch!(
        form.get_password(),
        password_touched,
        |password_value, touched| {
            if touched {
                if password_value.is_empty() {
                    password_error.set("Password is required".to_string());
                } else if password_value.len() < 6 {
                    password_error.set("Password must be at least 6 characters".to_string());
                } else {
                    password_error.set(String::new());
                }
            }
        }
    );
    html! {
        div {
            class: c_page_container()
            { page_header("Form Demo", "Two-way binding and validation example.") }
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
                        value: form.get_username()
                        class: if { username_error.get().is_empty() } { c_form_input_no_transition() } else { c_form_input_error() }
                        oninput: on_input_value(form.get_username())
                        onblur: move |_event: NativeEvent| {
                            username_touched.set(true);
                        }
                    }
                    if { !username_error.get().is_empty() } {
                        p {
                            class: c_field_error_text()
                            username_error
                        }
                    } else {
                        ""
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
                        value: form.get_email()
                        class: if { email_error.get().is_empty() } { c_form_input_no_transition() } else { c_form_input_error() }
                        oninput: on_input_value(form.get_email())
                        onblur: move |_event: NativeEvent| {
                            email_touched.set(true);
                        }
                    }
                    if { !email_error.get().is_empty() } {
                        p {
                            class: c_field_error_text()
                            email_error
                        }
                    } else {
                        ""
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
                        value: form.get_password()
                        class: if { password_error.get().is_empty() } { c_form_input_no_transition() } else { c_form_input_error() }
                        oninput: on_input_value(form.get_password())
                        onblur: move |_event: NativeEvent| {
                            password_touched.set(true);
                        }
                    }
                    if { !password_error.get().is_empty() } {
                        p {
                            class: c_field_error_text()
                            password_error
                        }
                    } else {
                        ""
                    }
                }
                div {
                    class: c_form_checkbox_row()
                    input {
                        r#type: "checkbox"
                        checked: form.get_agree()
                        class: c_form_checkbox()
                        onchange: on_change_checked(form.get_agree())
                    }
                    label {
                        class: c_form_checkbox_label()
                        "I agree to the terms and conditions"
                    }
                }
                build_error_node(form.get_errors())
                primary_button {
                    label: "Submit"
                    onclick: form_on_submit(form)
                    "Submit"
                }
                build_submitted_node(form.get_submitted())
            }
        }
    }
}
