use crate::*;

/// Extension trait for `Element` providing DOM attribute/property manipulation methods.
///
/// Since Rust's orphan rules prevent adding inherent methods to foreign types like
/// `web_sys::Element`, this trait provides the same functionality through an extension
/// trait pattern. All methods are available on any `Element` reference via trait dispatch.
pub(crate) trait ElementExt {
    /// Removes or clears a DOM attribute/property, depending on the attribute name.
    ///
    /// For `value`, sets the DOM property to an empty string rather than calling
    /// `remove_attribute`, because `remove_attribute("value")` only removes the
    /// HTML attribute and does not clear the displayed value of input elements.
    /// For boolean properties (`checked`, `disabled`, `selected`, `readonly`),
    /// sets the DOM property to `false` rather than calling `remove_attribute`,
    /// because `remove_attribute` on a previously-set attribute may not correctly
    /// reset the property in all browsers.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name of the attribute or property to remove.
    fn remove_attribute_or_property(&self, name: &str);

    /// Sets a DOM attribute or property, depending on the attribute name.
    ///
    /// For `value`, uses the DOM property to ensure input elements update correctly.
    /// For boolean attributes (`checked`, `disabled`, `selected`, `readonly`),
    /// uses the DOM property so that the browser honors the value correctly
    /// (HTML attributes are present-or-absent, not true/false strings).
    /// For all other attributes, uses `set_attribute`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name of the attribute or property to set.
    /// - `&str` - The value to assign.
    fn set_attribute_or_property(&self, name: &str, value: &str);

    /// Appends a signal address to the element's `data-euv-signal-addrs` attribute.
    ///
    /// Uses `push_str` to avoid `format!` allocation overhead.
    ///
    /// # Arguments
    ///
    /// - `usize` - The signal inner address to append.
    fn track_signal_addr(&self, addr: usize);
}

/// Implementation of `ElementExt` for `Element`.
impl ElementExt for Element {
    fn remove_attribute_or_property(&self, name: &str) {
        if name == ATTR_VALUE {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_value(EMPTY_STRING);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_value(EMPTY_STRING);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_value(EMPTY_STRING);
                return;
            }
        }
        if name == ATTR_CHECKED
            && let Some(input) = self.dyn_ref::<HtmlInputElement>()
        {
            input.set_checked(false);
            return;
        }
        if name == ATTR_DISABLED {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_disabled(false);
                return;
            }
            if let Some(button) = self.dyn_ref::<HtmlButtonElement>() {
                button.set_disabled(false);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_disabled(false);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_disabled(false);
                return;
            }
        }
        if name == ATTR_SELECTED
            && let Some(option) = self.dyn_ref::<HtmlOptionElement>()
        {
            option.set_selected(false);
            return;
        }
        if name == ATTR_READONLY {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_read_only(false);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_read_only(false);
                return;
            }
        }
        if name == ATTR_MULTIPLE {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_multiple(false);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_multiple(false);
                return;
            }
        }
        let _ = self.remove_attribute(name);
    }

    fn set_attribute_or_property(&self, name: &str, value: &str) {
        if name == ATTR_VALUE {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_value(value);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_value(value);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_value(value);
                return;
            }
        }
        if name == ATTR_CHECKED
            && let Some(input) = self.dyn_ref::<HtmlInputElement>()
        {
            input.set_checked(value == BOOL_TRUE);
            return;
        }
        if name == ATTR_DISABLED {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_disabled(value == BOOL_TRUE);
                return;
            }
            if let Some(button) = self.dyn_ref::<HtmlButtonElement>() {
                button.set_disabled(value == BOOL_TRUE);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_disabled(value == BOOL_TRUE);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_disabled(value == BOOL_TRUE);
                return;
            }
        }
        if name == ATTR_SELECTED
            && let Some(option) = self.dyn_ref::<HtmlOptionElement>()
        {
            option.set_selected(value == BOOL_TRUE);
            return;
        }
        if name == ATTR_READONLY {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_read_only(value == BOOL_TRUE);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_read_only(value == BOOL_TRUE);
                return;
            }
        }
        if name == ATTR_MULTIPLE {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_multiple(value == BOOL_TRUE);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_multiple(value == BOOL_TRUE);
                return;
            }
        }
        let _ = self.set_attribute(name, value);
    }

    fn track_signal_addr(&self, addr: usize) {
        let existing: String = self
            .get_attribute(DATA_EUV_SIGNAL_ADDRS)
            .unwrap_or_default();
        let mut updated: String = existing;
        if !updated.is_empty() {
            updated.push(CHAR_SIGNAL_ADDRS_SEPARATOR);
        }
        updated.push_str(&addr.to_string());
        let _ = self.set_attribute(DATA_EUV_SIGNAL_ADDRS, &updated);
    }
}
