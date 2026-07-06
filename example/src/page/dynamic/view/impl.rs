use super::*;

/// Provides static label strings for `DynamicTagType` button display.
impl DynamicTagType {
    /// Returns the static display label for the tag type variant.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The display label string.
    pub(crate) fn label(self) -> &'static str {
        match self {
            DynamicTagType::Div => "div",
            DynamicTagType::Span => "span",
            DynamicTagType::EuvCard => "euv card",
            DynamicTagType::Badge => "badge",
        }
    }
}

/// Implements `Display` for `DynamicTagType` to provide the tag name string used
/// by the html! macro's dynamic tag syntax.
impl std::fmt::Display for DynamicTagType {
    /// Formats the tag type variant as its tag name string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter to write the tag name string into.
    ///
    /// # Returns
    ///
    /// - `Result` - Whether the formatting succeeded.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag_name: &str = match self {
            DynamicTagType::Div => "div",
            DynamicTagType::Span => "span",
            DynamicTagType::EuvCard => "euv_card",
            DynamicTagType::Badge => "badge",
        };
        f.write_str(tag_name)
    }
}
