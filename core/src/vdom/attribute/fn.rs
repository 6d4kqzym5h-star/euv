use crate::*;

/// Returns a mutable reference to the global injected classes set.
///
/// Lazily initializes the set on first access.
///
/// # Returns
///
/// - `&mut HashSet<String>` - A mutable reference to the injected classes set.
///
/// # Safety
///
/// Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
pub(crate) fn get_injected_classes_mut() -> &'static mut HashSet<String> {
    unsafe {
        if (*INJECTED_CLASSES.get_0().get()).is_none() {
            (*INJECTED_CLASSES.get_0().get()) = Some(HashSet::new());
        }
        (*INJECTED_CLASSES.get_0().get())
            .as_mut()
            .unwrap_unchecked()
    }
}
