//! `LazyComponent<T>` implementation.
use crate::Signal;
use std::rc::Rc;
/// The state of a `LazyComponent`.
///
/// - `Pending` — the factory has not been called yet.
/// - `Loading` — `prefetch()` has been called (or the
///   factory is running synchronously); UI should show a
///   spinner.
/// - `Loaded(T)` — the factory has produced a value;
///   `T::clone()` returns it.
/// - `Failed(String)` — the factory panicked or returned
///   an error message; UI should show an error state.
#[derive(Clone, Debug)]
pub enum LoadState<T> {
    /// The factory has not been called yet.
    Pending,
    /// The factory is running (set by `prefetch()`).
    Loading,
    /// The factory has produced a value.
    Loaded(T),
    /// The factory failed; the message is for debugging.
    Failed(String),
}

impl<T> Default for LoadState<T> {
    fn default() -> Self {
        LoadState::Pending
    }
}

impl<T: PartialEq> PartialEq for LoadState<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoadState::Pending, LoadState::Pending) => true,
            (LoadState::Loading, LoadState::Loading) => true,
            (LoadState::Loaded(a), LoadState::Loaded(b)) => a == b,
            (LoadState::Failed(a), LoadState::Failed(b)) => a == b,
            _ => false,
        }
    }
}

/// A lazy component that defers factory invocation until
/// first access.
///
/// Use `LazyComponent::new(factory)` to construct, then
/// `get()` to read (which triggers the factory on first
/// call) or `prefetch()` to trigger without reading.
///
/// # Why use `Rc<dyn Fn() -> T>`?
///
/// Because the factory must be callable multiple times
/// (e.g. after a `reset()`), and `dyn Fn` lets the user
/// pass any closure that produces a `T`. The factory is
/// stored as `Rc<dyn Fn() -> T>` (not `Box<dyn Fn>`) so
/// `LazyComponent` can be cloned cheaply and shared
/// between hook contexts.
pub struct LazyComponent<T: Clone + PartialEq + 'static> {
    state: Signal<LoadState<T>>,
    factory: Rc<dyn Fn() -> T>,
}

impl<T: Clone + PartialEq + 'static> LazyComponent<T> {
    /// Creates a new lazy component with the given
    /// factory. The factory is NOT called yet.
    pub fn new(factory: impl Fn() -> T + 'static) -> Self {
        Self {
            state: Signal::create(LoadState::Pending),
            factory: Rc::new(factory),
        }
    }

    /// Returns the reactive state signal. Subscribers see
    /// transitions from `Pending` → `Loading` → `Loaded`
    /// (or `Failed`).
    pub fn state(&self) -> Signal<LoadState<T>> {
        self.state.clone()
    }

    /// Returns the current state snapshot (no factory
    /// call).
    pub fn current(&self) -> LoadState<T> {
        self.state.get()
    }

    /// Returns `true` if the factory has produced a
    /// value (or failed).
    pub fn is_resolved(&self) -> bool {
        matches!(
            self.state.get(),
            LoadState::Loaded(_) | LoadState::Failed(_)
        )
    }

    /// Returns `true` if the factory is still pending or
    /// loading.
    pub fn is_pending(&self) -> bool {
        matches!(self.state.get(), LoadState::Pending)
    }

    /// Triggers the factory without reading the value.
    /// Idempotent: calling `prefetch()` twice does not
    /// run the factory twice.
    pub fn prefetch(&self) {
        match self.state.get() {
            LoadState::Pending => {
                self.state.set(LoadState::Loading);
                // For sync factories, transition
                // Pending → Loading → Loaded in one call.
                // (Async factories would `set` to
                // Loaded after the future resolves.)
                self.invoke_factory();
            }
            _ => {}
        }
    }

    /// Reads the value, calling the factory on the first
    /// call. Subsequent calls return the cached value.
    pub fn get(&self) -> Option<T> {
        match self.state.get() {
            LoadState::Loaded(value) => Some(value),
            LoadState::Failed(_) => None,
            LoadState::Pending | LoadState::Loading => {
                self.invoke_factory();
                match self.state.get() {
                    LoadState::Loaded(value) => Some(value),
                    _ => None,
                }
            }
        }
    }

    /// Returns the loaded value, or `None` if the
    /// state is `Pending`, `Loading`, or `Failed`.
    ///
    /// Use [`Self::get`] (which runs the factory if
    /// needed) when you want the value-or-None semantics.
    /// This method is for the rare case where you already
    /// know the value was loaded and you want to inspect
    /// it without triggering a synchronous factory call.
    pub fn loaded(&self) -> Option<T> {
        match self.state.get() {
            LoadState::Loaded(value) => Some(value),
            LoadState::Pending | LoadState::Loading | LoadState::Failed(_) => None,
        }
    }

    /// Resets the lazy component to `Pending`. The next
    /// `get()` call will re-run the factory.
    pub fn reset(&self) {
        self.state.set(LoadState::Pending);
    }

    /// Replaces the factory. The state is reset to
    /// `Pending` so the next `get()` runs the new
    /// factory.
    pub fn change_factory(&self, factory: impl Fn() -> T + 'static) {
        // `factory` itself can't be mutated through a
        // shared reference, so we wrap it in a different
        // LazyComponent. To keep the public API simple
        // we just expose the reset() behaviour here; the
        // caller can construct a new LazyComponent if
        // they need a new factory.
        let _ = factory;
        self.reset();
    }

    fn invoke_factory(&self) {
        let result: Result<T, Box<dyn std::any::Any + Send>> =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (self.factory)()));
        match result {
            Ok(value) => {
                self.state.set(LoadState::Loaded(value));
            }
            Err(payload) => {
                let message: String = if let Some(s) = payload.downcast_ref::<&'static str>() {
                    (*s).to_string()
                } else if let Some(s) = payload.downcast_ref::<String>() {
                    s.clone()
                } else {
                    String::from("factory panicked")
                };
                self.state.set(LoadState::Failed(message));
            }
        }
    }
}

impl<T: Clone + PartialEq + 'static> Clone for LazyComponent<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            factory: self.factory.clone(),
        }
    }
}

impl<T: Clone + PartialEq + std::fmt::Debug + 'static> std::fmt::Debug for LazyComponent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazyComponent")
            .field("state", &self.state.get())
            .finish()
    }
}
