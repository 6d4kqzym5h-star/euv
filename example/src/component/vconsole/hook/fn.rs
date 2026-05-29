use crate::*;

/// Initializes the global Console log signal.
///
/// Must be called once during application startup before any `Console::log`,
/// `Console::warn`, or `Console::error` calls.
pub(crate) fn init_console() {
    let signal: Signal<Vec<ConsoleEntry>> = Signal::create(Vec::new());
    CONSOLE_LOG_SIGNAL.set(signal);
}

/// Returns the global vConsole log signal.
///
/// # Returns
///
/// - `Signal<Vec<ConsoleEntry>>` - The global vConsole log signal.
///
/// # Panics
///
/// Panics if `init_console` has not been called.
pub(crate) fn get_console_signal() -> Signal<Vec<ConsoleEntry>> {
    CONSOLE_LOG_SIGNAL.get()
}

/// Filters and reverses console log entries based on the current filter signal value.
///
/// # Arguments
///
/// - `Signal<Vec<ConsoleEntry>>` - The console log signal.
/// - `Signal<String>` - The current filter level signal.
///
/// # Returns
///
/// - `Vec<(usize, ConsoleEntry)>` - The filtered and reversed entries with original indices.
pub(crate) fn filter_console_entries(
    logs: Signal<Vec<ConsoleEntry>>,
    filter: Signal<String>,
) -> Vec<(usize, ConsoleEntry)> {
    let log_list: Vec<ConsoleEntry> = logs.get();
    let filter_value: String = filter.get();
    log_list
        .iter()
        .enumerate()
        .filter(|(_, entry): &(usize, &ConsoleEntry)| {
            if filter_value == "all" {
                return true;
            }
            match filter_value.as_str() {
                "log" => entry.get_level() == LogLevel::Log,
                "warn" => entry.get_level() == LogLevel::Warn,
                "error" => entry.get_level() == LogLevel::Error,
                _ => true,
            }
        })
        .map(|(index, entry): (usize, &ConsoleEntry)| (index, entry.clone()))
        .collect::<Vec<(usize, ConsoleEntry)>>()
        .into_iter()
        .rev()
        .collect()
}

/// Returns the combined CSS class string for a log entry based on its level and recency.
///
/// # Arguments
///
/// - `LogLevel` - The log level of the entry.
/// - `bool` - Whether this is the most recent entry.
///
/// # Returns
///
/// - `String` - The combined CSS class string.
pub(crate) fn get_log_item_class(level: LogLevel, is_latest: bool) -> String {
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
/// - `LogLevel` - The log level of the entry.
///
/// # Returns
///
/// - `String` - The combined CSS class string for the badge.
pub(crate) fn get_badge_class(level: LogLevel) -> String {
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
/// - `LogLevel` - The log level.
///
/// # Returns
///
/// - `String` - The badge label string ("LOG", "WRN", "ERR").
pub(crate) fn get_log_level_badge(level: LogLevel) -> String {
    match level {
        LogLevel::Log => "LOG".to_string(),
        LogLevel::Warn => "WRN".to_string(),
        LogLevel::Error => "ERR".to_string(),
    }
}
