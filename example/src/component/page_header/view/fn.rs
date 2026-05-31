use crate::*;

/// Renders a standard page header with a title and subtitle.
///
/// Produces a consistent `div > h1 + p` structure used by every demo page
/// in the example application.
///
/// # Arguments
///
/// - `PageHeaderProps` - The typed props containing title and subtitle.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The page header virtual DOM tree.
#[component]
pub(crate) fn page_header(node: VirtualNode<PageHeaderProps>) -> VirtualNode {
    let PageHeaderProps { title, subtitle } = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_header()
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
