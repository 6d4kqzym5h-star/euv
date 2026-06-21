use crate::*;

/// A home page component displaying the Euv framework showcase.
///
/// Features a hero section with animated gradient, feature highlights,
/// package metadata cards, and interactive demos — all with modern
/// glass-morphism design language.
///
/// # Returns
///
/// - `VirtualNode` - The home page virtual DOM tree.
#[component]
pub(crate) fn page_home(node: VirtualNode<PageHomeProps>) -> VirtualNode {
    let PageHomeProps = node.try_get_props().unwrap_or_default();
    let native_bridge_state: UseNativeBridge = use_native_bridge();
    load_native_bridge_data(native_bridge_state);
    let version: String = format!("v{EUV_VERSION}");
    html! {
        div {
            class: c_page_container()
            div {
                class: c_home_hero()
                div {
                    class: c_home_hero_content()
                    h1 {
                        class: c_home_hero_title()
                        BRAND_NAME
                    }
                    div {
                        class: c_home_hero_badge_row()
                        div {
                            class: c_home_hero_badge()
                            version.clone()
                        }
                    }
                    p {
                        class: c_home_hero_subtitle()
                        EUV_DESCRIPTION
                    }
                    div {
                        class: c_home_hero_actions()
                        a {
                            class: c_home_btn_primary()
                            href: EUV_REPOSITORY
                            target: "_blank"
                            onclick: external_link_handler(EUV_REPOSITORY.to_string())
                            "GitHub"
                        }
                        a {
                            class: c_home_btn_secondary()
                            href: "#/counter"
                            onclick: link_handler("/counter".to_string())
                            "Explore"
                        }
                    }
                }
            }
            div {
                class: c_home_stats()
                div {
                    class: c_home_stat_card()
                    div {
                        class: c_home_stat_icon()
                        "⚡"
                    }
                    div {
                        class: c_home_stat_value()
                        "WASM"
                    }
                    div {
                        class: c_home_stat_label()
                        "Runtime"
                    }
                }
                div {
                    class: c_home_stat_card()
                    div {
                        class: c_home_stat_icon()
                        "🦀"
                    }
                    div {
                        class: c_home_stat_value()
                        "Rust"
                    }
                    div {
                        class: c_home_stat_label()
                        "Language"
                    }
                }
                div {
                    class: c_home_stat_card()
                    div {
                        class: c_home_stat_icon()
                        "🎨"
                    }
                    div {
                        class: c_home_stat_value()
                        "VDOM"
                    }
                    div {
                        class: c_home_stat_label()
                        "Architecture"
                    }
                }
                div {
                    class: c_home_stat_card()
                    div {
                        class: c_home_stat_icon()
                        "📦"
                    }
                    div {
                        class: c_home_stat_value()
                        "4"
                    }
                    div {
                        class: c_home_stat_label()
                        "Crates"
                    }
                }
            }
            div {
                h2 {
                    class: c_home_section_title()
                    "Features"
                }
                p {
                    class: c_home_section_desc()
                    "Everything you need for declarative cross-platform UI development."
                }
                div {
                    class: c_home_feature_grid()
                    euv_card {
                        title: "Reactive Signals"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_header()
                                div {
                                    class: c_feature_icon()
                                    "⚡"
                                }
                                h4 {
                                    class: c_feature_name()
                                    "Signal-Based Reactivity"
                                }
                            }
                            p {
                                class: c_feature_desc()
                                "Fine-grained reactive state management with automatic dependency tracking."
                            }
                        }
                    }
                    euv_card {
                        title: "Virtual DOM"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_header()
                                div {
                                    class: c_feature_icon()
                                    "🌲"
                                }
                                h4 {
                                    class: c_feature_name()
                                    "Efficient Diffing"
                                }
                            }
                            p {
                                class: c_feature_desc()
                                "Virtual DOM with optimized reconciliation for smooth 60fps updates."
                            }
                        }
                    }
                    euv_card {
                        title: "HTML Macros"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_header()
                                div {
                                    class: c_feature_icon()
                                    "🏗️"
                                }
                                h4 {
                                    class: c_feature_name()
                                    "Declarative Syntax"
                                }
                            }
                            p {
                                class: c_feature_desc()
                                "Write UI with familiar HTML-like macros that compile to efficient Rust."
                            }
                        }
                    }
                    euv_card {
                        title: "Cross-Platform"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_header()
                                div {
                                    class: c_feature_icon()
                                    "🌐"
                                }
                                h4 {
                                    class: c_feature_name()
                                    "WebAssembly Powered"
                                }
                            }
                            p {
                                class: c_feature_desc()
                                "Run anywhere with WASM — browsers, servers, and native via bridge."
                            }
                        }
                    }
                }
            }
            div {
                h2 {
                    class: c_home_section_title()
                    "Package Info"
                }
                euv_card {
                    title: "Project Details"
                    euv_info {
                        label: "Name"
                        EUV_PACKAGE_NAME
                    }
                    euv_info {
                        label: "Version"
                        version.clone()
                    }
                    euv_info {
                        label: "Edition"
                        EUV_EDITION
                    }
                    euv_info {
                        label: "License"
                        EUV_LICENSE
                    }
                    euv_info {
                        label: "Authors"
                        EUV_AUTHORS
                    }
                    euv_info {
                        label: "Repository"
                        a {
                            class: c_info_link()
                            href: EUV_REPOSITORY
                            target: "_blank"
                            onclick: external_link_handler(EUV_REPOSITORY.to_string())
                            EUV_REPOSITORY_NAME
                        }
                    }
                }
                euv_card {
                    title: "Build Information"
                    euv_info {
                        label: "Date"
                        EUV_BUILD_DATE
                    }
                    euv_info {
                        label: "Time"
                        EUV_BUILD_CLOCK
                    }
                    euv_info {
                        label: "Timestamp"
                        EUV_BUILD_TIMESTAMP
                    }
                }
            }
            if { !native_bridge_state.get_loading().get() && native_bridge_state.get_available().get() } {
                div {
                    h2 {
                        class: c_home_section_title()
                        "Native Bridge"
                    }
                    euv_card {
                        title: "bridge Integration"
                        euv_info {
                            label: "Permissions"
                            native_bridge_state.get_permissions()
                        }
                    }
                }
            }
        }
    }
}
