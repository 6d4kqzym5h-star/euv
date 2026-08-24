use super::*;

/// Returns the cached `Document` for the current page, falling back to
/// `window().document()` on the first call. `Document` is page-scoped (it
/// stays valid until the document is replaced), so a single resolved
/// reference is safe to reuse across the lifetime of an `euv-example`
/// mount. Subsequent calls just clone the cached handle, eliminating the
/// two JS-boundary crossings (`window()` + `document()`) every DOM node
/// creation used to pay.
///
/// OPT 8: per-page `Document` cache via `thread_local!`. The lazy
/// `OnceCell`-style fallback makes this safe even before
/// `App::mount` has finished initialising.
pub(crate) fn cached_document() -> Option<Document> {
    DOCUMENT_CACHE.with(|cell: &UnsafeCell<Option<Document>>| {
        let cached_ptr: *mut Option<Document> = cell.get();
        unsafe {
            if let Some(doc) = &*cached_ptr {
                return Some(doc.clone());
            }
        }
        let window_value: Window = window()?;
        let document: Document = window_value.document()?;
        DOCUMENT_CACHE.with(|cell: &UnsafeCell<Option<Document>>| unsafe {
            *cell.get() = Some(document.clone());
        });
        Some(document)
    })
}
