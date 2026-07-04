use crate::*;

/// Renders the desktop layout with a persistent left sidebar navigation.
///
/// # Arguments
///
/// - `DesktopLayoutProps` - The typed props containing route, theme, root class, and panel signals.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The desktop application shell virtual DOM tree.
#[component]
pub(crate) fn desktop_layout(node: VirtualNode<DesktopLayoutProps>) -> VirtualNode {
    let DesktopLayoutProps {
        route_signal,
        theme_signal,
        root_class_signal,
        panel_open,
    }: DesktopLayoutProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: root_class_signal
            nav {
                class: c_app_nav()
                a {
                    href: GITHUB_URL
                    target: "_blank"
                    onclick: Router::external_link_handler(GITHUB_URL)
                    class: c_nav_header()
                    euv_logo {
                        variant: LogoButtonVariant::Nav
                    }
                    span {
                        class: c_nav_brand_title()
                        BRAND_NAME
                    }
                }
                p {
                    class: c_nav_section_label()
                    "Pages"
                }
                build_desktop_nav_items {
                    route_signal: route_signal
                }
                div {
                    class: c_nav_theme_toggle()
                    button {
                        class: c_nav_theme_button()
                        onclick: ThemeState::toggle(theme_signal)
                        div {
                            class: if { theme_signal.get() == THEME_DARK } {
                                c_theme_icon_sun()
                            } else {
                                c_theme_icon_moon()
                            }
                        }
                    }
                }
                a {
                    href: GITHUB_URL
                    target: "_blank"
                    onclick: Router::external_link_handler(GITHUB_URL)
                    class: c_nav_footer()
                    div {
                        class: c_nav_footer_divider()
                    }
                    span {
                        class: c_nav_footer_text()
                        "Built with "
                        span {
                            class: c_nav_footer_brand()
                            "Euv & Wasm"
                        }
                    }
                }
            }
            main {
                class: c_app_main()
                page_router {
                    route_signal
                }
            }
            euv_vconsole_panel {
                panel_open
            }
        }
    }
}

/// Renders the mobile layout with a top header bar and a slide-out navigation drawer.
///
/// # Arguments
///
/// - `MobileLayoutProps` - The typed props containing route, theme, root class, panel, and drawer signals.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The mobile application shell virtual DOM tree.
#[component]
pub(crate) fn mobile_layout(node: VirtualNode<MobileLayoutProps>) -> VirtualNode {
    let MobileLayoutProps {
        route_signal,
        theme_signal,
        root_class_signal,
        panel_open,
        drawer_open,
    }: MobileLayoutProps = node.try_get_props().unwrap_or_default();
    let on_overlay_click = move |_: Event| {
        Router::overlay_stack_close();
        drawer_open.set(false);
    };
    let on_drawer_close_click = move |_: Event| {
        Router::overlay_stack_close();
        drawer_open.set(false);
    };
    html! {
        div {
            class: root_class_signal
            header {
                class: c_mobile_header()
                div {
                    class: c_mobile_header_left()
                    button {
                        class: if { drawer_open.get() } {
                            c_mobile_menu_button_active()
                        } else {
                            c_mobile_menu_button()
                        }
                        onclick: UseEuvLayout::use_drawer_toggle(drawer_open)
                        "☰"
                    }
                    a {
                        href: GITHUB_URL
                        target: "_blank"
                        onclick: Router::external_link_handler(GITHUB_URL)
                        class: c_mobile_header_logo()
                        euv_logo {
                            variant: LogoButtonVariant::Nav
                        }
                        span {
                            class: c_nav_brand_title()
                            BRAND_NAME
                        }
                    }
                }
                button {
                    class: c_mobile_theme_button()
                    onclick: ThemeState::toggle(theme_signal)
                    div {
                        class: if { theme_signal.get() == THEME_DARK } {
                            c_theme_icon_sun()
                        } else {
                            c_theme_icon_moon()
                        }
                    }
                }
            }
            main {
                class: c_mobile_main()
                page_router {
                    route_signal
                }
            }
            euv_vconsole_panel {
                panel_open
            }
            div {
                class: if { drawer_open.get() } {
                    c_mobile_overlay().to_string()
                } else {
                    format!("{} {}", c_mobile_overlay().get_name(), c_mobile_overlay_hidden().get_name())
                }
                onclick: on_overlay_click
            }
            nav {
                class: if { drawer_open.get() } {
                    c_mobile_nav_drawer().to_string()
                } else {
                    format!("{} {}", c_mobile_nav_drawer().get_name(), c_mobile_nav_drawer_closed().get_name())
                }
                div {
                    class: c_mobile_nav_drawer_header()
                    div {
                        class: c_mobile_header_logo()
                        a {
                            href: GITHUB_URL
                            target: "_blank"
                            onclick: Router::external_link_handler(GITHUB_URL)
                            class: c_mobile_header_logo()
                            euv_logo {
                                variant: LogoButtonVariant::Nav
                            }
                            span {
                                class: c_nav_brand_title()
                                BRAND_NAME
                            }
                        }
                    }
                    button {
                        class: c_mobile_drawer_close_button()
                        onclick: on_drawer_close_click
                        "✕"
                    }
                }
                p {
                    class: c_nav_section_label()
                    "Pages"
                }
                build_mobile_nav_items {
                    route_signal: route_signal
                    drawer_open: drawer_open
                }
                a {
                    href: GITHUB_URL
                    target: "_blank"
                    onclick: Router::external_link_handler(GITHUB_URL)
                    class: c_nav_footer()
                    div {
                        class: c_nav_footer_divider()
                    }
                    span {
                        class: c_nav_footer_text()
                        "Built with "
                        span {
                            class: c_nav_footer_brand()
                            "Euv & Wasm"
                        }
                    }
                }
            }
        }
    }
}

