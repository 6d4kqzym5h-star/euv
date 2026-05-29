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
/// - `Signal<bool>` - The reactive signal controlling panel visibility.
///
/// # Returns
///
/// - `VirtualNode` - The vConsole panel virtual DOM tree.
pub(crate) fn vconsole_panel(panel_open: Signal<bool>) -> VirtualNode {
    let console_signal: Signal<Vec<ConsoleEntry>> = get_console_signal();
    let log_count: usize = console_signal.get().len();
    html! {
        vconsole_fab(panel_open, log_count)
        vconsole_drawer(console_signal, panel_open, log_count)
    }
}

/// Renders the floating action button for the vConsole panel.
fn vconsole_fab(panel_open: Signal<bool>, log_count: usize) -> VirtualNode {
    let is_open: bool = panel_open.get();
    let fab_class: String = if is_open {
        format!(
            "{} {}",
            c_vconsole_button().get_name(),
            c_vconsole_fab_hidden().get_name()
        )
    } else {
        c_vconsole_button().get_name().to_string()
    };
    if log_count > 0 {
        let badge_display: String = if log_count > 99 {
            "99+".to_string()
        } else {
            log_count.to_string()
        };
        html! {
            button {
                class: fab_class
                onclick: move |_event: Event| {
                    push_state_on_open();
                    panel_open.set(true);
                }
                "E"
                span {
                    class: c_vconsole_badge()
                    badge_display
                }
            }
        }
    } else {
        html! {
            button {
                class: fab_class
                onclick: move |_event: Event| {
                    push_state_on_open();
                    panel_open.set(true);
                }
                "E"
            }
        }
    }
}

/// Renders the vConsole drawer panel with log entries, level filter, and controls.
fn vconsole_drawer(
    console_signal: Signal<Vec<ConsoleEntry>>,
    panel_open: Signal<bool>,
    log_count: usize,
) -> VirtualNode {
    let filter_signal: Signal<String> = use_signal(|| "all".to_string());
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
                            "\u{00d7}"
                        }
                    }
                }
                div {
                    class: c_vconsole_filter_bar()
                    button {
                        class: if { filter_signal.get() == "all" } { c_vconsole_filter_active() } else { c_vconsole_filter_button() }
                        onclick: move |_event: Event| {
                            filter_signal.set("all".to_string());
                        }
                        "All"
                    }
                    button {
                        class: if { filter_signal.get() == "log" } { c_vconsole_filter_active_log() } else { c_vconsole_filter_button() }
                        onclick: move |_event: Event| {
                            filter_signal.set("log".to_string());
                        }
                        "Log"
                    }
                    button {
                        class: if { filter_signal.get() == "warn" } { c_vconsole_filter_active_warn() } else { c_vconsole_filter_button() }
                        onclick: move |_event: Event| {
                            filter_signal.set("warn".to_string());
                        }
                        "Warn"
                    }
                    button {
                        class: if { filter_signal.get() == "error" } { c_vconsole_filter_active_error() } else { c_vconsole_filter_button() }
                        onclick: move |_event: Event| {
                            filter_signal.set("error".to_string());
                        }
                        "Error"
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
    filter: Signal<String>,
) -> VirtualNode {
    if filter_console_entries(logs, filter).is_empty() {
        return html! {
            div {
                class: c_vconsole_empty()
                "No logs yet."
            }
        };
    }
    html! {
        for (index, entry) in { filter_console_entries(logs, filter) } {
            div {
                key: index.to_string()
                class: get_log_item_class(entry.get_level(), index == logs.get().len() - 1)
                span {
                    class: get_badge_class(entry.get_level())
                    get_log_level_badge(entry.get_level())
                }
                entry.get_message().clone()
            }
        }
    }
}
