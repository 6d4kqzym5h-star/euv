use crate::*;

/// SAFETY: `InjectedClassesCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for InjectedClassesCell {}

/// Visual equality comparison for attribute values.
///
/// Compares values by their visual output rather than identity. `Signal`
/// values are compared by their current resolved string; when both signals
/// share the same inner pointer, they are always considered **unequal**
/// because the signal may have mutated between VDOM snapshots and `.get()`
/// would return the same current value for both, masking the change.
/// `Event` values are always considered equal (re-binding is handled by the
/// handler registry), and `CssClass` values are compared by class name.
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
            (AttributeValue::Text(old_val), AttributeValue::Text(new_val)) => old_val == new_val,
            (AttributeValue::Signal(old_sig), AttributeValue::Signal(new_sig)) => {
                if old_sig.get_inner_addr() == new_sig.get_inner_addr() {
                    return false;
                }
                old_sig.get() == new_sig.get()
            }
            (AttributeValue::Signal(old_sig), AttributeValue::Text(new_val)) => {
                old_sig.get() == *new_val
            }
            (AttributeValue::Text(old_val), AttributeValue::Signal(new_sig)) => {
                *old_val == new_sig.get()
            }
            (AttributeValue::Event(_), AttributeValue::Event(_)) => true,
            (AttributeValue::Css(old_css), AttributeValue::Css(new_css)) => {
                old_css.get_name() == new_css.get_name()
            }
            (AttributeValue::Dynamic(old_dyn), AttributeValue::Dynamic(new_dyn)) => {
                old_dyn == new_dyn
            }
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

    /// Creates a new CSS class with the given name, style declarations, and pseudo rules.
    ///
    /// Automatically injects the base styles, pseudo-class/pseudo-element rules,
    /// and media query rules into the DOM upon creation.
    ///
    /// # Arguments
    ///
    /// - `String` - The class name.
    /// - `String` - The CSS style declarations.
    /// - `Vec<PseudoRule>` - The pseudo-class and pseudo-element rules.
    /// - `Vec<MediaRule>` - The media query rules.
    ///
    /// # Returns
    ///
    /// - `Self` - A new CSS class with injected styles and pseudo rules.
    pub fn new_with_rules(
        name: String,
        style: String,
        pseudo_rules: Vec<PseudoRule>,
        media_rules: Vec<MediaRule>,
    ) -> Self {
        let mut css_class: CssClass = CssClass::default();
        css_class.set_name(name);
        css_class.set_style(style);
        css_class.set_pseudo_rules(pseudo_rules);
        css_class.set_media_rules(media_rules);
        css_class.inject_style();
        css_class
    }

    /// Parses pseudo-class/pseudo-element rules from a compact serialization string.
    ///
    /// The serialization format is: `:selector { key: value; key: value; }:another { ... }`
    /// This is used by the `class!` macro for fully static class definitions
    /// where pseudo rules can be computed at compile time.
    ///
    /// # Arguments
    ///
    /// - `&str` - The serialized pseudo rules string.
    ///
    /// # Returns
    ///
    /// - `Vec<PseudoRule>` - The parsed pseudo rules.
    pub fn parse_pseudo_rules(input: &str) -> Vec<PseudoRule> {
        let mut rules: Vec<PseudoRule> = Vec::new();
        let mut remaining: &str = input;
        while !remaining.is_empty() {
            let selector_end: Option<usize> = remaining.find(" { ");
            let Some(sel_end) = selector_end else {
                break;
            };
            let selector: &str = &remaining[..sel_end];
            let after_selector: &str = remaining[sel_end..].strip_prefix(" { ").unwrap_or("");
            let style_end: Option<usize> = after_selector.find('}');
            let Some(st_end) = style_end else {
                break;
            };
            let style: &str = &after_selector[..st_end];
            if !selector.is_empty() && !style.is_empty() {
                rules.push(PseudoRule::new(selector.to_string(), style.to_string()));
            }
            remaining = after_selector[st_end..].strip_prefix('}').unwrap_or("");
        }
        rules
    }

    /// Parses media query rules from a compact serialization string.
    ///
    /// The serialization format is: `@media query { key: value; key: value; }@media query2 { ... }`
    /// This is used by the `class!` macro for fully static class definitions
    /// where media rules can be computed at compile time.
    ///
    /// # Arguments
    ///
    /// - `&str` - The serialized media rules string.
    ///
    /// # Returns
    ///
    /// - `Vec<MediaRule>` - The parsed media rules.
    pub fn parse_media_rules(input: &str) -> Vec<MediaRule> {
        let mut rules: Vec<MediaRule> = Vec::new();
        let mut remaining: &str = input;
        while !remaining.is_empty() {
            if !remaining.starts_with("@media ") {
                break;
            }
            let after_prefix: &str = remaining.strip_prefix("@media ").unwrap_or("");
            let query_end: Option<usize> = after_prefix.find(" { ");
            let Some(q_end) = query_end else {
                break;
            };
            let query: &str = &after_prefix[..q_end];
            let after_query: &str = after_prefix[q_end..].strip_prefix(" { ").unwrap_or("");
            let style_end: Option<usize> = after_query.find('}');
            let Some(st_end) = style_end else {
                break;
            };
            let style: &str = &after_query[..st_end];
            if !query.is_empty() && !style.is_empty() {
                rules.push(MediaRule::new(query.to_string(), style.to_string()));
            }
            remaining = after_query[st_end..].strip_prefix('}').unwrap_or("");
        }
        rules
    }

    /// Injects this class's styles into the DOM if not already present.
    ///
    /// Uses a global `HashSet` to track injected class names, avoiding the
    /// expensive `existing_css.contains(css)` full-text search on every call.
    /// Builds the class rule, pseudo-class rules, and media rules as CSS text,
    /// then appends them directly to the `<style>` element via
    /// `append_child` with a new text node — no read-modify-write of the
    /// entire stylesheet content.
    ///
    /// # Panics
    ///
    /// Panics if `window()` or `document()` is unavailable on the current platform.
    pub fn inject_style(&self) {
        if !Self::mark_injected(self.get_name().clone()) {
            return;
        }
        let class_rule: String = format!(".{} {{ {} }}", self.get_name(), self.get_style());
        let mut css: String = class_rule;
        for pseudo_rule in self.get_pseudo_rules() {
            if !pseudo_rule.get_style().is_empty() {
                let pseudo_rule_str: String = format!(
                    ".{}{} {{ {} }}",
                    self.get_name(),
                    pseudo_rule.get_selector(),
                    pseudo_rule.get_style()
                );
                css = format!("{}\n{}", css, pseudo_rule_str);
            }
        }
        for media_rule in self.get_media_rules() {
            if !media_rule.get_query().is_empty() {
                let media_rule_str: String = format!(
                    "@media {} {{ .{} {{ {} }} }}",
                    media_rule.get_query(),
                    self.get_name(),
                    media_rule.get_style()
                );
                css = format!("{}\n{}", css, media_rule_str);
            }
        }
        Self::append_css(&css);
    }

    /// Marks a class name as injected in the global `HashSet`.
    ///
    /// Returns `false` if the class was already injected (no-op), `true`
    /// if this is the first injection.
    ///
    /// # Arguments
    ///
    /// - `String` - The class name to mark as injected.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if newly injected, `false` if already present.
    fn mark_injected(class_name: String) -> bool {
        mark_injected_class(class_name)
    }

    /// Appends CSS text directly to the shared `<style>` element.
    ///
    /// Creates a new text node and appends it as a child of the `<style>`
    /// element, avoiding the read-modify-write pattern of reading the entire
    /// `innerText`, concatenating, and setting it back.
    ///
    /// # Arguments
    ///
    /// - `&str` - The CSS text to append.
    ///
    /// # Panics
    ///
    /// Panics if `window()` or `document()` is unavailable on the current platform.
    fn append_css(css: &str) {
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
                document.head().unwrap().append_child(&el).unwrap();
                el
            }
        };
        if !css.is_empty() {
            let text_node: Text = document.create_text_node(css);
            style_element.append_child(&text_node).unwrap();
        }
    }

    /// Injects CSS text into the shared `<style>` element in the DOM.
    ///
    /// Delegates to [`CssClass::append_css`] for the actual DOM append.
    /// Unlike the previous implementation, this does not read the existing
    /// stylesheet content or perform a full-text `contains` search.
    ///
    /// # Arguments
    ///
    /// - `&str` - The CSS text to inject (e.g., reset styles, keyframes, media queries).
    ///
    /// # Panics
    ///
    /// Panics if `window()` or `document()` is unavailable on the current platform.
    pub fn inject_css(css: &str) {
        Self::append_css(css);
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
