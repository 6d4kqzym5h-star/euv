use crate::*;

/// A badge component for displaying status indicators.
///
/// Supports an optional `outline` attribute. When present, the badge renders
/// with a transparent background and a colored border instead of a solid fill.
///
/// # Arguments
///
/// - `MyBadgeProps` - The typed props containing color, text, outline, and on_click.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A styled span badge element.
#[component]
pub(crate) fn my_badge(node: VirtualNode<MyBadgeProps>) -> VirtualNode {
    let MyBadgeProps {
        color,
        text,
        outline,
        on_click,
    } = node.try_get_props().unwrap_or_default();
    if outline {
        html! {
            span {
                class: c_badge_outline()
                style: {
                    color: { color };
                    border-color: { color };
                }
                onclick: on_click
                text
            }
        }
    } else {
        html! {
            span {
                class: c_badge()
                style: {
                    background: { color };
                }
                onclick: on_click
                text
            }
        }
    }
}
