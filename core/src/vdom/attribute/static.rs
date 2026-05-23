use crate::*;

/// Global set of CSS class names that have already been injected into the DOM.
pub(crate) static mut INJECTED_CLASSES: InjectedClassesCell =
    InjectedClassesCell(UnsafeCell::new(None));
