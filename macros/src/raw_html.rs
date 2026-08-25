//! `unsafe_no_inline!` macro implementation.

//!
//! Parses a string literal and emits a `RawHtml` value
//! constructed via `RawHtml::new`. The macro enforces
//! that the input is a string literal so the `unsafe_no_`
//! prefix carries its security warning forward to the
//! call site.

use super::*;
/// Parses the input of the `unsafe_no_inline!` macro.
///
/// Accepts a single string literal and emits
/// `::euv::vdom::RawHtml::new(value.to_string())`.
pub(crate) fn parse_unsafe_no_inline(input: TokenStream) -> TokenStream {
    let literal: LitStr = parse_macro_input!(input as LitStr);
    let value: String = literal.value();
    let expanded: TokenStream = quote! {
        ::euv::vdom::RawHtml::new(#value.to_string())
    }
    .into();
    expanded
}
