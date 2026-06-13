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
            class: c_not_found_container()
            div {
                class: c_not_found_code()
                "404"
            }
            h2 {
                class: c_not_found_title()
                "Page Not Found"
            }
            p {
                class: c_not_found_text()
                "The requested page does not exist."
            }
            primary_button {
                label: "Back"
                onclick: Some(Rc::new(move |_: Event| {
                    navigate("/");
                }))
                "Back"
            }
        }
    }
}
