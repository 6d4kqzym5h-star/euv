use crate::*;

/// Represents the root of an `html!` macro invocation.
///
/// Contains zero or more top-level HTML nodes. The generated output depends
/// on the number of children:
/// - 0 nodes → `VirtualNode::Empty`
/// - 1 node → the node's token stream directly
/// - N nodes → `VirtualNode::Fragment(vec![...])`
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlRoot {
    /// The top-level nodes parsed from the macro input.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) children: Vec<HtmlNode>,
}

/// Represents a reactive `for` loop in HTML.
///
/// Syntax:
/// - `for pattern in {expr} { children }`
/// - `for pattern in expr { children }`
///
/// The pattern is a Rust binding pattern (e.g., `item` or `(index, item)`).
/// The iterable may be wrapped in braces or written as a bare expression.
/// Each iteration's body is rendered as HTML and collected into a
/// `VirtualNode::Fragment`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlFor {
    /// The binding pattern for loop variables (e.g., `item` or `(index, item)`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pattern: proc_macro2::TokenStream,
    /// The iterable expression (from the braces after `in`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) iterable: Expr,
    /// The HTML nodes rendered for each iteration.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) body: Vec<HtmlNode>,
}

/// Represents a conditional in HTML.
///
/// Supports two syntaxes:
/// - Reactive: `if {expr} { children } [else if {expr} { children }]* [else { children }]`
///   The condition expression in braces is treated as a signal that triggers re-rendering.
/// - Inline: `if condition { children } [else if condition { children }]* [else { children }]`
///   The condition is a plain Rust boolean expression, evaluated once at render time.
///   This form is typically used inside `for` loops where the condition depends on loop variables.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlIf {
    /// Whether this conditional is reactive (condition wrapped in braces as a signal).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_reactive: bool,
    /// The list of condition-branch pairs.
    ///
    /// For reactive conditionals, each condition is a braced signal expression.
    /// For inline conditionals, each condition is a plain Rust expression.
    /// The last entry may have `None` as condition (representing `else`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) branches: Vec<(Option<Expr>, Vec<HtmlNode>)>,
}

/// Represents a reactive or inline `if` conditional in attribute value position.
///
/// Supports two syntaxes:
/// - Reactive: `if {expr} { value } [else if {expr} { value }]* [else { value }]`
///   The condition expression in braces is treated as a signal that triggers re-rendering.
/// - Inline: `if condition { value } [else if condition { value }]* [else { value }]`
///   The condition is a plain Rust boolean expression, evaluated once at render time.
///   This form is typically used inside `for` loops where the condition depends on loop variables.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlAttrIf {
    /// Whether this conditional is inline (condition is a plain expression, not a braced signal).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_inline: bool,
    /// The list of condition-branch pairs.
    ///
    /// For reactive conditionals, each condition is a braced signal expression.
    /// For inline conditionals, each condition is a plain Rust expression.
    /// The last entry may have `None` as condition (representing `else`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) branches: Vec<(Option<Expr>, Expr)>,
    /// The default token stream for the implicit else branch when no explicit else exists.
    ///
    /// For `class` attributes, this is an empty string. For `style` attributes,
    /// this is also an empty string. For other attributes, an empty string is used.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) else_default: proc_macro2::TokenStream,
}

/// Represents a reactive or inline `match` expression in attribute value position.
///
/// Supports two syntaxes:
/// - Reactive: `match {expr} { pattern => value, ... }`
///   The expression in braces is treated as a signal that triggers re-rendering.
/// - Inline: `match expr { pattern => value, ... }`
///   The expression is a plain Rust expression, evaluated once at render time.
///   This form is typically used inside `for` loops where the scrutinee depends on loop variables.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlAttrMatch {
    /// Whether this match is inline (scrutinee is a plain expression, not a braced signal).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_inline: bool,
    /// The expression to match against.
    ///
    /// For reactive match, this is a braced signal expression.
    /// For inline match, this is a plain Rust expression.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scrutinee: Expr,
    /// The match arms: each arm has a pattern as a raw token stream and a body expression.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) arms: Vec<(proc_macro2::TokenStream, Expr)>,
}

/// Represents a reactive `match` expression in HTML.
///
/// Syntax: `match {expr} { pattern => { children } ... }`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlMatch {
    /// The expression to match against (from the braces after `match`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scrutinee: Expr,
    /// The match arms: each arm has a pattern as a raw token stream and a body of HTML nodes.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) arms: Vec<(proc_macro2::TokenStream, Vec<HtmlNode>)>,
}

/// Represents an HTML element with a tag name, attributes, and children.
///
/// Stores the parsed structure of an HTML element for token generation.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlElement {
    /// The tag identifier (used for function calls on Ident tags).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tag: Ident,
    /// The actual tag name string, which may differ from the identifier
    /// when using string literal tags for custom HTML5 elements.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tag_name: String,
    /// Whether the tag was parsed from an identifier (not a string literal).
    /// Ident tags are treated as function calls; string literal tags produce
    /// native HTML elements directly.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_ident_tag: bool,
    /// The attributes of this element.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) attributes: HtmlAttrs,
    /// The child nodes.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) children: Vec<HtmlNode>,
}

/// Represents a dynamic tag in HTML.
///
/// Syntax: `{tag_expr} { attr: value, ... children ... }`.
///
/// The expression in braces evaluates to a tag name string at runtime.
/// If the tag name matches a registered user component function, that
/// component is called with the attributes and children. Otherwise,
/// a native HTML element is created.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlDynamicTag {
    /// The expression that evaluates to a tag name string.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tag_expr: Expr,
    /// The attributes passed to the dynamic element.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) attributes: HtmlAttrs,
    /// The child nodes of the dynamic element.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) children: Vec<HtmlNode>,
}

/// Stores metadata about a registered component function.
///
/// Contains the Props type name, the list of field names, and a map of
/// field name to type string declared in the Props struct, used for
/// attribute filtering and type-aware code generation in dynamic tags.
///
/// # Fields
///
/// - `props_type` - The Props type name (e.g., "PrimaryButtonProps").
/// - `props_fields` - The field names declared in the Props struct.
/// - `props_field_types` - Maps field name to its type string (e.g., "children" -> "VirtualNode").
#[derive(Clone, Data, Debug, New, serde::Deserialize, serde::Serialize)]
pub(crate) struct ComponentInfo {
    /// The Props type name (e.g., "PrimaryButtonProps").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) props_type: String,
    /// The field names declared in the Props struct.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) props_fields: Vec<String>,
    /// Maps field name to its type string (e.g., "children" -> "VirtualNode").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) props_field_types: HashMap<String, String>,
}
