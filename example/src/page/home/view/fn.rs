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
    let is_first_render: Signal<bool> = use_signal(|| true);
    if is_first_render.get() {
        is_first_render.set(false);
        load_native_bridge_data(native_bridge_state);
    }
    let version: String = format!("v{EUV_VERSION}");
    html! {
        div {
            class: c_page_container()
            // ── Hero Section ──
            div {
                class: c_home_hero()
                div {
                    class: c_home_hero_glow()
                }
                div {
                    class: c_home_hero_content()
                    h1 {
                        class: c_home_hero_title()
                        BRAND_NAME
                    }
                    div {
                        class: c_home_hero_badge()
                        version.clone()
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

            // ── Stats Row ──
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

            // ── Feature Cards ──
            div {
                class: c_home_section()
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
                    my_card {
                        title: "Reactive Signals"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_icon()
                                "⚡"
                            }
                            h4 {
                                class: c_feature_name()
                                "Signal-Based Reactivity"
                            }
                            p {
                                class: c_feature_desc()
                                "Fine-grained reactive state management with automatic dependency tracking."
                            }
                        }
                    }
                    my_card {
                        title: "Virtual DOM"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_icon()
                                "🌲"
                            }
                            h4 {
                                class: c_feature_name()
                                "Efficient Diffing"
                            }
                            p {
                                class: c_feature_desc()
                                "Virtual DOM with optimized reconciliation for smooth 60fps updates."
                            }
                        }
                    }
                    my_card {
                        title: "HTML Macros"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_icon()
                                "🏗️"
                            }
                            h4 {
                                class: c_feature_name()
                                "Declarative Syntax"
                            }
                            p {
                                class: c_feature_desc()
                                "Write UI with familiar HTML-like macros that compile to efficient Rust."
                            }
                        }
                    }
                    my_card {
                        title: "Cross-Platform"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_icon()
                                "🌐"
                            }
                            h4 {
                                class: c_feature_name()
                                "WebAssembly Powered"
                            }
                            p {
                                class: c_feature_desc()
                                "Run anywhere with WASM — browsers, servers, and native via Tauri."
                            }
                        }
                    }
                }
            }

            // ── Package Info ──
            div {
                class: c_home_section()
                h2 {
                    class: c_home_section_title()
                    "Package Info"
                }
                my_card {
                    title: "Project Details"
                    div {
                        class: c_info_row()
                        span {
                            class: c_info_label()
                            "Name"
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
                            "Version"
                        }
                        span {
                            class: c_info_value()
                            version
                        }
                    }
                    div {
                        class: c_info_row()
                        span {
                            class: c_info_label()
                            "Edition"
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
                            "License"
                        }
                        span {
                            class: c_info_value()
                            EUV_LICENSE
                        }
                    }
                    div {
                        class: c_info_row()
                        span {
                            class: c_info_label()
                            "Authors"
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
                            "Repository"
                        }
                        a {
                            class: c_info_link()
                            href: EUV_REPOSITORY
                            target: "_blank"
                            onclick: external_link_handler(EUV_REPOSITORY.to_string())
                            EUV_REPOSITORY_NAME
                        }
                    }
                }
                my_card {
                    title: "Build Information"
                    div {
                        class: c_info_row()
                        span {
                            class: c_info_label()
                            "Date"
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
                            "Time"
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
                            "Timestamp"
                        }
                        span {
                            class: c_info_value()
                            EUV_BUILD_TIMESTAMP
                        }
                    }
                }
            }

            // ── Native Bridge (conditional) ──
            if { !native_bridge_state.get_loading().get() && native_bridge_state.get_available().get() } {
                div {
                    class: c_home_section()
                    h2 {
                        class: c_home_section_title()
                        "Native Bridge"
                    }
                    my_card {
                        title: "Tauri Integration"
                        div {
                            class: c_info_row()
                            span {
                                class: c_info_label()
                                "Permissions"
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
}
