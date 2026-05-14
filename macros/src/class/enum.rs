use crate::*;

/// The value side of a CSS property in a class definition.
///
/// May be a static string literal or a dynamic expression referencing parameters.
pub(crate) enum ClassPropValue {
    /// A static string literal value.
    Literal(String),
    /// A dynamic expression (Rust code token stream).
    Expr(TokenStream2),
}
