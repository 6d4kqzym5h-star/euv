use crate::*;

/// A home page component displaying the euv version information.
///
/// # Returns
///
/// - `VirtualNode` - The home page virtual DOM tree.
#[component]
pub(crate) fn page_home(node: VirtualNode<PageHomeProps>) -> VirtualNode {
    let PageHomeProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Home"
                subtitle: "Welcome to the euv example application."
            }
            my_card {
                title: "Package Info"
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Name:"
                    }
                    span {
                        class: c_info_value()
                        EUV_PACKAGE_NAME
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Version:"
                    }
                    span {
                        class: c_info_value()
                        EUV_VERSION
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Build Time:"
                    }
                    span {
                        class: c_info_value()
                        EUV_BUILD_TIME
                    }
                }
            }
        }
    }
}
