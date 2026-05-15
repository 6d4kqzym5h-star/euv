use crate::*;

/// A modal dialog component with overlay, title, and close handler.
///
/// # Arguments
///
/// - `VirtualNode`: The props node containing title, onclick (close handler), and children.
///
/// # Returns
///
/// - `VirtualNode`: A modal overlay element.
pub fn my_modal(props: VirtualNode) -> VirtualNode {
    let title: String = props.try_get_prop(&Attribute::Title).unwrap_or_default();
    let on_close: Option<NativeEventHandler> = props.try_get_event(&NativeEventName::Click);
    let on_close_overlay: Option<NativeEventHandler> = on_close.clone();
    let on_close_button: Option<NativeEventHandler> = on_close.clone();
    let children: Vec<VirtualNode> = props.get_children();
    let children_node: VirtualNode = VirtualNode::Fragment(children);
    html! {
        div {
            class: c_modal_overlay()
            onclick: on_close_overlay
            div {
                class: c_modal_content()
                onclick: move |_event: NativeEvent| {}
                div {
                    class: c_modal_header()
                    h3 {
                        class: c_modal_title()
                        title
                    }
                    primary_button {
                        label: "Close"
                        onclick: on_close_button
                        "×"
                    }
                }
                div {
                    class: c_modal_body()
                    children_node
                }
            }
        }
    }
}
