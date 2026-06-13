use crate::*;

/// A modal dialog component with overlay, title, and close handler.
///
/// # Arguments
///
/// - `MyModalProps` - The typed props containing title and onclick.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A modal overlay element.
#[component]
pub(crate) fn my_modal(node: VirtualNode<MyModalProps>) -> VirtualNode {
    let MyModalProps {
        title,
        onclick,
        closing,
    }: MyModalProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.try_get_child_node();
    let on_modal_content_click = move |_: Event| {};
    html! {
        div {
            class: if { closing.get() } { c_modal_overlay_closing() } else { c_modal_overlay() }
            onclick: onclick.clone()
            div {
                class: if { closing.get() } { c_modal_content_closing() } else { c_modal_content() }
                onclick: on_modal_content_click
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
