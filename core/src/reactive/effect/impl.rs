use crate::*;

/// Clones the RenderEffect, sharing the same inner state.
impl Clone for RenderEffect {
    fn clone(&self) -> Self {
        *self
    }
}

/// Copies the RenderEffect, sharing the same inner state.
impl Copy for RenderEffect {}

/// Implementation of RenderEffect reactive operations.
impl RenderEffect {
    /// Creates a new RenderEffect that wraps the given closure.
    ///
    /// On creation, the closure is executed once immediately. During execution,
    /// any `Signal::get()` calls will register this effect as a subscriber.
    /// When any of those signals change, the closure is re-executed.
    ///
    /// # Arguments
    ///
    /// - `F` - The closure to execute reactively.
    ///
    /// # Returns
    ///
    /// - `Self` - A new RenderEffect handle.
    pub fn new<F>(effect_fn: F) -> Self
    where
        F: FnMut() + 'static,
    {
        let inner: Box<RenderEffectInner> = Box::new(RenderEffectInner {
            effect_fn: Box::new(effect_fn),
            dependencies: Vec::new(),
            running: false,
            disposed: false,
        });
        let leaked: &mut RenderEffectInner = Box::leak(inner);
        let effect: RenderEffect = RenderEffect {
            inner: leaked as *mut RenderEffectInner as usize,
        };
        effect.run_once();
        effect
    }

    /// Executes the effect closure once with dependency tracking.
    ///
    /// Before executing, removes this effect's subscriber entries from
    /// all previously tracked signals (old dependencies) via the global
    /// effect subscriber registry. Then sets this effect as the current
    /// tracking target and executes the closure. During execution,
    /// `Signal::get()` calls register this effect as a subscriber on
    /// those signals via `track_signal`. After execution, the previous
    /// tracking target is restored.
    pub(crate) fn run_once(&self) {
        let effect_addr: usize = self.inner;
        let inner: &mut RenderEffectInner = self.leak_mut();
        if inner.get_disposed() {
            return;
        }
        if inner.get_running() {
            return;
        }
        inner.set_running(true);
        cleanup_effect_dependencies(effect_addr, inner.get_mut_dependencies());
        let prev: Option<usize> = swap_current_effect(Some(effect_addr));
        let inner: &mut RenderEffectInner = self.leak_mut();
        (inner.get_mut_effect_fn())();
        set_current_effect(prev);
        let inner: &mut RenderEffectInner = self.leak_mut();
        inner.set_running(false);
    }

    /// Marks this effect as disposed and cleans up all its dependencies.
    ///
    /// After disposal, `run_once` becomes a complete no-op. All subscriber
    /// entries for this effect are removed from the global `EFFECT_SUBSCRIBERS`
    /// registry, preventing the effect from being scheduled or executed again.
    /// This is called when the associated DOM node is removed from the tree
    /// (e.g., during a match arm switch in routing) to prevent stale effects
    /// from causing infinite loops.
    pub fn dispose(&self) {
        let effect_addr: usize = self.inner;
        let inner: &mut RenderEffectInner = self.leak_mut();
        if inner.get_disposed() {
            return;
        }
        inner.set_disposed(true);
        cleanup_effect_dependencies(effect_addr, inner.get_mut_dependencies());
    }

    /// Returns a mutable reference to the inner state.
    ///
    /// # Returns
    ///
    /// - `&'static mut RenderEffectInner` - A mutable reference to the inner state.
    pub(crate) fn leak_mut(&self) -> &'static mut RenderEffectInner {
        unsafe { &mut *(self.inner as *mut RenderEffectInner) }
    }
}
