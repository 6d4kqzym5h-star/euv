use crate::*;

/// A modal dialog component with overlay, title, and close handler.
///
/// # Arguments
///
/// - `VirtualNode` - The props node containing title, onclick (close handler), and children.
///
/// # Returns
///
/// - `VirtualNode` - A modal overlay element.
#[component]
pub(crate) fn my_modal(props: VirtualNode) -> VirtualNode {
    let children: Vec<VirtualNode> = props.get_children();
    let MyModalProps { title, on_close }: MyModalProps = props.into();
    let children_node: VirtualNode = VirtualNode::Fragment(children);
    html! {
        div {
            class: c_modal_overlay()
            onclick: on_close.clone()
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
                        onclick: on_close
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
