//! euv Macros
//!
//! Procedural macros for the euv UI framework, including the `rsx!` macro
//! for declarative UI syntax, the `class!` macro for CSS class definitions,
//! and the `component` attribute macro.

mod class;
mod rsx;

pub(crate) use {class::*, rsx::*};

use {
    proc_macro::TokenStream,
    proc_macro2::TokenStream as TokenStream2,
    quote::{ToTokens, quote},
    syn::{
        Expr, Ident, LitStr, Result as SynResult, Token, braced,
        parse::{Parse, ParseStream},
        parse_macro_input,
        token::{Colon, Semi},
    },
};

/// The `rsx!` macro for writing declarative UI in euv.
///
/// This macro accepts a syntax similar to Dioxus RSX:
///
/// ```ignore
/// rsx! {
///     div {
///         class: c_container()
///         h1 { "Hello, euv!" }
///         button {
///             onclick: move |_| { /* handle click */ },
///             "Click me"
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    rsx::parse_rsx(input)
}

/// The `class!` macro for defining CSS classes with style properties.
///
/// Each class definition creates a `CssClass` function that can be used
/// in `rsx!` via the `class:` attribute. Styles are automatically injected
/// into the DOM on first use.
///
/// ```ignore
/// class! {
///     pub container {
///         max_width: "800px";
///         margin: "0 auto";
///     }
///     pub(crate) header {
///         font_size: "28px";
///     }
///     hidden {
///         display: "none";
///     }
/// }
/// ```
#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    class::parse_class(input)
}

/// The `component` attribute macro for marking component functions.
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
