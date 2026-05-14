use crate::*;

/// A badge component for displaying status indicators.
///
/// # Arguments
///
/// - `VirtualNode`: The props node containing color, text, and on_click.
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
    let on_click: Option<NativeEventHandler> = props.try_get_callback("on_click");
    rsx! {
        span {
            class: c_badge()
            style: {background: {color};}
            onclick: on_click
            text_prop
        }
    }
}
