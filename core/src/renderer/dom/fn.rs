use crate::*;

/// Checks whether a DOM node is currently connected to the document.
///
/// Uses the `isConnected` JavaScript property to determine if the node
/// is still attached to the live DOM tree.
///
/// # Arguments
///
/// - `&T`- A reference to any type that can be converted to `&Node`.
///
/// # Returns
///
/// - `bool`- `true` if the node is connected to the document, `false` otherwise.
pub(crate) fn is_node_connected<T>(node: &T) -> bool
where
    T: AsRef<Node>,
{
    let node_ref: &Node = node.as_ref();
    let result: Result<JsValue, JsValue> =
        Reflect::get(node_ref.as_ref(), &JsValue::from_str(IS_CONNECTED));
    match result {
        Ok(value) => value.is_truthy(),
        Err(_) => false,
    }
}

/// Returns true if the given attribute name is a boolean attribute that
/// requires DOM property-based manipulation instead of HTML attribute strings.
///
/// # Arguments
///
/// - `&str` - The attribute name to check.
///
/// # Returns
///
/// - `bool` - `true` if the attribute is a boolean property, `false` otherwise.
pub(crate) fn is_boolean_property(name: &str) -> bool {
    matches!(
        name,
        ATTR_CHECKED | ATTR_DISABLED | ATTR_SELECTED | ATTR_READONLY
    )
}

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
/// - `&Element` - The DOM element to modify.
/// - `&str` - The name of the attribute or property to remove.
pub(crate) fn remove_dom_attribute_or_property(element: &Element, name: &str) {
    if name == ATTR_VALUE {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_value(EMPTY_STRING);
            return;
        }
        if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
            textarea.set_value(EMPTY_STRING);
            return;
        }
        if let Some(select) = element.dyn_ref::<HtmlSelectElement>() {
            select.set_value(EMPTY_STRING);
            return;
        }
    }
    if name == ATTR_CHECKED
        && let Some(input) = element.dyn_ref::<HtmlInputElement>()
    {
        input.set_checked(false);
        return;
    }
    if name == ATTR_DISABLED {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_disabled(false);
            return;
        }
        if let Some(button) = element.dyn_ref::<HtmlButtonElement>() {
            button.set_disabled(false);
            return;
        }
        if let Some(select) = element.dyn_ref::<HtmlSelectElement>() {
            select.set_disabled(false);
            return;
        }
        if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
            textarea.set_disabled(false);
            return;
        }
    }
    if name == ATTR_SELECTED
        && let Some(option) = element.dyn_ref::<HtmlOptionElement>()
    {
        option.set_selected(false);
        return;
    }
    if name == ATTR_READONLY {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_read_only(false);
            return;
        }
        if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
            textarea.set_read_only(false);
            return;
        }
    }
    let _ = element.remove_attribute(name);
}

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
/// - `&Element` - The DOM element to modify.
/// - `&str` - The name of the attribute or property to set.
/// - `&str` - The value to assign.
pub(crate) fn set_dom_attribute_or_property(element: &Element, name: &str, value: &str) {
    if name == ATTR_VALUE {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_value(value);
            return;
        }
        if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
            textarea.set_value(value);
            return;
        }
        if let Some(select) = element.dyn_ref::<HtmlSelectElement>() {
            select.set_value(value);
            return;
        }
    }
    if name == ATTR_CHECKED
        && let Some(input) = element.dyn_ref::<HtmlInputElement>()
    {
        input.set_checked(value == BOOL_TRUE);
        return;
    }
    if name == ATTR_DISABLED {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_disabled(value == BOOL_TRUE);
            return;
        }
        if let Some(button) = element.dyn_ref::<HtmlButtonElement>() {
            button.set_disabled(value == BOOL_TRUE);
            return;
        }
        if let Some(select) = element.dyn_ref::<HtmlSelectElement>() {
            select.set_disabled(value == BOOL_TRUE);
            return;
        }
        if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
            textarea.set_disabled(value == BOOL_TRUE);
            return;
        }
    }
    if name == ATTR_SELECTED
        && let Some(option) = element.dyn_ref::<HtmlOptionElement>()
    {
        option.set_selected(value == BOOL_TRUE);
        return;
    }
    if name == ATTR_READONLY {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_read_only(value == BOOL_TRUE);
            return;
        }
        if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
            textarea.set_read_only(value == BOOL_TRUE);
            return;
        }
    }
    let _ = element.set_attribute(name, value);
}
