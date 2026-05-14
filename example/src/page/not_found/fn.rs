use crate::*;

/// A 404 not found page component.
///
/// # Returns
///
/// - `VirtualNode`: The 404 page virtual DOM tree.
pub fn page_not_found() -> VirtualNode {
    rsx! {
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
                label: "Go Home"
                onclick: move |_event: NativeEvent| {
                    navigate("/");
                }
                "Go Home"
            }
        }
    }
}
