//! euv_macros
//!
//! Procedural macros for the euv UI framework, including the `html!` macro
//! for declarative UI syntax, the `class!` macro for CSS class definitions,
//! the `vars!` macro for CSS custom properties, the `watch!` macro for
//! reactive side effects, the `computed!` macro for reactive computed signals,
//! and the `component` attribute macro.

mod class;
mod computed;
mod html;
mod ident;
mod var;
mod watch;

pub(crate) use {class::*, computed::*, html::*, ident::*, var::*, watch::*};

use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs::{read_dir, read_to_string},
    iter::Peekable,
    mem::MaybeUninit,
    path::PathBuf,
};

use {
    lombok_macros::*,
    proc_macro::TokenStream,
    proc_macro2::{Span, TokenTree},
    quote::{ToTokens, quote, quote_spanned},
    syn::{
        Attribute, Expr, Field, File, Ident, Item, LitStr, Path, Stmt, Token, Type, Visibility,
        braced, parenthesized, parse,
        parse::{Parse, ParseBuffer, ParseStream},
        parse_file, parse2,
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
    parse_html(input)
}

/// The `class!` macro for defining CSS classes with style properties.
///
/// Each class definition creates a `Css` function that can be used
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
    parse_class(input)
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
/// corresponding signal. Parameter types are optional and can be annotated
/// after a colon.
///
/// ```ignore
/// let count = use_signal(|| 0_i32);
/// let name = use_signal(|| String::from("euv"));
/// watch!(count, name, |count_val: i32, name_val: String| {
///     web_sys::console::log_1(&format!("count={}, name={}", count_val, name_val).into());
/// });
/// ```
#[proc_macro]
pub fn watch(input: TokenStream) -> TokenStream {
    parse_watch(input)
}

/// The `computed!` macro for creating reactive computed signals.
///
/// Watches one or more signals and derives a new signal whose value is
/// automatically computed from the closure return value whenever any input
/// signal changes. The closure must return a value of the specified return type.
///
/// The number of signal expressions must match the number of closure parameters.
/// Each closure parameter receives the current value (via `.get()`) of the
/// corresponding signal. Parameter types are optional and can be annotated
/// after a colon. The return type must be specified after `->`.
///
/// The result signal is created via `use_signal` and updated via `set()`
/// to mark its dependents dirty precisely. The initial value is computed immediately
/// during first render.
///
/// ```ignore
/// let first_name = use_signal(|| String::from("John"));
/// let last_name = use_signal(|| String::from("Doe"));
/// let full_name: Signal<String> = computed!(first_name, last_name, |first: String, last: String| -> String {
///     format!("{} {}", first, last)
/// });
/// ```
#[proc_macro]
pub fn computed(input: TokenStream) -> TokenStream {
    parse_computed(input)
}

/// The `vars!` macro for defining CSS custom properties.
///
/// Each variable block creates a `Css` function that, when called,
/// injects the CSS custom properties into the DOM. Variable names are
/// automatically prefixed with `--`.
///
/// Variable names can be written as unquoted kebab-case identifiers
/// (e.g., `bg-primary`) or as quoted string literals (e.g., `"bg-primary"`).
///
/// ```ignore
/// vars! {
///     pub c_theme_light {
///         bg-primary: "#f8f9fb";
///         text-primary: "#1a1a2e";
///     }
/// }
/// ```
#[proc_macro]
pub fn vars(input: TokenStream) -> TokenStream {
    parse_vars(input)
}

/// The `var!` macro for referencing CSS custom properties defined via `vars!`.
///
/// The variable name can be written as an unquoted kebab-case identifier
/// (e.g., `bg-primary`) or as a quoted string literal (e.g., `"bg-primary"`),
/// and expands to the CSS string `"var(--bg-primary)"`.
///
/// ```ignore
/// vars! {
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
    parse_var(input)
}

/// The `component` attribute macro for marking component functions.
///
/// Only functions annotated with `#[component]` are treated as components
/// in the `html!` macro. All other identifier tags are treated as native
/// HTML elements (with `Tag::Element`).
///
/// The `html!` macro scans the project source to find `#[component]`-annotated
/// functions at compile time, so this attribute must be present for the
/// `html!` macro to generate a component function call.
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
