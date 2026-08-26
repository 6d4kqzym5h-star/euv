use super::*;

/// A reactive boolean toggle.
///
/// Constructed via `Toggle::new(initial)`. The current
/// value is exposed as a `Signal<bool>` accessed through
/// `get_value()`. Mutators (`set_true`, `set_false`,
/// `toggle`, `set`) write through that signal, so any
/// `html!` body that reads `toggle.get_value().get()`
/// inside its render closure re-renders when the value
/// changes.
#[derive(Clone, Data, Debug)]
pub struct Toggle {
    /// The current value signal.
    pub(crate) value: Signal<bool>,
}
