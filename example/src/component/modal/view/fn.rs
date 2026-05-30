use crate::*;

/// A modal dialog component with overlay, title, and close handler.
///
/// # Arguments
///
/// - `MyModalProps` - The typed props containing title and onclick (close handler).
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A modal overlay element.
#[component]
pub(crate) fn my_modal(mut node: VirtualNode<MyModalProps>) -> VirtualNode {
    let MyModalProps { title, onclick } = node.try_take_props().unwrap_or_default();
    let children: VirtualNode = node.take_children();
    html! {
        div {
            class: c_modal_overlay()
            onclick: onclick.clone()
            div {
                class: c_modal_content()
                onclick: move |_event: Event| {}
                div {
                    class: c_modal_header()
                    h3 {
                        class: c_modal_title()
                        title
                    }
                    button {
                        class: c_modal_close_button()
                        onclick: onclick
                        "×"
                    }
                }
                div {
                    class: c_modal_body()
                    children
                }
            }
        }
    }
}
