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
            euv_header {
                icon: "🌐"
                title: "Browser APIs"
                subtitle: "Interact with localStorage, sessionStorage, clipboard, window, navigator, and location."
            }
            euv_card {
                title: "localStorage"
                p {
                    class: c_demo_text()
                    "Store and retrieve persistent data in the browser."
                }
                div {
                    class: c_browser_api_row()
                    euv_field {
                        id: LOCAL_STORAGE_KEY_ID
                        name: LOCAL_STORAGE_KEY_NAME
                        label: "Key"
                        input_type: BROWSER_TEXT_TYPE
                        placeholder: LOCAL_STORAGE_KEY_PLACEHOLDER
                        autocomplete: BROWSER_AUTOCOMPLETE_OFF
                        value: state.get_local_key()
                        error: None
                    }
                    euv_field {
                        id: LOCAL_STORAGE_VALUE_ID
                        name: LOCAL_STORAGE_VALUE_NAME
                        label: "Value"
                        input_type: BROWSER_TEXT_TYPE
                        placeholder: LOCAL_STORAGE_VALUE_PLACEHOLDER
                        autocomplete: BROWSER_AUTOCOMPLETE_OFF
                        value: state.get_local_value()
                        error: None
                    }
                }
                div {
                    class: c_browser_api_actions()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Set"
                        onclick: local_storage_on_set(state)
                        "Set"
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Get"
                        onclick: local_storage_on_get(state)
                        "Get"
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
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
            euv_card {
                title: "sessionStorage"
                p {
                    class: c_demo_text()
                    "Store data for the duration of the page session."
                }
                div {
                    class: c_browser_api_row()
                    euv_field {
                        id: SESSION_STORAGE_KEY_ID
                        name: SESSION_STORAGE_KEY_NAME
                        label: "Key"
                        input_type: BROWSER_TEXT_TYPE
                        placeholder: SESSION_STORAGE_KEY_PLACEHOLDER
                        autocomplete: BROWSER_AUTOCOMPLETE_OFF
                        value: state.get_session_key()
                        error: None
                    }
                    euv_field {
                        id: SESSION_STORAGE_VALUE_ID
                        name: SESSION_STORAGE_VALUE_NAME
                        label: "Value"
                        input_type: BROWSER_TEXT_TYPE
                        placeholder: SESSION_STORAGE_VALUE_PLACEHOLDER
                        autocomplete: BROWSER_AUTOCOMPLETE_OFF
                        value: state.get_session_value()
                        error: None
                    }
                }
                div {
                    class: c_browser_api_actions()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Set"
                        onclick: session_storage_on_set(state)
                        "Set"
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Get"
                        onclick: session_storage_on_get(state)
                        "Get"
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
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
            euv_card {
                title: "Clipboard API"
                p {
                    class: c_demo_text()
                    "Read from and write to the system clipboard."
                }
                euv_field {
                    id: CLIPBOARD_TEXT_ID
                    name: CLIPBOARD_TEXT_NAME
                    label: "Text to copy"
                    input_type: BROWSER_TEXT_TYPE
                    placeholder: CLIPBOARD_TEXT_PLACEHOLDER
                    autocomplete: BROWSER_AUTOCOMPLETE_OFF
                    value: state.get_clipboard_text()
                    error: None
                }
                div {
                    class: c_browser_api_actions()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Copy"
                        onclick: clipboard_on_copy(state)
                        "Copy"
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
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
            euv_card {
                title: "Window"
                p {
                    class: c_demo_text()
                    "Read the current window dimensions."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Refresh"
                        onclick: window_on_refresh_size(state)
                        "Refresh"
                    }
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
            euv_card {
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
            euv_card {
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
            euv_card {
                title: "Console"
                p {
                    class: c_demo_text()
                    "Send messages to the browser developer console."
                }
                euv_field {
                    id: CONSOLE_MESSAGE_ID
                    name: CONSOLE_MESSAGE_NAME
                    label: "Console message"
                    input_type: BROWSER_TEXT_TYPE
                    placeholder: CONSOLE_MESSAGE_PLACEHOLDER
                    autocomplete: BROWSER_AUTOCOMPLETE_OFF
                    value: state.get_console_input()
                    error: None
                }
                div {
                    class: c_browser_api_actions()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Log"
                        onclick: console_on_log(state.get_console_input())
                        "log"
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Warn"
                        onclick: console_on_warn(state.get_console_input())
                        "warn"
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Error"
                        onclick: console_on_error(state.get_console_input())
                        "error"
                    }
                }
            }
        }
    }
}
