use crate::*;

/// A type alias for a list of HTML attribute key-value pairs.
///
/// Used throughout the `html!` macro parsing to represent parsed attributes.
/// The key is a `proc_macro2::TokenStream` that evaluates to an attribute name string.
pub(crate) type HtmlAttrs = Vec<(proc_macro2::TokenStream, HtmlAttrValue)>;
