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
#[component]
pub(crate) fn primary_button(props: VirtualNode) -> VirtualNode {
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

/// A modal primary button component with the same style as `primary_button` but without full width on mobile.
///
/// Used inside modal dialogs where the button should not stretch to 100% width on mobile viewports.
///
/// # Arguments
///
/// - `VirtualNode` - The props node containing label, onclick, and children.
///
/// # Returns
///
/// - `VirtualNode` - A styled button element.
#[component]
pub(crate) fn modal_primary_button(props: VirtualNode) -> VirtualNode {
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
            class: if { disabled } { c_modal_primary_button_disabled() } else { c_modal_primary_button() }
            onclick: onclick
            children_node
        }
    }
}
