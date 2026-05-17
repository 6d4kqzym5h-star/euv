use crate::*;

/// Implementation of style CSS serialization.
impl Style {
    /// Adds a style property.
    ///
    /// Property names are automatically converted from snake_case to kebab-case
    /// (e.g., `flex_direction` becomes `flex-direction`).
    ///
    /// # Arguments
    ///
    /// - `N` - The property name (snake_case will be converted to kebab-case).
    /// - `V` - The property value.
    ///
    /// # Returns
    ///
    /// - `Self` - This style with the property added.
    pub fn property<N, V>(mut self, name: N, value: V) -> Self
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        self.get_mut_properties().push(StyleProperty::new(
            name.as_ref().replace('_', "-"),
            value.as_ref().to_string(),
        ));
        self
    }

    /// Converts the style to a CSS string.
    ///
    /// # Returns
    ///
    /// - `String` - The CSS string representation.
    pub fn to_css_string(&self) -> String {
        self.get_properties()
            .iter()
            .map(|p| format!("{}: {};", p.get_name(), p.get_value()))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// Provides a default empty style.
impl Default for Style {
    /// Returns a default `Style` with no properties.
    ///
    /// # Returns
    ///
    /// - `Self` - An empty style.
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
