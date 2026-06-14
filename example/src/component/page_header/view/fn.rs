use crate::*;

/// Renders a standard page header with an emoji icon, title, and subtitle.
///
/// Produces a consistent hero-style banner with gradient glow effect,
/// matching the home page design language. Every demo page uses this
/// component as its first child.
///
/// # Arguments
///
/// - `PageHeaderProps` - The typed props containing icon, title, and subtitle.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The page header virtual DOM tree.
#[component]
pub(crate) fn page_header(node: VirtualNode<PageHeaderProps>) -> VirtualNode {
    let PageHeaderProps {
        icon,
        title,
        subtitle,
    }: PageHeaderProps = node.try_get_props().unwrap_or_default();
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
