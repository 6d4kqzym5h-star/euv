use crate::*;

/// A custom attributes demo page showcasing data-* and aria-* attributes.
///
/// # Returns
///
/// - `VirtualNode`: The custom attributes demo page virtual DOM tree.
pub fn page_custom_attrs() -> VirtualNode {
    html! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Custom Attributes"
                }
                p {
                    class: c_page_subtitle()
                    "Using data-* and aria-* attributes on elements."
                }
            }
            my_card {
                title: "Data & ARIA Attributes"
                div {
                    data_role: "container"
                    data_id: "12345"
                    aria_label: "Demo section"
                    class: c_custom_attrs_demo()
                    p {
                        class: c_demo_text()
                        "This div has custom data-* and aria-* attributes."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Inspect the element to see data-role, data-id, and aria-label."
                    }
                }
            }
        }
    }
}
