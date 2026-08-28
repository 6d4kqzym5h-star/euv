use super::*;

/// A generic sticky anchor table-of-contents aligned with common docs
/// frameworks.
///
/// Renders nothing when `items` is empty. The caller controls the column
/// width and responsive visibility; the component provides the sticky inner
/// stack.
///
/// # Arguments
///
/// - `VirtualNode<EuvTocProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The TOC virtual DOM tree.
#[component]
pub fn euv_toc(node: VirtualNode<EuvTocProps>) -> VirtualNode {
    let EuvTocProps { title, items }: EuvTocProps = node.try_get_props().unwrap_or_default();
    if items.is_empty() {
        return html! {
            ""
        };
    }
    html! {
        div {
            class: c_euv_toc()
            div {
                class: c_euv_toc_title()
                {
                    title
                }
            }
            for item in items.iter() {
                a {
                    key: item.href
                    class: if { item.level > 2u8 } {
                        c_euv_toc_link_nested()
                    } else {
                        c_euv_toc_link()
                    }
                    href: item.href
                    {
                        item.text
                    }
                }
            }
        }
    }
}
