use crate::*;

/// A modal demo page showcasing different modal variations.
///
/// # Returns
///
/// - `VirtualNode` - The modal demo page virtual DOM tree.
pub(crate) fn page_modal() -> VirtualNode {
    let state: UseModal = use_modal();
    html! {
        div {
            class: c_page_container()
            page_header("Modal Dialog", "Overlay dialogs with different content patterns.")
            my_card {
                title: "Basic Modal"
                p {
                    class: c_demo_text()
                    "A simple modal with text content."
                }
                primary_button {
                    label: "Open"
                    onclick: modal_on_open_basic(state)
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
                    onclick: modal_on_open_confirm(state)
                    "Ask Confirm"
                }
                if { !state.get_confirm_result().get().is_empty() } {
                    div {
                        class: c_success_box()
                        state.get_confirm_result()
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
                    onclick: modal_on_open_form(state)
                    "Open Form"
                }
                if { !state.get_modal_submitted().get().is_empty() } {
                    div {
                        class: c_success_box()
                        state.get_modal_submitted()
                    }
                } else {
                    ""
                }
            }
            if { state.get_show_basic().get() } {
                my_modal {
                    title: "Basic Modal"
                    onclick: use_toggle(state.get_show_basic())
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
            if { state.get_show_confirm().get() } {
                my_modal {
                    title: "Confirm Action"
                    onclick: use_toggle(state.get_show_confirm())
                    p {
                        class: c_demo_text()
                        "Are you sure you want to proceed with this action?"
                    }
                    div {
                        class: c_modal_actions()
                        primary_button {
                            label: "Confirm"
                            onclick: modal_on_confirm(state)
                            "Confirm"
                        }
                        primary_button {
                            label: "Cancel"
                            onclick: use_toggle(state.get_show_confirm())
                            "Cancel"
                        }
                    }
                }
            } else {
                ""
            }
            if { state.get_show_form().get() } {
                my_modal {
                    title: "Quick Sign Up"
                    onclick: use_toggle(state.get_show_form())
                    div {
                        class: c_form_input_wrapper()
                        label {
                            r#for: "modal-name"
                            class: c_form_label()
                            "Name"
                        }
                        input {
                            id: "modal-name"
                            name: "name"
                            r#type: "text"
                            autocomplete: "name"
                            placeholder: "Enter your name"
                            value: state.get_modal_name()
                            class: if { state.get_name_error().get().is_empty() } { c_form_input_no_transition() } else { c_form_input_error() }
                            oninput: modal_on_input_name(state)
                        }
                        if { !state.get_name_error().get().is_empty() } {
                            p {
                                class: c_field_error_text()
                                state.get_name_error()
                            }
                        } else {
                            ""
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            r#for: "modal-email"
                            class: c_form_label()
                            "Email"
                        }
                        input {
                            id: "modal-email"
                            name: "email"
                            r#type: "email"
                            autocomplete: "email"
                            placeholder: "Enter your email"
                            value: state.get_modal_email()
                            class: if { state.get_email_error().get().is_empty() } { c_form_input_no_transition() } else { c_form_input_error() }
                            oninput: modal_on_input_email(state)
                        }
                        if { !state.get_email_error().get().is_empty() } {
                            p {
                                class: c_field_error_text()
                                state.get_email_error()
                            }
                        } else {
                            ""
                        }
                    }
                    if { !state.get_modal_error().get().is_empty() } {
                        div {
                            class: c_error_box()
                            state.get_modal_error()
                        }
                    } else {
                        ""
                    }
                    div {
                        class: c_modal_actions()
                        primary_button {
                            label: "Submit"
                            onclick: modal_on_form_submit(state)
                            "Submit"
                        }
                        primary_button {
                            label: "Cancel"
                            onclick: use_toggle(state.get_show_form())
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
