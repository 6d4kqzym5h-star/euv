use crate::*;

/// A card component that wraps children with a styled container.
///
/// # Arguments
///
/// - `MyCardProps` - The typed props containing title.
/// - `Vec<VirtualNode>` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - A styled card element.
#[component]
pub(crate) fn my_card(props: MyCardProps, children: Vec<VirtualNode>) -> VirtualNode {
    let MyCardProps { title, .. } = props;
    let children_node: Vec<VirtualNode> = children;
    html! {
        div {
            class: c_card()
            h3 {
                class: c_card_title()
                title
            }
            children_node
        }
    }
}
