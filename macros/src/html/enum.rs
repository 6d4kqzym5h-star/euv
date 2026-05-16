use crate::*;

/// Represents a single HTML node, which may be an element or text.
///
/// Parsed from the `html!` macro input before code generation.
pub(crate) enum HtmlNode {
    /// An HTML element.
    Element(HtmlElement),
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
    /// A reactive conditional: `if {signal} { html... } else if {signal} { html... } else { html... }`.
    ///
    /// Each condition is a signal expression in braces. When any signal changes,
    /// the entire conditional is re-evaluated and wrapped in a `DynamicNode`.
    If(HtmlIf),
    /// A reactive match: `match {signal} { pattern => { html... } ... }`.
    ///
    /// The signal expression in braces is re-evaluated on change, and the
    /// matching arm's HTML is rendered inside a `DynamicNode`.
    Match(HtmlMatch),
    /// A reactive for loop: `for pattern in {iterable} { html... }`.
    ///
    /// The pattern is a Rust binding pattern (e.g., `item` or `(index, item)`).
    /// The iterable expression in braces is re-evaluated on signal change,
    /// and each iteration's HTML is collected into a `DynamicNode` fragment.
    For(HtmlFor),
}

/// Represents the value side of an attribute.
///
/// Supports plain expressions, style objects, and reactive conditionals.
pub(crate) enum HtmlAttrValue {
    /// A normal Rust expression.
    Expr(Expr),
    /// A reactive conditional: `if {expr} { value } else if {expr} { value } else { value }`.
    If(HtmlAttrIf),
    /// A style object: {key: value; key2: value2;}
    /// The value can be either a string literal or an expression.
    Style(Vec<(String, HtmlStylePropValue)>),
}

/// Represents a single value in a style property.
///
/// May be a static string literal, a dynamic expression, or a reactive conditional.
pub(crate) enum HtmlStylePropValue {
    /// A static string literal.
    Literal(String),
    /// A dynamic expression.
    Expr(Expr),
    /// A reactive conditional: `if {expr} { value } else if {expr} { value } else { value }`.
    If(HtmlAttrIf),
}
