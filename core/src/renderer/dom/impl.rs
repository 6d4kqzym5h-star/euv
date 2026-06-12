use crate::*;

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
        let mut updated: String = self
            .get_attribute(DATA_EUV_SIGNAL_ADDRS)
            .unwrap_or_default();
        if !updated.is_empty() {
            updated.push(CHAR_SIGNAL_ADDRS_SEPARATOR);
        }
        updated.push_str(&addr.to_string());
        let _ = self.set_attribute(DATA_EUV_SIGNAL_ADDRS, &updated);
    }
}
