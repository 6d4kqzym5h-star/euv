use crate::*;

/// Renders a branded logo element displaying the "E" letter with a gradient background.
///
/// When `on_click` is provided, renders as a `<button>` element;
/// otherwise renders as a `<span>` element (e.g. for use inside anchor tags).
/// Used both as the navigation sidebar logo and as the vConsole floating action button.
///
/// # Arguments
///
/// - `LogoButtonProps` - The typed props containing variant, click handler.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A styled element with the "E" branding.
#[component]
pub(crate) fn logo_button(node: VirtualNode<LogoButtonProps>) -> VirtualNode {
    let LogoButtonProps { variant, on_click }: LogoButtonProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.try_get_child_node();
    let class_name: String = match variant {
        LogoButtonVariant::Nav => format!(
            "{} {}",
            c_logo_button().get_name(),
            c_logo_button_nav().get_name()
        ),
        LogoButtonVariant::Fab => format!(
            "{} {}",
            c_logo_button().get_name(),
            c_logo_button_fab().get_name()
        ),
    };
    if on_click.is_some() {
        html! {
            button {
                class: class_name
                onclick: on_click
                "E"
                children
            }
        }
    } else {
        html! {
            span {
                class: class_name
                "E"
            }
        }
    }
}
