use crate::*;

/// A badge component for displaying status indicators.
///
/// Supports an optional `outline` attribute. When present, the badge renders
/// with a transparent background and a colored border instead of a solid fill.
///
/// # Arguments
///
/// - `MyBadgeProps` - The typed props containing color, text, outline, and on_click.
/// - `Vec<VirtualNode>` - The children nodes (unused).
///
/// # Returns
///
/// - `VirtualNode` - A styled span badge element.
#[component]
pub(crate) fn my_badge(props: MyBadgeProps, _children: Vec<VirtualNode>) -> VirtualNode {
    let MyBadgeProps {
        color,
        text,
        outline,
        on_click,
    } = props;
    if outline {
        let border_color: String = color.clone();
        html! {
            span {
                class: c_badge_outline()
                style: { color: { &color }; border-color: { &border_color }; }
                onclick: on_click
                text
            }
        }
    } else {
        html! {
            span {
                class: c_badge()
                style: { background: { &color }; }
                onclick: on_click
                text
            }
        }
    }
}
