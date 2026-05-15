//! euv Macros
//!
//! Procedural macros for the euv UI framework, including the `html!` macro
//! for declarative UI syntax, the `class!` macro for CSS class definitions,
//! and the `component` attribute macro.

mod class;
mod html;
mod watch;

pub(crate) use {class::*, html::*, watch::*};

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

/// The `html!` macro for writing declarative UI in euv.
///
/// This macro accepts a syntax similar to Dioxus HTML:
///
/// ```ignore
/// html! {
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
pub fn html(input: TokenStream) -> TokenStream {
    html::parse_html(input)
}

/// The `class!` macro for defining CSS classes with style properties.
///
/// Each class definition creates a `CssClass` function that can be used
/// in `html!` via the `class:` attribute. Styles are automatically injected
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

/// The `watch!` macro for creating reactive side effects.
///
/// Watches one or more signals and executes a closure whenever any of them changes.
/// The closure is also executed once immediately with the current signal values
/// during initialisation. This initial execution is wrapped in a suppressed-update
/// scope so that any `.set()` calls inside the body do not trigger unnecessary
/// DynamicNode re-renders.
///
/// The number of signal expressions must match the number of closure parameters.
/// Each closure parameter receives the current value (via `.get()`) of the
/// corresponding signal.
///
/// ```ignore
/// let count = use_signal(|| 0_i32);
/// let name = use_signal(|| String::from("euv"));
/// watch!(count, name, |count_val, name_val| {
///     web_sys::console::log_1(&format!("count={}, name={}", count_val, name_val).into());
/// });
/// ```
#[proc_macro]
pub fn watch(input: TokenStream) -> TokenStream {
    watch::parse_watch(input)
}

/// The `component` attribute macro for marking component functions.
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
