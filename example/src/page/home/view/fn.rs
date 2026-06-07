use crate::*;

/// A home page component displaying the euv version information and
/// native bridge system info when the Tauri runtime is available.
///
/// # Returns
///
/// - `VirtualNode` - The home page virtual DOM tree.
#[component]
pub(crate) fn page_home(node: VirtualNode<PageHomeProps>) -> VirtualNode {
    let PageHomeProps = node.try_get_props().unwrap_or_default();
    let native_bridge_state: UseNativeBridge = use_native_bridge();
    let is_first_render: Signal<bool> = use_signal(|| true);
    if is_first_render.get() {
        is_first_render.set(false);
        load_native_bridge_data(native_bridge_state);
    }
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Home"
                subtitle: EUV_DESCRIPTION
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
                        "Edition:"
                    }
                    span {
                        class: c_info_value()
                        EUV_EDITION
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Repository:"
                    }
                    a {
                        class: c_info_link()
                        href: EUV_REPOSITORY
                        target: "_blank"
                        EUV_REPOSITORY_NAME
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Authors:"
                    }
                    span {
                        class: c_info_value()
                        EUV_AUTHORS
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "License:"
                    }
                    span {
                        class: c_info_value()
                        EUV_LICENSE
                    }
                }
            }
            my_card {
                title: "Build Info"
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Build Date:"
                    }
                    span {
                        class: c_info_value()
                        EUV_BUILD_DATE
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Build Clock:"
                    }
                    span {
                        class: c_info_value()
                        EUV_BUILD_CLOCK
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Build Timestamp:"
                    }
                    span {
                        class: c_info_value()
                        EUV_BUILD_TIMESTAMP
                    }
                }
            }
            if { !native_bridge_state.get_loading().get() && native_bridge_state.get_available().get() } {
                my_card {
                    title: "Native Bridge Info"
                    div {
                        class: c_info_row()
                        span {
                            class: c_info_label()
                            "From Cache:"
                        }
                        span {
                            class: c_info_value()
                            if { native_bridge_state.get_from_cache().get() } { "Yes" } else { "No" }
                        }
                    }
                    div {
                        class: c_info_row()
                        span {
                            class: c_info_label()
                            "Remote URL:"
                        }
                        span {
                            class: c_info_value()
                            native_bridge_state.get_remote_url()
                        }
                    }
                    div {
                        class: c_info_row()
                        span {
                            class: c_info_label()
                            "Source:"
                        }
                        span {
                            class: c_info_value()
                            native_bridge_state.get_source()
                        }
                    }
                    div {
                        class: c_info_row()
                        span {
                            class: c_info_label()
                            "Cache Path:"
                        }
                        span {
                            class: c_info_value()
                            native_bridge_state.get_cache_path()
                        }
                    }
                    div {
                        class: c_info_row()
                        span {
                            class: c_info_label()
                            "Permissions:"
                        }
                        span {
                            class: c_info_value()
                            native_bridge_state.get_permissions()
                        }
                    }
                }
            }
        }
    }
}
