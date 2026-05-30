use crate::*;

/// An async data demo page simulating network requests.
///
/// # Returns
///
/// - `VirtualNode` - The async demo page virtual DOM tree.
#[component]
pub(crate) fn page_async_demo(props: PageAsyncDemoProps) -> VirtualNode {
    let PageAsyncDemoProps = props;
    let fetch: UseFetch = use_fetch();
    html! {
        div {
            class: c_page_container()
            page_header("Async Data", "Simulating network requests with loading states.")
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
                if { fetch.get_loading().get() } {
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
                } else if { !fetch.get_error().get().is_empty() } {
                    div {
                        class: c_error_container()
                        div {
                            class: c_error_icon()
                            "!"
                        }
                        span {
                            class: c_error_text()
                            fetch.get_error()
                        }
                    }
                } else {
                    div {
                        class: c_data_box()
                        pre {
                            class: c_data_pre()
                            fetch.get_data()
                        }
                    }
                }
            }
        }
    }
}
