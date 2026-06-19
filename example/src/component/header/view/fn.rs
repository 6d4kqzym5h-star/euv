use crate::*;

/// Renders a standard header with an emoji icon, title, and subtitle.
///
/// Produces a consistent hero-style banner with gradient glow effect,
/// matching the home page design language. Every demo page uses this
/// component as its first child.
///
/// # Arguments
///
/// - `EuvHeaderProps` - The typed props containing icon, title, and subtitle.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The header virtual DOM tree.
#[component]
pub(crate) fn euv_header(node: VirtualNode<EuvHeaderProps>) -> VirtualNode {
    let EuvHeaderProps {
        icon,
        title,
        subtitle,
    }: EuvHeaderProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_hero()
            div {
                class: c_page_hero_glow()
            }
            div {
                class: c_page_hero_content()
                div {
                    class: c_page_hero_icon()
                    icon
                }
                h1 {
                    class: c_page_hero_title()
                    title
                }
                p {
                    class: c_page_hero_subtitle()
                    subtitle
                }
            }
        }
    }
}
