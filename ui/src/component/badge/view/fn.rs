use super::*;

/// A badge component for displaying status indicators with black/white colour scheme.
///
/// Supports solid (accent background) and outline (transparent with border) variants.
///
/// # Arguments
///
/// - `VirtualNode<EuvBadgeProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - A styled span badge element.
#[component]
pub fn euv_badge(node: VirtualNode<EuvBadgeProps>) -> VirtualNode {
    let EuvBadgeProps {
        text,
        outline,
        on_click,
    }: EuvBadgeProps = node.try_get_props().unwrap_or_default();
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
