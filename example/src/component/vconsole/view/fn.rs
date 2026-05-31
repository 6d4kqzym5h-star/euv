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
    let VconsolePanelProps { panel_open } = node.try_get_props().unwrap_or_default();
    let console_signal: Signal<Vec<ConsoleEntry>> = get_console_signal();
    let log_count: usize = console_signal.get().len();
    html! {
        vconsole_fab {
            panel_open: panel_open
            log_count: log_count
        }
        vconsole_drawer {
            console_signal: console_signal
            panel_open: panel_open
            log_count: log_count
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
        log_count,
    } = node.try_get_props().unwrap_or_default();
    let is_open: bool = panel_open.get();
    if is_open {
        return html! {
            div {
                class: c_vconsole_fab_hidden()
            }
        };
    }
    let fab_on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_event: Event| {
        push_state_on_open();
        panel_open.set(true);
    }));
    if log_count > 0 {
        let badge_display: String = if log_count > 99 {
            "99+".to_string()
        } else {
            log_count.to_string()
        };
        html! {
            logo_button {
                variant: LogoButtonVariant::Fab
                on_click: fab_on_click
                span {
                    class: c_vconsole_badge()
                    badge_display
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
        log_count,
    } = node.try_get_props().unwrap_or_default();
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
    html! {
        div {
            div {
                class: overlay_class
                onclick: move |_event: Event| {
                    back_on_close();
                    panel_open.set(false);
                }
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
                            format!(" ({log_count})")
                        }
                    }
                    div {
                        class: c_vconsole_header_actions()
                        button {
                            class: c_vconsole_clear_button()
                            onclick: move |_event: Event| {
                                Console::clear();
                            }
                            "Clear"
                        }
                        button {
                            class: c_vconsole_close_button()
                            onclick: move |_event: Event| {
                                back_on_close();
                                panel_open.set(false);
                            }
                            "×"
                        }
                    }
                }
                div {
                    class: c_vconsole_filter_bar()
                    button {
                        class: if { filter_signal.get() == LogFilter::All } { c_vconsole_filter_active() } else { c_vconsole_filter_button() }
                        onclick: move |_event: Event| {
                            filter_signal.set(LogFilter::All);
                        }
                        LogFilter::All.to_string()
                    }
                    button {
                        class: if { filter_signal.get() == LogFilter::Log } { c_vconsole_filter_active_log() } else { c_vconsole_filter_button() }
                        onclick: move |_event: Event| {
                            filter_signal.set(LogFilter::Log);
                        }
                        LogFilter::Log.to_string()
                    }
                    button {
                        class: if { filter_signal.get() == LogFilter::Warn } { c_vconsole_filter_active_warn() } else { c_vconsole_filter_button() }
                        onclick: move |_event: Event| {
                            filter_signal.set(LogFilter::Warn);
                        }
                        LogFilter::Warn.to_string()
                    }
                    button {
                        class: if { filter_signal.get() == LogFilter::Error } { c_vconsole_filter_active_error() } else { c_vconsole_filter_button() }
                        onclick: move |_event: Event| {
                            filter_signal.set(LogFilter::Error);
                        }
                        LogFilter::Error.to_string()
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
    let filtered: Vec<(usize, ConsoleEntry)> = filter_console_entries(logs, filter);
    if filtered.is_empty() {
        return html! {
            div {
                class: c_vconsole_empty()
                "No logs yet."
            }
        };
    }
    html! {
        for (index, entry) in {&filtered} {
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
