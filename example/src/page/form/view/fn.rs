use crate::*;

/// Builds the error display node if validation errors exist.
///
/// # Arguments
///
/// - `Signal<String>`: The reactive signal holding validation error messages.
///
/// # Returns
///
/// - `VirtualNode`: An error box element if errors exist, or `VirtualNode::Empty`.
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
/// - `Signal<String>`: The reactive signal holding the submission result message.
///
/// # Returns
///
/// - `VirtualNode`: A success box element if submitted, or `VirtualNode::Empty`.
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
/// - `VirtualNode`: The form demo page virtual DOM tree.
pub fn page_form() -> VirtualNode {
    let form: UseForm = use_form();
    watch!(
        form.username,
        form.email,
        form.password,
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
                        value: form.username
                        class: c_form_input_no_transition()
                        oninput: on_input_value(form.username)
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
                        value: form.email
                        class: c_form_input_no_transition()
                        oninput: on_input_value(form.email)
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
                        value: form.password
                        class: c_form_input_no_transition()
                        oninput: on_input_value(form.password)
                    }
                }
                div {
                    class: c_form_checkbox_row()
                    input {
                        r#type: "checkbox"
                        checked: form.agree
                        class: c_form_checkbox()
                        onchange: on_change_checked(form.agree)
                    }
                    label {
                        class: c_form_checkbox_label()
                        "I agree to the terms and conditions"
                    }
                }
                build_error_node(form.errors)
                primary_button {
                    label: "Submit"
                    onclick: form_on_submit(form)
                    "Submit"
                }
                build_submitted_node(form.submitted)
            }
        }
    }
}
