use crate::*;

/// A badge component for displaying status indicators.
///
/// Supports an optional `outline` attribute. When present, the badge renders
/// with a transparent background and a colored border instead of a solid fill.
///
/// # Arguments
///
/// - `VirtualNode`: The props node containing color, text, outline, and on_click.
///
/// # Returns
///
/// - `VirtualNode`: A styled span badge element.
pub fn my_badge(props: VirtualNode) -> VirtualNode {
    let color: String = props
        .try_get_prop(&Attribute::Other("color".to_string()))
        .unwrap_or_else(|| "#4f46e5".to_string());
    let text_prop: String = props
        .try_get_prop(&Attribute::Other("text".to_string()))
        .unwrap_or_default();
    let outline: bool = props
        .try_get_prop(&Attribute::Other("outline".to_string()))
        .is_some();
    let on_click: Option<NativeEventHandler> = props.try_get_callback("on_click");
    if outline {
        let border_color: String = color.clone();
        html! {
            span {
                class: c_badge_outline()
                style: {color: {color}; border_color: {border_color};}
                onclick: on_click
                text_prop
            }
        }
    } else {
        html! {
            span {
                class: c_badge()
                style: {background: {color};}
                onclick: on_click
                text_prop
            }
        }
    }
}
