use crate::*;

/// A form demo page with two-way binding and validation.
///
/// # Returns
///
/// - `VirtualNode` - The form demo page virtual DOM tree.
#[component]
pub(crate) fn page_form(mut node: VirtualNode<PageFormProps>) -> VirtualNode {
    let PageFormProps = node.try_take_props().unwrap_or_default();
    let form: UseForm = use_form();
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Form Demo"
                subtitle: "Two-way binding and validation example."
            }
            my_card {
                title: "Registration Form"
                div {
                    class: c_form_input_wrapper()
                    label {
                        r#for: FORM_USERNAME_ID
                        class: c_form_label()
                        "Username"
                    }
                    input {
                        id: FORM_USERNAME_ID
                        name: FORM_USERNAME_NAME
                        r#type: FORM_TEXT_TYPE
                        autocomplete: FORM_AUTOCOMPLETE_USERNAME
                        placeholder: FORM_USERNAME_PLACEHOLDER
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
                        r#for: FORM_EMAIL_ID
                        class: c_form_label()
                        "Email"
                    }
                    input {
                        id: FORM_EMAIL_ID
                        name: FORM_EMAIL_NAME
                        r#type: FORM_EMAIL_TYPE
                        autocomplete: FORM_AUTOCOMPLETE_EMAIL
                        placeholder: FORM_EMAIL_PLACEHOLDER
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
                        r#for: FORM_PASSWORD_ID
                        class: c_form_label()
                        "Password"
                    }
                    input {
                        id: FORM_PASSWORD_ID
                        name: FORM_PASSWORD_NAME
                        r#type: FORM_PASSWORD_TYPE
                        autocomplete: FORM_AUTOCOMPLETE_NEW_PASSWORD
                        placeholder: FORM_PASSWORD_PLACEHOLDER
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
                        id: FORM_AGREE_ID
                        name: FORM_AGREE_NAME
                        r#type: FORM_CHECKBOX_TYPE
                        autocomplete: FORM_AUTOCOMPLETE_OFF
                        checked: form.get_agree()
                        class: c_form_checkbox()
                        onchange: form_on_change_agree(form)
                    }
                    label {
                        r#for: FORM_AGREE_ID
                        class: c_form_checkbox_label()
                        "I agree to the terms and conditions"
                    }
                }
                if { !form.get_agree_error().get().is_empty() } {
                    p {
                        class: c_field_error_text()
                        form.get_agree_error()
                    }
                }
                if { !form.get_errors().get().is_empty() } {
                    div {
                        class: c_error_box()
                        form.get_errors()
                    }
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
                }
            }
        }
    }
}
