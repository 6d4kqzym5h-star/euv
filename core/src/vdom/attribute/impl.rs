use crate::*;

/// Visual equality comparison for attribute values.
///
/// Compares values by their visual output rather than identity. `Signal`
/// values are compared by their current resolved string, `Event` values
/// are always considered equal (re-binding is handled by the handler
/// registry), and `CssClass` values are compared by class name.
impl PartialEq for AttributeValue {
    /// Compares two attribute values for visual equality.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The first attribute value.
    /// - `&Self` - The second attribute value.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the values are visually equal.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AttributeValue::Text(a_val), AttributeValue::Text(b_val)) => a_val == b_val,
            (AttributeValue::Signal(a_sig), AttributeValue::Signal(b_sig)) => {
                a_sig.get() == b_sig.get()
            }
            (AttributeValue::Signal(a_sig), AttributeValue::Text(b_val)) => a_sig.get() == *b_val,
            (AttributeValue::Text(a_val), AttributeValue::Signal(b_sig)) => *a_val == b_sig.get(),
            (AttributeValue::Event(_), AttributeValue::Event(_)) => true,
            (AttributeValue::Css(a_css), AttributeValue::Css(b_css)) => {
                a_css.get_name() == b_css.get_name()
            }
            (AttributeValue::Dynamic(a_dyn), AttributeValue::Dynamic(b_dyn)) => a_dyn == b_dyn,
            _ => false,
        }
    }
}

/// Visual equality comparison for attribute entries.
///
/// Two attribute entries are equal when their names match and their values
/// are visually equal as defined by `AttributeValue::eq`.
impl PartialEq for AttributeEntry {
    /// Compares two attribute entries for visual equality.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The first attribute entry.
    /// - `&Self` - The second attribute entry.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if both names and values match.
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && self.get_value() == other.get_value()
    }
}

/// Visual equality comparison for CSS classes.
///
/// Two CSS classes are considered equal when their class names match,
/// since the name uniquely identifies the visual style rule.
impl PartialEq for CssClass {
    /// Compares two CSS classes by name.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The first CSS class.
    /// - `&Self` - The second CSS class.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the class names match.
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

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
            .map(|style: &StyleProperty| format!("{}: {};", style.get_name(), style.get_value()))
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Builds a CSS style string from an array of key-value pairs.
    ///
    /// This function is used by the `html!` macro to convert static `style:`
    /// attributes into a CSS string without allocating intermediate `Style`
    /// and `Vec<StyleProperty>` objects. Keys are converted from snake_case
    /// to kebab-case automatically.
    ///
    /// # Arguments
    ///
    /// - `&[(&str, &str)]` - An array of CSS property name-value pairs.
    ///
    /// # Returns
    ///
    /// - `String` - The CSS string (e.g., `"margin: 0 auto; max-width: 800px;"`).
    pub fn create_style_string(props: &[(&str, &str)]) -> String {
        let mut result: String = String::new();
        for (key, value) in props {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&key.replace('_', "-"));
            result.push_str(": ");
            result.push_str(value);
            result.push(';');
        }
        result
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

/// Implementation of CssClass construction and style injection.
impl CssClass {
    /// Creates a new CSS class with the given name and style declarations.
    ///
    /// Automatically injects the styles into the DOM upon creation.
    ///
    /// # Arguments
    ///
    /// - `String` - The class name.
    /// - `String` - The CSS style declarations.
    ///
    /// # Returns
    ///
    /// - `Self` - A new CSS class with injected styles.
    pub fn new(name: String, style: String) -> Self {
        let mut css_class: CssClass = CssClass::default();
        css_class.set_name(name);
        css_class.set_style(style);
        css_class.inject_style();
        css_class
    }

