use crate::*;

/// A primary button component with customizable label and click handler.
///
/// # Arguments
///
/// - `VirtualNode<PrimaryButtonProps>` - The props node containing label, onclick, and disabled.
///
/// # Returns
///
/// - `VirtualNode` - A styled button element.
#[component]
pub(crate) fn primary_button(mut node: VirtualNode<PrimaryButtonProps>) -> VirtualNode {
    let PrimaryButtonProps {
        label,
        onclick: click_handler,
        disabled,
    } = node.try_take_props().unwrap_or_default();
    let children: VirtualNode = node.take_children();
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(label.to_string(), None)),
        other => other,
    };
    html! {
        button {
            class: if { disabled } { c_primary_button_disabled() } else { c_primary_button() }
            onclick: click_handler
            content
        }
    }
}

/// A modal primary button component with the same style as `primary_button` but without full width on mobile.
///
/// Used inside modal dialogs where the button should not stretch to 100% width on mobile viewports.
///
/// # Arguments
///
/// - `VirtualNode<PrimaryButtonProps>` - The props node containing label, onclick, and disabled.
///
/// # Returns
///
/// - `VirtualNode` - A styled button element.
#[component]
pub(crate) fn modal_primary_button(mut node: VirtualNode<PrimaryButtonProps>) -> VirtualNode {
    let PrimaryButtonProps {
        label,
        onclick: click_handler,
        disabled,
    } = node.try_take_props().unwrap_or_default();
    let children: VirtualNode = node.take_children();
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(label.to_string(), None)),
        other => other,
    };
    html! {
        button {
            class: if { disabled } { c_modal_primary_button_disabled() } else { c_modal_primary_button() }
            onclick: click_handler
            content
        }
    }
}
