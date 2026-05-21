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
    let children: Vec<VirtualNode> = props.get_children();
    let PrimaryButtonProps {
        label,
        onclick,
        disabled,
    }: PrimaryButtonProps = props.into();
    let display_children: Vec<VirtualNode> = if children.is_empty() {
        vec![VirtualNode::Text(TextNode::new(label, None))]
    } else {
        children
    };
    let children_node: VirtualNode = VirtualNode::Fragment(display_children);
    html! {
        button {
            class: if { disabled } { c_primary_button_disabled() } else { c_primary_button() }
            onclick: onclick
            children_node
        }
    }
}
