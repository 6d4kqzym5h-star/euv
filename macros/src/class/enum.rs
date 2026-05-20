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
