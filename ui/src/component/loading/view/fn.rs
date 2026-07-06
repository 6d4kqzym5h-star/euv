use crate::*;

/// A loading indicator component with spinner and optional subtitle.
///
/// Renders a spinner with a title (and optional subtitle) in either inline
/// or overlay mode. In overlay mode the component is absolutely positioned
/// to cover its parent container, making it ideal for overlaying canvases
/// or other content areas during asynchronous initialization.
///
/// # Arguments
///
/// - `VirtualNode<EuvLoadingProps>` - The props node containing title, subtitle, overlay, and background.
///
/// # Returns
///
/// - `VirtualNode` - A styled loading indicator element.
#[component]
pub fn euv_loading(node: VirtualNode<EuvLoadingProps>) -> VirtualNode {
    let EuvLoadingProps {
        title,
        subtitle,
        overlay,
        background,
    }: EuvLoadingProps = node.try_get_props().unwrap_or_default();
    let has_subtitle: bool = !subtitle.is_empty();
    if overlay {
        html! {
            div {
                class: c_loading_overlay(background)
                div {
                    class: c_spinner()
                }
                div {
                    class: c_loading_text_col()
                    span {
                        class: c_loading_title()
                        title
                    }
                    if has_subtitle {
                        span {
                            class: c_loading_subtitle()
                            subtitle
                        }
                    }
                }
            }
        }
    } else {
        html! {
            div {
                class: c_loading_container()
                div {
                    class: c_spinner()
                }
                div {
                    class: c_loading_text_col()
                    span {
                        class: c_loading_title()
                        title
                    }
                    if has_subtitle {
                        span {
                            class: c_loading_subtitle()
                            subtitle
                        }
                    }
                }
            }
        }
    }
}
