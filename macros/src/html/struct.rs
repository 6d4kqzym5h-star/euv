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
    #[set(pub(crate))]
    pub(crate) children: Vec<HtmlNode>,
}

/// Represents a reactive `for` loop in HTML.
///
/// Syntax: `for pattern in {expr} { children }`.
///
/// The pattern is a Rust binding pattern (e.g., `item` or `(index, item)`).
/// The expression in braces must evaluate to an iterable. Each iteration's
/// body is rendered as HTML and collected into a `VirtualNode::Fragment`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlFor {
    /// The binding pattern for loop variables (e.g., `item` or `(index, item)`).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pattern: proc_macro2::TokenStream,
    /// The iterable expression (from the braces after `in`).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) iterable: Expr,
    /// The HTML nodes rendered for each iteration.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) body: Vec<HtmlNode>,
}

/// Represents a reactive `if` conditional in HTML.
///
/// Syntax: `if {expr} { children } [else if {expr} { children }]* [else { children }]`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlIf {
    /// The list of condition-branch pairs. Each condition is a braced expression.
    ///
    /// The last entry may have `None` as condition (representing `else`).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) branches: Vec<(Option<Expr>, Vec<HtmlNode>)>,
}

/// Represents a reactive `if` conditional in attribute value position.
///
/// Syntax: `if {expr} { value } [else if {expr} { value }]* [else { value }]`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlAttrIf {
    /// The list of condition-branch pairs. Each condition is a braced expression.
    ///
    /// The last entry may have `None` as condition (representing `else`).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) branches: Vec<(Option<Expr>, Expr)>,
}

/// Represents a reactive `match` expression in HTML.
///
/// Syntax: `match {expr} { pattern => { children } ... }`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct HtmlMatch {
    /// The expression to match against (from the braces after `match`).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scrutinee: Expr,
    /// The match arms: each arm has a pattern as a raw token stream and a body of HTML nodes.
    #[get(pub(crate))]
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
    #[set(pub(crate))]
    pub(crate) tag: Ident,
    /// The actual tag name string, which may differ from the identifier
    /// when using string literal tags for custom HTML5 elements.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tag_name: String,
    /// Whether the tag was parsed from an identifier (not a string literal).
    /// Ident tags are treated as function calls; string literal tags produce
    /// native HTML elements directly.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) is_ident_tag: bool,
    /// The attributes of this element.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) attributes: HtmlAttrs,
    /// The child nodes.
    #[get(pub(crate))]
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
    #[set(pub(crate))]
    pub(crate) tag_expr: Expr,
    /// The attributes passed to the dynamic element.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) attributes: HtmlAttrs,
    /// The child nodes of the dynamic element.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) children: Vec<HtmlNode>,
}

/// Stores metadata about a registered component function.
///
/// Contains the Props type name, the list of field names, and a map of
/// field name to type string declared in the Props struct, used for
/// attribute filtering and type-aware code generation in dynamic tags.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct ComponentInfo {
    /// The Props type name (e.g., "PrimaryButtonProps").
    pub(crate) props_type: String,
    /// The field names declared in the Props struct.
    pub(crate) props_fields: Vec<String>,
    /// Maps field name to its type string (e.g., "children" -> "VirtualNode").
    pub(crate) props_field_types: HashMap<String, String>,
}
