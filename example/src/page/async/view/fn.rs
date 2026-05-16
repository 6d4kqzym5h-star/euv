use crate::*;

/// Builds the status display node based on current async state.
///
/// # Arguments
///
/// - `Signal<bool>`: The loading state signal.
/// - `Signal<String>`: The error message signal.
/// - `Signal<String>`: The fetched data signal.
///
/// # Returns
///
/// - `VirtualNode`: The status indicator virtual DOM tree.
fn build_status_node(
    loading: Signal<bool>,
    error: Signal<String>,
    data: Signal<String>,
) -> VirtualNode {
    let is_loading: bool = loading.get();
    let error_text: String = error.get();
    let data_text: String = data.get();
    if is_loading {
        html! {
            div {
                class: c_loading_container()
                div {
                    class: c_spinner()
                }
                div {
                    class: c_loading_text_col()
                    span {
                        class: c_loading_title()
                        "Loading..."
                    }
                    span {
                        class: c_loading_subtitle()
                        "Fetching data from server"
                    }
                }
            }
        }
    } else if !error_text.is_empty() {
        html! {
            div {
                class: c_error_container()
                div {
                    class: c_error_icon()
                    "!"
                }
                span {
                    class: c_error_text()
                    error_text
                }
            }
        }
    } else {
        html! {
            div {
                class: c_data_box()
                pre {
                    class: c_data_pre()
                    data_text
                }
            }
        }
    }
}

/// An async data demo page simulating network requests.
///
/// # Returns
///
/// - `VirtualNode`: The async demo page virtual DOM tree.
pub fn page_async_demo() -> VirtualNode {
    let fetch: UseFetch = use_fetch();
    html! {
        div {
            class: c_page_container()
            { page_header("Async Data", "Simulating network requests with loading states.") }
            my_card {
                title: "Fetch Data"
                p {
                    class: c_fetch_hint()
                    "Click the button below to fetch data from httpbin.org"
                }
                primary_button {
                    label: "Fetch"
                    onclick: fetch_on_fetch(fetch)
                    "Fetch"
                }
                build_status_node(fetch.loading, fetch.error, fetch.data)
            }
        }
    }
}
