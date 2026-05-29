use crate::*;

/// A 404 not found page component.
///
/// # Returns
///
/// - `VirtualNode` - The 404 page virtual DOM tree.
pub(crate) fn page_not_found() -> VirtualNode {
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
                label: "Go Back"
                onclick: Some(Rc::new(move |_event: Event| {
                    navigate("/");
                }))
                "Go Back"
            }
        }
    }
}
