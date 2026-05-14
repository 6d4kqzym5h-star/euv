use crate::*;

/// A primary button component with customizable label and click handler.
///
/// # Arguments
///
/// - `VirtualNode`: The props node containing label, onclick, and children.
///
/// # Returns
///
/// - `VirtualNode`: A styled button element.
pub fn primary_button(props: VirtualNode) -> VirtualNode {
    let label: String = props
        .try_get_prop(&Attribute::Other("label".to_string()))
        .unwrap_or_else(|| "Button".to_string());
    let children: Vec<VirtualNode> = props.get_children();
    let onclick_handler: Option<NativeEventHandler> = props.try_get_event(&NativeEventName::Click);
    let child_node: VirtualNode = if let Some(first) = children.into_iter().next() {
        first
    } else {
        VirtualNode::Text(vdom::TextNode::new(label, None))
    };
    let child_node_clone: VirtualNode = child_node.clone();
    rsx! {
        button {
            class: c_primary_button()
            onclick: onclick_handler
            child_node_clone
        }
    }
}
