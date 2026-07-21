use super::*;

/// Global set of CSS class names that have already been injected into the DOM.
///
/// Used by `Css::inject_style` to skip redundant injections when the same
/// class is encountered multiple times (e.g., 100 `<li>` elements sharing
/// `c_list_item`). Without this dedup, each occurrence would append a
/// duplicate text node to the `<style>` element, causing both memory
/// bloat and O(N) style-recalc cost in the browser.
pub(crate) static mut INJECTED_CLASSES: LazyLock<InjectedClassesCell> =
    LazyLock::new(|| InjectedClassesCell(UnsafeCell::new(HashSet::new())));
