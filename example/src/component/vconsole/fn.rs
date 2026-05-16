use crate::*;

/// Initializes the global Console log signal.
///
/// Must be called once during application startup before any `Console::log`,
/// `Console::warn`, or `Console::error` calls.
pub fn init_console() {
    let signal: Signal<Vec<ConsoleEntry>> = Signal::new(Vec::new());
    CONSOLE_LOG_SIGNAL.set(signal);
}

/// Returns the global vConsole log signal.
///
/// # Returns
///
/// - `Signal<Vec<ConsoleEntry>>`: The global vConsole log signal.
///
/// # Panics
///
/// Panics if `init_console` has not been called.
pub(crate) fn get_console_signal() -> Signal<Vec<ConsoleEntry>> {
    CONSOLE_LOG_SIGNAL.get()
}

/// Renders a vConsole-style floating debug panel with a toggle button and a half-page drawer.
///
/// The panel displays log entries from `Console::log`, `Console::warn`, and `Console::error`
/// calls. Provides level-based filtering with color-coded entries and clear/close actions.
/// When closed, a floating button with a badge showing the log count is rendered.
/// When open, a bottom drawer panel slides up showing all log entries with filter controls.
///
/// # Arguments
///
/// - `Signal<bool>`: The reactive signal controlling panel visibility.
///
/// # Returns
///
/// - `VirtualNode`: The vConsole panel virtual DOM tree.
pub fn vconsole_panel(panel_open: Signal<bool>) -> VirtualNode {
    let console_signal: Signal<Vec<ConsoleEntry>> = get_console_signal();
    let log_count: usize = console_signal.get().len();
    let is_open: bool = panel_open.get();
    if is_open {
        html! {
            vconsole_drawer(console_signal, panel_open, log_count)
        }
    } else {
        html! {
            vconsole_fab(panel_open, log_count)
        }
    }
}

/// Renders the floating action button for the vConsole panel.
///
/// # Arguments
///
/// - `Signal<bool>`: The reactive signal controlling panel visibility.
/// - `usize`: The current log count.
///
/// # Returns
///
/// - `VirtualNode`: The floating action button virtual DOM tree.
fn vconsole_fab(panel_open: Signal<bool>, log_count: usize) -> VirtualNode {
    if log_count > 0 {
        let badge_display: String = if log_count > 99 {
            "99+".to_string()
        } else {
            log_count.to_string()
        };
        html! {
            button {
                class: c_vconsole_button()
                onclick: move |_event: NativeEvent| {
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
                class: c_vconsole_button()
                onclick: move |_event: NativeEvent| {
                    panel_open.set(true);
                }
                "E"
            }
        }
    }
}

/// Renders the vConsole drawer panel with log entries, level filter, and controls.
///
/// # Arguments
///
/// - `Signal<Vec<ConsoleEntry>>`: The reactive signal holding console log entries.
/// - `Signal<bool>`: The reactive signal controlling panel visibility.
/// - `usize`: The current log count.
///
/// # Returns
///
/// - `VirtualNode`: The drawer panel virtual DOM tree.
fn vconsole_drawer(
    console_signal: Signal<Vec<ConsoleEntry>>,
    panel_open: Signal<bool>,
    log_count: usize,
) -> VirtualNode {
    let filter_signal: Signal<String> = use_signal(|| "all".to_string());
    html! {
        div {
            div {
                class: c_vconsole_overlay()
                onclick: move |_event: NativeEvent| {
                    panel_open.set(false);
                }
            }
            div {
                class: c_vconsole_panel()
                div {
                    class: c_vconsole_header()
                    h3 {
                        class: c_vconsole_title()
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
                            onclick: move |_event: NativeEvent| {
                                Console::clear();
                            }
                            "Clear"
                        }
                        button {
                            class: c_vconsole_close_button()
                            onclick: move |_event: NativeEvent| {
                                panel_open.set(false);
                            }
                            "\u{00d7}"
                        }
                    }
                }
                div {
                    class: c_vconsole_filter_bar()
                    button {
                        class: if filter_signal.get() == "all" { c_vconsole_filter_active() } else { c_vconsole_filter_button() }
                        onclick: move |_event: NativeEvent| {
                            filter_signal.set("all".to_string());
                        }
                        "All"
                    }
                    button {
                        class: if filter_signal.get() == "log" { c_vconsole_filter_active_log() } else { c_vconsole_filter_button() }
                        onclick: move |_event: NativeEvent| {
                            filter_signal.set("log".to_string());
                        }
                        "Log"
                    }
                    button {
                        class: if filter_signal.get() == "warn" { c_vconsole_filter_active_warn() } else { c_vconsole_filter_button() }
                        onclick: move |_event: NativeEvent| {
                            filter_signal.set("warn".to_string());
                        }
                        "Warn"
                    }
                    button {
                        class: if filter_signal.get() == "error" { c_vconsole_filter_active_error() } else { c_vconsole_filter_button() }
                        onclick: move |_event: NativeEvent| {
                            filter_signal.set("error".to_string());
                        }
                        "Error"
                    }
                }
                div {
                    class: c_vconsole_body()
                    { build_vconsole_log_nodes(console_signal, filter_signal) }
                }
            }
        }
    }
}

