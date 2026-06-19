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
/// - `Signal<LogFilter>` - The current filter level signal.
///
/// # Returns
///
/// - `Vec<(usize, ConsoleEntry)>` - The filtered and reversed entries with original indices.
pub(crate) fn filter_console_entries(
    logs: Signal<Vec<ConsoleEntry>>,
    filter: Signal<LogFilter>,
) -> Vec<(usize, ConsoleEntry)> {
    let log_list: Vec<ConsoleEntry> = logs.get();
    let filter_value: LogFilter = filter.get();
    log_list
        .iter()
        .enumerate()
        .filter(|(_, entry): &(usize, &ConsoleEntry)| match filter_value {
            LogFilter::All => true,
            LogFilter::Log => entry.get_level() == LogLevel::Log,
            LogFilter::Warn => entry.get_level() == LogLevel::Warn,
            LogFilter::Error => entry.get_level() == LogLevel::Error,
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
///
/// # Returns
///
/// - `String` - The combined CSS class string.
pub(crate) fn get_log_item_class(level: LogLevel) -> String {
    let base_name: &'static str = c_vconsole_log_item().get_name();
    let level_class: &'static str = match level {
        LogLevel::Log => c_vconsole_log_latest().get_name(),
        LogLevel::Warn => c_vconsole_log_warn().get_name(),
        LogLevel::Error => c_vconsole_log_error().get_name(),
    };
    if level_class.is_empty() {
        base_name.to_string()
    } else {
        format!("{base_name} {level_class}")
    }
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

/// Creates a click event handler that opens the vConsole fab panel.
///
/// Pushes an overlay state and sets the panel visibility signal to true.
///
/// # Arguments
///
/// - `Signal<bool>` - The signal controlling panel visibility.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that opens the panel.
pub(crate) fn vconsole_fab_on_click(panel_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        overlay_push_state();
        panel_open.set(true);
    }))
}

/// Creates a click event handler that sets the log filter to "All".
///
/// # Arguments
///
/// - `Signal<LogFilter>` - The signal controlling the active log filter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets filter to All.
pub(crate) fn vconsole_on_filter_all(
    filter_signal: Signal<LogFilter>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        filter_signal.set(LogFilter::All);
    }))
}

/// Creates a click event handler that sets the log filter to "Log".
///
/// # Arguments
///
/// - `Signal<LogFilter>` - The signal controlling the active log filter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets filter to Log.
pub(crate) fn vconsole_on_filter_log(
    filter_signal: Signal<LogFilter>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        filter_signal.set(LogFilter::Log);
    }))
}

/// Creates a click event handler that sets the log filter to "Warn".
///
/// # Arguments
///
/// - `Signal<LogFilter>` - The signal controlling the active log filter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets filter to Warn.
pub(crate) fn vconsole_on_filter_warn(
    filter_signal: Signal<LogFilter>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        filter_signal.set(LogFilter::Warn);
    }))
}

/// Creates a click event handler that sets the log filter to "Error".
///
/// # Arguments
///
/// - `Signal<LogFilter>` - The signal controlling the active log filter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets filter to Error.
pub(crate) fn vconsole_on_filter_error(
    filter_signal: Signal<LogFilter>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        filter_signal.set(LogFilter::Error);
    }))
}
