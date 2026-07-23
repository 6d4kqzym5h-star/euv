use super::*;

/// A `Sync` wrapper holding a single value of type `T`.
///
/// ## Storage convention
///
/// This type aligns with the rest of the engine's `static mut`
/// convention used in `core::reactive::hook::impl`. The companion
/// storage helpers `try_get` / `try_get_mut` / `get` / `get_mut`
/// return `&'static mut T` and rely on the caller to honour the
/// single-writer contract. The engine is compiled for
/// `wasm32-unknown-unknown` which is single-threaded by contract,
/// so overlapping borrows violate the caller contract rather than
/// being trapped at runtime the way `RefCell` would.
///
/// `RefCell` is intentionally **not** used as the storage backend:
/// its runtime aliasing panic is unreachable in practice under
/// wasm's single-threaded model and adds code size and a redundant
/// safety net that the rest of the codebase does not provide.
///
/// ## `?Sized` bound and trait-object storage
///
/// The `T: ?Sized` bound is what lets the engine alias
/// `Rc<EngineCell<dyn Trait>>` directly (see `ComponentRc`,
/// `SceneRc`, `TickHandlerRc` in their respective `type.rs`). The
/// trait-object fat pointer is stored in-place inside the
/// `UnsafeCell<T>`; `get_mut -> &'static mut dyn Trait` lets
/// callers dispatch without an intermediate `Box`.
///
/// ## When to choose this vs [`MaybeEngineCell`]
///
/// The cell has no notion of "uninitialized" - use [`MaybeEngineCell`]
/// if the value may legitimately be absent (the typical case for
/// "global installed at mount, torn down at unmount" singletons).
/// For always-present values this is a zero-overhead wrapper around
/// `UnsafeCell<T>`.
///
/// ## Field access rule
///
/// The `inner` field is reached through the hand-written
/// `get_inner` / `set_inner` accessors declared in `cell/impl.rs`.
/// All read sites in this module call `get_inner().get()` and write
/// sites use `set_inner(UnsafeCell::new(value))` rather than touching
/// the field directly. Struct literals `Self { inner: ... }` are
/// permitted only inside the `new` constructor and the `Default`
/// impl, per the engine's "exceptions to no-direct-field-access"
/// convention.
///
/// ## Note on `#[derive(Data)]`
///
/// Lombok's `Data` derive is intentionally **not** applied here:
/// the derive requires `T: Sized`, which would defeat the `?Sized`
/// bound above and break the engine's `Rc<EngineCell<dyn Trait>>`
/// aliases. The accessor pair is hand-written in `cell/impl.rs`
/// with the same Lombok-style contract.
pub struct EngineCell<T: ?Sized> {
    /// Raw inner storage. `pub(crate)` only because the `new`
    /// constructor and the `Default` impl construct the struct via
    /// a struct-literal; in all other code paths the
    /// `get_inner` / `set_inner` accessors are the only access
    /// points.
    pub(crate) inner: UnsafeCell<T>,
}

/// A `Sync` wrapper that may or may not contain a value.
///
/// ## When to choose this vs [`EngineCell`]
///
/// Use this when the global state has a clear "not yet installed" or
/// "already torn down" state. Access is gated on the inner `Option`
/// being `Some`, so all the `try_*` accessors return `Option<...>`,
/// matching `core::reactive::hook::try_get_current`'s wording.
///
/// ## Field access rule
///
/// Same as [`EngineCell`]: `inner` is reached only through the
/// hand-written `get_inner` / `set_inner` accessors; the
/// struct-literal escape hatch is reserved for the `new` / `default`
/// constructors.
pub struct MaybeEngineCell<T> {
    /// Raw inner storage wrapping `Option<T>`. `pub(crate)` only
    /// because the constructor and the `Default` impl construct the
    /// struct via a struct-literal; in all other code paths the
    /// accessors in `cell/impl.rs` are the only access points.
    pub(crate) inner: UnsafeCell<Option<T>>,
}