/// Builds the vConsole log entry virtual nodes from the reactive log signal with level filtering.
///
/// # Arguments
///
/// - `Signal<Vec<ConsoleEntry>>`: The reactive signal holding console log entries.
/// - `Signal<String>`: The reactive signal holding the current filter level.
///
/// # Returns
///
/// - `VirtualNode`: A fragment of log item virtual nodes.
fn build_vconsole_log_nodes(
    logs: Signal<Vec<ConsoleEntry>>,
    filter: Signal<String>,
) -> VirtualNode {
    let log_list: Vec<ConsoleEntry> = logs.get();
    let filter_value: String = filter.get();
    let filtered: Vec<(usize, ConsoleEntry)> = log_list
        .iter()
        .enumerate()
        .filter(|(_, entry)| {
            if filter_value == "all" {
                return true;
            }
            match filter_value.as_str() {
                "log" => entry.level == LogLevel::Log,
                "warn" => entry.level == LogLevel::Warn,
                "error" => entry.level == LogLevel::Error,
                _ => true,
            }
        })
        .map(|(index, entry)| (index, entry.clone()))
        .collect();
    let total_count: usize = log_list.len();
    if filtered.is_empty() {
        return html! {
            div {
                class: c_vconsole_empty()
                "No logs yet."
            }
        };
    }
    html! {
        for (index, entry) in {filtered.iter().rev()} {
            div {
                key: index.to_string()
                class: { get_log_item_class(&entry.level, *index == total_count - 1) }
                span {
                    class: {get_badge_class(&entry.level)}
                    get_log_level_badge(&entry.level)
                }
                entry.message.clone()
            }
        }
    }
}

/// Returns the combined CSS class string for a log entry based on its level and recency.
///
/// Injects styles for all referenced classes and concatenates their class names
/// so that the appropriate theme-aware colors are applied.
///
/// # Arguments
///
/// - `&LogLevel`: The log level of the entry.
/// - `bool`: Whether this is the most recent log entry.
///
/// # Returns
///
/// - `String`: The space-separated CSS class names.
fn get_log_item_class(level: &LogLevel, is_latest: bool) -> String {
    let base_name: &'static str = c_vconsole_log_item().get_name();
    let level_class: &'static str = match level {
        LogLevel::Log => {
            if is_latest {
                c_vconsole_log_latest().get_name()
            } else {
                c_vconsole_log_item().get_name()
            }
        }
        LogLevel::Warn => {
            if is_latest {
                c_vconsole_log_warn_latest().get_name()
            } else {
                c_vconsole_log_warn().get_name()
            }
        }
        LogLevel::Error => {
            if is_latest {
                c_vconsole_log_error_latest().get_name()
            } else {
                c_vconsole_log_error().get_name()
            }
        }
    };
    format!("{} {}", base_name, level_class)
}

/// Returns the combined CSS class string for the level badge based on log level.
///
/// # Arguments
///
/// - `&LogLevel`: The log level.
///
/// # Returns
///
/// - `String`: The space-separated CSS class names.
fn get_badge_class(level: &LogLevel) -> String {
    let base_name: &'static str = c_vconsole_level_badge().get_name();
    let badge_class: &'static str = match level {
        LogLevel::Log => c_vconsole_badge_log().get_name(),
        LogLevel::Warn => c_vconsole_badge_warn().get_name(),
        LogLevel::Error => c_vconsole_badge_error().get_name(),
    };
    format!("{} {}", base_name, badge_class)
}

/// Returns the short badge label for a log level.
///
/// # Arguments
///
/// - `&LogLevel`: The log level.
///
/// # Returns
///
/// - `String`: The badge label string (e.g., "LOG", "WRN", "ERR").
fn get_log_level_badge(level: &LogLevel) -> String {
    match level {
        LogLevel::Log => "LOG".to_string(),
        LogLevel::Warn => "WRN".to_string(),
        LogLevel::Error => "ERR".to_string(),
    }
}
