use crate::*;

/// A Server-Sent Events (SSE) demo page showcasing real-time streaming from an SSE endpoint.
///
/// Renders a page header, a URL input card for connecting to an SSE server,
/// and a messages display card showing real-time event data.
///
/// # Returns
///
/// - `VirtualNode` - The SSE demo page virtual DOM tree.
#[component]
pub(crate) fn page_sse(node: VirtualNode<PageSseProps>) -> VirtualNode {
    let PageSseProps = node.try_get_props().unwrap_or_default();
    let state: UseSse = use_sse();
    sse_cleanup(state);
    html! {
        div {
            class: c_page_container()
            page_header {
                icon: "📡"
                title: "Server-Sent Events"
                subtitle: "Connect to an SSE endpoint and receive real-time streaming events."
            }
            my_card {
                title: "Connection"
                p {
                    class: c_demo_text()
                    "Enter the SSE endpoint URL and click Connect to start receiving server-sent events."
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
                        onclick: sse_on_disconnect(state)
                        "Close"
                    }
                } else {
                    button {
                        class: c_primary_button()
                        onclick: sse_on_connect(state)
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
            my_card {
                title: "Messages"
                if { state.get_messages().get().is_empty() } {
                    div {
                        class: c_sse_messages_empty()
                        "No messages received yet. Connect to an SSE endpoint to start receiving events."
                    }
                } else {
                    div {
                        class: c_sse_messages_list()
                        for (index, message) in { state.get_messages().get().iter().enumerate() } {
                            div {
                                key: index.to_string()
                                class: c_sse_message_item()
                                span {
                                    class: c_sse_message_index()
                                    format!("#{}", index + 1)
                                }
                                span {
                                    class: c_sse_message_data()
                                    message.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
