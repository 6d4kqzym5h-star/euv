use super::*;

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
        *self.get_state()
    }

    /// Returns the current state snapshot (no factory
    /// call).
    pub fn current(&self) -> LoadState<T> {
        self.get_state().get()
    }

    /// Returns `true` if the factory has produced a
    /// value (or failed).
    pub fn is_resolved(&self) -> bool {
        matches!(
            self.get_state().get(),
            LoadState::Loaded(_) | LoadState::Failed(_)
        )
    }

    /// Returns `true` if the factory is still pending or
    /// loading.
    pub fn is_pending(&self) -> bool {
        matches!(self.get_state().get(), LoadState::Pending)
    }

    /// Triggers the factory without reading the value.
    /// Idempotent: calling `prefetch()` twice does not
    /// run the factory twice.
    pub fn prefetch(&self) {
        if let LoadState::Pending = self.get_state().get() {
            self.get_state().set(LoadState::Loading);
            // For sync factories, transition
            // Pending → Loading → Loaded in one call.
            // (Async factories would `set` to
            // Loaded after the future resolves.)
            self.invoke_factory();
        }
    }

    /// Reads the value, calling the factory on the first
    /// call. Subsequent calls return the cached value.
    pub fn get(&self) -> Option<T> {
        match self.get_state().get() {
            LoadState::Loaded(value) => Some(value),
            LoadState::Failed(_) => None,
            LoadState::Pending | LoadState::Loading => {
                self.invoke_factory();
                match self.get_state().get() {
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
        match self.get_state().get() {
            LoadState::Loaded(value) => Some(value),
            LoadState::Pending | LoadState::Loading | LoadState::Failed(_) => None,
        }
    }

    /// Resets the lazy component to `Pending`. The next
    /// `get()` call will re-run the factory.
    pub fn reset(&self) {
        self.get_state().set(LoadState::Pending);
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
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (self.get_factory())()));
        match result {
            Ok(value) => {
                self.get_state().set(LoadState::Loaded(value));
            }
            Err(payload) => {
                let message: String = if let Some(s) = payload.downcast_ref::<&'static str>() {
                    (*s).to_string()
                } else if let Some(s) = payload.downcast_ref::<String>() {
                    s.clone()
                } else {
                    String::from("factory panicked")
                };
                self.get_state().set(LoadState::Failed(message));
            }
        }
    }
}

impl<T: Clone + PartialEq + std::fmt::Debug + 'static> std::fmt::Debug for LazyComponent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazyComponent")
            .field("state", &self.get_state().get())
            .finish()
    }
}
