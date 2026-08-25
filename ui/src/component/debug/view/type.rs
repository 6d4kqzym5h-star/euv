/// Type alias for the formatter that produces the displayable string
/// for an `euv_debug` component.
///
/// The closure runs every time the component re-renders. Inside the
/// closure the caller typically calls `.get()` on one or more
/// `Signal<T>` values, which subscribes the rendered vnode to
/// those signals — so the Debug readout updates live when the
/// underlying state changes, without needing a manual refresh
/// trigger.
///
/// Returning a `String` (rather than an arbitrary `VirtualNode`)
/// keeps the rendering path trivial: the formatter is only
/// responsible for converting reactive state to a printable
/// representation; layout and styling stay inside the component.
///
/// The closure is wrapped in `Option` so the props struct can
/// derive `Default` — see `EuvDebugProps` for the rationale.
pub type DebugValueFormatter = std::rc::Rc<dyn Fn() -> String>;
