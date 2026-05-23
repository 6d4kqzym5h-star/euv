use crate::*;

/// SAFETY: `CurrentHookContextCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for CurrentHookContextCell {}
