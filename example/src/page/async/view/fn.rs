use crate::*;

/// An async data demo page simulating network requests.
///
/// # Returns
///
/// - `VirtualNode` - The async demo page virtual DOM tree.
#[component]
pub(crate) fn page_async_demo(node: VirtualNode<PageAsyncDemoProps>) -> VirtualNode {
    let PageAsyncDemoProps = node.try_get_props().unwrap_or_default();
    let fetch: UseFetch = use_fetch();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "⏳"
                title: "Async Data"
                subtitle: "Simulating network requests with loading states."
            }
            euv_card {
                title: "Fetch Data"
                p {
                    class: c_fetch_hint()
                    "Click the button below to fetch data from ltpp.vip"
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Fetch"
                        onclick: fetch_on_fetch(fetch)
                        disabled: fetch.get_loading()
                    }
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
