use super::*;

/// Renders a standard header with an emoji icon, title, and subtitle.
///
/// Produces a consistent style banner with gradient glow effect,
/// matching the home page design language. Every demo page uses this
/// component as its first child.
///
/// # Arguments
///
/// - `VirtualNode<EuvHeaderProps>` - The props node containing icon, title, and subtitle.
///
/// # Returns
///
/// - `VirtualNode` - The header virtual DOM tree.
#[component]
pub fn euv_header(node: VirtualNode<EuvHeaderProps>) -> VirtualNode {
    let EuvHeaderProps {
        icon,
        title,
        subtitle,
    }: EuvHeaderProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page()
            div {
                class: c_page_glow()
            }
            div {
                class: c_page_content()
                div {
                    class: c_page_icon()
                    icon
                }
                h1 {
                    class: c_page_title()
                    title
                }
                p {
                    class: c_page_subtitle()
                    subtitle
                }
            }
        }
    }
}
