use crate::*;

/// A WebSocket chat page with auto-generated UUID and Ping keep-alive.
///
/// Renders a page header, a connection card, a send message card,
/// and a messages card that displays all received WebSocket messages
/// as raw text.
///
/// # Returns
///
/// - `VirtualNode` - The WebSocket chat page virtual DOM tree.
#[component]
pub(crate) fn page_websocket(node: VirtualNode<PageWebsocketProps>) -> VirtualNode {
    let PageWebsocketProps = node.try_get_props().unwrap_or_default();
    let state: UseWebSocket = use_websocket();
    ws_cleanup(state);
    html! {
        div {
            class: c_page_container()
            page_header {
                icon: "🔌"
                title: "WebSocket Chat"
                subtitle: "Connect to a WebSocket chat server with automatic UUID and Ping keep-alive."
            }
            my_card {
                title: "Connection"
                p {
                    class: c_demo_text()
                    "A random UUID is generated for each session. Click Connect to establish a real-time bidirectional connection. Ping messages are sent automatically to keep the connection alive."
                }
                if { state.get_connecting().get() } {
                    button {
                        class: c_primary_button()
                        disabled: true
                        "Wait"
                    }
                } else if { state.get_connected().get() } {
                    button {
                        class: c_sse_disconnect_button()
                        onclick: websocket_on_disconnect(state)
                        "Close"
                    }
                } else {
                    button {
                        class: c_primary_button()
                        onclick: websocket_on_connect(state)
                        "Connect"
                    }
                }
                if { !state.get_error().get().is_empty() } {
                    div {
                        class: c_camera_error_box()
                        state.get_error()
                    }
                }
            }
            if { state.get_connected().get() } {
                my_card {
                    title: "Send Message"
                    div {
                        class: c_ws_send_row()
                        input {
                            type: "text"
                            class: c_ws_message_input()
                            placeholder: WEBSOCKET_MESSAGE_PLACEHOLDER
                            value: state.get_message_input()
                            oninput: on_input_value(state.get_message_input())
                        }
                        button {
                            class: c_primary_button()
                            class: c_sse_action_button()
                            onclick: websocket_on_send(state)
                            "Send"
                        }
                    }
                }
            }
            my_card {
                title: "Messages"
                if { state.get_messages().get().is_empty() } {
                    div {
                        class: c_sse_messages_empty()
                        "No messages yet. Connect to start receiving."
                    }
                } else {
                    div {
                        class: c_ws_messages_list()
                        for (index, message) in { state.get_messages().get().iter().enumerate() } {
                            div {
                                key: index.to_string()
                                class: c_ws_message_item()
                                span {
                                    class: c_sse_message_index()
                                    format!("#{}", index + 1)
                                }
                                span {
                                    class: c_ws_message_data()
                                    message.data.clone()
                                }
                                if !message.time.is_empty() {
                                    span {
                                        class: c_ws_message_time()
                                        message.time.clone()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
