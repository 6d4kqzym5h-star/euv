use crate::*;

/// A modal dialog component with overlay, title, and close handler.
///
/// # Arguments
///
/// - `MyModalProps` - The typed props containing title and onclick (close handler).
/// - `Vec<VirtualNode>` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A modal overlay element.
#[component]
pub(crate) fn my_modal(props: MyModalProps, children: Vec<VirtualNode>) -> VirtualNode {
    let children_node: Vec<VirtualNode> = children;
    let MyModalProps { title, onclick } = props;
    html! {
        div {
            class: c_modal_overlay()
            onclick: onclick.clone()
            div {
                class: c_modal_content()
                onclick: move |_event: Event| { }
                div {
                    class: c_modal_header()
                    h3 {
                        class: c_modal_title()
                        title
                    }
                    modal_primary_button {
                        label: "Close"
                        onclick: onclick
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
