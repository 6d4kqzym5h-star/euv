//! euv_macros
//!
//! Procedural macros for the euv UI framework, including the `html!` macro
//! for declarative UI syntax, the `class!` macro for CSS class definitions,
//! the `css_vars!` macro for CSS custom properties, the `watch!` macro for
//! reactive side effects, and the `component` attribute macro.

mod class;
mod css_vars;
mod html;
mod kebab;
mod var;
mod watch;

pub(crate) use {class::*, css_vars::*, html::*, kebab::*, watch::*};

use std::iter::Peekable;

use {
    lombok_macros::*,
    proc_macro::TokenStream,
    proc_macro2::Span,
    quote::{ToTokens, quote, quote_spanned},
    syn::{
        Expr, ExprLit, Ident, Lit, LitStr, Path, Token, Type, Visibility, braced, parenthesized,
        parse,
        parse::{Parse, ParseBuffer, ParseStream},
        parse2,
        token::{Brace, Colon, Paren, Semi},
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

/// The `css_vars!` macro for defining CSS custom properties.
///
/// Each variable block creates a `CssClass` function that, when called,
/// injects the CSS custom properties into the DOM. Variable names are
/// automatically prefixed with `--`.
///
/// Variable names can be written as unquoted kebab-case identifiers
/// (e.g., `bg-primary`) or as quoted string literals (e.g., `"bg-primary"`).
///
/// ```ignore
/// css_vars! {
///     pub c_theme_light {
///         bg-primary: "#f8f9fb";
///         text-primary: "#1a1a2e";
///     }
/// }
/// ```
#[proc_macro]
pub fn css_vars(input: TokenStream) -> TokenStream {
    css_vars::parse_css_vars(input)
}

/// The `var!` macro for referencing CSS custom properties defined via `css_vars!`.
///
/// The variable name can be written as an unquoted kebab-case identifier
/// (e.g., `bg-primary`) or as a quoted string literal (e.g., `"bg-primary"`),
/// and expands to the CSS string `"var(--bg-primary)"`.
///
/// ```ignore
/// css_vars! {
///     pub c_theme {
///         bg-primary: "#f8f9fb";
///     }
/// }
/// class! {
///     pub c_container {
///         background: var!(bg-primary);
///     }
/// }
/// ```
#[proc_macro]
pub fn var(input: TokenStream) -> TokenStream {
    var::parse_var(input)
}

/// The `component` attribute macro for marking component functions.
///
/// This is a pass-through attribute macro that does not modify the item.
/// It serves as a marker to indicate that the annotated function is a
/// UI component that returns a `VirtualNode`.
///
/// # Arguments
///
/// - `TokenStream` - The attribute arguments (unused).
/// - `TokenStream` - The item being annotated (passed through unchanged).
///
/// # Returns
///
/// - `TokenStream` - The original item unchanged.
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
