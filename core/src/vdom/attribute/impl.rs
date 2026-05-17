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

/// Maps each `Attribute` variant to its corresponding DOM attribute string.
impl Attribute {
    /// Returns the string representation of this attribute name for DOM binding.
    ///
    /// Static variants return `Cow::Borrowed` (zero allocation), while
    /// `Data` and `Other` variants return `Cow::Owned` (heap allocation).
    ///
    /// # Returns
    ///
    /// - `Cow<'static, str>` - The attribute name as a static or owned string.
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Attribute::AccessKey => Cow::Borrowed("accesskey"),
            Attribute::Action => Cow::Borrowed("action"),
            Attribute::Alt => Cow::Borrowed("alt"),
            Attribute::AriaLabel => Cow::Borrowed("aria-label"),
            Attribute::AutoComplete => Cow::Borrowed("autocomplete"),
            Attribute::AutoFocus => Cow::Borrowed("autofocus"),
            Attribute::Checked => Cow::Borrowed("checked"),
            Attribute::Class => Cow::Borrowed("class"),
            Attribute::Cols => Cow::Borrowed("cols"),
            Attribute::ContentEditable => Cow::Borrowed("contenteditable"),
            Attribute::Data(name) => Cow::Owned(format!("data-{}", name)),
            Attribute::Dir => Cow::Borrowed("dir"),
            Attribute::Disabled => Cow::Borrowed("disabled"),
            Attribute::Draggable => Cow::Borrowed("draggable"),
            Attribute::EncType => Cow::Borrowed("enctype"),
            Attribute::For => Cow::Borrowed("for"),
            Attribute::Form => Cow::Borrowed("form"),
            Attribute::Height => Cow::Borrowed("height"),
            Attribute::Hidden => Cow::Borrowed("hidden"),
            Attribute::Href => Cow::Borrowed("href"),
            Attribute::Id => Cow::Borrowed("id"),
            Attribute::Lang => Cow::Borrowed("lang"),
            Attribute::Max => Cow::Borrowed("max"),
            Attribute::MaxLength => Cow::Borrowed("maxlength"),
            Attribute::Method => Cow::Borrowed("method"),
            Attribute::Min => Cow::Borrowed("min"),
            Attribute::MinLength => Cow::Borrowed("minlength"),
            Attribute::Multiple => Cow::Borrowed("multiple"),
            Attribute::Name => Cow::Borrowed("name"),
            Attribute::Pattern => Cow::Borrowed("pattern"),
            Attribute::Placeholder => Cow::Borrowed("placeholder"),
            Attribute::ReadOnly => Cow::Borrowed("readonly"),
            Attribute::Required => Cow::Borrowed("required"),
            Attribute::Rows => Cow::Borrowed("rows"),
            Attribute::Selected => Cow::Borrowed("selected"),
            Attribute::Size => Cow::Borrowed("size"),
            Attribute::SpellCheck => Cow::Borrowed("spellcheck"),
            Attribute::Src => Cow::Borrowed("src"),
            Attribute::Step => Cow::Borrowed("step"),
            Attribute::Style => Cow::Borrowed("style"),
            Attribute::TabIndex => Cow::Borrowed("tabindex"),
            Attribute::Target => Cow::Borrowed("target"),
            Attribute::Title => Cow::Borrowed("title"),
            Attribute::Type => Cow::Borrowed("type"),
            Attribute::Value => Cow::Borrowed("value"),
            Attribute::Width => Cow::Borrowed("width"),
            Attribute::Other(name) => Cow::Owned(name.clone()),
        }
    }
}
