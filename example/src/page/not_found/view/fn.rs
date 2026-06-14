use crate::*;

/// A 404 not found page component.
///
/// # Returns
///
/// - `VirtualNode` - The 404 page virtual DOM tree.
#[component]
pub(crate) fn page_not_found(node: VirtualNode<PageNotFoundProps>) -> VirtualNode {
    let PageNotFoundProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            page_header {
                icon: "🔍"
                title: "404 Not Found"
                subtitle: "The page you are looking for does not exist."
            }
            div {
                class: c_not_found_actions()
                primary_button {
                    label: "Back to Home"
                    onclick: Some(Rc::new(move |_: Event| {
                        navigate("/");
                    }))
                    "Back to Home"
                }
            }
        }
    }
}