/// Asynchronously fetches the docs status with retry logic.
///
/// Attempts to fetch the crate status JSON up to `VERSION_FETCH_MAX_RETRY_COUNT` times,
/// waiting `VERSION_FETCH_RETRY_DELAY_MS` milliseconds between each retry on failure.
///
/// # Returns
///
/// - `Result<DocsStatus, ()>` — The parsed docs status on success, or an error indicator on failure.
async fn fetch_docs_status_with_retry() -> Result<DocsStatus, ()> {
    let window_value: Window = window().expect("no global window exists");
    let mut attempt: u32 = 0;
    loop {
        let promise: Promise = window_value.fetch_with_str(DOCS_STATUS_URL);
        match JsFuture::from(promise).await {
            Ok(response) => {
                let response_value: Response = match response.dyn_into() {
                    Ok(value) => value,
                    Err(error) => {
                        Console::error(format!(
                            "Failed to convert fetch response: {}",
                            error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
                        ));
                        return Err(());
                    }
                };
                let text_string: String = match response_value.text() {
                    Ok(promise) => match JsFuture::from(promise).await {
                        Ok(value) => value.as_string().unwrap_or_default(),
                        Err(error) => {
                            Console::error(format!(
                                "Failed to read response text: {}",
                                error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
                            ));
                            return Err(());
                        }
                    },
                    Err(error) => {
                        Console::error(format!(
                            "Failed to get response text promise: {}",
                            error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
                        ));
                        return Err(());
                    }
                };
                Console::log(&text_string);
                return Ok(serde_json::from_str::<DocsStatus>(&text_string).unwrap_or_default());
            }
            Err(error) => {
                attempt += 1;
                if attempt >= VERSION_FETCH_MAX_RETRY_COUNT {
                    Console::error(format!(
                        "Failed to fetch version status after {VERSION_FETCH_MAX_RETRY_COUNT} attempts: {}",
                        error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
                    ));
                    return Err(());
                }
                Console::warn(format!(
                    "Failed to fetch version status (attempt {attempt}/{VERSION_FETCH_MAX_RETRY_COUNT}): {}. Retrying in {VERSION_FETCH_RETRY_DELAY_MS}ms...",
                    error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
                ));
                JsFuture::from(Promise::new(&mut |resolve: Function, _reject: Function| {
                    let window: Window = window().expect("no global window exists");
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            &resolve,
                            VERSION_FETCH_RETRY_DELAY_MS as i32,
                        )
                        .expect("failed to set timeout");
                }))
                .await
                .expect("timeout future failed");
            }
        }
    }
}

/// Asynchronously checks for euv documentation updates from docs.rs.
///
/// Fetches the crate status JSON with retry logic, parses the `DocsStatus` payload,
/// and if a newer version is available, invokes the bridge `update_cache` command
/// to synchronize the local cache.
///
/// # Returns
///
/// - `UpdateResult` — The documentation status, version, and whether an update was triggered.
async fn check_docs_update() -> UpdateResult {
    let parsed: DocsStatus = match fetch_docs_status_with_retry().await {
        Ok(status) => status,
        Err(()) => {
            return UpdateResult {
                doc_status: false,
                version: String::new(),
                updating: false,
            };
        }
    };
    if !matches!(
        CompareVersion::compare_version(parsed.get_version(), EUV_VERSION),
        Ok(VersionLevel::Greater)
    ) {
        Console::log(format!(
            "Current version v{EUV_VERSION} is already the latest version"
        ));
        return UpdateResult {
            doc_status: parsed.get_doc_status(),
            version: parsed.get_version().clone(),
            updating: false,
        };
    }
    if !BridgeConfig::is_available(None) {
        return UpdateResult {
            doc_status: parsed.get_doc_status(),
            version: parsed.get_version().clone(),
            updating: false,
        };
    }
    let mut updating: bool = false;
    if let Ok(promise) = BridgeConfig::invoke(INVOKE_UPDATE_CACHE, None, None) {
        updating = true;
        match JsFuture::from(promise).await {
            Ok(result) => Console::log(result.as_string().unwrap_or_default()),
            Err(error) => Console::error(error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())),
        }
    }
    UpdateResult {
        doc_status: parsed.get_doc_status(),
        version: parsed.get_version().clone(),
        updating,
    }
}

/// Renders the application shell with navigation and route-based page content.
///
/// Detects viewport size and switches between desktop sidebar layout and
/// mobile header + drawer layout accordingly.
///
/// # Returns
///
/// - `VirtualNode` - The root application virtual DOM tree.
pub(crate) fn app() -> VirtualNode {
    Console::init();
    let route_signal: Signal<String> = App::use_signal(Router::current_route);
    let panel_open: Signal<bool> = App::use_signal(|| false);
    let drawer_open: Signal<bool> = App::use_signal(|| false);
    let mobile_signal: Signal<bool> = UseEuvLayout::use_resize();
    let theme_state: ThemeState = ThemeState::use_theme_state(mobile_signal);
    let theme_signal: Signal<String> = theme_state.get_theme();
    let root_class_signal: Signal<String> = theme_state.get_root_class();
    UseCacheUpdate::use_cache_state().load(check_docs_update);
    Router::use_hash_change(route_signal);
    Router::use_scroll_to_top(route_signal);
    Router::use_overlay_history(drawer_open, mobile_signal);
    Router::use_scroll_drawer_to_active(drawer_open);
    UseEuvLayout::use_safe_area_fix();
    html! {
        if { mobile_signal.get() } {
            mobile_layout {
                route_signal
                theme_signal
                root_class_signal
                panel_open
                drawer_open
            }
        } else {
            desktop_layout {
                route_signal
                theme_signal
                root_class_signal
                panel_open
            }
        }
    }
}
