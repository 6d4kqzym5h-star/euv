/// The value side of a CSS variable definition.
///
/// Always stored as an expression token stream. String literals like `"#fff"`
/// and dynamic expressions are all parsed as Rust expressions.
#[derive(Clone, Debug)]
pub(crate) enum CssVarValue {
    /// A dynamic expression (Rust code token stream) that evaluates to a string.
    Expr(proc_macro2::TokenStream),
}
