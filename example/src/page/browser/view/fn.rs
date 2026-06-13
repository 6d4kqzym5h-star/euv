use crate::*;

/// A browser API demo page showcasing localStorage, sessionStorage, clipboard, window, navigator, and location.
///
/// # Returns
///
/// - `VirtualNode` - The browser API demo page virtual DOM tree.
#[component]
pub(crate) fn page_browser(node: VirtualNode<PageBrowserProps>) -> VirtualNode {
    let PageBrowserProps = node.try_get_props().unwrap_or_default();
    let state: UseBrowserApi = use_browser_api();
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Browser APIs"
                subtitle: "Interact with localStorage, sessionStorage, clipboard, window, navigator, and location."
            }
            my_card {
                title: "localStorage"
                p {
                    class: c_demo_text()
                    "Store and retrieve persistent data in the browser."
                }
                div {
                    class: c_browser_api_row()
                    div {
                        class: c_form_input_wrapper()
                        label {
                            for: LOCAL_STORAGE_KEY_ID
                            class: c_form_label()
                            "Key"
                        }
                        input {
                            id: LOCAL_STORAGE_KEY_ID
                            name: LOCAL_STORAGE_KEY_NAME
                            type: BROWSER_TEXT_TYPE
                            autocomplete: BROWSER_AUTOCOMPLETE_OFF
                            placeholder: LOCAL_STORAGE_KEY_PLACEHOLDER
                            value: state.get_local_key()
                            class: c_form_input_no_transition()
                            oninput: on_input_value(state.get_local_key())
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            for: LOCAL_STORAGE_VALUE_ID
                            class: c_form_label()
                            "Value"
                        }
                        input {
                            id: LOCAL_STORAGE_VALUE_ID
                            name: LOCAL_STORAGE_VALUE_NAME
                            type: BROWSER_TEXT_TYPE
                            autocomplete: BROWSER_AUTOCOMPLETE_OFF
                            placeholder: LOCAL_STORAGE_VALUE_PLACEHOLDER
                            value: state.get_local_value()
                            class: c_form_input_no_transition()
                            oninput: on_input_value(state.get_local_value())
                        }
                    }
                }
                div {
                    class: c_browser_api_actions()
                    primary_button {
                        label: "Set"
                        onclick: local_storage_on_set(state)
                        "Set"
                    }
                    primary_button {
                        label: "Get"
                        onclick: local_storage_on_get(state)
                        "Get"
                    }
                    primary_button {
                        label: "Remove"
                        onclick: local_storage_on_remove(state)
                        "Remove"
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        "Result: "
                    }
                    span {
                        class: c_browser_result_value()
                        state.get_local_result()
                    }
                }
            }
            my_card {
                title: "sessionStorage"
                p {
                    class: c_demo_text()
                    "Store data for the duration of the page session."
                }
                div {
                    class: c_browser_api_row()
                    div {
                        class: c_form_input_wrapper()
                        label {
                            for: SESSION_STORAGE_KEY_ID
                            class: c_form_label()
                            "Key"
                        }
                        input {
                            id: SESSION_STORAGE_KEY_ID
                            name: SESSION_STORAGE_KEY_NAME
                            type: BROWSER_TEXT_TYPE
                            autocomplete: BROWSER_AUTOCOMPLETE_OFF
                            placeholder: SESSION_STORAGE_KEY_PLACEHOLDER
                            value: state.get_session_key()
                            class: c_form_input_no_transition()
                            oninput: on_input_value(state.get_session_key())
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            for: SESSION_STORAGE_VALUE_ID
                            class: c_form_label()
                            "Value"
                        }
                        input {
                            id: SESSION_STORAGE_VALUE_ID
                            name: SESSION_STORAGE_VALUE_NAME
                            type: BROWSER_TEXT_TYPE
                            autocomplete: BROWSER_AUTOCOMPLETE_OFF
                            placeholder: SESSION_STORAGE_VALUE_PLACEHOLDER
                            value: state.get_session_value()
                            class: c_form_input_no_transition()
                            oninput: on_input_value(state.get_session_value())
                        }
                    }
                }
                div {
                    class: c_browser_api_actions()
                    primary_button {
                        label: "Set"
                        onclick: session_storage_on_set(state)
                        "Set"
                    }
                    primary_button {
                        label: "Get"
                        onclick: session_storage_on_get(state)
                        "Get"
                    }
                    primary_button {
                        label: "Remove"
                        onclick: session_storage_on_remove(state)
                        "Remove"
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        "Result: "
                    }
                    span {
                        class: c_browser_result_value()
                        state.get_session_result()
                    }
                }
            }
            my_card {
                title: "Clipboard API"
                p {
                    class: c_demo_text()
                    "Read from and write to the system clipboard."
                }
                div {
                    class: c_form_input_wrapper()
                    label {
                        for: CLIPBOARD_TEXT_ID
                        class: c_form_label()
                        "Text to copy"
                    }
                    input {
                        id: CLIPBOARD_TEXT_ID
                        name: CLIPBOARD_TEXT_NAME
                        type: BROWSER_TEXT_TYPE
                        autocomplete: BROWSER_AUTOCOMPLETE_OFF
                        placeholder: CLIPBOARD_TEXT_PLACEHOLDER
                        value: state.get_clipboard_text()
                        class: c_form_input_no_transition()
                        oninput: on_input_value(state.get_clipboard_text())
                    }
                }
                div {
                    class: c_browser_api_actions()
                    primary_button {
                        label: "Copy"
                        onclick: clipboard_on_copy(state)
                        "Copy"
                    }
                    primary_button {
                        label: "Paste"
                        onclick: clipboard_on_paste(state)
                        "Paste"
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        "Result: "
                    }
                    span {
                        class: c_browser_result_value()
                        state.get_clipboard_result()
                    }
                }
            }
            my_card {
                title: "Window"
                p {
                    class: c_demo_text()
                    "Read the current window dimensions."
                }
                primary_button {
                    label: "Refresh"
                    onclick: window_on_refresh_size(state)
                    "Refresh"
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Inner Size"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_window_size()
                        }
                    }
                }
            }
            my_card {
                title: "Navigator"
                p {
                    class: c_demo_text()
                    "Read browser and device information."
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "User Agent"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_user_agent()
                        }
                    }
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Language"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_language()
                        }
                    }
                }
            }
            my_card {
                title: "Location"
                p {
                    class: c_demo_text()
                    "Read the current page URL information."
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Href"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_location_url()
                        }
                    }
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Origin"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_location_origin_val()
                        }
                    }
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Pathname"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_location_pathname_val()
                        }
                    }
                }
            }
            my_card {
                title: "Console"
                p {
                    class: c_demo_text()
                    "Send messages to the browser developer console."
                }
                div {
                    class: c_form_input_wrapper()
                    label {
                        for: CONSOLE_MESSAGE_ID
                        class: c_form_label()
                        "Console message"
                    }
                    input {
                        id: CONSOLE_MESSAGE_ID
                        name: CONSOLE_MESSAGE_NAME
                        type: BROWSER_TEXT_TYPE
                        autocomplete: BROWSER_AUTOCOMPLETE_OFF
                        placeholder: CONSOLE_MESSAGE_PLACEHOLDER
                        value: state.get_console_input()
                        class: c_form_input_no_transition()
                        oninput: on_input_value(state.get_console_input())
                    }
                }
                div {
                    class: c_browser_api_actions()
                    primary_button {
                        label: "Log"
                        onclick: console_on_log(state.get_console_input())
                        "log"
                    }
                    primary_button {
                        label: "Warn"
                        onclick: console_on_warn(state.get_console_input())
                        "warn"
                    }
                    primary_button {
                        label: "Error"
                        onclick: console_on_error(state.get_console_input())
                        "error"
                    }
                }
            }
        }
    }
}
