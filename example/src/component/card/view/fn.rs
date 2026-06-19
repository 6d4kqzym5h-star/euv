use crate::*;

/// A card component that wraps children with a styled container.
///
/// # Arguments
///
/// - `MyCardProps` - The typed props containing title.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A styled card element.
#[component]
pub(crate) fn euv_card(node: VirtualNode<MyCardProps>) -> VirtualNode {
    let MyCardProps { title, .. }: MyCardProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.try_get_child_node();
    html! {
        div {
            class: c_card()
            h3 {
                class: c_card_title()
                title
            }
            children
        }
    }
}
