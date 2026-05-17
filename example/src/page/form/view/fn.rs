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
                        class: if { form.get_username_error().get().is_empty() } { c_form_input_no_transition() } else { c_form_input_error() }
                        oninput: form_on_input_username(form)
                    }
                    if { !form.get_username_error().get().is_empty() } {
                        p {
                            class: c_field_error_text()
                            form.get_username_error()
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
                        class: if { form.get_email_error().get().is_empty() } { c_form_input_no_transition() } else { c_form_input_error() }
                        oninput: form_on_input_email(form)
                    }
                    if { !form.get_email_error().get().is_empty() } {
                        p {
                            class: c_field_error_text()
                            form.get_email_error()
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
                        class: if { form.get_password_error().get().is_empty() } { c_form_input_no_transition() } else { c_form_input_error() }
                        oninput: form_on_input_password(form)
                    }
                    if { !form.get_password_error().get().is_empty() } {
                        p {
                            class: c_field_error_text()
                            form.get_password_error()
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
                        onchange: form_on_change_agree(form)
                    }
                    label {
                        class: c_form_checkbox_label()
                        "I agree to the terms and conditions"
                    }
                }
                if { !form.get_agree_error().get().is_empty() } {
                    p {
                        class: c_field_error_text()
                        form.get_agree_error()
                    }
                } else {
                    ""
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
