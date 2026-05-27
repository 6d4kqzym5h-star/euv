use crate::*;

/// Marks `CurrentHookContextCell` as `Sync` for single-threaded WASM contexts.
///
/// SAFETY: `CurrentHookContextCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl Sync for CurrentHookContextCell {}
