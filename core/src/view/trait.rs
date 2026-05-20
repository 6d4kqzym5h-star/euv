use crate::*;

/// Trait for types that can be converted into a `View`.
///
/// All types that can appear as children in `html!` implement this trait.
/// This includes DOM elements, text nodes, signals, closures, and nested
/// `html!` invocations.
pub trait IntoView {
    /// Converts this value into a `View` containing real DOM nodes.
    ///
    /// # Returns
    ///
    /// - `View` - A view with one or more DOM nodes and optional cleanups.
    fn into_view(self) -> View;
}

/// Trait for applying an attribute value directly to a DOM element.
///
/// This replaces the previous `AttributeValue` + `Renderer::create_dom_node`
/// pipeline for attributes. Instead of building a `Vec<AttributeEntry>` and
/// later iterating over it, values are applied immediately when the element
/// is created.
pub trait ApplyToElement {
    /// Applies this value as an attribute to the given DOM element.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The DOM element.
    /// - `&str` - The attribute name.
    fn apply_to_element(self, element: &Element, attr_name: &str);
}

/// Applies a `String` as a static DOM attribute.
impl ApplyToElement for String {
    fn apply_to_element(self, element: &Element, attr_name: &str) {
        if !self.is_empty() || is_boolean_property(attr_name) {
            set_dom_attribute_or_property(element, attr_name, &self);
        }
    }
}

/// Applies a `&str` as a static DOM attribute.
impl ApplyToElement for &str {
    fn apply_to_element(self, element: &Element, attr_name: &str) {
        self.to_string().apply_to_element(element, attr_name);
    }
}

/// Applies a `Signal<String>` as a reactive DOM attribute.
impl ApplyToElement for Signal<String> {
    fn apply_to_element(self, element: &Element, attr_name: &str) {
        bind_attr_signal(element, attr_name, self);
    }
}

/// Applies a `CssClass` by injecting styles and setting the class attribute.
impl ApplyToElement for CssClass {
    fn apply_to_element(self, element: &Element, attr_name: &str) {
        apply_css_class(element, attr_name, &self);
    }
}

/// Applies a `&CssClass` by injecting styles and setting the class attribute.
impl ApplyToElement for &CssClass {
    fn apply_to_element(self, element: &Element, attr_name: &str) {
        apply_css_class(element, attr_name, self);
    }
}

/// Applies a `bool` as a boolean DOM attribute.
impl ApplyToElement for bool {
    fn apply_to_element(self, element: &Element, attr_name: &str) {
        let value: String = self.to_string();
        value.apply_to_element(element, attr_name);
    }
}

/// Applies a `Signal<bool>` as a reactive boolean DOM attribute.
impl ApplyToElement for Signal<bool> {
    fn apply_to_element(self, element: &Element, attr_name: &str) {
        let string_signal: Signal<String> = Signal::new(self.get_untracked().to_string());
        let string_signal_clone: Signal<String> = string_signal;
        self.replace_subscribe(move || {
            string_signal_clone.set(self.get_untracked().to_string());
        });
        bind_attr_signal(element, attr_name, string_signal);
    }
}

/// Applies an `i32` as a text DOM attribute.
impl ApplyToElement for i32 {
    fn apply_to_element(self, element: &Element, attr_name: &str) {
        self.to_string().apply_to_element(element, attr_name);
    }
}

/// Applies a `usize` as a text DOM attribute.
impl ApplyToElement for usize {
    fn apply_to_element(self, element: &Element, attr_name: &str) {
        self.to_string().apply_to_element(element, attr_name);
    }
}

/// Applies an `f64` as a text DOM attribute.
impl ApplyToElement for f64 {
    fn apply_to_element(self, element: &Element, attr_name: &str) {
        self.to_string().apply_to_element(element, attr_name);
    }
}

/// Delegates `apply_to_element` through `AttrValueAdapter`.
impl<T> AttrValueAdapter<T>
where
    T: ApplyToElement,
{
    /// Applies the wrapped value as an attribute to a DOM element.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The DOM element.
    /// - `&str` - The attribute name.
    pub fn apply_to_element(self, element: &Element, attr_name: &str) {
        self.inner.apply_to_element(element, attr_name);
    }
}
