use crate::*;

/// Renders a vConsole-style floating debug panel with a toggle button and a half-page drawer.
///
/// The panel displays log entries from `Console::log`, `Console::warn`, and `Console::error`
/// calls. Provides level-based filtering with color-coded entries and clear/close actions.
/// When closed, a floating button with a badge showing the log count is rendered.
/// When open, a bottom drawer panel slides up showing all log entries with filter controls.
///
/// # Arguments
///
/// - `VconsolePanelProps` - The typed props containing the panel visibility signal.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The vConsole panel virtual DOM tree.
#[component]
pub(crate) fn vconsole_panel(node: VirtualNode<VconsolePanelProps>) -> VirtualNode {
    let VconsolePanelProps { panel_open }: VconsolePanelProps =
        node.try_get_props().unwrap_or_default();
    let console_signal: Signal<Vec<ConsoleEntry>> = get_console_signal();
    html! {
        vconsole_fab {
            panel_open: panel_open
            console_signal: console_signal
        }
        vconsole_drawer {
            console_signal: console_signal
            panel_open: panel_open
        }
    }
}

/// Renders the floating action button for the vConsole panel.
///
/// Uses the shared `logo_button` component with the `Fab` variant
/// to display the branded "E" button with gradient background.
///
/// # Arguments
///
/// - `VconsoleFabProps` - The typed props containing panel visibility signal and log count.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The floating action button virtual DOM tree.
#[component]
pub(crate) fn vconsole_fab(node: VirtualNode<VconsoleFabProps>) -> VirtualNode {
    let VconsoleFabProps {
        panel_open,
        console_signal,
    }: VconsoleFabProps = node.try_get_props().unwrap_or_default();
    let is_open: bool = panel_open.get();
    if is_open {
        return html! {
            div {
                class: c_vconsole_fab_hidden()
            }
        };
    }
    let fab_on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        overlay_push_state();
        panel_open.set(true);
    }));
    if !console_signal.get().is_empty() {
        html! {
            logo_button {
                variant: LogoButtonVariant::Fab
                on_click: fab_on_click
                span {
                    class: c_vconsole_badge()
                    if { console_signal.get().len() > 99 } { "99+" } else { console_signal.get().len().to_string() }
                }
            }
        }
    } else {
        html! {
            logo_button {
                variant: LogoButtonVariant::Fab
                on_click: fab_on_click
            }
        }
    }
}

/// Renders the vConsole drawer panel with log entries, level filter, and controls.
///
/// # Arguments
///
/// - `VconsoleDrawerProps` - The typed props containing console signal, panel visibility signal, and log count.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The drawer panel virtual DOM tree.
#[component]
pub(crate) fn vconsole_drawer(node: VirtualNode<VconsoleDrawerProps>) -> VirtualNode {
    let VconsoleDrawerProps {
        console_signal,
        panel_open,
    }: VconsoleDrawerProps = node.try_get_props().unwrap_or_default();
    let filter_signal: Signal<LogFilter> = use_signal(|| LogFilter::All);
    let is_open: bool = panel_open.get();
    let overlay_class: String = if is_open {
        c_vconsole_overlay().get_name().to_string()
    } else {
        format!(
            "{} {}",
            c_vconsole_overlay().get_name(),
            c_vconsole_overlay_hidden().get_name()
        )
    };
    let panel_class: String = if is_open {
        c_vconsole_panel().get_name().to_string()
    } else {
        format!(
            "{} {}",
            c_vconsole_panel().get_name(),
            c_vconsole_panel_closed().get_name()
        )
    };
    let on_overlay_click = move |_: Event| {
        overlay_back(None);
        panel_open.set(false);
    };
    let on_clear_click = move |_: Event| {
        Console::clear();
    };
    let on_close_click = move |_: Event| {
        overlay_back(None);
        panel_open.set(false);
    };
    let on_filter_all_click = move |_: Event| {
        filter_signal.set(LogFilter::All);
    };
    let on_filter_log_click = move |_: Event| {
        filter_signal.set(LogFilter::Log);
    };
    let on_filter_warn_click = move |_: Event| {
        filter_signal.set(LogFilter::Warn);
    };
    let on_filter_error_click = move |_: Event| {
        filter_signal.set(LogFilter::Error);
    };
    html! {
        div {
            div {
                class: overlay_class
                onclick: on_overlay_click
            }
            div {
                class: panel_class
                div {
                    class: c_vconsole_header()
                    h3 {
                        class: c_vconsole_title()
                        span {
                            class: c_vconsole_title_dot()
                        }
                        "Console"
                        span {
                            class: c_vconsole_count()
                            format!(" ({})", console_signal.get().len())
                        }
                    }
                    div {
                        class: c_vconsole_header_actions()
                        button {
                            class: c_vconsole_clear_button()
                            onclick: on_clear_click
                            "Clear"
                        }
                        button {
                            class: c_vconsole_close_button()
                            onclick: on_close_click
                            "×"
                        }
                    }
                }
                div {
                    class: c_vconsole_filter_bar()
                    my_badge {
                        color: var!(accent)
                        text: "All"
                        outline: { filter_signal.get() != LogFilter::All }
                        on_click: Some(Rc::new(on_filter_all_click))
                    }
                    my_badge {
                        color: var!(badge-bg-success)
                        text: "Log"
                        outline: { filter_signal.get() != LogFilter::Log }
                        on_click: Some(Rc::new(on_filter_log_click))
                    }
                    my_badge {
                        color: var!(badge-bg-warning)
                        text: "Warn"
                        outline: { filter_signal.get() != LogFilter::Warn }
                        on_click: Some(Rc::new(on_filter_warn_click))
                    }
                    my_badge {
                        color: var!(badge-bg-error)
                        text: "Error"
                        outline: { filter_signal.get() != LogFilter::Error }
                        on_click: Some(Rc::new(on_filter_error_click))
                    }
                }
                div {
                    class: c_vconsole_body()
                    build_vconsole_log_nodes(console_signal, filter_signal)
                }
            }
        }
    }
}

/// Builds the vConsole log entry virtual nodes from the reactive log signal with level filtering.
fn build_vconsole_log_nodes(
    logs: Signal<Vec<ConsoleEntry>>,
    filter: Signal<LogFilter>,
) -> VirtualNode {
    html! {
        if { filter_console_entries(logs, filter).is_empty() } {
            div {
                class: c_vconsole_empty()
                "No logs yet."
            }
        } else {
            for (index, entry) in { &filter_console_entries(logs, filter) } {
                div {
                    key: index.to_string()
                    class: get_log_item_class(entry.get_level())
                    span {
                        class: get_badge_class(entry.get_level())
                        get_log_level_badge(entry.get_level())
                    }
                    entry.get_message().clone()
                }
            }
        }
    }
}
