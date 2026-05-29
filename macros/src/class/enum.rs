/// The key side of a CSS property in a class definition.
///
/// Supports two forms:
/// - `Static`: A compile-time string literal or kebab-case identifier (e.g., `font_size`, `"background"`).
/// - `Dynamic`: A runtime expression wrapped in braces (e.g., `{key_var}`) that evaluates to a string.
#[derive(Clone, Debug)]
pub(crate) enum ClassPropKey {
    /// A static key known at compile time.
    Static(String),
    /// A dynamic expression that evaluates to a key string at runtime.
    Dynamic(proc_macro2::TokenStream),
}

/// The value side of a CSS property in a class definition.
///
/// Always stored as an expression token stream. String literals like `"#fff"`
/// and dynamic expressions (including `var!()` calls) are all parsed as
/// Rust expressions.
#[derive(Clone, Debug)]
pub(crate) enum ClassPropValue {
    /// A dynamic expression (Rust code token stream) that evaluates to a string.
    Expr(proc_macro2::TokenStream),
}
