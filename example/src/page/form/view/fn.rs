use crate::*;

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
            page_header("Form Demo", "Two-way binding and validation example.")
            my_card {
                title: "Registration Form"
                div {
                    class: c_form_input_wrapper()
                    label {
                        r#for: "form-username"
                        class: c_form_label()
                        "Username"
                    }
                    input {
                        id: "form-username"
                        name: "username"
                        r#type: "text"
                        autocomplete: "username"
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
                        r#for: "form-email"
                        class: c_form_label()
                        "Email"
                    }
                    input {
                        id: "form-email"
                        name: "email"
                        r#type: "email"
                        autocomplete: "email"
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
                        r#for: "form-password"
                        class: c_form_label()
                        "Password"
                    }
                    input {
                        id: "form-password"
                        name: "password"
                        r#type: "password"
                        autocomplete: "new-password"
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
                        id: "form-agree"
                        name: "agree"
                        r#type: "checkbox"
                        autocomplete: "off"
                        checked: form.get_agree()
                        class: c_form_checkbox()
                        onchange: form_on_change_agree(form)
                    }
                    label {
                        r#for: "form-agree"
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
                if { !form.get_errors().get().is_empty() } {
                    div {
                        class: c_error_box()
                        form.get_errors()
                    }
                } else {
                    ""
                }
                primary_button {
                    label: "Submit"
                    onclick: form_on_submit(form)
                    "Submit"
                }
                if { !form.get_submitted().get().is_empty() } {
                    div {
                        class: c_success_box()
                        form.get_submitted()
                    }
                } else {
                    ""
                }
            }
        }
    }
}
