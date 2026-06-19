use crate::*;

/// A tag component for displaying semantic status indicators.
///
/// Supports solid and outline variants across five semantic colour types.
/// Replaces the previous `my_badge` component with a more structured API.
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
            EuvTagColor::Success => html! {
                span {
                    class: c_badge_success()
                    onclick: on_click
                    text
                }
            },
            EuvTagColor::Error => html! {
                span {
                    class: c_badge_error()
                    onclick: on_click
                    text
                }
            },
            EuvTagColor::Warning => html! {
                span {
                    class: c_badge_warning()
                    onclick: on_click
                    text
                }
            },
            EuvTagColor::Info => html! {
                span {
                    class: c_badge_info()
                    onclick: on_click
                    text
                }
            },
            EuvTagColor::Purple => html! {
                span {
                    class: c_euv_tag_solid_purple()
                    onclick: on_click
                    text
                }
            },
        },
        EuvTagVariant::Outline => match color {
            EuvTagColor::Success => html! {
                span {
                    class: c_euv_tag_outline_success()
                    onclick: on_click
                    text
                }
            },
            EuvTagColor::Error => html! {
                span {
                    class: c_euv_tag_outline_error()
                    onclick: on_click
                    text
                }
            },
            EuvTagColor::Warning => html! {
                span {
                    class: c_euv_tag_outline_warning()
                    onclick: on_click
                    text
                }
            },
            EuvTagColor::Info => html! {
                span {
                    class: c_euv_tag_outline_info()
                    onclick: on_click
                    text
                }
            },
            EuvTagColor::Purple => html! {
                span {
                    class: c_euv_tag_outline_purple()
                    onclick: on_click
                    text
                }
            },
        },
    }
}
