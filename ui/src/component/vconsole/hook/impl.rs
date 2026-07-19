use crate::*;

/// Implements the Console struct providing web console API methods.
///
/// Each method outputs to both the browser developer console and the
/// vConsole panel signal, with appropriate log level classification.
/// Methods are associated functions that internally access the global
/// Console instance, so callers never need to hold a reference.
impl Console {
    /// Initializes the global Console log signal.
    ///
    /// Must be called once during application startup before any `Console::log`,
    /// `Console::warn`, or `Console::error` calls.
    pub fn init() {
        let signal: Signal<Vec<ConsoleEntry>> = Signal::create(Vec::new());
        CONSOLE_LOG_SIGNAL.set(signal);
    }

    /// Logs an informational message (equivalent to console.log).
    ///
    /// # Arguments
    ///
    /// - `M: AsRef<str>` - The message to log.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    pub fn log<M>(message: M)
    where
        M: AsRef<str>,
    {
        let message_ref: &str = message.as_ref();
        console::log_1(&message_ref.into());
        Self::append_entry(ConsoleEntry::new(LogLevel::Log, message_ref.to_string()));
    }

    /// Logs a warning message (equivalent to console.warn).
    ///
    /// # Arguments
    ///
    /// - `M: AsRef<str>` - The warning message to log.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    pub fn warn<M>(message: M)
    where
        M: AsRef<str>,
    {
        let message_ref: &str = message.as_ref();
        console::warn_1(&message_ref.into());
        Self::append_entry(ConsoleEntry::new(LogLevel::Warn, message_ref.to_string()));
    }

    /// Logs an error message (equivalent to console.error).
    ///
    /// # Arguments
    ///
    /// - `M: AsRef<str>` - The error message to log.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    pub fn error<M>(message: M)
    where
        M: AsRef<str>,
    {
        let message_ref: &str = message.as_ref();
        console::error_1(&message_ref.into());
        Self::append_entry(ConsoleEntry::new(LogLevel::Error, message_ref.to_string()));
    }

    /// Clears all log entries from the vConsole panel signal.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    pub fn clear() {
        let log: Signal<Vec<ConsoleEntry>> = Self::get_signal();
        log.set(Vec::new());
    }

    /// Returns the global vConsole log signal.
    ///
    /// # Returns
    ///
    /// - `Signal<Vec<ConsoleEntry>>` - The console log signal.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    pub(crate) fn get_signal() -> Signal<Vec<ConsoleEntry>> {
        CONSOLE_LOG_SIGNAL.get()
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
    pub(crate) fn fab_on_click(panel_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let closer: Rc<dyn Fn()> = Rc::new(move || {
                panel_open.set(false);
            });
            Router::overlay_stack_push(closer);
            panel_open.set(true);
        }))
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
    pub(crate) fn filter_entries(
        logs: Signal<Vec<ConsoleEntry>>,
        filter: Signal<LogFilter>,
    ) -> Vec<(usize, ConsoleEntry)> {
        let log_list: Vec<ConsoleEntry> = logs.get();
        let filter_value: LogFilter = filter.get();
        let mut result: Vec<(usize, ConsoleEntry)> = log_list
            .iter()
            .enumerate()
            .filter(|(_, entry): &(usize, &ConsoleEntry)| match filter_value {
                LogFilter::All => true,
                LogFilter::Log => entry.get_level() == LogLevel::Log,
                LogFilter::Warn => entry.get_level() == LogLevel::Warn,
                LogFilter::Error => entry.get_level() == LogLevel::Error,
            })
            .map(|(index, entry): (usize, &ConsoleEntry)| (index, entry.clone()))
            .collect();
        result.reverse();
        result
    }

    /// Appends an entry to the vConsole log signal, trimming if over capacity.
    ///
    /// # Arguments
    ///
    /// - `ConsoleEntry` - The console entry to append.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    fn append_entry(entry: ConsoleEntry) {
        let log: Signal<Vec<ConsoleEntry>> = Self::get_signal();
        let mut current: Vec<ConsoleEntry> = log.get();
        current.push(entry);
        if current.len() > MAX_CONSOLE_LOG_ENTRIES {
            let excess: usize = current.len() - MAX_CONSOLE_LOG_ENTRIES;
            current.drain(0..excess);
        }
        log.set(current);
    }
}

/// Implements the Display trait for LogFilter to render filter button labels.
impl std::fmt::Display for LogFilter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label: &str = match self {
            LogFilter::All => "All",
            LogFilter::Log => "Log",
            LogFilter::Warn => "Warn",
            LogFilter::Error => "Error",
        };
        write!(formatter, "{}", label)
    }
}

/// Implementation of log level badge rendering.
impl LogLevel {
    /// Returns the short badge label for a log level.
    ///
    /// # Returns
    ///
    /// - `&str` - The badge label string ("LOG", "WRN", "ERR").
    pub(crate) fn badge(self) -> &'static str {
        match self {
            LogLevel::Log => "LOG",
            LogLevel::Warn => "WRN",
            LogLevel::Error => "ERR",
        }
    }
}

/// Implementation of log filter event handlers.
impl LogFilter {
    /// Creates a click event handler that sets the log filter to "All".
    ///
    /// # Arguments
    ///
    /// - `Signal<LogFilter>` - The signal controlling the active log filter.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets filter to All.
    pub(crate) fn on_filter_all(filter_signal: Signal<LogFilter>) -> Option<Rc<dyn Fn(Event)>> {
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
    pub(crate) fn on_filter_log(filter_signal: Signal<LogFilter>) -> Option<Rc<dyn Fn(Event)>> {
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
    pub(crate) fn on_filter_warn(filter_signal: Signal<LogFilter>) -> Option<Rc<dyn Fn(Event)>> {
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
    pub(crate) fn on_filter_error(filter_signal: Signal<LogFilter>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            filter_signal.set(LogFilter::Error);
        }))
    }
}
