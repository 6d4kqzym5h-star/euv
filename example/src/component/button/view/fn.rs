use crate::*;

/// A primary button component with customizable label and click handler.
///
/// # Arguments
///
/// - `PrimaryButtonProps` - The typed props containing label, onclick, and disabled.
/// - `Vec<VirtualNode>` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A styled button element.
#[component]
pub(crate) fn primary_button(props: PrimaryButtonProps, children: Vec<VirtualNode>) -> VirtualNode {
    let PrimaryButtonProps {
        label,
        onclick,
        disabled,
    } = props;
    let children_node: Vec<VirtualNode> = if children.is_empty() {
        vec![VirtualNode::Text(TextNode::new(label, None))]
    } else {
        children
    };
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
/// - `PrimaryButtonProps` - The typed props containing label, onclick, and disabled.
/// - `Vec<VirtualNode>` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A styled button element.
#[component]
pub(crate) fn modal_primary_button(
    props: PrimaryButtonProps,
    children: Vec<VirtualNode>,
) -> VirtualNode {
    let PrimaryButtonProps {
        label,
        onclick,
        disabled,
    } = props;
    let children_node: Vec<VirtualNode> = if children.is_empty() {
        vec![VirtualNode::Text(TextNode::new(label, None))]
    } else {
        children
    };
    html! {
        button {
            class: if { disabled } { c_modal_primary_button_disabled() } else { c_modal_primary_button() }
            onclick: onclick
            children_node
        }
    }
}
