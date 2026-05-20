use crate::*;

/// A browser API demo page showcasing localStorage, sessionStorage, clipboard, window, navigator, and location.
///
/// # Returns
///
/// - `VirtualNode` - The browser API demo page virtual DOM tree.
pub fn page_browser() -> VirtualNode {
    let state: UseBrowserApi = use_browser_api();
    html! {
        div {
            class: c_page_container()
            page_header("Browser APIs", "Interact with localStorage, sessionStorage, clipboard, window, navigator, and location.")
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
                            r#for: "local-storage-key"
                            class: c_form_label()
                            "Key"
                        }
                        input {
                            id: "local-storage-key"
                            name: "local_key"
                            r#type: "text"
                            autocomplete: "off"
                            placeholder: "Storage key..."
                            value: state.get_local_key()
                            class: c_form_input_no_transition()
                            oninput: on_input_value(state.get_local_key())
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            r#for: "local-storage-value"
                            class: c_form_label()
                            "Value"
                        }
                        input {
                            id: "local-storage-value"
                            name: "local_value"
                            r#type: "text"
                            autocomplete: "off"
                            placeholder: "Storage value..."
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
                        "Set Item"
                    }
                    primary_button {
                        label: "Get"
                        onclick: local_storage_on_get(state)
                        "Get Item"
                    }
                    primary_button {
                        label: "Remove"
                        onclick: local_storage_on_remove(state)
                        "Remove Item"
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
                            r#for: "session-storage-key"
                            class: c_form_label()
                            "Key"
                        }
                        input {
                            id: "session-storage-key"
                            name: "session_key"
                            r#type: "text"
                            autocomplete: "off"
                            placeholder: "Session key..."
                            value: state.get_session_key()
                            class: c_form_input_no_transition()
                            oninput: on_input_value(state.get_session_key())
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            r#for: "session-storage-value"
                            class: c_form_label()
                            "Value"
                        }
                        input {
                            id: "session-storage-value"
                            name: "session_value"
                            r#type: "text"
                            autocomplete: "off"
                            placeholder: "Session value..."
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
                        "Set Item"
                    }
                    primary_button {
                        label: "Get"
                        onclick: session_storage_on_get(state)
                        "Get Item"
                    }
                    primary_button {
                        label: "Remove"
                        onclick: session_storage_on_remove(state)
                        "Remove Item"
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
                        r#for: "clipboard-text"
                        class: c_form_label()
                        "Text to copy"
                    }
                    input {
                        id: "clipboard-text"
                        name: "clipboard_text"
                        r#type: "text"
                        autocomplete: "off"
                        placeholder: "Enter text to copy..."
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
                        "Copy to Clipboard"
                    }
                    primary_button {
                        label: "Paste"
                        onclick: clipboard_on_paste(state)
                        "Read Clipboard"
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
                    "Refresh Size"
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
                        r#for: "console-message"
                        class: c_form_label()
                        "Console message"
                    }
                    input {
                        id: "console-message"
                        name: "console_message"
                        r#type: "text"
                        autocomplete: "off"
                        placeholder: "Type a message to log..."
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
                        "console.log"
                    }
                    primary_button {
                        label: "Warn"
                        onclick: console_on_warn(state.get_console_input())
                        "console.warn"
                    }
                    primary_button {
                        label: "Error"
                        onclick: console_on_error(state.get_console_input())
                        "console.error"
                    }
                }
            }
        }
    }
}
