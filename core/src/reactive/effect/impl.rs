use crate::*;

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
    pub fn create<F>(effect_fn: F) -> Self
    where
        F: FnMut() + 'static,
    {
        let effect_inner: Rc<RefCell<RenderEffectInner>> = Rc::new(RefCell::new(
            RenderEffectInner::new(Box::new(effect_fn), Vec::new(), false, false),
        ));
        let effect: RenderEffect = RenderEffect::new(effect_inner.clone());
        let addr: usize = effect.get_inner_addr();
        render_effect_registry_mut().insert(addr, effect_inner);
        effect.run_once();
        effect
    }

    /// Returns the memory address of the inner `Rc` for identity comparison.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the inner `Rc`.
    pub(crate) fn get_inner_addr(&self) -> usize {
        Rc::as_ptr(self.get_inner()) as usize
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
        let effect_addr: usize = self.get_inner_addr();
        {
            let mut inner: RefMut<RenderEffectInner> = self.get_inner().borrow_mut();
            if inner.get_disposed() {
                return;
            }
            if inner.get_running() {
                return;
            }
            inner.set_running(true);
            cleanup_effect_dependencies(effect_addr, inner.get_mut_dependencies());
        }
        let prev: Option<usize> = swap_current_effect(Some(effect_addr));
        {
            let mut inner: RefMut<RenderEffectInner> = self.get_inner().borrow_mut();
            (inner.get_mut_effect_fn())();
        }
        set_current_effect(prev);
        {
            let mut inner: RefMut<RenderEffectInner> = self.get_inner().borrow_mut();
            inner.set_running(false);
        }
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
        let effect_addr: usize = self.get_inner_addr();
        let mut inner: RefMut<RenderEffectInner> = self.get_inner().borrow_mut();
        if inner.get_disposed() {
            return;
        }
        inner.set_disposed(true);
        cleanup_effect_dependencies(effect_addr, inner.get_mut_dependencies());
    }
}

/// SAFETY: `EffectSubscribersCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for EffectSubscribersCell {}

/// SAFETY: `CurrentEffectCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for CurrentEffectCell {}

/// SAFETY: `RenderEffectRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for RenderEffectRegistryCell {}
