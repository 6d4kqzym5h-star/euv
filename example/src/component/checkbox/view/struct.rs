use crate::*;

/// Props for the `euv_checkbox` component.
///
/// Defines the strongly-typed interface for a labeled checkbox.
#[derive(Clone, Default)]
pub(crate) struct EuvCheckboxProps {
    /// The unique identifier for the checkbox element.
    pub(crate) id: &'static str,
    /// The HTML name attribute for the checkbox element.
    pub(crate) name: &'static str,
    /// The autocomplete hint for the browser.
    pub(crate) autocomplete: &'static str,
    /// The boolean signal bound to the checkbox checked state.
    pub(crate) checked: Signal<bool>,
    /// The label text displayed next to the checkbox.
    pub(crate) label: &'static str,
}
