use crate::*;

/// Represents a single RSX node, which may be an element or text.
///
/// Parsed from the `rsx!` macro input before code generation.
pub(crate) enum RsxNode {
    /// An RSX element.
    Element(RsxElement),
    /// A text string literal.
    Text(String),
    /// A bare Rust expression (identifiers without braces), converted to a
    /// `VirtualNode` via `IntoNode::into_node`. This is a static one-shot
    /// conversion — no re-rendering on signal changes.
    Expr(Expr),
    /// A braced Rust expression `{expr}` in a child position, automatically
    /// wrapped into a `DynamicNode` that re-renders when signals change.
    /// The expression is evaluated inside a `move || { ... }` closure each
    /// time the dynamic node's render function is called.
    Dynamic(Expr),
    /// A reactive conditional: `if {signal} { rsx... } else if {signal} { rsx... } else { rsx... }`.
    ///
    /// Each condition is a signal expression in braces. When any signal changes,
    /// the entire conditional is re-evaluated and wrapped in a `DynamicNode`.
    If(RsxIf),
    /// A reactive match: `match {signal} { pattern => { rsx... } ... }`.
    ///
    /// The signal expression in braces is re-evaluated on change, and the
    /// matching arm's RSX is rendered inside a `DynamicNode`.
    Match(RsxMatch),
}

/// Represents the value side of an attribute.
///
/// Supports plain expressions and style objects with multiple properties.
pub(crate) enum RsxAttrValue {
    /// A normal Rust expression.
    Expr(Expr),
    /// A style object: {key: value; key2: value2;}
    /// The value can be either a string literal or an expression.
    Style(Vec<(Ident, StylePropValue)>),
}

/// Represents a single value in a style property.
///
/// May be a static string literal or a dynamic expression.
pub(crate) enum StylePropValue {
    /// A static string literal.
    Literal(String),
    /// A dynamic expression.
    Expr(Expr),
}
