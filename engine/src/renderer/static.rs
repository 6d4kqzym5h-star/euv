use super::*;

/// Interior-mutable slot for the renderer's pending error-scope value.
///
/// This is the `euv-engine` analog of euv-core's `HandlerRegistryCell`
/// (`core/src/renderer/registry/struct.rs:62`): a single-element
/// `Sync` wrapper that holds an `Option<JsValue>` behind an
/// `UnsafeCell`.
///
/// # Why this type exists
///
/// `WebGpuRenderer::pending_error` needs interior mutability
/// because:
///
/// 1. `pop_error_sync` takes `&self` (the WebGPU hot path cannot
///    be `async`), but the spawned `wasm_bindgen_futures::spawn_local`
///    future must mutate the slot to store the resolved
///    `Promise<GPUError?>` value.
/// 2. `take_last_error` also takes `&self` and drains the slot
///    on the next render tick.
///
/// The first implementation used `Rc<RefCell<Option<JsValue>>>`,
/// which works but pays for:
///
/// - a `RefCell::borrow_mut` runtime borrow check on every
///   write (the panic path is unreachable in practice — only
///   the spawn_local future and `take_last_error` ever touch
///   the slot, and they never overlap because the future is
///   a microtask drained before the next render tick).
/// - a heap allocation for the `RefCell`'s borrow state.
///
/// The newtype keeps the interior-mutability primitive (`Rc`),
/// because the spawn_local future needs its own owning handle,
/// but swaps the inner cell from `RefCell` to `UnsafeCell`:
///
/// - zero runtime borrow check (the WASM single-threaded
///   scheduler makes the borrow impossible to violate).
/// - zero allocation (the cell is just a `*mut Option<JsValue>`
///   sitting inside the `Rc`-managed box).
///
/// # Sync safety
///
/// `PendingErrorCell` is **not** `Sync` by default (`UnsafeCell`
/// explicitly opts out). We hand-implement `Sync` for it because
/// the renderer is only ever used in the WASM single-threaded
/// runtime; the `Rc` ensures the same instance is never shared
/// across threads (it is not `Send`/`Sync` either), and the
/// WASM main thread is the only place that ever touches the
/// slot. This matches euv-core's pattern
/// (`unsafe impl Sync for HandlerRegistryCell {}`).
///
/// If the engine is ever compiled for a multi-threaded target
/// (native, `wasm-bindgen-rayon`), this `unsafe impl Sync` is
/// unsound and must be removed.
pub struct PendingErrorCell(
    /// Interior-mutable storage for the optional `JsValue`.
    UnsafeCell<Option<JsValue>>,
);

impl PendingErrorCell {
    /// Creates a new `PendingErrorCell` with the given initial value.
    ///
    /// # Arguments
    ///
    /// - `Option<JsValue>` - The initial value; `None` means
    ///   "no error pending since construction or last take".
    ///
    /// # Returns
    ///
    /// - `Self` - A new cell wrapping the value.
    pub(crate) fn new(value: Option<JsValue>) -> Self {
        Self(UnsafeCell::new(value))
    }

    /// Returns a raw pointer to the underlying `Option<JsValue>`.
    ///
    /// This is the `unsafe` accessor the renderer uses to read
    /// or write the slot without going through a `RefCell` runtime
    /// borrow check. Callers must uphold the WASM single-thread
    /// invariant: at most one `&mut` reference to the inner
    /// value is alive at any time, and that reference is
    /// confined to one of two call sites (`pop_error_sync`'s
    /// `spawn_local` future and `take_last_error`) which never
    /// overlap.
    ///
    /// # Returns
    ///
    /// - `*mut Option<JsValue>` - raw pointer to the inner slot.
    ///   The caller is responsible for `unsafe { &mut *ptr }` to
    ///   get a `&mut Option<JsValue>`.
    pub(crate) fn as_ptr(&self) -> *mut Option<JsValue> {
        self.0.get()
    }
}

/// SAFETY: `PendingErrorCell` is only used in single-threaded WASM contexts.
///
/// The `Rc<PendingErrorCell>` field on `WebGpuRenderer` is itself not
/// `Send` / `Sync`, so the same instance cannot be shared across threads.
/// Within the WASM main thread, the `Rc`'s reference count is only
/// touched there too, and the `UnsafeCell::get()` access is confined
/// to two call sites (`pop_error_sync` and `take_last_error`) that
/// never overlap (the spawn_local microtask is drained before the
/// next render tick).
unsafe impl Sync for PendingErrorCell {}
