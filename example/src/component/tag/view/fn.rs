use crate::*;

/// A tag component for displaying semantic status indicators.
///
/// Supports solid and outline variants across black and white colour types.
///
/// # Arguments
///
/// - `VirtualNode<EuvTagProps>` - The props node containing color, variant, text, and on_click.
///
/// # Returns
///
/// - `VirtualNode` - A styled span tag element.
#[component]
pub(crate) fn euv_tag(node: VirtualNode<EuvTagProps>) -> VirtualNode {
    let EuvTagProps {
        color,
        variant,
        text,
        on_click,
    }: EuvTagProps = node.try_get_props().unwrap_or_default();
    match variant {
        EuvTagVariant::Solid => match color {
            EuvTagColor::Black => html! {
                span {
                    class: c_euv_tag_solid_black()
                    onclick: on_click
                    text
                }
            },
            EuvTagColor::White => html! {
                span {
                    class: c_euv_tag_solid_white()
                    onclick: on_click
                    text
                }
            },
        },
        EuvTagVariant::Outline => match color {
            EuvTagColor::Black => html! {
                span {
                    class: c_euv_tag_outline_black()
                    onclick: on_click
                    text
                }
            },
            EuvTagColor::White => html! {
                span {
                    class: c_euv_tag_outline_white()
                    onclick: on_click
                    text
                }
            },
        },
    }
}
