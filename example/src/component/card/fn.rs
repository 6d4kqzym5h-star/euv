use crate::*;

/// A card component that wraps children with a styled container.
///
/// # Arguments
///
/// - `VirtualNode`: The props node containing title and children.
///
/// # Returns
///
/// - `VirtualNode`: A styled card element.
pub fn my_card(props: VirtualNode) -> VirtualNode {
    let title: String = props.try_get_prop(&Attribute::Title).unwrap_or_default();
    let children: Vec<VirtualNode> = props.get_children();
    let children_node: VirtualNode = VirtualNode::Fragment(children);
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
