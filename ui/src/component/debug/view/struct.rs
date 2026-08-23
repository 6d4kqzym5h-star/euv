use super::*;

/// Props for the `euv_debug` component.
///
/// Defines the strongly-typed interface for the dev-only Debug
/// readout. The component renders a labeled `<pre data-euv-debug>`
/// block whose body is the result of invoking `value` on every
/// render, so users can inspect reactive state without breaking
/// out of the component tree.
///
/// `value` is wrapped in `Option<DebugValueFormatter>` (rather
/// than `DebugValueFormatter` directly) so the struct can
/// derive `Default` — the upstream `#[derive(New)]` macro
/// expects every props struct to also be `Default` (it generates
/// `X::new(...)` AND `X::default()` from the same field list).
/// A bare `Rc<dyn Fn() -> String>` has no `Default` impl (you
/// cannot synthesise an arbitrary formatter), so wrapping in
/// `Option` keeps the derive happy and gives us a sensible
/// "no formatter configured" fallback.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvDebugProps {
    /// Short label printed before the value (e.g. "count", "user").
    /// Rendered as `<span data-euv-debug-label>{label}</span>` so
    /// styling hooks (`[data-euv-debug-label]`) can target it
    /// independently from the value.
    #[get(type(copy))]
    pub label: &'static str,
    /// Optional closure that produces the displayable string.
    /// Runs on every render — typically the body reads one or
    /// more `Signal<T>::get()` values to subscribe the readout
    /// to live state. `None` renders an empty placeholder so the
    /// caller can still construct the component without a
    /// formatter (handy in tests and stub components).
    #[debug(skip)]
    pub value: Option<DebugValueFormatter>,
    /// When `true`, the value is rendered inside `<pre>` with
    /// `white-space: pre-wrap` so multi-line strings (JSON,
    /// stack traces) preserve formatting. When `false`, the
    /// value is rendered inside a single-line `<code>`.
    #[get(type(copy))]
    pub expanded: bool,
}