    /// Injects this class's styles into the DOM if not already present.
    ///
    /// Creates a `<style>` element with id `euv-css-injected` on first call,
    /// then appends the class rule. Subsequent calls for the same class name
    /// are no-ops. On first creation, also injects global CSS keyframes
    /// required by built-in animations.
    ///
    /// # Panics
    ///
    /// Panics if `window()` or `document()` is unavailable on the current platform.
    pub fn inject_style(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let style_id: &str = "euv-css-injected";
            let document: Document = window()
                .expect("no global window exists")
                .document()
                .expect("no document exists");
            let style_element: HtmlStyleElement = match document.get_element_by_id(style_id) {
                Some(el) => el.dyn_into::<HtmlStyleElement>().unwrap(),
                None => {
                    let el: HtmlStyleElement = document
                        .create_element("style")
                        .unwrap()
                        .dyn_into::<HtmlStyleElement>()
                        .unwrap();
                    el.set_id(style_id);
                    let keyframes: &str = "@keyframes euv-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } } @keyframes euv-fade-in { from { opacity: 0; } to { opacity: 1; } } @keyframes euv-scale-in { from { transform: scale(0.9); opacity: 0; } to { transform: scale(1); opacity: 1; } } @keyframes euv-pulse { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.2); } } @keyframes euv-slide-up { from { transform: translateY(100%); } to { transform: translateY(0); } } @keyframes euv-slide-left { from { transform: translateX(-100%); } to { transform: translateX(0); } } @keyframes euv-fade-in-up { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: translateY(0); } }";
                    let global: &str = "html, body, #app { height: 100%; margin: 0; padding: 0; overflow: hidden; } * { -webkit-tap-highlight-color: transparent; }";
                    let media_queries: &str = "@media (max-width: 767px) { .c_app_nav { display: none; } .c_app_main { padding: 20px 16px; max-width: 100%; } .c_page_title { font-size: 22px; } .c_page_subtitle { font-size: 14px; } .c_card { padding: 16px; margin: 12px 0; border-radius: 10px; } .c_card_title { font-size: 16px; } .c_form_grid { grid-template-columns: 1fr; } .c_browser_api_row { grid-template-columns: 1fr; } .c_modal_content { max-width: 100%; width: calc(100% - 32px); border-radius: 16px 16px 0 0; position: fixed; bottom: 0; left: 16px; height: 80vh; animation: euv-slide-up 0.25s ease; } .c_modal_overlay { align-items: flex-end; } .c_event_stats { gap: 12px; flex-wrap: wrap; } .c_event_section_row { gap: 12px; flex-wrap: wrap; } .c_event_section_col { min-width: 100%; } .c_counter_value { font-size: 20px; } .c_timer_value { font-size: 36px; } .c_not_found_code { font-size: 56px; } .c_not_found_container { padding: 40px 20px; } .c_list_input_row { flex-direction: column; } .c_vconsole_button { bottom: 16px; right: 16px; width: 44px; height: 44px; border-radius: 12px; } .c_tab_bar { flex-wrap: wrap; } .c_primary_button { padding: 10px 18px; font-size: 14px; } .c_badge { padding: 4px 10px; font-size: 11px; } .c_badge_outline { padding: 4px 10px; font-size: 11px; } .c_browser_info_grid { grid-template-columns: 1fr; } .c_anim_spin { font-size: 36px; } .c_anim_spin_stopped { font-size: 36px; } .c_anim_pulse { font-size: 36px; } .c_anim_pulse_stopped { font-size: 36px; } }";
                    el.set_inner_text(&format!("{} {} {}", global, keyframes, media_queries));
                    document.head().unwrap().append_child(&el).unwrap();
                    el
                }
            };
            let existing_css: String = style_element.inner_text();
            let class_rule: String = format!(".{} {{ {} }}", self.get_name(), self.get_style());
            if !existing_css.contains(&class_rule) {
                let new_css: String = if existing_css.is_empty() {
                    class_rule
                } else {
                    format!("{}\n{}", existing_css, class_rule)
                };
                style_element.set_inner_text(&new_css);
            }
        }
    }
}

/// Displays the CSS class name.
///
/// This enables `format!("{}", css_class)` to produce the class name string,
/// which is required for reactive `if` conditions in `class:` attributes.
impl std::fmt::Display for CssClass {
    /// Formats the CSS class as its name string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter` - The formatter.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result` - The formatting result.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
