use crate::*;

/// Renders a standard page header with a title and subtitle.
///
/// Produces a consistent `div > h1 + p` structure used by every demo page
/// in the example application.
///
/// # Arguments
///
/// - `&str`: The page title text.
/// - `&str`: The page subtitle / description text.
///
/// # Returns
///
/// - `VirtualNode`: The page header virtual DOM tree.
pub fn page_header(title: &str, subtitle: &str) -> VirtualNode {
    html! {
        div {
            class: c_page_header()
            h1 {
                class: c_page_title()
                title
            }
            p {
                class: c_page_subtitle()
                subtitle
            }
        }
    }
}
