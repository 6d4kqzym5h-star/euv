use crate::*;

/// Represents a single HTML node, which may be an element or text.
///
/// Parsed from the `html!` macro input before code generation.
#[derive(Clone, Debug)]
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
    /// A dynamic tag: `{tag_expr} { attr: value, ... children ... }`.
    ///
    /// The expression in braces evaluates to a tag name string at runtime.
    /// If the tag name matches a registered user component, the component
    /// function is called with the provided attributes and children.
    /// Otherwise, a native HTML element is created.
    DynamicTag(HtmlDynamicTag),
}

/// Represents the key side of an HTML attribute.
///
/// Supports two forms:
/// - `Static`: A compile-time identifier key (e.g., `class`, `onclick`, `"data-id"`).
/// - `Dynamic`: A runtime expression wrapped in braces (e.g., `{key_var}`) that evaluates to a string.
#[derive(Clone, Debug)]
pub(crate) enum HtmlAttrKey {
    /// A static attribute key known at compile time.
    Static(Ident),
    /// A dynamic expression that evaluates to an attribute key string at runtime.
    Dynamic(proc_macro2::TokenStream),
}

/// Represents the value side of an attribute.
///
/// Supports plain expressions, style objects, reactive conditionals,
/// and merged multi-class/multi-style attribute values.
#[derive(Clone, Debug)]
pub(crate) enum HtmlAttrValue {
    /// A normal Rust expression.
    Expr(Expr),
    /// A reactive conditional: `if {expr} { value } else if {expr} { value } else { value }`.
    If(HtmlAttrIf),
    /// A style object: `{key: value; key2: value2;}`.
    ///
    /// The value can be either a string literal or an expression.
    Style(Vec<(String, HtmlStylePropValue)>),
    /// Multiple class attribute values merged from repeated `class:` declarations.
    ///
    /// Each entry is an independent expression (e.g., `c_foo()`, `c_bar()`).
    Classes(Vec<HtmlAttrValue>),
    /// Multiple style attribute values merged from repeated `style:` declarations.
    ///
    /// Each entry is an independent `Style` value.
    Styles(Vec<HtmlAttrValue>),
}

/// Represents a single value in a style property.
///
/// May be a static string literal, a dynamic expression, or a reactive conditional.
#[derive(Clone, Debug)]
pub(crate) enum HtmlStylePropValue {
    /// A static string literal.
    Literal(String),
    /// A dynamic expression.
    Expr(Expr),
    /// A reactive conditional in attribute value position.
    ///
    /// Syntax: `if {expr} { value } [else if {expr} { value }]* [else { value }]`.
    If(HtmlAttrIf),
}
