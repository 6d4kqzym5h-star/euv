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
pub(crate) fn my_card(mut node: VirtualNode<MyCardProps>) -> VirtualNode {
    let MyCardProps { title, .. } = node.try_take_props().unwrap_or_default();
    let children: VirtualNode = node.take_children();
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
