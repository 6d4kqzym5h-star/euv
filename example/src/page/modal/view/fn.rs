use crate::*;

/// A modal demo page showcasing different modal variations.
///
/// # Returns
///
/// - `VirtualNode` - The modal demo page virtual DOM tree.
#[component]
pub(crate) fn page_modal(node: VirtualNode<PageModalProps>) -> VirtualNode {
    let PageModalProps = node.try_get_props().unwrap_or_default();
    let state: UseModal = use_modal();
    html! {
        div {
            class: c_page_container()
            page_header {
                icon: "💬"
                title: "Modal Dialog"
                subtitle: "Overlay dialogs with different content patterns."
            }
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
                }
            }
            my_card {
                title: "Nested Modals"
                p {
                    class: c_demo_text()
                    "Three stacked modal layers. The system back gesture closes them one at a time, newest first."
                }
                primary_button {
                    label: "Open"
                    onclick: modal_on_open_nested_1(state)
                    "Open Layer 1"
                }
            }
            if { state.get_show_basic().get() } {
                my_modal {
                    title: "Basic Modal"
                    onclick: modal_dismiss_handler(state.get_show_basic())
                    p {
                        class: c_demo_text()
                        "This is a basic modal dialog. Click the close button or the overlay to dismiss."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Modals are useful for displaying focused content that requires user attention."
                    }
                }
            }
            if { state.get_show_confirm().get() } {
                my_modal {
                    title: "Confirm Action"
                    onclick: modal_dismiss_handler(state.get_show_confirm())
                    p {
                        class: c_demo_text()
                        "Are you sure you want to proceed with this action?"
                    }
                    div {
                        class: c_modal_actions()
                        modal_primary_button {
                            label: "Confirm"
                            onclick: modal_on_confirm(state)
                            "Confirm"
                        }
                        modal_primary_button {
                            label: "Cancel"
                            onclick: modal_on_cancel_confirm(state)
                            "Cancel"
                        }
                    }
                }
            }
            if { state.get_show_form().get() } {
                my_modal {
                    title: "Quick Sign Up"
                    onclick: modal_dismiss_handler(state.get_show_form())
                    div {
                        class: c_form_input_wrapper()
                        label {
                            for: MODAL_NAME_ID
                            class: c_form_label()
                            "Name"
                        }
                        input {
                            id: MODAL_NAME_ID
                            name: MODAL_NAME_NAME
                            type: MODAL_TEXT_TYPE
                            autocomplete: MODAL_AUTOCOMPLETE_NAME
                            placeholder: MODAL_NAME_PLACEHOLDER
                            value: state.get_modal_name()
                            class: if { state.get_name_error().get().is_empty() } { c_form_input_no_transition() } else { c_form_input_error() }
                            oninput: modal_on_input_name(state)
                        }
                        if { !state.get_name_error().get().is_empty() } {
                            p {
                                class: c_field_error_text()
                                state.get_name_error()
                            }
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            for: MODAL_EMAIL_ID
                            class: c_form_label()
                            "Email"
                        }
                        input {
                            id: MODAL_EMAIL_ID
                            name: MODAL_EMAIL_NAME
                            type: MODAL_EMAIL_TYPE
                            autocomplete: MODAL_AUTOCOMPLETE_EMAIL
                            placeholder: MODAL_EMAIL_PLACEHOLDER
                            value: state.get_modal_email()
                            class: if { state.get_email_error().get().is_empty() } { c_form_input_no_transition() } else { c_form_input_error() }
                            oninput: modal_on_input_email(state)
                        }
                        if { !state.get_email_error().get().is_empty() } {
                            p {
                                class: c_field_error_text()
                                state.get_email_error()
                            }
                        }
                    }
                    if { !state.get_modal_error().get().is_empty() } {
                        div {
                            class: c_error_box()
                            state.get_modal_error()
                        }
                    }
                    div {
                        class: c_modal_actions()
                        modal_primary_button {
                            label: "Submit"
                            onclick: modal_on_form_submit(state)
                            "Submit"
                        }
                        modal_primary_button {
                            label: "Cancel"
                            onclick: modal_on_cancel_form(state)
                            "Cancel"
                        }
                    }
                }
            }
            if { state.get_show_nested_1().get() } {
                my_modal {
                    title: "Nested Modal · Layer 1"
                    onclick: modal_dismiss_handler(state.get_show_nested_1())
                    p {
                        class: c_demo_text()
                        "This is the first layer. Open another modal on top of it."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Use the system back gesture to close the topmost layer first."
                    }
                    div {
                        class: c_modal_actions()
                        modal_primary_button {
                            label: "Open Layer 2"
                            onclick: modal_on_open_nested_2(state)
                            "Open Layer 2"
                        }
                    }
                }
            }
            if { state.get_show_nested_2().get() } {
                my_modal {
                    title: "Nested Modal · Layer 2"
                    onclick: modal_dismiss_handler(state.get_show_nested_2())
                    p {
                        class: c_demo_text()
                        "This is the second layer, stacked over layer 1."
                    }
                    div {
                        class: c_modal_actions()
                        modal_primary_button {
                            label: "Open Layer 3"
                            onclick: modal_on_open_nested_3(state)
                            "Open Layer 3"
                        }
                    }
                }
            }
            if { state.get_show_nested_3().get() } {
                my_modal {
                    title: "Nested Modal · Layer 3"
                    onclick: modal_dismiss_handler(state.get_show_nested_3())
                    p {
                        class: c_demo_text()
                        "This is the third and innermost layer."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Press back three times: layer 3, then 2, then 1 close in order before leaving the page."
                    }
                }
            }
        }
    }
}
