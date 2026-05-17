use crate::*;

/// A primary button component with customizable label and click handler.
///
/// # Arguments
///
/// - `VirtualNode` - The props node containing label, onclick, and children.
///
/// # Returns
///
/// - `VirtualNode` - A styled button element.
pub fn primary_button(props: VirtualNode) -> VirtualNode {
    let label: String = props
        .try_get_prop(&Attribute::Other("label".to_string()))
        .unwrap_or_else(|| "Button".to_string());
    let children: Vec<VirtualNode> = props.get_children();
    let onclick_handler: Option<NativeEventHandler> = props.try_get_event(&NativeEventName::Click);
    let display_children: Vec<VirtualNode> = if children.is_empty() {
        vec![VirtualNode::Text(TextNode::new(label, None))]
    } else {
        children
    };
    let children_node: VirtualNode = VirtualNode::Fragment(display_children);
    html! {
        button {
            class: c_primary_button()
            style: { display: "inline-flex"; align-items: "center"; gap: "8px"; }
            onclick: onclick_handler
            children_node
        }
    }
}
