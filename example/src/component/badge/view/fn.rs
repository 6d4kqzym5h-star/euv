use crate::*;

/// A badge component for displaying status indicators with black/white colour scheme.
///
/// Supports solid (accent background) and outline (transparent with border) variants.
///
/// # Arguments
///
/// - `VirtualNode<MyBadgeProps>` - The props node containing text, outline, and on_click.
///
/// # Returns
///
/// - `VirtualNode` - A styled span badge element.
#[component]
pub(crate) fn my_badge(node: VirtualNode<MyBadgeProps>) -> VirtualNode {
    let MyBadgeProps {
        text,
        outline,
        on_click,
    }: MyBadgeProps = node.try_get_props().unwrap_or_default();
    if outline {
        html! {
            span {
                class: c_badge_outline()
                onclick: on_click
                text
            }
        }
    } else {
        html! {
            span {
                class: c_badge()
                onclick: on_click
                text
            }
        }
    }
}
