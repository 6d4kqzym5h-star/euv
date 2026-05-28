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
pub(crate) fn my_card(props: MyCardProps) -> VirtualNode {
    let MyCardProps {
        title, children, ..
    } = props;
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
